fn main() {
    // call the new_func function
    new_func();
    param_func(10,20);
    println!("{} is a return value",testint());
    conflow1(15);
    conflow2();
    conflow3();
    labloop();
    arrcall();
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

fn conflow3() {
    let mut count: i32 = 0;

    loop {
        println!("Iteration: {count}");
        count += 1;
        if count == 10 {
            println!("Ended at {count}");
            break;
            
        }
    }
}

fn labloop() {
    let mut count = 0;
    'counting_up: loop { //labels this outter loop counting_up
        println!("count = {count}"); //prints 0
        let mut remaining = 10;

        loop { //unlabelled inner loop
            println!("remaining = {remaining}"); //prints 10
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //breaks the outter loop named counting_up
            }
            remaining -= 1; //decreases from 10 to 9, initiating the break of the unlabelled loop
        }

        count += 1; //increases the count to 1, loop continues because count is not 2
        /*

            The outter loop reinitializes `remaining` to 10 at count 2

        */
    }
    println!("End count = {count}");

}

fn arrcall() {
    let list:[i32;5]= [10, 20, 30, 40, 50];
    

    for i in list {
        print!("{i} ");
    }
}