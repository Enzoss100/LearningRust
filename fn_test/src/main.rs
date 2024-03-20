fn main() {
    // call the new_func function
    new_func();
    param_func(10,20);
    println!("{} is a return value",testint());
}


// this new_func is a new function that prints a string
fn new_func(){
    println!("This is a new function");
}

// this function has 2 parameters looking for i32 integers to be printed out
fn param_func(x: i32, y: i32){
    println!("{x} is the first paramenter passed as an argument in this function");
    println!("{y} is the second parameter passed to this function");
}

fn testint() -> i32 { // this function expects an i32 data type as a return value
    return 100; // this returns the value 100 to the compiler
}