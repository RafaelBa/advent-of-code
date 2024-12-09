use std::fs;

pub fn day(n: u8) -> String {
    let file_path = format!("{}/day_{:02}.txt", BASE_PATH, n);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

static BASE_PATH: &str = "inputs";
