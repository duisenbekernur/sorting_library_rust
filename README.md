# Sorting Library Rust

## Usage

This Sorting Rust library provides sorting algorithms:

- Quick Sort
- Selection Sort
- Insertion Sort
- Merge Sort

![img.png](src/images/main.png)
![img_1.png](src/images/res.png)

To use the sorting algorithms provided by this library, simply import the functions into your Rust project and call them as needed.

```rust
use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};
```

## Examples

Here are some examples demonstrating the usage of sorting algorithms provided by this library:

### Sorting a vector of integers

```rust
use sorting_library::{quick_sort};

fn main() {
    let mut numbers = vec![1, 5, 9, 2, 25, 3, 1];
    quick_sort(&mut numbers);
    println!("Quick Sorted: {:?}", numbers);
}
```

### Sorting a vector of strings

```rust
use sorting_library::{selection_sort};

fn main() {
    let mut strings = vec!["john", "almat", "askar", "ten", "eleven"];
    selection_sort(&mut strings);
    println!("Sorted Strings: {:?}", strings);
}
```

### Sorting a vector of floats

```rust
use sorting_library::{insertion_sort};

fn main() {
    let floats = vec![2.5, 3.6, 1.2, 4.4, 1.1];
    let mut as_f64: Vec<f64> = floats.iter().map(|&x| x as f64).collect();
    insertion_sort(&mut as_f64);
    println!("Insertion Sorted: {:?}", as_f64);
}
```

### Sorting a vector of characters

```rust
use sorting_library::{merge_sort};

fn main() {
    let mut chars = vec!['k', 'u', 'a', 'w', 'f'];
    merge_sort(&mut chars);
    println!("Merge Sorted: {:?}", chars);
}
```

