use ferris_says::say; // import crate 'say' function
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello world!!";
    let width = 20;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

    data_type();
    str_example();
    decision();
    loop_();
    collection();
    ownership();
    borrowing();
    slice();

    let mut num = 10;
    fn_pass_reference(&mut num);
    println!("num: {}", num);
}

fn data_type() {
    let x_str = "Hello world";
    let x_float = 10.10;
    let x_num = 100_000;
    let x_bool = true;
    let x_icon = "👌";
    let x_char = 'A';

    let mut x_mut_var = 10; // mutable variable
    x_mut_var = 12;

    const XX:f64 = 155.51; // constant

}

fn str_example() {
    // String literal
    let str_literal = "This is string literal";

    // String object
    let mut str_obj_1 = String::new();
    str_obj_1.push_str("String object.");
    let str_obj_2 = String::from("For string object");
    
    println!("{}", str_obj_1);
    println!("{}", str_obj_2);

    let s1 = "one".to_string();
    let s2 = "two".to_string();
    let s3 = s1 + &s2;
    let s4 = format!("{} {}", "hi", "hello");
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}

fn decision() {
    let country_code = "TH";
    let country = match country_code {
        "US" => "United state",
        "UK" => "United kingdom",
        "JP" => "Japan",
        "CN" => "China",
        "TH" => "Thailand",
        _ => "Unknown"
    };

    println!("country {}", country);
}

fn loop_() {
    for i in 0..10 {
        println!("i: {}", i);
    }
}

fn fn_pass_reference(value:&mut i32) {
    *value += 10;
    println!("in function: {}", value);
}

fn collection() {
    // Tuple
    let mut tuple = ("Hello", 29, 39.4);
    tuple.0 = "Hi world";
    println!("tuple: {:?}", tuple);

    // Array
    let mut arr:[i32;10] = [0;10];
    for i in arr.iter_mut() {
        *i += 2;
    }
    println!("arr: {:?}", arr);
    
}

fn ownership() {
    let x = String::from("x owner this string.");
    let y = x;
    test(y);
    // let z = test2(y);
    // println!("onwership: {}", y);

    fn test(text:String) {
        println!("{}", text);
    }

    fn test2(text:String) -> String {
        return text;
        // println!("in function = {}", text);
    }
}

fn borrowing() {
    let x = String::from("Borrowing.");
    some_fn(&x);
    println!("borrowing: {}", x);

    fn some_fn(x:&String) {
        println!("borrowing: {}", x);
    }
}

fn slice() {
    let arr = [1,2,3,4,5];
    let sli = &arr[2..arr.len()];

    let mut arr2 = [10,20,30,40];
    let sli2 = &mut arr2[2..4];

    test(sli);
    mute_slice(sli2);
    
    println!("arr: {:?}", arr);
    println!("arr2: {:?}", arr2);

    fn test(sub_arr: &[i32]) {
        println!("slice 1: {:?}", sub_arr);
    }

    fn mute_slice(sub_arr: &mut [i32]) {
        // เปลี่ยนค่า
        sub_arr[0] = 100;
        println!("slice 2: {:?}", sub_arr);
    }
}