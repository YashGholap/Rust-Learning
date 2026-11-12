// fn main(){
//     //scalars
//     let age: u32 = 18;
//     let num : i32 = -100;
//     let temperature: f64 = 36.6;
//     let active : bool = true;
//     let grade: char = 'A';


//     println!("Age: {}", age);
//     println!("Number: {}", num);
//     println!("Temperature: {}", temperature);
//     println!("Active: {}", active);
//     println!("Grade: {}", grade);

//     // compound types

//     //tuple
//     let person: (&str, u32, u32) = ("Alice", 30, 50000);
//     println!("Name: {}, Age: {}, Salary: {}", person.0, person.1, person.2);

//     //array
//     let scores: [i32;3] = [90, 85, 95];
//     println!("First Score : {}", scores[0]);


// }

//functions

// fn main() {
//     greet("Alice");
//     let result =add(5, 10);
//     println!("The sum is: {}", result);
// }

// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b // is an expression, value automatically becomes return value of the function
// }

// control flow
// fn main(){
//     let _number = 6;

//     // if number % 2 == 0 {
//     //     println!("The number is even.");
//     // } else {
//     //     println!("The number is odd.");
//     // }

//     // let label = if number % 2 == 0 {"even"} else {"odd"};
//     // println!("The number is {}.", label);

//     //loop
//     // for loop
//     // let mut counter = 0;
//     // let result = loop {
//     //      counter += 1;
//     //      if counter == 5{
//     //         break counter * 2
//     //      }
//     // };

//     // println!("Loop result: {}", result); 

//     //while loop
//     // let mut n = 3;
//     // while n!= 0 {
//     //     println!("{}", n);
//     //     n -= 1;
//     // }

//     // println!("liftoff!");

//     // for loop in range
//     // for i in 1..4 {
//     //     println!("Number: {}", i);
//     // }

//     // for loop in array
//     let scores = [90, 85, 95];
//     for score in scores {
//         println!("Score is : {}", score);
//     }

// }



// ownership and borrowing
// fn main() {
//     let s1 = String::from("Hello"); // create a heap allocated growing string.
//     let s2 = s1.clone();

//     println!("{}", s1);
//     println!("{}", s2);

// }


// reference
// fn main(){
//     let s1 = String::from("Hello");
//     let len = calculate_length(&s1); // pass reference of s1 to the function

//     println!("The length of '{}' is {}.", s1, len);
    
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main(){
//     let mut s1 = String::from("Hello");
//     add_exclamation(&mut s1);
//     println!("{}", s1);
// }

// fn add_exclamation(s: &mut String) {
//     s.push_str("!");
// }

//structs
// #[derive(Debug)] // auto generate debug printing support

// struct User {
//     username: String,
//     age: u32,
//     active: bool,
// }

// fn main(){
//     let user1 = User{
//         username: String::from("Yash"),
//         age: 26,
//         active: true,
//     };

//     println!("{} is {} years old and is {}.", user1.username, user1.age, if user1.active { "active" } else { "not active" });
//     println!("{}",user1.describe());
// }

// impl User {
//     fn describe(&self) -> String{
//         format!("{} (active: {})", self.username, self.active)
//     }
// }

//enums

// enum TrafficLight {
//     Red, // variants without any data
//     Yellow,
//     Green,
// }

// enum Shape {
//     Circle(f64), // variant with data
//     Rectangle(f64, f64),
// }

// fn area(shape: &Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }

// fn main(){
//     let light  = TrafficLight::Red;

//     let c = Shape::Circle(2.5);
//     let r = Shape::Rectangle(3.0, 4.0);

//     println!("Area of circle: {}", area(&c));
//     println!("Area of rectangle: {}", area(&r));

//     match light {
//         TrafficLight::Red => println!("Stop!"),
//         TrafficLight::Yellow => println!("Caution!"),
//         TrafficLight::Green => println!("Go!"),
//     }


// }


// option result and error handling
// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err(String::from("Cannot divide by Zero."))
//     } else {
//         Ok(a/b)
//     }
// }


// fn main() {
//     let result = divide(10.0, 0.0);

//     match result {
//         Ok(value) => println!("Result: {}", value),
//         Err(error) => println!("Error: {}", error)
//     }
// }


// vectors
// fn main(){
    // let mut scores: Vec<i32> = Vec::new();

    // scores.push(90);
    // scores.push(85);

    // for score in &scores {
    //     println!("Score is {}", score);
    // }

    // println!("First score: {}", scores[0]);


//     let owned : String = String::from("Hello");
//     let borrowed: &str = "World";

//     println!("Owned: {}, Borrowed: {}", owned, borrowed);
// }


// hashmaps

// use std::collections::HashMap;

// fn main(){
//     let mut ages : HashMap<String, u32> = HashMap::new();
//     ages.insert(String::from("Yash"), 26);
//     ages.insert(String::from("Alice"), 30);

//     if let Some(age) = ages.get("Yash") {
//         println!("Yash's age is {}", age);
//     } else {
//         println!("Yash's age not found.");
//     }

//     for (name, age) in &ages { // order is not guaranteed in hashmaps
//         println!("{} is {} years old.", name, age);
//     }
// }


// traits and generics

// fn largest <T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];
//     let result = largest(&numbers);
//     println!("The largest number is {}", result);

//     let chars = vec!['A','B', 'Z'];
//     let char_result = largest(&chars);
//     println!("The largest character is {}", char_result);
// }


trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
}

struct Tweet {
    text: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.headline)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet: {}", self.text)
    }
}

fn main(){
    let article = Article {
        headline: String::from("Rust is awesome!"),
    };

    let tweet = Tweet {
        text: String::from("Learning Rust is fun!"),
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}

// async

// async fn slow_hello() -> String {
//     "Hello from async function!".to_string()
// }

// #[tokio::main]
// async fn main() {
//     let greeting = slow_hello().await;

//     println!("{}", greeting);

// }
