#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

    let mut name = String::new();
    let greeting = "Nice to meet you";
    println!("Hello, What is your name?");

    io::stdin().read_line(&mut name)
        .expect("Didn't Recieve Input");
    
    //println!("Hello, {}!", name.trim_end());
    println!("Hello, {}! {}", name.trim_end(), greeting);

    // -----------------------------
    // Variables
    // ------------------------------
    const ONE_MIL: u32 = 1_000_000;
    const PIT: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}",age, ONE_MIL);

    /*
    bool - Boolean

    Unsigned integers:  u8, u16, u32, u64, u128

    Signed integers:  i8, i16, i32, i64, i128

    Floating point numbers:  f32, f64

    Platform specific integers
    usize - Unsigned integer. Same number of bits as the platform's pointer type.

    isize - Signed integer. Same number of bits as the platform's pointer type.

    char - Unicode scalar value
    &str - String slice
    String - Owned string
    */

    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);

    //Char
    let my_grade = 'A';

    // Precision
    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.000000000000000);

    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.000000000000000);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    // If mut then vale can be chaged
    num_3 += 1;

    // Random Values
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    // -----------------------------
    // If Expressions
    // ------------------------------
    let age = 8;
    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    }else if (age == 21) || (age == 50){
        println!("Important Birthday");
    }else if age >= 65{
        println!("Important Birthday");
    }else {
        println!("Not an Important Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };

    println!("Can Vote: {} ", can_vote);

    // -----------------------------
    // Match Expressions
    // ------------------------------
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => print!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    // Another example
    let my_age  = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),      
    };

    // -----------------------------
    // Arrays
    // ------------------------------
    let arr_1 = [1,2,3,4];
    println!("1st :  {}", arr_1[0]);
    println!("Length :  {}", arr_1.len());

    let arr_2 = [1,2,3,4,5,6,7,8,9];

    // -----------------------------
    // Loop functions
    // ------------------------------
    println!("Loop Function\n");
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] %2 == 0{
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9{
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

    // -----------------------------
    // while functions
    // ------------------------------
    let mut loop_idx = 0;
    println!("While Loop\n");
    while loop_idx < arr_2.len(){
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // -----------------------------
    // For functions
    // ------------------------------
    println!("For Loop\n");
    for val in arr_2.iter(){
        println!("Arr : {}", val);
    }

    // -----------------------------
    // Tuple
    // ------------------------------
    let my_tuple : (u8, String, f64) = (47, 
                                        "Derek".to_string(),
                                        50_000.00);

    println!("Name : {}", my_tuple.1);
    
    let(v1, v2, v3) = my_tuple;

    println!("Age: {}", v1);

    // -----------------------------
    // Strings
    // ------------------------------
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" Word");

    println!("st1 is {}", st1);
    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

}