use std::io::{self, Write};

fn main() {
    println!(
        "================================\nRust Login Simulator\n================================"
    );

    let valid_username: &str = "admin";
    let valid_password: &str = "rust123";
    let max_attempts: u8 = 3;

    let mut attempts: u8 = 0;
    let mut is_authenticated: bool = false;
    let mut is_locked: bool = false;

    let mut username = String::new();
    let mut password = String::new();

    while attempts < max_attempts && !is_authenticated && !is_locked {
        println!();

        username.clear();
        password.clear();

        print!("Username: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let bytes_read = io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        if bytes_read == 0 {
            println!("\nInput closed (EOF). Exiting.");
            break;
        }

        print!("Password: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let bytes_read = io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");
        if bytes_read == 0 {
            println!("\nInput closed (EOF). Exiting.");
            break;
        }

        let username = username.trim();
        let password = password.trim();

        let login: &str = if username == valid_username && password == valid_password {
            is_authenticated = true;
            "Login Successful"
        } else if username.is_empty() || password.is_empty() {
            "Empty Input"
        } else {
            attempts += 1;
            if attempts >= max_attempts {
                is_locked = true;
            }
            "Login Failed"
        };

        let authenticated: &str = if is_authenticated {
            "PASSED"
        } else {
            "NOT PASSED"
        };

        let active: &str = if is_locked { "LOCKED" } else { "ACTIVE" };

        println!();
        if is_authenticated {
            println!("{login}");
            println!("Authentication: {authenticated}");
            println!("System Status: {active}");
            println!("================================");
        } else {
            println!("{login}");
            println!("Attempts: {attempts}/{max_attempts}");

            let count: u8 = max_attempts - attempts;
            println!("Remaining Attempts: {count}");
        }

        if is_locked {
            println!();
            println!("Account Locked");
            println!("Authentication: {authenticated}");
            println!("System Status: {active}");
            println!("================================");
        }
    }
}
