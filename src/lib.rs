use std::time::{Duration, Instant};

/*
* Based on pattern-defeating quicksort by Orson Peters:
* https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable
*/
pub fn built_in_sort(mut list:Vec<u32>) -> Duration {
    let start = Instant::now();
    list.sort_unstable();
    start.elapsed()
}

/*
* Based on the optimizing bubble sort algorithm:
* https://en.wikipedia.org/wiki/Bubble_sort#Optimizing_bubble_sort
*/
pub fn bubble_sort(mut list: Vec<u32>) -> Duration {
    let start = Instant::now();
    let mut change = true;
    let mut n = list.len();

    while change {
        change = false;
        for i in 1..n {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                change = true;
            }
        }
        n = n - 1;
    }
    start.elapsed()
}

/*
* Based on the selection sort algorithm:
* https://en.wikipedia.org/wiki/Selection_sort
*/
pub fn selection_sort(mut list: Vec<u32>) -> Duration {
    let start = Instant::now();

    for i in 0..list.len() - 1 {
        let mut minimum = i;
        for j in i..list.len() {
            if list[j] < list[minimum] {
                minimum = j;
            }
        }
        if i != minimum {
            list.swap(i, minimum);
        }
    }
    start.elapsed()
}

/*
* Based on the quicksort algorithm:
* https://en.wikipedia.org/wiki/Quicksort#Algorithm
*/
pub fn quicksort(mut list: Vec<u32>) -> Duration {
    let start = Instant::now();
    let end = list.len() - 1;
    quicksort_algorithm(list.as_mut_slice(), 0, end);
    start.elapsed()
}

fn quicksort_algorithm(list: &mut [u32], start: usize, end: usize) {
    if start >= end { return }

    let pivot = partition(list.as_mut() , start, end);
    quicksort_algorithm(list.as_mut(), start, pivot - 1);
    quicksort_algorithm(list.as_mut(), pivot + 1, end);
}

fn partition(list: &mut [u32], start: usize, end: usize) -> usize {
    let (mut pivot, mut pointer) = (start, start);

    while pointer < end {
        if list[pointer] <= list[end] {
            list.swap(pivot, pointer);
            pivot += 1;
        }
        pointer += 1
    }
    list.swap(pivot, end);
    pivot
}
