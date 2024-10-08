// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
// }

// fn main() {
//     let t = true;

//     let f = false; // with explicit type annotation
// }

// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// fn main() {
//     let x = 2.0;

//     println!("{x}");
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let months = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];

//     let a: [i32; 5] = [1, 2, 3, 4, 5];

//     let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

//     let first = a[0];
//     let second = a[1];
// }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// fn main() {
//     let message = "The temperature today is:";

//     let x = [message, 100]; // does not compile, !mixed types! in array

//     println!("{} {}", x[0], x[1]);
// }

// pub fn data_types() {
//     let t = ([1; 2], [3; 4]);

//     let (a, b) = t;

//     println!("{}", a[0] + t.1[0]); // 1 + 3 = 4
// }

use std::collections::HashMap;

#[allow(warnings)]
fn main() {
    // INTEGERS
    let small_num_01: u8 = 255;
    let big_num_01: u128 = 123456789012345678;
    let small_num_02: i8 = -127;
    let big_num_02: i128 = -123456789012345678;

    const _SUBSCRIBER_COUNT: u32 = 100_000;

    println!("small_num_01: {}", { small_num_01 });
    println!("big_num_01: {}", { big_num_01 });
    println!("small_num_02: {}", { small_num_02 });
    println!("big_num_02: {}", { big_num_02 });

    // NUMERAL SYSTEM
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("decimal: {}", { decimal });
    println!("hex: {}", { hex });
    println!("octal: {}", { octal });
    println!("binary: {}", { binary });
    println!("byte: {}", { byte });

    // FLOATING POINTS
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x = {}, y = {}", x, y);

    // NUMERIC OPERATIONS
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = 43 % 5;

    println!("sum: {}", { sum });
    println!("difference: {}", { difference });
    println!("product: {}", { product });
    println!("quotient: {}", { quotient });
    println!("remainder: {}", { remainder });

    // BOOLEANS
    let is_true = true;
    let is_false: bool = false;
    println!("is_true = {}, is_false = {}", is_true, is_false);

    let age: u8 = 18;

    if age >= 18 {
    } else {
    }

    if is_true {
        println!("is_true is true");
    } else {
        println!("is_true is false");
    }

    let not_is_true = !is_true;
    println!("not_is_true = {}", not_is_true);

    // CHARACTERS
    let z = 'z';
    let x = '❌';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", z, x, heart_eyed_cat);

    for char in "Ciao, नमस्ते, ".chars() {
        println!("{}", char)
    }

    // TUPLES
    let tuple = (500, 6.4, 'x');
    let (_x, y, _z) = tuple;
    println!("The value of y is {}", y);

    let _five_hundred = tuple.0;
    let _six_point_four = tuple.1;
    let x_char = tuple.2;
    println!("The value of x_char is {}", x_char);

    // ARRAYS
    let array = [1, 2, 3, 4, 5];
    let _byte = [0; 8];
    let first = array[0];
    let second = array[1];
    println!("Number Array: {:?}", array);
    println!("first {}, second {}", first, second);

    for element in array.iter() {
        println!("element: {}", element);
    }

    // SLICES
    let number_slices = &[1, 2, 3, 4, 5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices = &["Lion", "Buffalo", "Squirrel"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices = &[
        &"Harry Potter".to_string(),
        &"Inheritance Games".to_string(),
        &"Hunger Games".to_string(),
    ];
    println!("Book Slices: {:?}", book_slices);

    // Strings vs String Slices (&str)
    // A- Strings [growable(grow/shrink), mutable(add,delete), owned]
    let mut stone_cold = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");

    // B- &str (String Slices... specific size, immutable, reference to another)
    let string = String::from("Hello, World");
    let slice: &str = &string[0..5];
    // let slice: &str = &string;
    println!("Slice Value: {}", slice);

    // STRUCTS
    struct Person {
        age: u8,
        first_name: String,
        last_name: String,
    };

    let person = Person {
        first_name: String::from("Samuel"),
        last_name: "Chukwuzube".to_owned(),
        age: 24,
    };
    println!(
        "My name is {} {} and I am {} years old.",
        person.first_name, person.last_name, person.age
    );

    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut elon = User {
        active: true,
        username: String::from("elon"),
        email: String::from("elon@example.com"),
        sign_in_count: 1,
    };
    elon.email = String::from("sam@hotmail.com");

    println!("User email is {}", elon.email);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    };

    let mark = User {
        username: String::from("mark"),
        email: String::from("mark@example.com"),
        ..elon
    };

    // Tuple STRUCTS
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _white = Color(255, 255, 255);

    // Unit-like STRUCTS
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // ENUMS

    enum IpAddressKind {
        V4,
        V6,
    }

    enum IpAddress {
        V4(u8, u8, u8, u8), // enhanced enums
        V6(String),
    }
    struct IPAddress {
        kind: IpAddressKind,
        address: String,
    }

    let _v4 = IpAddressKind::V4;
    let _v6 = IpAddressKind::V6;

    fn route(ip_kind: IpAddressKind) {}

    route(IpAddressKind::V6);

    let home = IPAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    let traffic_light = TrafficLight::Red;

    match traffic_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow Down!"),
        TrafficLight::Green => println!("Go!"),
    };

    let current_light = match traffic_light {
        TrafficLight::Red => "Stop!",
        TrafficLight::Yellow => "Slow Down!",
        TrafficLight::Green => "Go!",
    };

    println!("The current traffic light says {}", current_light);

    tell_height(182);
    human_id("Samuel", 28, 182.2);

    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };

    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmi(weight, height);

    println!("Your BMI is: {:.2}", bmi);

    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4];
    v1.push(1);
    v2.push(5);

    // ---------------------- HASHMAPS ---------------------------  //
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    let team_name = String::from("yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn tell_height(height: u32) {
    println!("My height is {}cm.", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}. I am {} years old, and my height is {}cm.",
        name, age, height
    )
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
