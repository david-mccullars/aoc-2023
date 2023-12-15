pub mod template;

use num::Num;
use regex::Captures;
use std::fmt::Debug;
use std::fmt::Display;

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

// xs:      🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
// ys[0]:                                                 ⬜⬜⬜⬜⬜
//          🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦
// ys[1]:                                                                           ⬜⬜⬜⬜⬜⬜⬜⬜⬜
//          🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[2]:                                                                                            ⬜⬜
//          🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[3]:                 ⬜
//          🟩🟩🟩        🟧🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[4]:                             ⬜⬜⬜
//          🟩🟩🟩        🟧🟫🟫🟫🟫🟫🟪🟪🟪🟨🟨🟨              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[5]: ⬜⬜⬜⬜⬜⬜
//          🟩🟩🟩        🟦🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
#[allow(dead_code)]
pub fn range_split<T: Num + Ord + Copy + Debug + Display>(
    xs: Vec<(T, T)>,
    ys: Vec<(T, T)>,
) -> Vec<(T, T)> {
    let mut changed = xs;
    for y in ys {
        changed = changed.iter().flat_map(|x| range_split1(x, &y)).collect();
    }
    changed
}

#[allow(dead_code)]
#[inline]
pub fn range_split1<T: Num + Ord + Copy + Display>(x: &(T, T), y: &(T, T)) -> Vec<(T, T)> {
    //     |  x  |                      |  x  |
    //              |  y  |         |     y       |
    //     |     |                      |     |
    if (x.1 <= y.0 || y.1 <= x.0) || (y.0 <= x.0 && x.1 <= y.1) {
        vec![*x]

    //     |      x      |
    //         |  y  |
    //     |   |     |   |
    } else if x.0 < y.0 && y.1 < x.1 {
        vec![(x.0, y.0), (y.0, y.1), (y.1, x.1)]

    //     |  x  |
    //  |  y  |
    //     |  |  |
    } else if y.0 <= x.0 {
        vec![(x.0, y.1), (y.1, x.1)]

    //     |  x  |
    //        |  y  |
    //     |  |  |
    } else if x.0 <= y.0 {
        vec![(x.0, y.0), (y.0, x.1)]
    } else {
        eprintln!(
            "Unexpected ranges: ({}, {}) and ({}, {})",
            x.0, x.1, y.0, y.1
        );
        std::process::exit(1);
    }
}

#[allow(dead_code)]
#[inline]
pub fn range_overlap1<T: Num + Ord + Copy + Display>(x: &(T, T), y: &(T, T)) -> bool {
    x.0 < y.1 && y.0 < x.1
}
