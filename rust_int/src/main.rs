
use std::io::{Write, Read};
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::vec::Vec;

static SUCCESS_MSG: &'static str = "coAmpInitializeSuccess";

fn main() {
    let init = start(true);

    match init {
        Err(err) => println!("error!"),
        Ok(output) => (),
    }

    let mut v: Vec<i32> = Vec::new();
    

}

fn start(begin: bool) -> Result<(), std::io::Error> {
    let child = (Command::new("../emg")
                            .stdout(Stdio::piped())
                            .spawn())?;

    let mut s = String::new();

    match child.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => return Err(why),
        Ok(_) => (),
    }

    println!("{}", s);

    println!("{:?}", save_data( String::from(s) ));
    return Ok(());
}


fn save_data(s: String) -> [u32; 8]{
    let mut data: [u32; 8] = [0; 8];
    let mut counter = 0;
    let zero_ASCII: u32 = ('0') as u32;
    for char in s.chars(){
        data[counter] = (char as u32) - zero_ASCII;
        counter += 1;
    }
    return data;
}




// fn start(begin: bool) -> Result<bool, std::io::Error> {
//     let child = (Command::new("../emg")
//                             .stdout(Stdio::piped())
//                             .spawn())?;

//     let mut s = String::new();

//     match child.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => return Err(why),
//         Ok(_) => (),
//     }

//     if s == SUCCESS_MSG
//     {
//         return Ok(true);
//     }
//     else
//     {
//         return Ok(false);
//     }

// }