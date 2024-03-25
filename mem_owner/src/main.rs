fn main() {
    let mut new_string = String::from("hello");
    /* 
        this String initializes the word "hello", using to dynamically allocate the string value to
        heap memory
    */
    new_string.push_str(" world!"); // appends a literal to a String initialized variable
    println!("{}", new_string);

    let string1 = String::from("Hello");
    let string2 = string1.clone();
    /*
        without the .clone() function to copy data from string1
        the compiler thinks that the memory of data allocated for string1 is destroyed
        and it also thinks that the data from string1 was moved to string2

        if you run this without .clone(), you will see a compiler error that states that a value
        is being borrowed from a place where data has been moved already

        Ex:
         
        let string1 = String::from("Hello");
        let string2 = string1;
    
    */
    println!("{}, {} World!", string1, string2);
}
