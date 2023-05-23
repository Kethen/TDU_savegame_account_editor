### notes
- currently these offsets were found:
	- save file offsets by inspecting diffs of online/offline decrypted save files against project paradise
		- tdudec (http://aluigi.altervista.org/papers.htm#others-file) was used for savefile decryption, huge thanks to Luigi Auriemma for reversing the encryption and releasing an open source implementation
	- in-memory offsets using ghidra and x64dbg

### commondt.sav
- 0x91: 01 is online, 00 is offline
- 0x98-0xb8: nick name
- 0xba-0xee: email
- 0xf0-0x10f: password

### playersave
- 0x6-0x7: sensitive to nick name and online state, not sure what exactly it does
- 0x8-0xb: hash of nickname
- 0xc-0xf: hash of email
- 0x10-0x13: hash of password
	- see src/util.rs for hashing algorithm
	- empty email/password digests are just 0:u32, used in offline saves
- 0x1a-0x39: nickname

### ProfileList.dat
- starts with header 0x5d
- each profile name starts with 0x00
- has trailer 0xff 0xff 0x58 0x8d 0x00 0x00 0x00 0x00 0x00 0x00

### memory offsets
- 0x0089a730: parsing and checking playersave against nickname, email, password
	- 0x0089a79e: reading nickname, email, password hashes from decrypted save file buffer
		- it is also possible to nop out checking of hashes so that any playersave can be used any commondt.sav
- 0x00624880: hashing nickname, email, password
