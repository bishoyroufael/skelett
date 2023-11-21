use colored::*;

pub fn err(msg: &str) -> String {
    return format!("[err] {}", msg).red().to_string();
}

pub fn info(msg: &str) -> String {
    return format!("[info] {}", msg).yellow().to_string();
}
