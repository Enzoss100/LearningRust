fn main() {
    let mut new_string = String::from("hello");
    /* 
        this String initializes the word "hello", using to dynamically allocate the string value to
        heap memory
    */
    new_string.push_str(" world!"); // appends a literal to a String initialized variable
    println!("{}", new_string);
}
