fn main() {
    // call the new_func function
    new_func();
    param_func(10,20);
    println!("{} is a return value",testint());
    conflow1(15);
    conflow2();
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

fn conflow1(y: i32) {
    if y > 10 {
        println!("{y} is greater than 10");
    }
    else{
        println!("{y} is less than 10");
    }
}

fn conflow2() {
    let con = true;
    /* 
        here we can see that conditional statements can be used in let statements to initialize variables
        differently

        there are no semicolons in at the end of '12' and '19' because they are both expressions, not
        statements

        there is a semicolon at the end of the else statement to encapsulate the entire initialization of
        z into one singular statement
    */
    let z = if con {
                12
            }
            else {
                19
            };
    println!("{z} is the value of the variable 'z'");
}