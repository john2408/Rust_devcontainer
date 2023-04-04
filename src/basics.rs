#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

    // for declaring mutable variables
    let mut name = String::new();
    let greeting = "Nice to meet you";
    println!("Hello, What is your name?");

    io::stdin().read_line(&mut name)
        .expect("Didn't Recieve Input");
    
    //println!("Hello, {}!", name.trim_end());
    println!("Hello, {}! {}", name.trim_end(), greeting);

    // -----------------------------
    // Variables
    //
    // const always with uppercase
    // Shadowing: same variables name, different data types
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

    Unsigned integers:  u8, u16, u32, u64, u128, usize
    Signed integers:  i8, i16, i32, i64, i128, isize
    Floating point numbers:  f32, f64

    Platform specific integers
    usize - Unsigned integer. can only represent positive values. 
    isize - Signed integer. Can Represent both positive and negative values. 

    In Rust, signed integers are represented using the i prefix, 
    followed by the number of bits used to represent the integer. 
    For example, i32 is a 32-bit signed integer, 
    and can represent values between -2^31 and 2^31 - 1.

    Unsigned integers, on the other hand, 
    are represented using the u prefix, 
    followed by the number of bits used to represent the integer. 
    
    char - Unicode scalar value
    &str - String slice
    String - Owned string

    _tmp = for temporal variables user "_" --> Rust compiler will ignore it
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

    // If mut then vale can be changed
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

    // Working with Strings and Vectors
    // Eliminate duplicates
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1{
        println!("{}",char);
    }

    // Convert to heap allocated string
    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    // Convert str into array of bytes
    let byte_arr1 = st5.as_bytes();

    // Slice a string
    let st6 = &st5[0..6];
    println!("String lenght : {}", st6.len());
    st5.clear();
    
    // Combine strings
    let st6 = String::from("Just Some");
    let st7 = String::from(" word");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("{}", char);
    }

    // ------------------------------
    // Casting Variables
    // ------------------------------
    let int_u8 : u8 = 5;
    let int2_u8 : u8 = 4;
    let int3_u23: u32 = (int_u8 as u32) + (int2_u8 as u32);


    // ------------------------------
    // Enumerated type
    // ------------------------------
    enum Day{
        Monday,  
        Tuesday,
        Wednesday, 
        Thursday, 
        Friday, 
        Saturday, 
        Sunday
    }

    // Implementing as variable type
    impl Day{
        fn is_weekend(&self) -> bool{
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
                
            }
        }
    }

    // test it
    let today:Day = Day::Monday;
    match today{
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut Day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay Day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => print!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend());

    // ------------------------------
    // Vectors
    // Similar to array, size must be defined, 
    // can mutate if mut
    // ------------------------------

    // Create empty vector
    let vec1: Vec<i32> = Vec::new();

    // Create vector and defined values
    let mut vec2 = vec![1,2,3,4];
    
    //add value to end of vector
    vec2.push(5);

    println!("1st: {}", vec2[0]);

    let second: &i32 = &vec2[1];

    match vec2.get(1){
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }

    for val in &mut vec2{
        *val *= 2; // *i to be able to manipulate the i variable
    }

    for val in &vec2{
        println!("{}", val);
    }

    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

    // ------------------------------
    // Functions
    // ------------------------------


    say_hello();

    get_sum(5, 4);

    println!("Sum is {}", get_sum2(5, 4));

    println!("Sum is {}", get_sum3(5, 4));

    let (val_1, val2) = get_2(3);

    println!("Nums : val1: {}, val2: {}", val_1, val2);

    let num_list = vec![1,2,3,4,5];

    println!("Sum of list is {}", sum_list(&num_list));



}

fn sum_list(list: &[i32]) -> i32{
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}


fn say_hello(){
    println!("Hello");
}

fn get_sum(x : i32, y: i32){
    println!("{} + {} = {}", x, y, x +y)
}

fn get_sum2(x : i32, y: i32) -> i32{
    x + y
}

fn get_sum3(x : i32, y: i32) -> i32{
    return x + y
}

fn get_2(x: i32) -> (i32, i32){
    return (x+1, x+2)
}