use std::sync::{Arc, atomic::AtomicU64};

pub struct AudioStat {
    /// Input callback produced a sample,
    /// but the input ring buffer was already full.
    ///
    /// This usually means the DSP/output side cannot
    /// consume incoming audio fast enough.
    pub input_overflow_count: Arc<AtomicU64>,

    /// DSP/output callback requested an input sample,
    /// but the input ring buffer was empty.
    ///
    /// A zero sample is substituted.
    /// May indicate input starvation or device drift.
    pub input_underrun_count: Arc<AtomicU64>,

    /// DSP produced a processed audio block,
    /// but the output block queue was already full.
    ///
    /// The processed block is dropped.
    /// May indicate output is consuming audio too slowly.
    pub output_overflow_count: Arc<AtomicU64>,

    /// Output callback requested a processed sample,
    /// but no processed audio block was available.
    ///
    /// A zero sample is substituted.
    /// Usually results in audible clicks or dropouts.
    pub output_underrun_count: Arc<AtomicU64>,

    /// Total amount of DSP blocks successfully processed.
    ///
    /// Useful for profiling and calculating runtime statistics.
    pub processed_block_count: Arc<AtomicU64>,

    /// Maximum amount of pending input samples observed
    /// since the stream was started.
    pub max_input_buffer_fill: Arc<AtomicU64>,

    /// Maximum amount of pending processed output samples observed
    /// since the stream was started.
    pub max_output_buffer_fill: Arc<AtomicU64>,

    /// Number of output blocks successfully queued
    /// after DSP processing.
    pub output_block_count: Arc<AtomicU64>,
}

impl AudioStat {
    pub fn new() -> Self {
        AudioStat {
            input_overflow_count: Arc::new(AtomicU64::new(0)),
            input_underrun_count: Arc::new(AtomicU64::new(0)),
            output_overflow_count: Arc::new(AtomicU64::new(0)),
            output_underrun_count: Arc::new(AtomicU64::new(0)),
            processed_block_count: Arc::new(AtomicU64::new(0)),
            max_input_buffer_fill: Arc::new(AtomicU64::new(0)),
            output_block_count: Arc::new(AtomicU64::new(0)),
            max_output_buffer_fill: Arc::new(AtomicU64::new(0)),
        }
    }
}
