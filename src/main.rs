use std::env;
use std::time::Instant;
use std::io::{self, Write};
use std::process::Command;
fn main() {
    let args = env::args().skip(1);

    let mut last_arg: String = String::new();
    let mut min_input: u64 = 20;

    for arg  in args {        
        if last_arg  == "-t" || last_arg  ==  "-T" {
            min_input = arg.parse().expect("Positive Integer");
        }
        last_arg = arg.clone();
    }

    let now: Instant = Instant::now();
    let mut last_time: u64 = now.elapsed().as_secs();
    println!("oasdjoaj");

    let sec_input = min_input * 60;

    while  now.elapsed().as_secs() < min_input {
        if last_time < now.elapsed().as_secs() {
            //println!("{}/{}", now.elapsed().as_secs(), min_input);
            log_progress_bar(now.elapsed().as_secs(), min_input);
            last_time = now.elapsed().as_secs();
        }
    }

    println!("finished");
    sound_alarm();
}

fn log_progress_bar(prog: u64, lim: u64){
    if prog > lim {};
    let mut use_lim: u64 = lim.clone();
    let mut use_prog: u64 = prog.clone();

    if lim > 20 {
        use_lim = 20;
        use_prog = (prog as f64 * (20.0/lim as f64)) as u64;    
    }
    print!("\x1B[1A"); // Move o cursor uma linha para cima
    print!("\x1B[2K"); // Limpa a linha atual
    print!("[");
    for i in 0..use_lim {
        if i < use_prog{
            print!("#");
        }else{
            print!(" ");
        }
    }
    println!("] {}/{}", prog, lim);

    io::stdout().flush().unwrap();
}

fn sound_alarm() {
    let path_str: String =String::from("/usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga");

    Command::new("ffplay")
    .arg("-nodisp")
    .arg("-autoexit")
    .arg(path_str)
    .output()
    .expect("Error sounding the alarm")
    .stdout;
}
