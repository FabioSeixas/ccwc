use std::env::args;
use std::fs;
use std::str;

// fn main() {
//     let mut user_input = String::new();
//
//     let stdin = io::stdin();
//
//     match stdin.read_line(&mut user_input) {
//         Ok(n) => {
//             println!("{n} bytes read");
//             println!("{user_input}");
//         }
//         Err(error) => println!("error: {error}")
//     };
//
// }

fn main() {
    // let argss = args();
    // let maax = argss.max().expect("some error");
    //
    // println!("hey {maax}")
    //
    // for argument in args() {
    //     println!("hey {argument}")
    // }
    let my_collection: Vec<String> = args().collect();
    if my_collection.len() < 2 {
        return;
    }
    let command = &my_collection[1];
    let target = &my_collection[2];

    println!("command: {command}");
    println!("target: {target}");

    if command == "-l" {
        let lines_count = count_input_lines(&target);
        println!("number of lines: {lines_count}")
    }

    if command == "-c" {
        let bytes_count = count_bytes(&target);
        println!("bytes: {bytes_count}")
    }

    if command == "-w" {
        let words_count = count_words(&target);
        println!("words: {words_count}")
    }
}

fn get_parsed_buffer_and_size(x: &str) -> (String, usize) {
    let buffer = read_target(&x.to_string());
    let parsed = parse_to_uft8(&buffer);
    (String::from(parsed), buffer.len())
}

fn count_bytes(x: &str) -> usize {
    let (_, buffer_len) = get_parsed_buffer_and_size(x);
    buffer_len
}

fn count_input_lines(x: &str) -> usize {
    let (parsed, _) = get_parsed_buffer_and_size(&x);
    let lines_vec = split_lines(&parsed);
    lines_vec.len() - 1
}

fn count_words(x: &String) -> usize {
    let (parsed, _) = get_parsed_buffer_and_size(&x);
    split_and_count_words(&parsed)
}

fn read_target(target: &String) -> Vec<u8> {
    fs::read(target).expect("Error while reading the file")
}

fn parse_to_uft8(x: &Vec<u8>) -> &str {
    str::from_utf8(x).expect("error while parsing file content")
}

fn split_lines(x: &str) -> Vec<&str> {
    x.split("\n").collect()
}

fn split_and_count_words(x: &str) -> usize {
    let mut words_count = 0;
    let lines = split_lines(&x);
    for line in lines {
        let line_len = line.len();
        if line_len <= 1 {
            continue;
        };
        println!("{line}");
        println!("{line_len}");
        let c: Vec<&str> = line.trim().split(" ").collect();
        words_count += c.len();
    }
    words_count
}
