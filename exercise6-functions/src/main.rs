fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // line without semicolon is an expression this returns a value
    //lines with semicolons are called statements

}