use std::io;

fn nth_number_in_fib(mut n: usize) -> u32 {
	let mut prev = [0, 1];
	while n > 1 {
		let next: u32 = prev[0] + prev[1];
		prev[0] = prev[1];
		prev[1] = next;
		n = n - 1;
	}
	prev[n]
}

fn get_n_from_user() -> usize {
    println!("Enter sequence index");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: usize = n.trim().parse()
        .expect("Please enter a number");
    n
}

fn main() {
	let n: usize = get_n_from_user();
    println!("fibonacci({}) == {}", n, nth_number_in_fib(n));
}
