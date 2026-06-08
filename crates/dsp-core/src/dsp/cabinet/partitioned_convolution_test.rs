//cargo test -p dsp-core partitioned_convolution -- --nocapture

#[cfg(test)]
mod tests {
    use crate::dsp::{
        BlockProcessingNode, cabinet::partitioned_convolution::PartitionedConvolution,
    };

    const BLOCK_SIZE: usize = 512;
    const RESPONSE_SIZE: usize = 4096;
    const FFT_SIZE: usize = 1024;
    const PARTITION_COUNT: usize = 8;
    const SPECTRUM_SIZE: usize = 513;

    #[test]
    fn impulse_response_returns_impulse_block() {
        let mut response = [0.0; 4096];
        response[0] = 1.0;

        let mut convolution = PartitionedConvolution::<512, 4096, 1024, 8, 513>::new(response);

        let mut samples = [0.0; 512];
        samples[0] = 1.0;

        convolution.process_block(&mut samples);

        println!("samples[0] = {}", samples[0]);
        println!("samples[1..8] = {:?}", &samples[1..8]);

        assert!(
            (samples[0] - 1.0).abs() < 1.0e-5,
            "expected samples[0] to be near 1.0, got {}",
            samples[0]
        );

        for (index, sample) in samples[1..].iter().enumerate() {
            assert!(
                sample.abs() < 1.0e-5,
                "expected samples[{}] to be near 0.0, got {}",
                index + 1,
                sample
            );
        }
    }

    #[test]
    fn delayed_impulse_response_delays_signal() {
        let mut response = [0.0; 4096];
        response[100] = 1.0;

        let mut convolution = PartitionedConvolution::<512, 4096, 1024, 8, 513>::new(response);

        let mut samples = [0.0; 512];
        samples[0] = 1.0;

        convolution.process_block(&mut samples);

        assert!((samples[100] - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn delayed_impulse_across_blocks_uses_overlap_and_history() {
        let mut response = [0.0; 4096];
        response[700] = 1.0;

        let mut convolution = PartitionedConvolution::<512, 4096, 1024, 8, 513>::new(response);

        let mut block_0 = [0.0; 512];
        block_0[0] = 1.0;

        let mut block_1 = [0.0; 512];

        convolution.process_block(&mut block_0);
        convolution.process_block(&mut block_1);

        assert!((block_1[188] - 1.0).abs() < 1.0e-5);
    }

    fn pseudo_random(seed: &mut u32) -> f32 {
        *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);

        let normalized = (*seed as f32) / (u32::MAX as f32);
        normalized * 2.0 - 1.0
    }

    fn direct_convolution(input: &[f32], response: &[f32]) -> Vec<f32> {
        let mut output = vec![0.0; input.len() + response.len() - 1];

        for input_index in 0..input.len() {
            for response_index in 0..response.len() {
                output[input_index + response_index] +=
                    input[input_index] * response[response_index];
            }
        }

        output
    }

    #[test]
    fn partitioned_convolution_matches_direct_convolution_for_random_single_block() {
        let mut seed = 12345;

        let mut response = [0.0; RESPONSE_SIZE];
        for sample in &mut response {
            *sample = pseudo_random(&mut seed) * 0.1;
        }

        let mut input_block = [0.0; BLOCK_SIZE];
        for sample in &mut input_block {
            *sample = pseudo_random(&mut seed) * 0.1;
        }

        let expected = direct_convolution(&input_block, &response);

        let mut convolution = PartitionedConvolution::<
            BLOCK_SIZE,
            RESPONSE_SIZE,
            FFT_SIZE,
            PARTITION_COUNT,
            SPECTRUM_SIZE,
        >::new(response);

        convolution.process_block(&mut input_block);

        for i in 0..BLOCK_SIZE {
            assert!(
                (input_block[i] - expected[i]).abs() < 1.0e-4,
                "Mismatch at sample {i}: expected {}, got {}, diff {}",
                expected[i],
                input_block[i],
                (input_block[i] - expected[i]).abs()
            );
        }
    }

    #[test]
    fn partitioned_convolution_matches_direct_convolution_across_multiple_blocks() {
        let mut seed = 54321;

        let mut response = [0.0; RESPONSE_SIZE];
        for sample in &mut response {
            *sample = pseudo_random(&mut seed) * 0.1;
        }

        let block_count = 12;
        let input_len = BLOCK_SIZE * block_count;

        let mut full_input = vec![0.0; input_len];

        for sample in &mut full_input {
            *sample = pseudo_random(&mut seed) * 0.1;
        }

        let expected = direct_convolution(&full_input, &response);

        let mut convolution = PartitionedConvolution::<
            BLOCK_SIZE,
            RESPONSE_SIZE,
            FFT_SIZE,
            PARTITION_COUNT,
            SPECTRUM_SIZE,
        >::new(response);

        let mut actual = Vec::with_capacity(input_len);

        for block in full_input.chunks_exact(BLOCK_SIZE) {
            let mut block_array = [0.0; BLOCK_SIZE];
            block_array.copy_from_slice(block);

            convolution.process_block(&mut block_array);

            actual.extend_from_slice(&block_array);
        }

        for i in 0..actual.len() {
            assert!(
                (actual[i] - expected[i]).abs() < 1.0e-4,
                "Mismatch at sample {i}: expected {}, got {}, diff {}",
                expected[i],
                actual[i],
                (actual[i] - expected[i]).abs()
            );
        }
    }
}
