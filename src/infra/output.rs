pub struct ConsoleOutputAdapter;

impl crate::ports::OutputPort for ConsoleOutputAdapter {
    fn write_line(&mut self, level: crate::ports::OutputLevel, message: &str) {
        match level {
            crate::ports::OutputLevel::Info => println!("{}", message),
            crate::ports::OutputLevel::Warn => eprintln!("[warn] {}", message),
            crate::ports::OutputLevel::Error => eprintln!("[error] {}", message),
        }
    }
}
