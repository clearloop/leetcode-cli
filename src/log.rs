#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        use colored::Colorize;
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
