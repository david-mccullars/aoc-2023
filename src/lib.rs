pub mod template;

use regex::Captures;

#[macro_export]
macro_rules! regex {
    ($name:ident, $e:expr) => {
        lazy_static::lazy_static! {
            static ref $name: regex::Regex = regex::Regex::new($e).unwrap_or_else(|e| {
                eprintln!("Failed to parse regex: {}", e);
                std::process::exit(1);
            });
        }
    };
}

#[allow(dead_code)]
pub fn name_to_digit(s: &str) -> Option<u32> {
    match s {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn str_to_vec<T: std::str::FromStr + Clone>(s: &str) -> Vec<T> {
    s.split_whitespace()
        .map(|s| match s.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Failed to parse string: {}", s);
                std::process::exit(1);
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn capture_to_vec<T: std::str::FromStr + Clone>(captures: &Captures, group: usize) -> Vec<T> {
    match captures.get(group) {
        Some(val) => str_to_vec(val.as_str()),
        None => {
            eprintln!("No such capture group: {:?}", captures);
            std::process::exit(1);
        }
    }
}
