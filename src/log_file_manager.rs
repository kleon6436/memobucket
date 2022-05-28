use std::fs::File;
use std::io::{Write, Read};

pub struct LogFileManager {
    out_file_path: String,
}

impl LogFileManager {
    pub fn new(filepath: &str) -> LogFileManager {
        LogFileManager { out_file_path: filepath.to_string() }
    }

    pub fn write_log(self, message: &str){
       // Open a file in write-only
       let mut file = File::create(self.out_file_path).expect("Create failed.");
       println!("Add message: {:?}", &message);
       file.write_all(message.as_bytes()).expect("Write failed.");
    }

    pub fn read_log(self) -> String {
        let mut file = File::open(self.out_file_path).expect("Open failed");
        let mut data = String::new();
        
        file.read_to_string(&mut data).expect("Failed to read data.");
        println!("Read message: {:?}", &data);
        return data;
    }
}