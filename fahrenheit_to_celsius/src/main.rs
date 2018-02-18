use std::io;

fn to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * (5.0/9.0)
}

fn get_fahrenheit_from_user() -> f32 {
    println!("Enter temperature in farenheit");

    let mut farenheit = String::new();

    io::stdin().read_line(&mut farenheit)
        .expect("Failed to read line");

    let farenheit: f32 = farenheit.trim().parse()
        .expect("Please enter a number");
    farenheit

}

fn main() {
    let farenheit: f32 = get_fahrenheit_from_user();
    println!("{}°F == {}°C", farenheit, to_celsius(farenheit));
}
