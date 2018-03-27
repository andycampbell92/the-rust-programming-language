use std::io;

fn main() {
    let mut phrase = String::new();

    println!("Enter phrase");
    io::stdin().read_line(&mut phrase)
        .expect("Failed to read line");

    let mut n = String::new();

    println!("Enter word count");
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: usize = n.trim().parse()
        .expect("Please enter a number");

    let s: &str = n_th_word(&phrase, n);
    println!("The world number {} in the phrase is {}", n, s);
}

fn n_th_word(s: &String, n: usize) -> &str {
    let bytes = s.as_bytes();
    let mut word_count: usize = 1;
    let mut start_index: usize = 0;
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if word_count == n {
                return &s[start_index..i];
            }
            else {
                start_index = i + 1;
                word_count = word_count + 1;
            }
        }
    }

    if word_count == n {
        &s[start_index..]
    } else {
        panic!("Word {} not in phrase", n);
    }
}
