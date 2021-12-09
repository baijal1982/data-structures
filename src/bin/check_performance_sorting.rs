extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::time::Instant;

use data_structures::sorting::heap_sort::Heap;
use data_structures::sorting::merge_sort;
use data_structures::sorting::quick_sort;
use data_structures::sorting::selection_sort;
use data_structures::sorting::insertion_sort;

fn main() {
    let mut arr = Vec::new();
    let mut rng = thread_rng();
    for i in 0..100000 {
        arr.push(rng.gen_range(0, 100));
    }


    //insertion_sort 

        
    let before = Instant::now();
    let size = arr.len();
   insertion_sort::sort( &mut arr);
    println!("Elapsed time: {:.1?} in insertion sort", before.elapsed());
    
    // selection  sort
    
    let before = Instant::now();
    let size = arr.len();
   selection_sort::sort(&mut arr,size);
    println!("Elapsed time: {:.1?} in selection sort", before.elapsed());

    // merge sort
    let before = Instant::now();
    merge_sort::sort(&mut arr);
    println!("Elapsed time: {:.1?} for merge sort ", before.elapsed());

    // heap sort

    let before = Instant::now();
    let mut heap = Heap::new(&mut arr);
     heap.sort();
    println!("Elapsed time: {:.1?} for heap sort ", before.elapsed());


    //quick sort 

    let before = Instant::now();
    let size = arr.len() - 1;
    quick_sort::quick_sort(&mut arr);
    println!("Elapsed time: {:.1?} in quick sort", before.elapsed());



}
