use std::default::Default;
use std::ops::AddAssign;

// Note: the canonical way to sum a slice is:
//
// arr.iter().sum()

pub fn sum_slice<T>(arr: &[T]) -> T where T: AddAssign + Copy + Default {
    let mut sum = Default::default();

    for v in arr {
        sum += *v
    }

    sum
}

pub fn sum_slice_boxed<T>(arr: &[Box<T>]) -> T where T: AddAssign + Copy + Default {
    let mut sum = Default::default();

    for v in arr {
        sum += **v
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_slice_works() {
        let arr = &[1,2,3,4,5];
        assert_eq!(sum_slice(arr), 15);
    }

    #[test]
    fn sum_slice_boxed_works() {
        let arr = &[Box::new(1),Box::new(2),Box::new(3),Box::new(4),Box::new(5)];
        assert_eq!(sum_slice_boxed(arr), 15);
    }
}
