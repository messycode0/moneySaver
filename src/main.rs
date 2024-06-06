use std::path::Path;
// use std::io;

pub mod first_start;
pub mod util;

fn main() {
    println!("Money Tracker!");

    util::line_break();
    println!("Start-up Data");

    let usrname: String = whoami::username();
    println!("Username: {}", usrname);

    //let mut save_file_exist = Command::new("bash");
    //save_file_exist.arg("-c").arg("echo hello BASH").output().expect("Failed to check");

    let data_file_exist: bool = Path::new(&format!("/home/{usrname}/Documents/MSDF")).exists();
    println!("Data File Exists: {}", data_file_exist);
    println!("Documents Directory Exists: {}", Path::new(&format!("/home/{usrname}/Documents/")).exists());
    util::line_break();

    // MSDF :: Money Saver Data File
    if data_file_exist == false {
        println!("Data File DOES NOT exist!...\nStarting first time start...");
        let _fso = first_start::first_start(&usrname);

        println!("\nfso.is_ok: {}", _fso.is_ok() );
        if _fso.is_err() == true {
            println!("well something went wrong...\n");
            // panic!("{}", _fso);
        }
    }

}

