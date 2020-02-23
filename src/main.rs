use rand::Rng;
mod lib;

fn main() {
    let rand_vector = generate_random_numbers(10000);
    let built_in_time = lib::built_in_sort(rand_vector.clone());
    let bubble_time = lib::bubble_sort(rand_vector.clone());
    let selection_time = lib::selection_sort(rand_vector.clone());
    let quicksort_time = lib::quicksort(rand_vector.clone());

    println!("Built-in sort: {} ms", built_in_time.as_millis());
    println!("Bubble sort: {} ms", bubble_time.as_millis());
    println!("Selection sort: {} ms", selection_time.as_millis());
    println!("Quick sort: {} ms", quicksort_time.as_millis());
}

fn generate_random_numbers(size: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(0, 100))
        .collect()
}
