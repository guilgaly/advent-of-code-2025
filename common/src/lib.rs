pub use ascii;
pub use itertools;
pub use lazy_static;
use std::fmt::Display;
// pub use lcmx;
pub use maplit;
pub use ndarray;
pub use regex;
pub use sscanf;
pub use sscanf::regex::Regex;

use std::time::Instant;

pub fn time_execution<T>(name: &str, f: impl Fn() -> T) -> T
where
    T: Display,
{
    let before = Instant::now();
    let result = f();
    println!(
        "{} elapsed time: {:.2?}, result: {}",
        name,
        before.elapsed(),
        result
    );
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
