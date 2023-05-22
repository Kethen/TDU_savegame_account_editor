### notes
- currently these offsets were found inspecting diffs of online/offline decrypted save files against project paradise
- online/offline state on commondt.sav and playersave has to match, or it will not load

### commondt.sav
- 0x91: 01 is online, 00 is offline
- 0x98-0xb8: nick name
- 0xba-0xee: email
- 0xf0-0x10f: password

### playersave
- 0x6-0x7: sensitive to nick name and online state
- 0x8-0xb: hash of nickname
- 0xc-0xf: hash of email
- 0x10-0x13: hash of password
	- see src/util.rs for hashing algorithm
	- empty email/password digests are just 0:u32, used in offline saves
- 0x1a-0x39: nickname
