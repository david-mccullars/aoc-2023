pub mod template;

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
