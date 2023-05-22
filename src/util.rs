pub fn hash_byte_string(to_be_hashed: &std::vec::Vec<u8>) -> [u8; 4]{
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

pub fn read_commondt(commondt:&std::vec::Vec<u8>) -> Result<PlayerIdentifier, &'static str>{
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

	return Ok(PlayerIdentifier{
		nickname: nickname,
		email: email,
		password: password,
	});
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
