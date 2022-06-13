mod algo;

use crate::algo::{binary_search,wide_search};

fn main() {

    let vec = vec![1, 2, 3, 4];

    let x = wide_search::search(&vec, 4);

    match x {
        None => println!("None found"),
        Some(_x) => println!("Some found"),
    }

}
