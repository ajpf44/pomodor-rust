use std::process::Command;
use std::env;
use std::iter::FromIterator;
use std::fs::File;
use std::io::prelude::*;
/*TODO: 
    - listar os diretórios da home
*/

fn main() {
    let home_dir: String = env::var("HOME").expect("Error getting home env variable");

    let output = Command::new("ls")
    .arg(home_dir.clone())
    .arg("-a")
    .output()
    .expect("Error listing directories");

    let user_str: String =  String::from_utf8(output.stdout).expect("Error parsing a string from vec8");
    let vec_dir: Vec<&str> = Vec::from_iter(user_str.split("\n"));
    
    let var_dir_str: &str = ".var";
    let var_path = home_dir + "/" + var_dir_str;
    if !vec_dir.contains(&var_dir_str) {
        Command::new("mkdir")
        .arg(var_path.clone())
        .output()
        .expect("Error creating var_dir_str");
        
        println!("{var_dir_str} created");
    }else{
        println!("{var_dir_str} already is created");
    }   

    let new_output = Command::new("ls")
    .arg(var_path.clone())
    .arg("")
    .output()
    .expect("Error listing directories");

    let var_dirs = String::from_utf8(new_output.stdout).expect("error parsing");
    let var_dirs_arr = Vec::from_iter(var_dirs.split("\n"));

    let pomo_path = var_path.clone() + "/" + "pomodors";
    if !var_dirs_arr.contains(&"pomodors") {
        Command::new("mkdir")
        .arg(pomo_path.clone())
        .output()
        .expect("Error creating pomodors dir");
        println!("pomodors dir created on .var");
    } else{
        println!("pomodors already existis on .var");
    }

    println!("{pomo_path}/");

    let previous_content: String = read_file_to_string(pomo_path.clone());
    let task_to_write: &str = "3 - continou a desenvolver pomodors, modos de editar arquivos\n";

    let content:String = previous_content + task_to_write;

    if write_file(pomo_path.clone(), content).is_ok() {
        println!("writted")
    } else{
        println!("Error writting a file")
    }
}


//write a new content in a file
//the previous content will be lost
fn write_file(path: String, content: String) -> std::io::Result<()> {
    let file_path = path + "/" + &get_date().trim() + ".txt";
    let mut file: File = File::create(file_path).expect("Error opening file");
    file.write(content.as_bytes())?;
    Ok(())
}

fn read_file_to_string(path: String) -> String { 

    let file_path = path + "/" + &get_date().trim() + ".txt";
    let mut file: File = File::open(file_path).expect("Error opening file");

    let mut prev_content = String::new();
    file.read_to_string(&mut prev_content).expect("Error reading previous content to stirng");

    prev_content
    
}

//get date in a format YYYY-MM-DD
fn get_date() -> String {
    let cmd = Command::new("date")
    .arg("+%F")
    .output().expect("Error using native command date");

    String::from_utf8(cmd.stdout).expect("Error parsing the string")
}
