fn add_one(mut val: i32) -> i32 {
    val += 1;
    val
}

fn main() {
    // {
    //     let val: i32 = 0;
    //     println!("{}", val);
    // }
    // println!("{}", val); // val is not in scope, will error

    let val: i32 = 0;
    add_one(val);
    println!("{}",val); // prints 0

    let mut val: i32 = 0;
    val = add_one(val);
    println!("{}", val); // prints 1
}
