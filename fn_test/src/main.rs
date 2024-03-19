fn main() {
    // call the new_func function
    new_func();
    param_func(10,20);
}


//this new_func is a new function that prints a string
fn new_func(){
    println!("This is a new function");
}

fn param_func(x: i32, y: i32){
    println!("{x} is the first paramenter passed as an argument in this function");
    println!("{y} is the second parameter passed to this function");
}