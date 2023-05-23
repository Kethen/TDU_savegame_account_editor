mod util;
mod tdudec;

fn main() {
	test_commondt_read();
	test_commondt_write();
	test_playersave_write();
	test_profilelist_read_write();
}

fn test_commondt_write(){
	let commondt = std::fs::read("commondt.sav").unwrap();
	let mut commondt_decrypted = tdudec::decrypt_save(&commondt);
	let player_identifier = util::PlayerIdentifier{
		nickname: String::from("Katie3"),
		email: String::from("katie@katie.inc"),
		password: String::from("12345678"),
	};
	util::patch_commondrt(&mut commondt_decrypted, &player_identifier, true).unwrap();
	let commondt_modified_encrypted = tdudec::encrypt_save(&commondt_decrypted);
	std::fs::write("commondt.sav.modified", &commondt_modified_encrypted);

	let commondt = std::fs::read("commondt.sav").unwrap();
	let mut commondt_decrypted = tdudec::decrypt_save(&commondt);

	util::patch_commondrt(&mut commondt_decrypted, &player_identifier, false).unwrap();
	let commondt_modified_encrypted = tdudec::encrypt_save(&commondt_decrypted);
	std::fs::write("commondt.sav.modified2", &commondt_modified_encrypted);
}

fn test_playersave_write(){
	let playersave = std::fs::read("playersave").unwrap();
	let mut playersave_decrypted = tdudec::decrypt_save(&playersave);
	let player_identifier = util::PlayerIdentifier{
		nickname: String::from("Katie3"),
		email: String::from("katie@katie.inc"),
		password: String::from("12345678"),
	};

	util::patch_playersave(&mut playersave_decrypted, &player_identifier, true).unwrap();
	let playersave_modified_encrypted = tdudec::encrypt_save(&playersave_decrypted);
	std::fs::write("playersave.modified", &playersave_modified_encrypted);

	let playersave = std::fs::read("playersave").unwrap();
	let mut playersave_decrypted = tdudec::decrypt_save(&playersave);

	util::patch_playersave(&mut playersave_decrypted, &player_identifier, false).unwrap();
	let playersave_modified_encrypted = tdudec::encrypt_save(&playersave_decrypted);
	std::fs::write("playersave.modified2", &playersave_modified_encrypted);
}

fn test_commondt_read(){
	let commondt = std::fs::read("commondt.sav").unwrap();
	let commondt_decrypted = tdudec::decrypt_save(&commondt);
	let login = util::read_commondt(&commondt_decrypted).unwrap();
	print!("nickname: {}\n", login.nickname);
	print!("email: {}\n", login.email);
	print!("password: {}\n", login.password);
}

fn test_profilelist_read_write(){
	let profile_list1 = std::fs::read("ProfileList.dat").unwrap();
	let profile_list2 = std::fs::read("ProfileList.dat2").unwrap();

	let profile_list1 = util::read_profile_list(&profile_list1).unwrap();
	let profile_list2 = util::read_profile_list(&profile_list2).unwrap();

	let profile_list1 = util::write_profile_list(&profile_list1);
	let profile_list2 = util::write_profile_list(&profile_list2);

	std::fs::write("ProfileList.dat.rewritten", &profile_list1);
	std::fs::write("ProfileList.dat2.rewritten", &profile_list2);
}
