#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", " Info".cyan().bold(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{} {}", " Warn".yellow().bold(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{} {}", "Error".red().bold(), format!($($arg)*));
    };
}

use colored::Colorize;
pub trait Logger {
    fn info(&self) -> String;
    fn warn(&self) -> String;
    fn error(&self) -> String;
}

impl Logger for str {
    fn info(&self) -> String {
        return format!("{} {}", " Info".cyan().bold(), &self);
    }
    
    fn warn(&self) -> String {
        return format!("{} {}", " Warn".yellow().bold(), &self);
    }

    fn error(&self) -> String {
        return format!("{} {}", "Error".red().bold(), &self);
    }
}
