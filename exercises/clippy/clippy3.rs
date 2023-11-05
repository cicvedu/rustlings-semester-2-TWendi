// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



// #[allow(unused_variables, unused_assignments)]
use std::mem;

fn main() {
    // Since we're not using `my_option` for anything, let's just remove it.
    // If we wanted to keep it, we would rename it to `_my_option`.

    let my_arr = &[
        -1, -2, -3, // Add commas here
        -4, -5, -6, // Add commas here
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // Use the clear method to empty the vector
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Use std::mem::swap to swap values
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

