use std::io;

pub fn line_break() {
    println!("=================================");
}

// Dont ask why but it has to be like this... I dont get it...
// read_line_ms("n".to_string());  
pub fn read_line_ms(_x: String) -> String { 
    let mut answer: String = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read answer...");
    let answer: &str = answer.trim();
    return answer.to_string();
}