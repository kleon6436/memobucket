use std::{path::Path, fs::{self, File}, io::BufRead};
use chrono::Utc;
use std::io::{Write, BufReader};

pub struct LogFileManager {
    out_file_path: String,
}

impl LogFileManager {
    pub fn new(filepath: &str) -> LogFileManager {
        LogFileManager { out_file_path: filepath.to_string() }
    }

    pub fn write_log(self, message: &str){
        if Path::new(&self.out_file_path).exists() {
            // 存在する場合は、ファイルを開く
            let file = fs::File::open(self.out_file_path).expect("Failed to open file.");
            for line in BufReader::new(file).lines() {
                // Todo: 編集作業
                println!("{}", line.unwrap());
            }
            
        } else {
            // 存在しない場合は、ファイルを新規作成
            let mut file = fs::File::create(self.out_file_path).expect("Failed to create file.");
            println!("Add message: {:?}", &message);

            // 現在の日付を「# 2022-05-28」といった形でファイルに記述
            let mut today = Utc::today().to_string();
            today = today.replace("UTC", "");
            writeln!(file, "# {}", today).unwrap();

            // メッセージをログに書き込む
            write!(file, "- {}", message).unwrap();
        }
    }

    pub fn read_log(self, file_path: &str){
        if !Path::new(file_path).exists() {
            println!("Log file is not found.");
            return;
        }

        let file = fs::File::open(file_path).expect("Failed to open file.");
        show_file(file);
    }

    pub fn show_log(self){
        if !Path::new(&self.out_file_path).exists() {
            println!("Log file is not found.");
            return;
        }

        // ファイルを読み込み、先頭行から順番に表示する
        let file = fs::File::open(self.out_file_path).expect("Failed to open file.");
        show_file(file);
    }
}

fn show_file(file: File){
    for line in BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}