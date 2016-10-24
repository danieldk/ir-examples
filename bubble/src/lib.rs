#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub fn bubble_sort0<E: Ord>(arr: &mut [E]) {
    loop {
        let mut sorted = true;

        for i in 1..arr.len() {
            if arr[i] < arr[i - 1] {
                arr.swap(i - 1, i);
                sorted = false;
            }
        }

        if sorted {
            break;
        }
    }
}

pub fn bubble_sort<E: Ord>(arr: &mut [E]) {
    let mut n = arr.len();

    loop {
        let mut sorted = true;

        for i in 1..n {
            if arr[i] < arr[i-1] {
                arr.swap(i - 1, i);
                sorted = false;
            }
        }

        if sorted {
            break;
        }

        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    quickcheck!{
        fn prop_bubblesort0_sorted(xs: Vec<usize>) -> bool {
            let mut bubble_sorted = xs.clone();
            bubble_sort0(&mut bubble_sorted);

            is_sorted(&bubble_sorted)
        }
    }

    quickcheck!{
        fn prop_bubblesort0_equal_vecsort(xs: Vec<usize>) -> bool {
            let mut sorted = xs.clone();
            sorted.sort();

            let mut bubble_sorted = xs.clone();
            bubble_sort0(&mut bubble_sorted);

            sorted == bubble_sorted
        }
    }

    quickcheck!{
        fn prop_bubblesort_sorted(xs: Vec<usize>) -> bool {
            let mut bubble_sorted = xs.clone();
            bubble_sort(&mut bubble_sorted);

            is_sorted(&bubble_sorted)
        }
    }

    quickcheck!{
        fn prop_bubblesort_equal_vecsort(xs: Vec<usize>) -> bool {
            let mut sorted = xs.clone();
            sorted.sort();

            let mut bubble_sorted = xs.clone();
            bubble_sort(&mut bubble_sorted);

            sorted == bubble_sorted
        }
    }

    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        for pair in arr.windows(2) {
            if pair[0] > pair[1] {
                return false;
            }
        }

        true
    }
}
