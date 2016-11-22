use std::fmt::Display;
use std::process;

pub fn or_exit<T, E: Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}

pub fn is_sorted_uniq<E>(r: &[E]) -> bool
    where E: PartialOrd
{
    for i in 1..r.len() {
        if r[i - 1] >= r[i] {
            return false;
        }
    }

    return true;
}