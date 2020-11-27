use std::fs;
use std::collections::HashMap;
use std::io;
use std::io::{BufReader, BufRead};

fn main() {

    println!("Hello, world!");
    let abbr  = open_abbr("Bible_Abbreviations.csv");

}


// Open bible text and put into array that can be stepped through line by line
fn open_bible(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);

    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn open_abbr(filename: &str) -> HashMap<String, String> {
    let mut abbr: HashMap<String, String> = HashMap::new();

    let file_in = fs::File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);

    for line in file_reader.lines() {
        let s:String = line.unwrap();
        let s_slice: &str = &s[..];
        let key_value_pair:Vec<String> = s_slice.split(',').map(|s| s.to_string()).collect();
        abbr.insert(key_value_pair[0], key_value_pair[1]);
    }

    return abbr
}

fn bible_search(b: &str, c: &str, v: &str) {
    let bible = open_bible("Bible.txt");
    // let book = convert_book(b);
    // let chapter = convertChapter(book, c);
    let book_of = String::from("THE BOOK OF ");

    let mut foundBook: bool = false;
    let mut foundChapter: bool = false;
    let mut foundVerse:bool = false;

    for mut line in bible.unwrap() {
        let startLine = String::from(line.truncate(13));

        // Break if found the next book or EOF
        if foundBook && startLine == book_of || "" == line {

        }
    }
}






fn convert_chapter(b: &str, c: &str) -> String {
    let mut chapter:String = String::new();

    if b == "THE BOOK OF PSAMS" {
        chapter = String::from("Psalms");
        chapter.insert_str(0, c);
    } else {
        chapter = String::from("CHAPTER");
        chapter.insert_str(0, c);
    }
    chapter
}


fn pretty_print(text: String) {

    // Print to File
    let words = text.split_whitespace();
    let mut lines:Vec<String> = Vec::new();
    let mut current:String = String::new();

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
        print!("{}", line);
    }
}