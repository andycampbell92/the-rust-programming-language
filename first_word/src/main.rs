use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter phrase");
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: usize = first_word(&n);
    println!("first word ends at index {}", n);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if(item == b' ') {
            return i;
        }
    }
    s.len()
}
