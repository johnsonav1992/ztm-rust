// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

use std::vec;

fn main() {
    let vec: Vec<i32> = vec![10, 20, 30, 40];

    for elem in vec.iter() {
        match elem {
            30 => println!("thirty"),
            _ => println!("{:?}", elem),
        }
    }

    println!("Length: {:?}", vec.len());
}
