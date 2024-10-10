#[derive(Clone)]
pub struct ModelOptions {
    pub n_gpu_layers: u32,
}

impl Default for ModelOptions {
    fn default() -> Self {
        Self { n_gpu_layers: 1000 }
    }
}

#[derive(Clone)]
pub struct ContextOptions {
    pub context_size: u32,
    pub seed: u32,
    pub n_threads: i32,
    pub n_threads_batch: i32,
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            context_size: 2048,
            seed: 0,
            n_threads: (num_cpus::get() - 1) as i32,
            n_threads_batch: (num_cpus::get() / 2) as i32,
        }
    }
}

#[derive(Clone)]
pub struct RunOptions {
    pub n_batch: usize,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self { n_batch: 512 }
    }
}
