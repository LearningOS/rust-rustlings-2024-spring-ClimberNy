// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!{}",2);
    };
}

fn main() {
    let a :Result<i32, i32>= Result::<i32, i32>::Ok(2);
    println!("{:?}",a.unwrap());
    my_macro!();
}
