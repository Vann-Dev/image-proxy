use colored::Colorize;

pub enum Mode {
    Info,
    Warn,
    Danger,
}

pub fn log(mode: Mode, value: String) {
    match mode {
        Mode::Info => println!("{} {}", "[Info]".green(), value),
        Mode::Warn => println!("{} {}", "[Warn]".yellow(), value),
        Mode::Danger => println!("{} {}", "[Danger]".red(), value),
    }
}
