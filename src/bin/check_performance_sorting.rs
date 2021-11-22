extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::time::Instant;

use data_structures::sorting::heap_sort::Heap;
use data_structures::sorting::merge_sort;
use data_structures::sorting::quick_sort;

fn main() {
    // let mut arr = [0i64; 10000000];
    let mut arr = Vec::new();
    let mut rng = thread_rng();
    for i in 0..10000000 {
        arr.push(rng.gen_range(0, 100));
    }
    // merge sort
    //  let before = Instant::now();
    /// merge_sort::sort(&mut arr);
    // println!("Elapsed time: {:.1?} for merge sort ", before.elapsed());

    // heap sort

    //let before = Instant::now();
    // let mut heap = Heap::new(&mut arr);
    // heap.sort();
    // println!("Elapsed time: {:.1?} for heap sort ", before.elapsed());
    let before = Instant::now();
    let size = arr.len() - 1;
   // quick_sort::quick_sort(&mut arr, 0, size);
    quick_sort::quick_sort(&mut arr);

    println!("Elapsed time: {:.1?} in quick sort", before.elapsed());
   // println!("array now is {:?}",arr);
}
