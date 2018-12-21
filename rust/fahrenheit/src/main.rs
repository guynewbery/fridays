use std::io;

fn main() {
    println!("Fahrenheit to Celsius");
	
	loop {
		println!("Please enter a temperature in Fahrenheit");

		let mut fahrenheit = String::new();

		io::stdin().read_line(&mut fahrenheit)
			.expect("Failed to read line");
		
		let fahrenheit: i32 = match fahrenheit.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		let celsius = (fahrenheit - 32) * 5/9;
		
		println!("{}°F = {}°C", fahrenheit, celsius);
	}
}
