mod util;

fn main() {
	test("Katie");
}

fn test(in_string: &'static str) {
	print!("hashing {}\n", in_string);
	let test_string = String::from(in_string).into_bytes();
    let hash = util::hash_byte_string(&test_string);
    for byte in hash{
    	print!("{:x} ", byte);
    }
    print!("\n");
}
