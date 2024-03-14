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
}
