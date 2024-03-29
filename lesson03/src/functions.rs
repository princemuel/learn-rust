// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");// }

// fn f(x) {

//     println!("{x}");

//   }

//   fn main() {

//     f(0);

//   }

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}")
// }

// fn plus_one(x: i32) -> i32 {
//     return x + 1;
// }

fn main() {
    let x = {
        let y = 6;
        y + 1
    };

    let y = sum_difference(5, x);

    // println!("sum = {}, difference = {}", y.0, y.1);
    println!("the sum and difference is {:?}", y); // {:?} is used to print the tuple
}

fn sum_difference(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}
