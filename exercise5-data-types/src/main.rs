fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    let five_hundred = tup.0 // Access via index like this 

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}