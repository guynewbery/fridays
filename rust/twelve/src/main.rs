fn main() 
{
	let days = 
	[
		"first",
		"second",
		"third",
		"fouth",
		"fifth",
		"sixth",
		"seventh",
		"eighth",
		"ninth",
		"tenth",
		"eleventh",
		"twelveth"
	];
	
	let gifts = 
	[
		"partridge in a pear tree",
		"Two turtle doves",
		"Three French hens",
		"Four calling birds",
		"Five gold rings",
		"Six geese a-laying",
		"Seven swans a-swimming",
		"Eight maids a-milking",
		"Nine ladies dancing",
		"Ten lords a-leaping",
		"Eleven pipers piping",
		"Twelve drummers drumming"
	];
	
	for day in 0..12 
	{
		println!("\nOn the {} day of Christmas my true love sent to me", days[day]);
		
		for gift in (0..day+1).rev() 
		{
			if gift == 0
			{
				if day == 0
				{
					println!("A {}", gifts[gift]);
				}
				else
				{
					println!("And a {}", gifts[gift]);
				}
			}
			else
			{
				println!("{}", gifts[gift]);
			}			
		}
	}
}
