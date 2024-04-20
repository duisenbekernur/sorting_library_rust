mod sorting;

use sorting::sorting::*;

fn main() {
    let mut numbers = vec![1, 5, 9, 2, 25, 3, 1];
    quick_sort(&mut numbers);
    println!("Quick Sort: {:?}", numbers);

    let mut strings = vec!["john", "almat", "askar", "ten", "eleven"];
    selection_sort(&mut strings);
    println!("Selection Sort: {:?}", strings);

    let floats = vec![2.5, 3.6, 1.2, 4.4, 1.1];
    let mut as_f64: Vec<f64> = floats.iter().map(|&x| x as f64).collect();
    insertion_sort(&mut as_f64);
    println!("Insertion Sorted: {:?}", as_f64);

    let mut chars = vec!['k', 'u', 'a', 'w', 'f'];
    merge_sort(&mut chars);
    println!("Merge Sorted: {:?}", chars);
}
