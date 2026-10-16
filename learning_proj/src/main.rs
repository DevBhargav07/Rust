use std::io;
use std::time::Instant;
use std::collections::HashMap;

fn arthmetics(num1: i32, num2: i32){
    println!("Arthmetic Operations for {} and {}", num1, num2);
    let add = num1 + num2;
    let sub = num1 - num2;
    let mul = num1 * num2;
    let div = num1 / num2;
    let modu = num1 % num2;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Modu: {}", modu);

}

fn greetings(name: &str, day: i32) {
    let result: &str = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid Day",
    };
    println!("Hi {}, {} it is for you, Have a good day!", name, result);
}

fn try_loops(age: i32) {
    let mut cnt: i32 = 1;
    loop {
        println!("Hi!");
        if cnt == age {
            break;
        }
        cnt += 1;
    }
    println!("Stopping loop after reaching the limit: {}", age);
}


fn loops(num: i64){
    let mut current_num: i64 = num;
    loop {
        if current_num == 1000009 {
            break;
        }
        current_num += 1;
    }

}

fn get_details() -> (String, i32) {

    println!("Please Enter your name: ");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    let name = name.trim().to_string();

    println!("Hi {}, what's your age?", name);

    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read age");

    // converting string to i32
    let age: i32 = age_input
        .trim()
        .parse()
        .expect("Please enter a valid integer to age");
    (name, age)
}

fn learn_tuples(){
    // tuple is used to return multiple values and to store multiple type values
    // you can access a value by using the . (dot) tuple.0
    let person = ("Something", 22, true);
    println!("Name of the person: {}", person.0);
    println!("Age of the person: {}", person.1);
    println!("Eligible to vote?: {}", person.2);
}

fn learn_hash_maps(){
    // hashmaps are like dictionaries in python
    // stores key value pairs in rust
    let mut cities = HashMap::new();

    cities.insert("India", "New Delhi");
    cities.insert("Uk", "England");
    cities.insert("Australia", "Canberra");
    println!("Country    \t Capital City");
    for (country, city) in &cities {
        println!("{} \t {}", country, city);
    }
    // println!("ALL cities: {:?}", cities);
    if let Some(city) = cities.get("India") {
        println!("The capital city of India is: {}", city);
    } else {
        println!("India is not in the Map, go find the correct map");
    }
    cities.remove("Uk");
    println!("{:?}",cities);
}

fn learn_vectors(){
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("All Numbers: {:?}", numbers);
    numbers.push(6);
    println!("All Numbers: {:?} -> adding at the end", numbers);
    numbers.pop();
    println!("All Numbers: {:?} -> deleting last element", numbers);
    numbers.push(6);
    numbers.remove(1);
    println!("All Numbers: {:?} -> missing val at index 1", numbers);
    numbers.insert(1, 9);
    println!("All Numbers: {:?} -> inserting 9 at index 1", numbers);
    println!("Length of numbers: {}", numbers.len());
    
    // looping a vector
    for num in numbers {
        // remember: &numbers will not work for i32 
        print!("{}\t", num);
    }
    println!();
    let groceries = vec!["apple", "banana", "grapes"];
    for num in &groceries {
        // works for the strings in a vector
        print!("{}\t", num);
    }
    println!();
    
    // in rust using & means taking the value for borrowing
    // if you use already declared varaible then the ownership will change
    // let a = 10;
    // let b = a; the value will be taken to b and a will not hold that value anymore
    // if you try to print a then it will throw error No Value
    // to don't take the ownership and to work with the value we use & - borowwing

}

fn main(){

    let start_time = Instant::now();
    println!("Loop Started");
    loops(1);
    let elapsed = start_time.elapsed();
    println!("Program is finished at: {:.9?}s", elapsed.as_secs_f64());
    learn_vectors();
    // print!("Hii, let's start learning\n");
    //print!("Learning about macros ending with ! marks\n");
    //println!("Works just like functions, but not really functions");
    let (name, age) = get_details();
    greetings(&name, 3);

    // using mut
    let mut mutable_name = name;
    println!("Hii {}, welcome back!", mutable_name);
    mutable_name = "My name".to_string();
    println!("Hii {}, welcome back!", mutable_name);

    println!("My age is {}", age);
    //name = "My name"; // throws error without using mut
    // name = "My name";
    // println!("Hii {}, welcome back!", name);

    // let age: i32 = 25;
    // println!("My age is {}", age);
    // let _my_bool: bool = true;
    // let _my_letter: char = 'D';
    // let _my_double: f64 = 12.423;
    const NUMBER1: i32 = 10;
    const NUMBER2: i32 = 2;
    arthmetics(NUMBER1, NUMBER2);
    // try_loops(age);
    learn_tuples();
    learn_hash_maps();
}
