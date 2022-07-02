use chrono::Utc;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub struct LogFileManager {
    out_file_path: String,
}

impl LogFileManager {
    pub fn new(filepath: &str) -> Self {
        LogFileManager {
            out_file_path: filepath.to_string(),
        }
    }

    pub fn write_log(&self, message: &str) {
        if Path::new(&self.out_file_path).exists() {
            let mut lines = VecDeque::new();

            // 存在する場合は、ファイルを開く
            for readline in BufReader::new(fs::File::open(&self.out_file_path).unwrap()).lines() {
                match readline {
                    Ok(line) => {
                        lines.push_back(line);
                    }
                    Err(e) => {
                        // ファイルの読み込み失敗でエラー終了
                        println!("Failed to read file. {e:?}");
                        return;
                    }
                }
            }

            // ヘッダーを作成する
            let mut date_header = "# ".to_string();
            let today = Utc::today().to_string().replace("UTC", "");
            date_header = date_header + &today;

            // 同じ日付のヘッダーが存在するか確認
            let check_result = check_header(&lines, &date_header);
            if check_result.0 {
                // 編集処理
                lines.insert(check_result.1 + 1, "- ".to_string() + message + "");
            } else {
                // 新規にヘッダーも含めて作成する
                // メッセージ => ヘッダーの順に追加する
                lines.push_front("- ".to_string() + message);
                lines.push_front(date_header);
            }

            // ファイルに書き込む
            let remove_result = fs::remove_file(&self.out_file_path);
            if let Ok(_) = remove_result {
                let mut buf_writer = BufWriter::new(fs::File::create(&self.out_file_path).unwrap());
                for line in lines {
                    writeln!(buf_writer, "{}", line).unwrap();
                }
            } else {
                println!("Failed to update log file.");
                return;
            }
        } else {
            // 存在しない場合は、ファイルを新規作成
            let mut buf_writer = BufWriter::new(
                fs::File::create(&self.out_file_path).expect("Failed to create log file."),
            );

            // 現在の日付を「# 2022-05-28」といった形でファイルに記述
            let mut date_header = "# ".to_string();
            let today = Utc::today().to_string().replace("UTC", "");
            date_header = date_header + &today;
            writeln!(buf_writer, "{}", date_header).unwrap();

            // メッセージをログに書き込む
            writeln!(buf_writer, "{}", "- ".to_string() + message).unwrap();

            println!("Add message: {:?}", &message);
        }
    }

    pub fn read_log(&self, file_path: &str) {
        if !Path::new(file_path).exists() {
            println!("Log file is not found.");
            return;
        }

        let file = fs::File::open(file_path).expect("Failed to open file.");
        show_file(file);
    }

    pub fn show_log(&self) {
        if !Path::new(&self.out_file_path).exists() {
            println!("Log file is not found. Path: {}", &self.out_file_path);
            return;
        }

        // ファイルを読み込み、先頭行から順番に表示する
        let file = fs::File::open(&self.out_file_path).expect("Failed to open file.");
        show_file(file);
    }

    pub fn remove_log_file(&self, file_path: &str) {
        let remove_result = fs::remove_file(file_path);
        if let Ok(_) = remove_result {
            println!("Success to remove log file. Path: {}", file_path);
        } else {
            println!("Failed to update log file. Path: {}", file_path);
        }
    }
}

fn show_file(file: File) {
    for line in BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}

fn check_header(lines: &VecDeque<String>, header: &str) -> (bool, usize) {
    let mut is_same = false;
    let mut same_header_num = 0;
    let mut count: usize = 0;

    for line in lines {
        if line.eq(header) {
            // 同じものがあれば、フラグ更新
            is_same = true;
            same_header_num = count;
        }

        count += 1;
    }

    return (is_same, same_header_num);
}
