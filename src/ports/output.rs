/// Output severity level for user-facing rendering.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputLevel {
    Info,
    Warn,
    Error,
}

/// Output capability boundary used by app workflows.
#[allow(dead_code)]
pub trait OutputPort {
    fn write_line(&mut self, level: OutputLevel, message: &str);
}
