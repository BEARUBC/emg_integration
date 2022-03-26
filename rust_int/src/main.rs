use std::{thread, time};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::io::{Write, Read, BufReader, BufRead};
use std::vec::Vec;

static SUCCESS_MSG: &'static str = "coAmpInitializeSuccess";

fn main() {
    let init = start(true);

    match init {
        Err(err) => println!("error!"),
        Ok(output) => (),
    }
    
}

fn start(begin: bool) -> Result<(), std::io::Error> {
    let mut child = Command::new("../emg")
                            .stdout(Stdio::piped())
                            .stdin(Stdio::piped())
                            .spawn()?;

    let mut stdout = child.stdout.take().expect("Failed to get stdout");
    let mut s = String::new();

    let mut f = BufReader::new(stdout);
    




    loop {
        f.read_line(&mut s).unwrap();
        println!("{}", s);
    };

        println!("finished iteration");
        // stdout.read_exact(&mut bytes).unwrap();
        // println!("{:?}", bytes);

        // println!("{}", s);

        // if !s.is_empty(){
        //     println!("not empty");
        //     println!("{}", s);
        // }
    


    // let read_thread = thread::spawn(move || {
    //     loop{
    //         let mut resp_string = String::new();

    //         stdout.read_to_string(&mut resp_string).expect("Failed to read");

    //         if !resp_string.is_empty(){
    //             println!("not empty");
    //             println!("{}", resp_string);
    //         }
    //     }
    // });

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