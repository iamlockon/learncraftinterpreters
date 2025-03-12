use crate::scanner::Scanner;

#[derive(Debug, Default)]
pub struct Runner {
    had_error: bool,
}


impl Runner {
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn run_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let source = std::fs::read_to_string(path).expect
            ("Failed to read file");
        self.run(&source)?;
        Ok(()) 
    }
    
    pub(crate) fn run_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut line = String::with_capacity(256);
    
        loop {
            std::io::stdin().read_line(&mut line)?;
            if line.is_empty() {
                break;
            }
            self.run(&line)?;
            line.clear();
            self.had_error = false;
        }
        Ok(())
    }
    
    fn run(&self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        for ref token in tokens {
            println!("{token:?}");
        }
        Ok(())
    }
    
    pub(crate) fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }
    
    fn report(line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
    }
}
