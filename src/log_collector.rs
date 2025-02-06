pub struct LogCollector {
    pub messages: Vec<String>,
    pub bytes_written: usize,
    pub bytes_limit: Option<usize>,
    pub limit_warning: bool,
}