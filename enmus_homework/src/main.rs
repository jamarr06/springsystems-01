use std::io;
use std::process::Command;

// enum for the file operations
enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

// function that runs the commands
fn perform_operation(operation: FileOperation) {
    match operation {

        FileOperation::List(path) => {
            Command::new("ls")
                .arg(path)
                .status()
                .expect("failed to run ls");
        }

        FileOperation::Display(path) => {
            Command::new("cat")
                .arg(path)
                .status()
                .expect("failed to run cat");
        }

        FileOperation::Create(path, content) => {
            let cmd = format!("echo '{}' > {}", content, path);

            let result = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status();

            match result {
                Ok(_) => println!("file created"),
                Err(_) => println!("could not create file"),
            }
        }

        FileOperation::Remove(path) => {
            let result = Command::new("rm")
                .arg(path)
                .status();

            match result {
                Ok(_) => println!("file removed"),
                Err(_) => println!("could not remove file"),
            }
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("failed to run pwd");
        }
    }
}

fn main() {

    println!("Welcome to the File Operations Program!");

    loop {

        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("Enter your choice (0-5):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {

            "1" => {
                println!("Enter directory path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::List(path.trim().to_string()));
            }

            "2" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Display(path.trim().to_string()));
            }

            "3" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                println!("Enter content:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                perform_operation(FileOperation::Create(
                    path.trim().to_string(),
                    content.trim().to_string(),
                ));
            }

            "4" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                perform_operation(FileOperation::Remove(path.trim().to_string()));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option, try again.");
            }
        }
    }
}