// TODO: Partitioned Convolution / Cabinet Engine
//
// - Add long random-stream test: compare FFT convolution against direct convolution
//   over many blocks, not only one or two blocks.
// - Add tests for different response sizes:
//   1024, 2048, 4096, 8192.
// - Add benchmark against direct convolution:
//   direct 1024/2048/4096 vs FFT 1024/2048/4096/8192.
// - Add null test helper:
//   direct_output - fft_output should be close to zero.
// - Add real cabinet response tests using actual WAV cabinet files.
// - Add denormal protection for very small floating-point values.
// - Replace RingBuffer::get() copy with get_ref() to avoid copying FrequencyBlock.
// - Add quality presets:
//   LowLatency = 1024,
//   Standard = 4096,
//   Studio = 8192.
// - Add PreparedCabinetResponse / PreparedConvolutionData cache,
//   so cabinet responses are not FFT-processed every time the chain is rebuilt.
// - Later: support dynamic block sizes or internal buffering if the audio callback
//   does not always provide exactly BLOCK_SIZE samples.
// - Later: investigate non-uniform partitioned convolution for lower latency.

use std::{array, sync::Arc};

use realfft::{ComplexToReal, RealFftPlanner, RealToComplex, num_complex::Complex32};

use crate::dsp::{BlockProcessingNode, helpers::ring_buffer::RingBuffer};

#[derive(Clone, Copy)]
struct FrequencyBlock<const SPECTRUM_SIZE: usize> {
    spectrum: [Complex32; SPECTRUM_SIZE],
}

impl<const SPECTRUM_SIZE: usize> Default for FrequencyBlock<SPECTRUM_SIZE> {
    fn default() -> Self {
        FrequencyBlock {
            spectrum: [Complex32::default(); SPECTRUM_SIZE],
        }
    }
}
/// Uniform partitioned convolution engine for real-time cabinet response processing.
///
/// Generic parameters:
///
/// - `BLOCK_SIZE`:
///   Number of input audio samples processed per block.
///   This must match the internal DSP block size used by the audio engine.
///   Example: `512`.
///
/// - `RESPONSE_SIZE`:
///   Total length of the prepared cabinet response in samples.
///   This should be divisible by `BLOCK_SIZE`.
///   Example: `4096`.
///
/// - `FFT_SIZE`:
///   Size of the FFT input buffer.
///   For overlap-add partitioned convolution this must be `BLOCK_SIZE * 2`.
///   Example: `1024`.
///
/// - `PARTITION_COUNT`:
///   Number of response partitions.
///   This must be `RESPONSE_SIZE / BLOCK_SIZE`.
///   Example: `4096 / 512 = 8`.
///
/// - `SPECTRUM_SIZE`:
///   Number of complex frequency bins produced by a real FFT.
///   For `realfft`, this must be `FFT_SIZE / 2 + 1`.
///   Example: `1024 / 2 + 1 = 513`.
///
/// The cabinet response is split into `PARTITION_COUNT` chunks of `BLOCK_SIZE`.
/// Each chunk is zero-padded to `FFT_SIZE`, transformed with FFT, and stored
/// as a frequency-domain block. Runtime processing should transform each input
/// block, multiply it with the stored response partitions, sum the spectra,
/// run IFFT, then use overlap-add to produce the final output block.
pub struct PartitionedConvolution<
    const BLOCK_SIZE: usize,      //512
    const RESPONSE_SIZE: usize,   // 4096
    const FFT_SIZE: usize,        // 1024
    const PARTITION_COUNT: usize, // 8
    const SPECTRUM_SIZE: usize,   // 513
> {
    input_history_fft: RingBuffer<PARTITION_COUNT, FrequencyBlock<SPECTRUM_SIZE>>,
    partitions: [FrequencyBlock<SPECTRUM_SIZE>; PARTITION_COUNT],

    input_time_buffer: [f32; FFT_SIZE],
    ifft_time_buffer: [f32; FFT_SIZE],
    overlap: [f32; BLOCK_SIZE],

    input_spectrum: FrequencyBlock<SPECTRUM_SIZE>,
    accumulator_spectrum: FrequencyBlock<SPECTRUM_SIZE>,

    r2c: Arc<dyn RealToComplex<f32>>,
    c2r: Arc<dyn ComplexToReal<f32>>,
}
// quality presets:
// type Cab1k = PartitionedConvolution<512, 1024, 1024, 2, 513>;
// type Cab2k = PartitionedConvolution<512, 2048, 1024, 4, 513>;
// type Cab4k = PartitionedConvolution<512, 4096, 1024, 8, 513>;
// type Cab8k = PartitionedConvolution<512, 8192, 1024, 16, 513>;
impl<
    const BLOCK_SIZE: usize,
    const RESPONSE_SIZE: usize,
    const FFT_SIZE: usize,
    const PARTITION_COUNT: usize,
    const SPECTRUM_SIZE: usize,
> PartitionedConvolution<BLOCK_SIZE, RESPONSE_SIZE, FFT_SIZE, PARTITION_COUNT, SPECTRUM_SIZE>
{
    pub fn new(response: [f32; RESPONSE_SIZE]) -> Self {
        assert_eq!(
            RESPONSE_SIZE % BLOCK_SIZE,
            0,
            "RESPONSE_SIZE must be divisible by BLOCK_SIZE"
        );

        assert_eq!(
            RESPONSE_SIZE / BLOCK_SIZE,
            PARTITION_COUNT,
            "PARTITION_COUNT must equal RESPONSE_SIZE / BLOCK_SIZE"
        );

        assert_eq!(
            FFT_SIZE,
            BLOCK_SIZE * 2,
            "FFT_SIZE must equal BLOCK_SIZE * 2"
        );

        assert_eq!(
            SPECTRUM_SIZE,
            FFT_SIZE / 2 + 1,
            "For real FFT, SPECTRUM_SIZE must equal FFT_SIZE / 2 + 1"
        );

        let mut planner = RealFftPlanner::<f32>::new();
        let r2c = planner.plan_fft_forward(FFT_SIZE);
        let c2r = planner.plan_fft_inverse(FFT_SIZE);

        let partitions: [FrequencyBlock<SPECTRUM_SIZE>; PARTITION_COUNT] =
            array::from_fn(|partition_index| {
                let start = partition_index * BLOCK_SIZE;
                let end = start + BLOCK_SIZE;
                let chunk = &response[start..end];

                let mut time_buffer = r2c.make_input_vec();
                time_buffer[..BLOCK_SIZE].copy_from_slice(chunk);
                time_buffer[BLOCK_SIZE..].fill(0.0);

                let mut spectrum = r2c.make_output_vec();
                r2c.process(&mut time_buffer, &mut spectrum).unwrap();

                let mut spectrum_arr = [Complex32::default(); SPECTRUM_SIZE];

                spectrum_arr.copy_from_slice(&spectrum);

                FrequencyBlock {
                    spectrum: spectrum_arr,
                }
            });

        PartitionedConvolution {
            partitions,
            input_history_fft: RingBuffer::new(),
            overlap: [0.0; BLOCK_SIZE],
            r2c,
            c2r,
            input_time_buffer: [0.0; FFT_SIZE],
            ifft_time_buffer: [0.0; FFT_SIZE],
            input_spectrum: FrequencyBlock {
                spectrum: [Complex32::default(); SPECTRUM_SIZE],
            },
            accumulator_spectrum: FrequencyBlock {
                spectrum: [Complex32::default(); SPECTRUM_SIZE],
            },
        }
    }
}

impl<
    const BLOCK_SIZE: usize,
    const RESPONSE_SIZE: usize,
    const FFT_SIZE: usize,
    const PARTITION_COUNT: usize,
    const SPECTRUM_SIZE: usize,
> BlockProcessingNode
    for PartitionedConvolution<BLOCK_SIZE, RESPONSE_SIZE, FFT_SIZE, PARTITION_COUNT, SPECTRUM_SIZE>
{
    fn process_block(&mut self, samples: &mut [f32]) {
        debug_assert_eq!(
            samples.len(),
            BLOCK_SIZE,
            "PartitionedConvolution expects fixed block size"
        );

        self.input_time_buffer[..BLOCK_SIZE].copy_from_slice(samples);
        self.input_time_buffer[BLOCK_SIZE..].fill(0.0);

        self.r2c
            .process(
                &mut self.input_time_buffer,
                &mut self.input_spectrum.spectrum,
            )
            .unwrap();

        self.input_history_fft.push(self.input_spectrum);

        self.accumulator_spectrum
            .spectrum
            .fill(Complex32::default());

        for partition_index in 0..PARTITION_COUNT {
            let input_block = self.input_history_fft.get_ref(partition_index);
            let partition = &self.partitions[partition_index];

            for bin_index in 0..SPECTRUM_SIZE {
                self.accumulator_spectrum.spectrum[bin_index] +=
                    partition.spectrum[bin_index] * input_block.spectrum[bin_index];
            }
        }

        self.c2r
            .process(
                &mut self.accumulator_spectrum.spectrum,
                &mut self.ifft_time_buffer,
            )
            .unwrap();

        let inverse_scale = 1.0 / FFT_SIZE as f32;

        for i in 0..FFT_SIZE {
            self.ifft_time_buffer[i] *= inverse_scale;

            if i < BLOCK_SIZE {
                self.ifft_time_buffer[i] += self.overlap[i]
            }
        }

        samples.copy_from_slice(&self.ifft_time_buffer[..BLOCK_SIZE]);
        self.overlap
            .copy_from_slice(&self.ifft_time_buffer[BLOCK_SIZE..]);
    }
}

// Підготовка cabinet response:
// 1. Взяти cabinet response.
// 2. Розбити на підмасиви по 512.
// 3. Кожен підмасив доповнити нулями до 1024.
// 4. Застосувати FFT до кожного.
// 5. Зберегти FFT partitions.

// Runtime:
// 1. Приходить audio block 512.
// 2. Доповнити його нулями до 1024.
// 3. Застосувати FFT.//
// 4. Зберегти FFT audio block в history.
// 5. Для кожної partition:
//    current/previous audio FFT × відповідний cabinet FFT.
// 6. Скласти всі frequency результати.
// 7. Застосувати IFFT.
// 8. Перші 512 samples + overlap → output.
// 9. Останні 512 samples → overlap buffer.
// 10. Без алокацій у real-time.

#[cfg(test)]
#[path = "partitioned_convolution_test.rs"]
mod partitioned_convolution_test;
