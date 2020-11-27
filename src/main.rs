// Jacob Sheets
// L6
// Bible Search Project
// 11/21/2020
// Description: Bible search project that searches the bible till user stops with "q" and prints to file.
// Limitations: I could not get the abbreviations to work before the deadline, and was trying to fix before sending in late. Could not break through.

use std::io::{Write};
use std::io::{BufRead, BufReader};
use ::std::io;
use std::process::exit;
// use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;

fn main() {

    let run = true;
    println!("Welcome To Bible Search Tool");
    println!("Rust Version");

    while run == true {

        println!("enter q to quit");
        println!("Book: ");

        // Quit
        let quit_check = input();
        if quit_check == "q" {
            exit(0);
        }

        // Get Book
        // let abbr = abbr_checker(quit_check);
        let book= convert_book(quit_check);
        let full_book = book.to_uppercase();

        // Get Chapter
        println!("Chapter: ");
        let chapter = convert_chapter(book, input());

        // Get Verse
        println!("Verse: ");
        let verse = input();

        // Search Bible
        let answer = search_bible(full_book, chapter, verse);

        // Print to Screen and Verse File
        pretty_print(answer);
    }
}


// Collect input fn
fn input () -> String {
    let mut x = String::with_capacity(5);
    io::stdin().read_line(&mut x).expect("Error reading input");
    let x = x.trim().parse().expect("Error parsing number");
    x
}

// fn abbr_checker(book: String) -> String {
//     let mut abbr = String::new();
//
//     let file_in = fs::File::open("Bible_Abbreviations.csv").unwrap();
//     let reader = BufReader::new(file_in);
//
//     for (_index, line) in reader.lines().enumerate() {
//         let line = line.unwrap(); // Ignore errors.
//         let x = line.split(',').map(|s| s.to_string()).collect();
//         println!("{}",x);
//         if x == book {
//             abbr = line;
//         }
//     }
//
//     return abbr
// }


// Book Formatter & Abbr Finder
fn convert_book(b: String) -> String {

    let bookof = "THE BOOK OF ";
    let book = format!("{}{}", bookof, b);

    book
}

// Chapter Formatter
fn convert_chapter(b: String, c: String) -> String {
    let mut _chapter = String::new();
    if b == "THE BOOK OF PSALMS" {
        let psalm = "PSALM";
        _chapter = format!("{} {}",psalm, c);
    } else {
        let chap = "CHAPTER";
        _chapter = format!("{} {}", chap, c);
    }

    _chapter
}


// Main Search
fn search_bible(b: String, c: String, v: String) -> String {

    let filename = "Bible.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut found_string = String::new();

    let mut found_book = false;
    let mut found_chapter = false;
    let mut found_verse = false;


    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let mut subline = "";

        if line.len() > 12 {
            subline = &line[..11];
        }

        // Verse was not found
        if found_book && subline == "THE BOOK OF " {
            break;
        }

        // Skip over line breaks
        if line== "\n" {
            continue;
        }

        // Find book
        if b == line {
            found_book = true;
        }

        // Find chapter
        if c == line && found_book {
            found_chapter = true;
        }

        // Grab verse number
        let verse_num = line
            .split_whitespace()
            .next()
            .unwrap_or("");

        // Find Verse
        if verse_num == v && found_chapter && found_book {
            found_string = line;
            found_verse = true;
            break;
        }
    }

    let mut _errorcode = String::new();
    // Errors
    if found_book == false {
        _errorcode = String::from("**Book not Found**");
        return _errorcode
    }
    if found_chapter == false {
        _errorcode = String::from("**Chapter not Found**");
        return _errorcode
    }
    if found_verse == false {
        _errorcode = String::from("**Verse not Found**");
        return _errorcode
    } else {
        format!("{}{}:{}", &b[12..], &c[7..], String::from(found_string))

    }
}


// Print Formatter & File Printer
fn pretty_print(text: String) {

    if &text[..2] == "**" {
        println!("{}", text);
        return
    } else {
        let words = text.split_whitespace();
        let mut lines: Vec<String> = Vec::new();
        let mut current: String = String::new();

        for word in words {
            if current.len() + 1 + word.len() > 80 {
                lines.push(current);
                current = String::from(word);
            } else {
                current = current + " " + word;
            }
        }
        lines.push(current);

        for line in lines {
            // Print to file
            println!("{}", line);
            print_to_file(line);
        }
    }
}


// Verse File Printer
fn print_to_file(new_line: String) {
    match OpenOptions::new().create(true).write(true).append(true).open("verses.txt") {
        Ok(ref mut file) => {
            writeln!(file, "{}", new_line).unwrap();
        },
        Err(err) => { panic!("Failed to open log file: {}", err); }
    }
}