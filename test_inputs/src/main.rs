use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;
use std::process::Command;
use std::path::Path;
use std::collections::HashSet;

// Define label1 and label2 outside any function
static mut LABEL1: String = String::new();
static mut LABEL2: String = String::new();

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
    let mut attempts = 3;

    loop {
        print!("Enter the file name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("Failed to read line");

        let filename = filename.trim();

        if !filename.ends_with("_fromRust") {
            println!("Filename must end with '_fromRust'.");
            attempts -= 1;
            if attempts == 0 {
                println!("Exceeded maximum attempts. Exiting the program.");
                process::exit(0);
            }
            println!("Attempts remaining: {}", attempts);
            continue;
        }

        // Remove the .txt extension if present
        let filename = if filename.ends_with(".txt") {
            &filename[..filename.len() - 4]
        } else {
            filename
        };

        if !Path::new(&format!("{}.txt", filename)).exists() {
            println!("File '{}' is not present near this program!", filename);
            println!("Would you like to create a new file instead? (1)");
            println!("Would you like to exit the program instead?  (2)");
            print_separator(40);

            print!("Enter an Option: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut option = String::new();
            io::stdin().read_line(&mut option).expect("Failed to read line");
            let option = option.trim();

            match option {
                "1" => {
                    create_new_file();
                    return;
                }
                "2" => {
                    exit_program();
                }
                _ => {
                    println!("Invalid option!");
                    continue;
                }
            }
        }

        println!("Please ensure that the file has a proper name convention for this program");
        println!("(example_fromRust.txt)");
        println!();

        match fs::read_to_string(&format!("{}.txt", filename)) {
            Ok(file_content) => {
                println!("====================================================");
                println!("Contents of the file  : {}.txt", filename);
                println!("{}", file_content);
                println!("====================================================");

                let mut labels = HashSet::new();
                for line in file_content.lines() {
                    let parts: Vec<&str> = line.trim().split(':').map(|s| s.trim()).collect();
                    if parts.len() >= 2 {
                        labels.insert(parts[0]);
                    }
                }

                if !labels.is_empty() {
                    println!("What would you like to do to this file?");
                    println!("[1] Add more entries");
                    println!("[2] Change Label Names");
                    println!("[3] Delete the File");

                    for (index, label) in labels.iter().enumerate() {
                        println!("[{}] Delete Entries by {}", index + 4, label);
                    }
                    print_separator(40);

                    print!("Enter an Option: ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    let mut option = String::new();
                    io::stdin().read_line(&mut option).expect("Failed to read line");
                    let option = option.trim();

                    // Consume the newline character
                    let _ = io::stdin().read_line(&mut String::new()).expect("Failed to read line");

                    // Continue with your logic based on the option
                } else {
                    println!("Invalid file format: Unable to determine labels.");
                    continue;
                }
            }
            Err(err) => {
                println!("Failed to read file '{}.txt': {}", filename, err);
                attempts -= 1;
                if attempts == 0 {
                    println!("Exceeded maximum attempts. Exiting the program.");
                    process::exit(0);
                }
                println!("Attempts remaining: {}", attempts);
            }
        }
    }
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

    let (label1, label2) = get_labels();
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

        let _ = writeln!(file, "{:<width$} : {}", label1, input_label1.trim(), width = max_label_length);
        let _ = writeln!(file, "{:<width$} : {}", label2, input_label2.trim(), width = max_label_length);
        let _ = writeln!(file);

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

                for (entry1, entry2) in &entries {
                    println!("{:<width$} : {}", label1, entry1, width = max_label_length);
                    println!("{:<width$} : {}", label2, entry2, width = max_label_length);
                    println!();
                }

                println!("Exiting to Main Menu...");
                thread::sleep(Duration::from_secs(5));

                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

fn get_label(order: &str) -> String {
    print!("Enter the {} Label: ", order);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut label = String::new();
    io::stdin().read_line(&mut label).expect("Failed to read line");
    label.trim().to_owned()
}

fn get_labels() -> (String, String) {
    let label1 = get_label("First");
    let label2 = get_label("Second");
    (label1.clone(), label2.clone())
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
    println!("Version 1.4");
    println!();
    print_separator(41);
    println!("Please Select One(1) of the Given Options");
    println!("1 - Back to Main Menu\n2 - Exit Program");
    print_separator(41);
    println!();
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
    println!();
    print_separator(41);
    println!("Please Select One(1) of the Given Options");
    println!("1 - Back to Main Menu\n2 - Exit Program");
    print_separator(41);
    println!();
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
