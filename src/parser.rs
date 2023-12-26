#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[macro_export]
macro_rules! parser {
    // For single expression, just return the type
    ($func:ident, $regex:expr, $rule:expr) => {
        paste::item! {
            regex!([<parse_regex_ $func>], $regex);

            fn [<parse_ $func>](input: &str) -> Option<$func> {
                if let Some(mat) = [<parse_regex_ $func>].captures(input) {
                    let mut groups = mat.iter().skip(1);
                    let value = groups.next().flatten().map(|g| g.as_str()).map($rule).unwrap_or_else(|| {
                        eprintln!("Expected more capture groups for {}", stringify!($regex).to_string());
                        std::process::exit(1);
                    });
                    Some(value)
                } else {
                    None
                }
            }
        }
    };

    // For multiple expressions, return a tuple
    ($func:ident, $regex:expr, $($rule:expr),+) => {
        paste::item! {
            regex!([<parse_regex_ $func>], $regex);

            fn [<parse_ $func>](input: &str) -> Option<$func> {
                if let Some(mat) = [<parse_regex_ $func>].captures(input) {
                    let mut groups = mat.iter().skip(1);
                    Some((
                        $(groups.next().flatten().map(|g| g.as_str()).map($rule).unwrap_or_else(|| {
                            eprintln!("Expected more capture groups for {}", stringify!($regex).to_string());
                            std::process::exit(1);
                        }),)*
                    ))
                } else {
                    None
                }
            }
        }
    };
}

#[macro_export]
macro_rules! str_parser {
    () => {
        |g| g
    };
}

#[macro_export]
macro_rules! string_parser {
    () => {
        |g| g.to_string()
    };
}

#[macro_export]
macro_rules! char_parser {
    () => {
        |g| g.chars().next().unwrap()
    };
}

#[macro_export]
macro_rules! u8_parser {
    () => {
        |g| g.parse::<u8>().unwrap()
    };
}

#[macro_export]
macro_rules! u16_parser {
    () => {
        |g| g.parse::<u16>().unwrap()
    };
}

#[macro_export]
macro_rules! u32_parser {
    () => {
        |g| g.parse::<u32>().unwrap()
    };
}

#[macro_export]
macro_rules! usize_parser {
    () => {
        |g| g.parse::<usize>().unwrap()
    };
}

#[macro_export]
macro_rules! u64_parser {
    () => {
        |g| g.parse::<u64>().unwrap()
    };
}

#[macro_export]
macro_rules! i8_parser {
    () => {
        |g| g.parse::<i8>().unwrap()
    };
}

#[macro_export]
macro_rules! i16_parser {
    () => {
        |g| g.parse::<i16>().unwrap()
    };
}

#[macro_export]
macro_rules! i32_parser {
    () => {
        |g| g.parse::<i32>().unwrap()
    };
}

#[macro_export]
macro_rules! isize_parser {
    () => {
        |g| g.parse::<isize>().unwrap()
    };
}

#[macro_export]
macro_rules! i64_parser {
    () => {
        |g| g.parse::<i64>().unwrap()
    };
}

#[macro_export]
macro_rules! f8_parser {
    () => {
        |g| g.parse::<f8>().unwrap()
    };
}

#[macro_export]
macro_rules! f16_parser {
    () => {
        |g| g.parse::<f16>().unwrap()
    };
}

#[macro_export]
macro_rules! f32_parser {
    () => {
        |g| g.parse::<f32>().unwrap()
    };
}

#[macro_export]
macro_rules! f64_parser {
    () => {
        |g| g.parse::<f64>().unwrap()
    };
}

#[macro_export]
macro_rules! list_parser {
    ($expr:expr) => {
        |g| g.split(",").map($expr).collect()
    };

    ($expr:expr, $delim:expr) => {
        |g| g.split($delim).map($expr).collect()
    };
}
