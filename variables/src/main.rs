fn sum(a: i32, b:i32) -> i32 {
    println!("first statement");
    println!("second statement");
    a + b
}

fn main() {
    let mut x = 2; // must include 'mut' in declara&tion to be able to edit it later
    let y = 5; // similar to a constant, not changeable 
    x = x + 1;
    println!("Hello, world! {} + {} = {}", x, y, x+y);

    let a: u8 = 255;                    // (2^8)  - 1
    let b: u16 = 65535;                 // (2^16) - 1
    let c: u32 = 4294967295;            // (2^32) - 1
    let d: u64 = 18446744073709551615;  // (2^64) - 1

    println!("{}, {}, {}, {}", a, b, c, d);

    let a_min: u8 = std::u8::MIN;
    let b_min: u16 = std::u16::MIN;
    let c_min: u32 = std::u32::MIN;
    let d_min: u64 = std::u64::MIN;
    let e_min: usize = std::usize::MIN;

    let a_max: u8 = std::u8::MAX;
    let b_max: u16 = std::u16::MAX;
    let c_max: u32 = std::u32::MAX;
    let d_max: u64 = std::u64::MAX;
    let e_max: usize = std::usize::MAX;

    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    let a_min: i8 = std::i8::MIN;
    let b_min: i16 = std::i16::MIN;
    let c_min: i32 = std::i32::MIN;
    let d_min: i64 = std::i64::MIN;
    let e_min: isize = std::isize::MIN;

    let a_max: i8 = std::i8::MAX;
    let b_max: i16 = std::i16::MAX;
    let c_max: i32 = std::i32::MAX;
    let d_max: i64 = std::i64::MAX;
    let e_max: isize = std::isize::MAX;

    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    let a_min: f32 = std::f32::MIN;
    let b_min: f64 = std::f64::MIN;

    let a_max: f32 = std::f32::MAX;
    let b_max: f64 = std::f64::MAX;

    println!("{}, {}", a_min, b_min);
    println!("{}, {}", a_max, b_max);

    let ch1: char = 'X';
    let ch2 = '\u{2603}';

    println!("{}, {}", ch1, ch2);

    let on: bool = true;
    let off: bool = false;

    println!("{}, {}, {}", on, off, !on);
    
    let first = 7.0;    //must have matching types to perform math operations
    let second = 4.0;

    println!("{}, {}, {}, {}, {}", 
        first+second,
        first-second,
        first*second,
        first/second,
        first%second
    );

    let btc = "Bitcoin";
    let eth = "Ethereum";
    let sol = "Solana";

    println!("Normal Order: {}, {}, {}", btc, eth, sol);
    println!("Flippening: {1}, {0}, {2}, {2}", btc, eth, sol);
    println!(
        "Kwargs: {solana}, {ethereum}, {bitcoin}",
        bitcoin=btc,
        ethereum=eth,
        solana=sol    
    );


    let message = format!(       
        "Kwargs: {solana}, {ethereum}, {bitcoin}",
        bitcoin=btc,
        ethereum=eth,
        solana=sol   
    );
    println!("Look I've made this special {message}", message=message);

    let cheat_code: [u32; 4] = [19, 65, 9, 17];
    let zeros = [0.0; 10];
    println!("Array: {:?}", cheat_code);
    println!("First element of the array: {}", cheat_code[0]);

    println!("Arrah(length: {}): {:?}", zeros.len(), zeros);

    let slice = &cheat_code[1..3];
    println!("Slice of cheat_code: {:?} {}", slice, slice.len());

    let name: &str = "Zsolt";

    let location: String = "Sliema, Malta".to_string();
    let title: String = String::from("IT Engineer");

    let title_slice = &title[0..6];

    println!("Hi, my name is {}.", name);
    println!("I live in {}.", location);
    println!("This name is {} characters long.", name.len());
    println!("My title is {}.", title);
    println!("Title as a string slice: {}.", title_slice);


    let name = "Zsolt";
    let name_len = name.len();

    if name_len % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    if name_len ==4 {
        println!("Four");
    } else if name_len ==5 {
        println!("Five");
    } else if name_len == 6 {
        println!("Six");
    } else {
        println!("Different");
    }

    let first = false;
    let second = true;

    if first && second {
        println!("Both are true");
    } else if first || second {
        println!("At least one is true");
    } else {
        println!("Neither is true");
    }

    match name_len {
        4 => {println!("Four");}
        5 => {println!("Five");}
        6 => {println!("Six");}
        _ => {println!("Different");}
    }

    match first {
        true => {println!("True");}
        false => {println!("False");}
    }

    let message3 = match name_len {
        4 => { "Four" }
        5 => {"Five"}
        6 => {"Six"}
        _ => {"Different"}
    };

    println!("{}", message3);

    let n = 10; 
    let mut i = 1;

    loop {
        println!("loop: {}", i);
        i = i + 1;
        if i > n {
            break;
        }
    }

    println!("in-between loops: {}", i);
    i = 1;

    while !(i >= n) {
        println!("While: {}", i);
        i = i + 1;
    }

    for i in 1..=10 {
        println!("for: {}", i);
    }

    for value in cheat_code {
        println!("cheat code by value: {}", value);
    }

    let a = 2; 
    let b = 5;
    println!("{} + {} = {}", a, b, sum(a,b));

}
