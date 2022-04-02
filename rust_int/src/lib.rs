use std::{thread, time};
use std::thread::JoinHandle;
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::io::{Write, Read, BufReader, BufRead};
use std::io::Error as StdError;
use std::vec::Vec;

pub struct EMG_INTEGRATION {
    pipe: std::process::ChildStdout,
}


trait Get_Data{
    fn get_data(&self, data_num: u8) -> Vec<[u8; 8]>;
}


impl EMG_INTEGRATION{
    pub fn new(&self) -> Result<EMG_INTEGRATION, StdError>{
        let mut child = Command::new("../emg")
                            .stdout(Stdio::piped())
                            .stdin(Stdio::piped())
                            .spawn()?;
        
        Ok( 
                EMG_INTEGRATION{
                    pipe: child.stdout.take().expect("Failed to get stdout")
                },
        )            
    }

}

impl Get_Data for EMG_INTEGRATION{
    fn get_data(&self, data_num: u8) -> Vec<[u8; 8]> {
        let data = Vec::new();
        let mut pipe = &(self.pipe);
        let mut f = BufReader::new(pipe);


        return data;
    }
}