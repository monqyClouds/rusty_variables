use std::io;

fn main() {
    let arr = [5, 6, 7, 8, 9];

    println!("Please enter an array index...");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Error! can not read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Error! entered value not a number");

    let element = arr[index];

    println!(
        "The value of the element at index {} is {}",
        index, element
    );

}
 