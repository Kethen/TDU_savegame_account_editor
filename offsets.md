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
- 0x8-0xb: sensitive to nick name
- 0xc-0x17:
	- online example: E9 59 B4 08 1D 01 99 7F 00 00 72 46
	- online example2: 07 6D F7 EA 1D 01 99 7F 00 00 73 82
	- offline example: 00 00 00 00 00 00 00 00 00 1B 69 4E
	- offline example2: 00 00 00 00 00 00 00 00 00 17 BE F3
	- the online/offline flag is somewhere inside the string
	- the offline examples seem to be usable on different playersaves, can be used to downgrade online saves to offline, but the online examples seem to be tied to nick name
	- no I don't know how it works, that'd require some reverse engineering on the save data serialization code
- 0x1a-0x39: nick name
