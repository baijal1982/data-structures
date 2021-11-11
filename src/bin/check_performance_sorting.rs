extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::time::Instant;


use data_structures::sorting::merge_sort;

fn main() {
   // let mut arr = [0i64; 10000000];
   let mut arr = Vec::new();
    let mut rng = thread_rng();
    for i in 0..100000000 {
    
       arr.push(rng.gen_range(0, 100));
    }
    let before = Instant::now();
 merge_sort::sort(&mut arr);
  println!("Elapsed time: {:.1?} in my algo", before.elapsed());
 
    
}