use ferris_says::say; // import crate 'say' function
use std::io::{stdout, BufWriter};
use std::thread;
use std::time::Duration;

// import from file.
mod test_module;

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
    structure();
    module();
    err_handling();
    trait_struct();
    args();
    concurrency();

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

    const XX: f64 = 155.51; // constant
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
        _ => "Unknown",
    };

    println!("country {}", country);
}

fn loop_() {
    for i in 0..10 {
        println!("i: {}", i);
    }
}

fn fn_pass_reference(value: &mut i32) {
    *value += 10;
    println!("in function: {}", value);
}

fn collection() {
    // Tuple
    let mut tuple = ("Hello", 29, 39.4);
    tuple.0 = "Hi world";
    println!("tuple: {:?}", tuple);

    // Array
    let mut arr: [i32; 10] = [0; 10];
    for i in arr.iter_mut() {
        *i += 2;
    }
    println!("arr: {:?}", arr);

    // Vector
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(10);
    vec.push(100);
    vec[2] = 1000;
    println!("vector: {:?}", vec);

    // HashMap
    use std::collections::HashMap;
    let mut map:HashMap<String, i32> = HashMap::new();
    map.insert("Year".to_string(), 1995);

    // HashSet
    use std::collections::HashSet;
    let mut set:HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
}

fn ownership() {
    let x = String::from("x owner this string.");
    let y = x;
    test(y);
    // let z = test2(y);
    // println!("onwership: {}", y);

    fn test(text: String) {
        println!("{}", text);
    }

    fn test2(text: String) -> String {
        return text;
        // println!("in function = {}", text);
    }
}

fn borrowing() {
    let x = String::from("Borrowing.");
    some_fn(&x);
    println!("borrowing: {}", x);

    fn some_fn(x: &String) {
        println!("borrowing: {}", x);
    }
}

fn slice() {
    let arr = [1, 2, 3, 4, 5];
    let sli = &arr[2..arr.len()];

    let mut arr2 = [10, 20, 30, 40];
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

fn structure() {
    // Declaring structure
    struct User {
        first_name: String,
        last_name: String,
        age: i32,
    }
    // method
    impl User {
        fn new() -> User {
            return User {
                first_name: String::from(""),
                last_name: String::default(),
                age: 0,
            };
        }
        fn static_method() {
            println!("User's static method.");
        }
        fn hello(&self) {
            println!("Hello, My name is {}.", self.first_name);
        }
    }

    let mut user = User {
        first_name: String::from("Tony"),
        last_name: String::from(""),
        age: 0,
    };
    let mut user2 = User::new();

    user.age = 45;
    user.hello();
    User::static_method();
    println!("struct user: {}", user.first_name);
}

fn enumeration() {
    enum Country {
        Thailand,
        Japan,
        China,
        USA,
        India,
    }
    struct Person {
        name: String,
        country: Country,
    }

    let x = Country::Thailand;
    let p1 = Person {
        name: String::from("Hi"),
        country: Country::Thailand,
    };
}

fn module() {
    pub mod say_hi {
        pub fn hello() {
            println!("Hello world! from module");
        }
        pub fn hello_name(name: String) {
            println!("Hello {}! from module", name);
        }
    }

    // ใช้แบบ import
    use say_hi::*;
    hello_name("Mars".to_string());

    // ใช้แบบ ชื่อmodule::ชื่อfunction
    say_hi::hello();
    
    // from test_module.rs
    test_module::hello_test_module();
    test_module::hello_world();
}

fn err_handling() {
    fn is_even(num:i32) -> Result<bool,String> {
        if num%2 == 0 {
            Ok(true)
        } else {
            Err("Not an even.".to_string())
        }
    }

    let result = is_even(5);
    match &result {
        Ok(data) => println!("Ok : {}", data),
        Err(msg) => println!("Err: {}", msg)
    }

    let boolean = is_even(10).unwrap();
    println!("is_even: {}", boolean);
}

/// like an interface in OOP
fn trait_struct() {
    trait Vehicle {
        fn forward(&self);
        fn backward(&self);
        fn start(&self);
    }

    struct Car {
        model:&'static str
    }

    impl Vehicle for Car {
        fn start(&self) {
            println!("Start engine");
        }
        fn forward(&self) {
            println!("Move forward");
        }
        fn backward(&self) {
            println!("Move backward");
        }
    }

    impl std::fmt::Display for Car {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Car model: {}", self.model)
        }
    }

    let car = Car{model:"XT45"};
    car.start();
    car.forward();
    println!("{}", car);
}

fn args() {
    let cmd_line = std::env::args();
    println!("args: {:?}", cmd_line);
}

fn  concurrency() {
    let handle = thread::spawn(move || {
        println!("[thread] stat.");
        for i in 0..10 {
            println!("in thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 0..5 {
        println!("in main: {}",i);
        thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap();
}