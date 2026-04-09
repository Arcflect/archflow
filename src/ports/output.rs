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

/// Default console output adapter.
pub struct StdOutputPort;

impl OutputPort for StdOutputPort {
    fn write_line(&mut self, level: OutputLevel, message: &str) {
        match level {
            OutputLevel::Info => println!("{}", message),
            OutputLevel::Warn => eprintln!("[warn] {}", message),
            OutputLevel::Error => eprintln!("[error] {}", message),
        }
    }
}
