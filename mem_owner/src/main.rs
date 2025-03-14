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
    println!();
    println!();

    let string3 = String::from("Memory"); 
    // Here string3 owns the memory allocated for the String initialized
    let (string4, len) = calc_len(string3);
    /* 
        This tuple of string4 and len are initialized to the function of 
        calc_len that takes the value of string3
    */
    println!("The length of {} is {}", string4, len); // prints the values in the tuple individually

    let string5 = String::from("New String");
    let len2 = ref_borrow(&string5); // here we have len2 borrowing the value of string5

    println!("The length of {string5} is {len2}");

    let mut string6 = String::from("Enzoss100");
    change_str(&mut string6);

    let retstring = stringret(); // you can initialize variables with the return value of a function
    println!("{}", retstring);
    
    println!();

    stringslice();
    println!();
    _stringslice2();

    

}



fn calc_len(s: String) -> (String, usize){ 
    // the function expects a String and will return a String and usize
    let length = s.len(); // returns the length of the string obtained by s
    return (s, length) // returns the values of s and length which are String and usize in data type

    /*
        In terms of ownership, the value of string3 is passed to the function
        The function now having ownership of the value can now deal with it as needed

        The function checks through the pointer of string3, and uses the len() function to check 
        the length of the string given in the heap memory (not the capacity)
    */
}

fn ref_borrow(s_borrow: &String)  -> usize {
    s_borrow.len()
    // here we have a function with a borrowed parameter which is a String that will be returning 
    // a usize data type value
}

fn change_str(s_borrow: &mut String) {
    s_borrow.push_str(", The Rust Programmer");
    println!("{}", s_borrow)
    // This function uses a borrowed mutable string
    // prints the mutated referenced string
}

fn stringret() -> String {
    let newstring = String::from("String Returned");

    return newstring;
    // this returns a String data type
    // this kind of returning of values in a function helps avoiding "Dangling Pointers"
    // you can see that there is no reference for the variable newstring
}

fn stringslice() {
    let slice = String::from("New Slice Type");

    let sl1 = &slice[0..3];
    let sl2 = &slice[4..9];
    let sl3 = &slice[10..14];

    println!("{}",sl1); // prints New
    println!("{}",sl2); // prints Slice
    println!("{}",sl3); // prints Type
    println!("{}{}{}", sl1, sl2, sl3); // prints NewSliceType

    /*
        It can be seen in this function that the range of each string slice starting from 0 is n-1

        Example:
            New has 3 characters, from 0 to 2
            but the slice type reads even the character before N
            so 0 to 2 would result in "Ne" instead of "New"

    */

}


fn _stringslice2() {
    let slice = String::from("Testing Word");

    let sl1 = &slice[..4];
    let sl2 = &slice[..7];
    let sl3 = &slice[..8];
    let sl4 = &slice[..9];
    let sl5 = &slice[8..];
    let sl6 = &slice[7..];

    println!("{}",sl1); // Test
    println!("{}",sl2); // Testing
    println!("{}",sl3); // Testing /whitespace/
    println!("{}",sl4); // Testing W
    println!("{}",sl5); // Word
    println!("{}",sl6); // /whitespace/ Word

    /*
        This function shows in action how string slices are printed including the whitespace
        Running this function allows the user to analyse what is being printed out character by character
    */

}