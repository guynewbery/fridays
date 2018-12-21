use std::io;

fn main() {
    println!("Fibonacci!");
	
	loop {
		println!("Please enter a number");

		let mut input = String::new();

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");
		
		let input: i32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		let mut loops = input - 2;		
		let mut fib = 1;
		let mut fibprev = 1;
		
		while loops > 0 {
			let temp = fib + fibprev;
			fibprev = fib;
			fib = temp;
			loops -= 1;
		}
		
		println!("The {}th number of the Fibonacci sequence is {}", input, fib);
	}
}
