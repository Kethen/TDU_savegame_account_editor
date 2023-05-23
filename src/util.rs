fn hash_byte_string(to_be_hashed: &std::vec::Vec<u8>) -> [u8; 4]{
	let mut pad:[u32; 0x100] = [0;0x100];
	let mut uVar2:u32 = 0;
	// tdu 1.66a exe at 0x00622f50
	while uVar2 < 0x100{
		let mut uVar1:u32 = uVar2 >> 1;
	    if (uVar2 & 1) != 0 {
			uVar1 = uVar1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    if (uVar1 & 1) == 0 {
			uVar1 = uVar1 >> 1;
	    }
	    else {
			uVar1 = uVar1 >> 1 ^ 0xedb88320;
	    }
	    pad[usize::try_from(uVar2).unwrap()] = uVar1;
	    uVar2 = uVar2 + 1;
	}

	// tdu 1.66a exe at 0x00624440
	let mut uVar1:u32 = 0xffffffff;
	for byte in to_be_hashed{
		uVar1 = uVar1 >> 8 ^ pad[usize::try_from(u32::try_from(*byte).unwrap() ^ uVar1 & 0xff).unwrap()];
	}

	return (!uVar1).to_be_bytes();
}

pub struct PlayerIdentifier{
	pub nickname: String,
	pub email: String,
	pub password: String,
}

pub fn read_commondt(commondt:&std::vec::Vec<u8>) -> Result<(PlayerIdentifier, bool), &'static str>{
	if commondt.len() < 0x10f{
		return Err("commondt is too small");
	}

	let nickname = match String::from_utf8(commondt[0x98..0xb8].to_vec()){
		Ok(s) => s,
		Err(_) => {return Err("cannot decode nickname from commondt");}
	};
	let email = match String::from_utf8(commondt[0xba..0xee].to_vec()){
		Ok(s) => s,
		Err(_) => {return Err("cannot decode email from commondt");}
	};
	let password = match String::from_utf8(commondt[0xf0..0x10f].to_vec()){
		Ok(s) => s,
		Err(_) => {return Err("cannot decode password from commondt");}
	};

	let online = if commondt[0x91] == 1{
		true
	}else{
		false
	};

	return Ok((PlayerIdentifier{
		nickname: nickname,
		email: email,
		password: password,
	}, online));
}

pub fn patch_commondrt(commondt:&mut std::vec::Vec<u8>, player_identifier: &PlayerIdentifier, online:bool) -> Result<(), &'static str>{
	if commondt.len() < 0x10f{
		return Err("commondt is too small");
	}

	let nickname_bytes = player_identifier.nickname.clone().into_bytes();
	let email_bytes = player_identifier.email.clone().into_bytes();
	let password_bytes = player_identifier.password.clone().into_bytes();

	if nickname_bytes.len() > 30{
		return Err("nickname is longer than 30 bytes");
	}

	if email_bytes.len() > 50{
		return Err("email is longer than 50 bytes");
	}

	if password_bytes.len() > 30{
		return Err("password is longer than 30 bytes");
	}

	for i in 0x98..0xb9{
		commondt[i] = 0;
	}

	for i in 0xba..0xef{
		commondt[i] = 0;
	}

	for i in 0xf0..0x110{
		commondt[i] = 0;
	}

	if online{
		commondt[0x91] = 1;
	}else{
		commondt[0x91] = 0;
	}

	let mut i:usize = 0;
	while i < nickname_bytes.len() {
		commondt[i + 0x98] = nickname_bytes[i];
		i = i + 1;
	}

	if !online{
		return Ok(());
	}

	i = 0;
	while i < email_bytes.len() {
		commondt[i + 0xba] = email_bytes[i];
		i = i + 1;
	}

	i = 0;
	while i < password_bytes.len() {
		commondt[i + 0xf0] = password_bytes[i];
		i = i + 1;
	}

	return Ok(());	
}

pub fn patch_playersave(playersave:&mut std::vec::Vec<u8>, player_identifier:&PlayerIdentifier, online:bool) -> Result<(), &'static str>{
	if playersave.len() < 0x13{
		return Err("playersave is too small");
	}

	let nickname_bytes = player_identifier.nickname.clone().into_bytes();
	let email_bytes = player_identifier.email.clone().into_bytes();
	let password_bytes = player_identifier.password.clone().into_bytes();

	if nickname_bytes.len() > 30{
		return Err("nickname is longer than 30 bytes");
	}

	let mut i:usize = 0;
	for byte in hash_byte_string(&nickname_bytes){
		playersave[i + 0x8] = byte;
		i = i + 1;
	}

	if online{
		i = 0;
		for byte in hash_byte_string(&email_bytes){
			playersave[i + 0xc] = byte;
			i = i + 1;
		}

		i = 0;
		for byte in hash_byte_string(&password_bytes){
			playersave[i + 0x10] = byte;
			i = i + 1;
		}
	}else{
		for i in 0xc..0x14{
			playersave[i] = 0;
		}
	}

	for i in 0x1a..0x3a{
		playersave[i] = 0;
	}

	let mut i:usize = 0;
	for byte in nickname_bytes{
		playersave[i + 0x1a] = byte;
		i = i + 1;
	}

	return Ok(());
}

pub fn read_profile_list(profile_list:&std::vec::Vec<u8>) -> Result<std::vec::Vec<String>, &'static str>{
	if profile_list.len() < 3{
		return Err("profile_list too short");
	}
	let mut i:usize = 2;
	let mut ret:std::vec::Vec<String> = std::vec::Vec::<String>::new();
	let mut profile_name:std::vec::Vec<u8> = std::vec::Vec::<u8>::new();
	while i < profile_list.len(){
		if profile_list[i] == 0 || profile_list[i] == 0xff{
			// eh, just eh, misunderstood the serialization originally
			if profile_list[i] == 0{
				profile_name.pop();
			}
			let new_string = match String::from_utf8(profile_name.clone()){
				Ok(s) => s,
				Err(_) => {return Err("failed decoding profile name");},
			};
			ret.push(new_string);
			profile_name.clear();
		}else{
			profile_name.push(profile_list[i]);
		}
		if profile_list[i] == 0xff{
			return Ok(ret);
		}
		i = i + 1;
	}
	return Err("profile_list ended unexpectedly");
}

pub fn write_profile_list(profile_list:&std::vec::Vec<String>) -> std::vec::Vec<u8>{
	let mut ret:std::vec::Vec<u8> = std::vec::Vec::<u8>::new();
	for (i, string) in profile_list.iter().enumerate(){
		let bytes = string.clone().into_bytes();
		let len = u16::try_from(bytes.len()).unwrap();
		for byte in len.to_le_bytes(){
			ret.push(byte);
		}
		for byte in bytes{
			ret.push(byte);
		}
	}
	ret.append(&mut vec![0xff, 0xff, 0x96, 0x8b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
	return ret;
}
