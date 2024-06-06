use std::path::Path;
use std::io::{self, ErrorKind, Write};
use std::process::exit;
use std::fs::File;

fn main() {
    println!("Money Tracker!");

    line_break();

    let usrname: String = whoami::username();
    println!("Username: {}", usrname);

    //let mut save_file_exist = Command::new("bash");
    //save_file_exist.arg("-c").arg("echo hello BASH").output().expect("Failed to check");

    let data_file_exist: bool = Path::new(&format!("/home/{usrname}/Documents/MSDF")).exists();
    println!("Data File Exists: {}", data_file_exist);
    line_break();

    // MSDF :: Money Saver Data File
    if data_file_exist == false {
        println!("Data File DOES NOT exist!...\nStarting first time start...");
        let _fso = first_start(&usrname);

        println!("\nfso.is_ok: {}", _fso.is_ok() );
        if _fso.is_err() == true {
            println!("well something went wrong...\n");
            // panic!("{}", _fso);
        }
    }

}

fn first_start(usrname: &str) -> std::io::Result<()>{
    line_break();
    println!("Welcome to first start");
    println!("\nHI! My name is Kimmy, I am here to help you save money for an item. Lets get started");
    println!("Type \"yes\" to allow me to create a MSDF file in your Documents directiory. This is so I can remember things.");

    println!("\n(Wanting to make {} ) (y/N)", &format!("/home/{usrname}/Documents/MSDF"));

    let answer = read_line_ms("n".to_string());   

    // Checks if user allowed to cont
    match answer.to_lowercase().as_str() {
        "yes" => println!("Continuing"),
        "y" => println!("Continuing"),
        "" => {
            println!("You just pressed return...");
            println!("Try again.");
            exit(0); 
        },
        _ => {
            println!("That is not a yes.. so breaking out. try again.");
            exit(0);
        }
    }

    // I DONT UNDERSTAND WHY THIS DOESNT WORK! HOW TF DO YOU COMPARE STRINGS!?!?!
    //
    // if answer.as_bytes() != "yes".as_bytes() || answer.as_bytes() != "y".as_bytes() {
    //     println!("sorry, cant continue. breaking out...");
    //     exit(0);
    // }else {
    //     println!("cont")
    // }
    
    println!("creating file...");
    let msdf_file_create_var_res = File::create(&format!("/home/{usrname}/Documents/MSDF"));
    let mut msdf_file_create_var = match msdf_file_create_var_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("\n{:?}, \nThis is an easy fix, Could NOT Find/Open: {}/Documents\n\nPlease Create /Documents\n", error, usrname);

            },
            _ => {
                panic!("Problem creating the file: {:?}", error);
            }
        }
    };
    let _ =msdf_file_create_var.write_all(b"Written from MoneySaver :: This is data saved, you may change it here or from the program.\n");
    println!("file created.");

    line_break();
    // Ask for what to save for and price etc.
    println!("\nnow a couple of questions.. what are you trying to save up for?");
    let  item_to_save_up = read_line_ms("n".to_string());  
    line_break();

    println!("now, how much does it cost?");
    let item_cost = read_line_ms("n".to_string());  
    line_break();

    println!("how much do you have right now?");
    let curr_amount = read_line_ms("n".to_string());  
    line_break();

    //convert &str to i32
    //let item_cost_int: i32 = item_cost.parse().unwrap();
    //let curr_amount_int: i32 = curr_amount.parse().unwrap();
    //println!("DEBUG INT: {} {}", item_cost_int, curr_amount_int); 

    println!("saving into file... {} {} {}", item_to_save_up, item_cost, curr_amount);
    

    let write_data_buffer: String = format!("Item:{}\nPrice:{}\nAmount:{}\n", item_to_save_up, item_cost, curr_amount);
    msdf_file_create_var.write_all(write_data_buffer.as_bytes())?;
    // msdf_file_create_var.write_all(item_cost.as_bytes())?;
    // msdf_file_create_var.write_all(curr_amount.as_bytes())?;


    /*  
        I rememberd I dont need to do this... its just for me...
        
    let match_math_buf: i32 = item_cost_int - curr_amount_int;
    println!("{}", match_math_buf);
    
    let a_buf: f64 = (match_math_buf as f64) * 0.10;
    match match_math_buf {
        m if m < 0 => println!("You can buy it!!"),
        m if m > 0 && m <= (a_buf as i32) => println!("You're about 10 percent away"), // Changed condition here
        _ => {
            println!("Something went wrong on matching numbers...");
        }
    } 
   
    */
    

    Ok(())  

}

fn line_break() {
    println!("=================================");
}

// Dont ask why but it has to be like this... I dont get it...
// read_line_ms("n".to_string());  
fn read_line_ms(_x: String) -> String { 
    let mut answer: String = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read answer...");
    let answer: &str = answer.trim();
    return answer.to_string();
}