use std::io;
use regex::Regex;

fn main() {
	let mut math_string = String::new();
	println!("Please input the math expression to parse: ");
	io::stdin().read_line(&mut math_string);
	do_math(&math_string.trim());
}

fn do_math(test_string: &str){
	println!("parsing string \"{}\".", test_string);
	let regex_string = Regex::new("[a-zA-Z!@#$%&()^,.<>?_=`~;:'\"\\[\\]{}|]").unwrap();
	let regex_digits_only = Regex::new("\\D").unwrap();
	let parsed_string = regex_string.replace_all(test_string, "");
	let split_iterator: Vec<&str> = regex_digits_only.split(&parsed_string).collect();
	for x in split_iterator {
		println!("{:?}", x.trim());
	}
}