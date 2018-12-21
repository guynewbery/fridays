fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
	
	for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}