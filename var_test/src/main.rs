//allows the compiler to read non-snake-case name convention without compiler warnings
#[allow(non_snake_case)]
//allows the compiler to read non-uppercase-globals in variable name convention without compiler warnings
#[allow(non_upper_case_globals)]

fn main() {
    //Mutability showcase

    println!("This is a showcase of Rust Variable Mutability");
    let mut x = 5; //initializes a variable that is mutable with a value of 5
    println!("The value is: {x}");
    x = 6; //changes the variable value from 5 to 6
    println!("The value is: {x}");

    println!();
    
    //Using Constants

    println!("This is a showcase of Rust Constants");
    const y: i32 = 10; //must dictate if the integer is a signed or unsigned integer
    //here we use a signed 32-bit integer

    println!("Constant value is: {y}");
    //y = 11; ==> this statement cannot be used because constants take in only one value
    //constants are always immutable, and unlike variables, cannot be made mutable

    println!{};

    //Shadowing variables showcase

    println!("This is a showcase of Rust Variable Shadowing");

    let xnew = 5;
    let xnew = xnew + 1;

    {// here we declare an inner scope
        let xnew = x * 4;
        println!("the inner scope xnew value is {xnew}");
    }
    println!("the outer scope xnew value is {xnew}");

    /*
        This section will be for testing out Rust Variable types
    */

    println!();
    println!();
    println!("This section deals with Rust Data Types");

    //Integers (signed and unsigned)

    let a: i8 = 127; // signed 8-bit integer
    let b: u8 = 255; // unsigned 8-bit integer

    println!("{a} is a signed 8-bit integer");
    println!("{b} is an unsigned 8-bit integer");

    let c: i16 = 32_767; // signed 16-bit integer
    let d: u16 = 65_535; // unsigned 16-bit integer

    println!("{c} is a signed 16-bit integer");
    println!("{d} is an unsigned 16-bit integer");

    let e: i32 = 2_147_483_647; // signed 32-bit integer
    let f: u32 = 4_294_967_295; // unsigned 32-bit integer

    println!("{e} is a signed 32-bit integer");
    println!("{f} is an unsigned 32-bit integer");

    let g: i64 = 9_223_372_036_854_775_807; // signed 64-bit integer
    let h: u64 = 18_446_744_073_709_551_615; // unsigned 64-bit integer

    println!("{g} is a signed 64-bit integer");
    println!("{h} is an unsigned 64-bit integer");

    let i: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // signed 128-bit integer
    let j: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // unsigned 128-bit integer

    println!("{i} is a signed 128-bit integer");
    println!("{j} is an unsigned 128-bit integer");

    /*
    
        As a side note, the `arch` data type is used to identify integers based on CPU architecture
        This means that the 2 base types of the `arch` data type are 32-bit and 64-bit

        the two types of `arch` declarations are `isize` and `usize` taking the range of 32-bit
        and 64-bit integers for signed and unsigned respectively
    
    
    */

    // floating point integers

    let k: f32 = 2.147483; // 32-bit floating point integer
    let l: f64 = 9.233372036; // 64-bit floating point integer

    println!("{k} is a 32-bit floating point integer");
    println!("{l} is a 64-bit floating point integer");

    // boolean data type

    let m: bool = true;
    let n: bool = false;

    println!("{m} is a TRUE boolean");
    println!("{n} is a FALSE boolean");

    // char data type

    let o: char = 'A'; // char variables are stored in single quotations ('a')
    
    println!("{o} is a character");

    // Compound Data Types

    println!();
    println!();
    println!("This section will talk about Compound Data Types");

    // Tuple Data Type

    let p_hold: (char, i32, f32) = ('B',600,32.222); // stores the tuple values in a "holder"
    let (p1,p2,p3) = p_hold; // tuple indices are distributed accordingly to p1, p2 and p3

    /*
        The Tuple data type is able to store multiple data types into a single variable
    
    */

    println!("{p1} is a character variable");
    println!("{p2} is a signed 32-bit integer");
    println!("{p3} is a 32-bit floating point integer");

    // Deconstructing tuples using p_hold

    let p4 = p_hold.0;
    let p5 = p_hold.1;
    let p6 = p_hold.2;

    println!();
    println!("Deconstructing the p_hold tuple can look like this");
    println!("{}, {}, and {} are all parts of a tuple", p4,p5,p6); // this is another way to print variable values using concatenation



}
