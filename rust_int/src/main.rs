
use std::io::{Write, Read};
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};

fn main() {
    // println!("Hello, world!");

    // let mut child = Command::new("../a")
    //                         .stdout(Stdio::piped())
    //                         .spawn()
    //                         .expect("failed to execute child");

    // let ecode = child.wait()
    //                 .expect("failed to wait on child");

    // assert!(ecode.success());
    // let mut stdout = child.stdout.take().expect("Failed to get stdout");
    // let mut resp_string = String::new();
    // stdout.read_to_string(&mut resp_string).expect("Failed to read");
    // println!("{}",resp_string);

    let init = start(true);
    match init {
        Err(err) => println!("error!"),
        Ok(output) => if(output) { println!("success") } else { println!("not init") },
    }
}

fn start(begin: bool) -> Result<bool, std::io::Error> {
    let child = (Command::new("../emg")
                            .stdout(Stdio::piped())
                            .spawn())?;
    let mut s = String::new();
    match child.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => return Err(why),
        Ok(_) => (),
    }
    if s == "coAmpInitializeSuccess"
    {
        return Ok(true);
    }
    else
    {
        return Ok(false);
    }

}
