use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() {
    loop {
        clear_screen();
        println!("Welcome to Test Inputs!");
        println!("Please Select One(1) of the Given Options:");
        println!("1 - Start program\n2 - Exit program\n3 - About page\n4 - Updates");
        print_separator(41); // Separator length matches the longest option text length
        let mut choice = String::new();
        println!(); // Break line before taking user input
        print!("Enter an Option: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => {
                start_program();
            },
            "2" => {
                exit_program();
            },
            "3" => {
                about_page();
            },
            "4" => {
                updates_page();
            },
            _ => println!("Invalid option!"),
        }
    }
}

fn clear_screen() {
    if cfg!(windows) {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

fn print_separator(len: usize) {
    // Print a separator line
    println!("{}", "-".repeat(len));
}

fn start_program() {
    clear_screen();
    println!("This program will ask you for user inputs and put them into a text file.");

    println!("1 - Create New File\n2 - Edit Existing File\n3 - Return to Main Page");
    print_separator(37); // Separator length matches the longest option text length
    let mut choice = String::new();
    println!();
    print!("Enter an Option: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim() {
        "1" => create_new_file(),
        "2" => edit_existing_file(),
        "3" => {},
        _ => {
            println!("Invalid option!");
            start_program();
        }
    }
}

fn edit_existing_file() {
    // Logic for editing an existing file goes here
    println!("Editing existing file...");
}


fn create_new_file() {
    clear_screen();
    let mut filename = String::new();
    print!("Enter filename: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut filename).expect("Failed to read line");
    let filename = filename.trim().to_owned() + "_fromRust.txt";

    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create file: {}", e);
            start_program();
            return;
        }
    };

    // Prompt for the first and second labels
    print!("Enter the First Label: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut label1 = String::new();
    io::stdin().read_line(&mut label1).expect("Failed to read line");
    let label1 = label1.trim().to_owned();

    print!("Enter the Second Label: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut label2 = String::new();
    io::stdin().read_line(&mut label2).expect("Failed to read line");
    let label2 = label2.trim().to_owned();

    // Determine the length of the longest label
    let max_label_length = label1.len().max(label2.len());

    let mut entries: Vec<(String, String)> = Vec::new(); // Store added entries

    loop {
        print!("Enter your input for {}: ", label1);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input_label1 = String::new();
        io::stdin().read_line(&mut input_label1).expect("Failed to read line");

        print!("Enter your input for {}: ", label2);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input_label2 = String::new();
        io::stdin().read_line(&mut input_label2).expect("Failed to read line");

        // Write the entries to the file with aligned colons
        let _ = writeln!(file, "{:<width$} : {}", label1, input_label1.trim(), width = max_label_length);
        let _ = writeln!(file, "{:<width$} : {}", label2, input_label2.trim(), width = max_label_length);
        let _ = writeln!(file); // Insert a blank line after each entry

        // Add entries to the vector
        entries.push((input_label1.trim().to_owned(), input_label2.trim().to_owned()));

        println!();
        println!("Do you want to add more entries? (Y/N)");
        print!("Enter an Option: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut add_more = String::new();
        io::stdin().read_line(&mut add_more).expect("Failed to read line");
        let add_more = add_more.trim().to_lowercase();

        match add_more.as_str() {
            "y" => continue,
            "n" => {
                println!("Entries added successfully!");
                println!();

                // Print added entries
                for (entry1, entry2) in &entries {
                    println!("{:<width$} : {}", label1, entry1, width = max_label_length);
                    println!("{:<width$} : {}", label2, entry2, width = max_label_length);
                    println!(); // Insert a blank line after each entry
                }

                println!("Exiting to Main Menu...");

                // Wait for 5 seconds
                thread::sleep(Duration::from_secs(5));

                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}






fn exit_program() {
    clear_screen();
    println!("Exiting program...");
    thread::sleep(Duration::from_secs(3));
    process::exit(0);
}

fn about_page() {
    clear_screen();
    println!("By Enzoss100");
    println!("Programmed in: Rust Programming language");
    println!("Version 1.4"); // Changed version
    println!(); // Added break line
    print_separator(41); // Separator length matches the longest option text length
    println!("Please Select One(1) of the Given Options"); // Added message
    println!("1 - Back to Main Menu\n2 - Exit Program");  
    print_separator(41); // Added separator  
    println!(); // Added break line
    loop {
        print!("Enter an Option: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => break,
            "2" => exit_program(),
            _ => println!("Invalid option!"),
        }
    }
}

fn updates_page() {
    clear_screen();
    println!("Updates:");
    println!("Program Version 1.4");
    println!("Changes:");
    println!("1. Added user input line descriptor for all inputs");
    println!(); // Added break line
    print_separator(41); // Separator length matches the longest option text length
    println!("Please Select One(1) of the Given Options"); // Added message
    println!("1 - Back to Main Menu\n2 - Exit Program");
    print_separator(41); // Added separator
    println!(); // Added break line
    loop {
        print!("Enter an Option: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => break,
            "2" => exit_program(),
            _ => println!("Invalid option!"),
        }
    }
}
