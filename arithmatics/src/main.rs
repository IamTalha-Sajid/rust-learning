fn main() {
    // Integer practice
    let x = 32;      // Inferred to be i32 by default
    let y: u8 = 33;  // Explicit type declaration

    // Correctly format the `println!` macro
    println!("{}, {}", x, y);  // Print `x` and `y` with correct syntax
    
    //assert
    // assert!(0.1 + 0.2 == 0.3);  This will fail because of floating point precision as 0.1 + 0.2 = 0.30000000000000004 

    //Floating point practice
    assert!(0.1_f32 + 0.2_f32 == 0.3 as f32);
    println!("Success");
}