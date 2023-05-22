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
