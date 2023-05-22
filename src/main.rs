mod util;
mod tdudec;

fn main() {
	test_hash("Katie");
	test_commondt_read();
	test_commondt_write();
}

fn test_commondt_write(){
	let commondt = std::fs::read("commondt.sav").unwrap();
	let mut commondt_decrypted = tdudec::decrypt_save(&commondt);
	let player_identifier = util::PlayerIdentifier{
		nickname: String::from("Katie2"),
		email: String::from("katie@katie.inc"),
		password: String::from("hahahaha1234"),
	};
	util::patch_commondrt(&mut commondt_decrypted, &player_identifier, true).unwrap();
	let commondt_modified_encrypted = tdudec::encrypt_save(&commondt_decrypted);
	std::fs::write("commondt.sav.modified", &commondt_modified_encrypted);

	util::patch_commondrt(&mut commondt_decrypted, &player_identifier, false).unwrap();
	let commondt_modified_encrypted = tdudec::encrypt_save(&commondt_decrypted);
	std::fs::write("commondt.sav.modified2", &commondt_modified_encrypted);
}

fn test_commondt_read(){
	let commondt = std::fs::read("commondt.sav").unwrap();
	let commondt_decrypted = tdudec::decrypt_save(&commondt);
	let login = util::read_commondt(&commondt_decrypted).unwrap();
	print!("nickname: {}\n", login.nickname);
	print!("email: {}\n", login.email);
	print!("password: {}\n", login.password);
}

fn test_hash(in_string: &'static str) {
	print!("hashing {}\n", in_string);
	for byte in String::from(in_string).into_bytes(){
		print!("{:x} ", byte);
	}
	print!("\n");
	let test_string = String::from(in_string).into_bytes();
    let hash = util::hash_byte_string(&test_string);
    for byte in hash{
    	print!("{:x} ", byte);
    }
    print!("\n");
}
