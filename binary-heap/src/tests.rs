use super::{BinaryHeap, heapsort};

quickcheck! {
    fn from_vector_peek_and_pop_prop(xs: Vec<u8>) -> bool {
        let mut sorted = xs.clone();
        sorted.sort();
        let heap = BinaryHeap::from(xs);
        check_peek_pop(sorted, heap)
    }
}

quickcheck! {
    fn into_sorted_vec_is_sorted_prop(xs: Vec<u8>) -> bool {
        let mut sorted = xs.clone();
        sorted.sort();

        let heap_sorted = heapsort(xs);

        heap_sorted == sorted
    }
}

quickcheck! {
    fn insert_peek_and_pop_prop(xs: Vec<u8>) -> bool {
        let mut sorted = xs.clone();
        sorted.sort();

        let mut heap = BinaryHeap::new();
        for v in xs {
            heap.insert(v);
        }

        check_peek_pop(sorted, heap)
    }
}

#[test]
fn peek_mut_restores_heap_prop() {
    let data = vec![10, 2, 3, 8, 3, 15];
    let check = vec![1, 2, 3, 3, 8, 10];

    let mut heap = BinaryHeap::from(data);

    if let Some(mut largest) = heap.peek_mut() {
        *largest = 1;
    }

    assert_eq!(heap.into_sorted_vec(), check)
}

fn check_peek_pop(mut sorted: Vec<u8>, mut heap: BinaryHeap<u8>) -> bool {
    while !sorted.is_empty() {
        if heap.peek().unwrap() != sorted.last().unwrap() {
            return false;
        }

        if heap.pop().unwrap() != sorted.pop().unwrap() {
            return false;
        }
    }

    heap.is_empty()
}
