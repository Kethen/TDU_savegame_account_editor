// re-written from http://aluigi.altervista.org/papers.htm#others-file Test Drive Unlimited savegames/files decrypter/encrypter 0.1 c source
// Copyright 2023 Katharine Chui

// original license of tdudec.c:
/*
    Copyright 2009 Luigi Auriemma

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307 USA

    http://www.gnu.org/licenses/gpl-2.0.txt
*/

// endianess
// "\x2C\x43\xEA\x64\x24\x5B\xA3\xF8\x81\xCD\x8E\x01\xAC\xBE\x26\x83"
pub const SAVE_KEY: [u32; 4] = [0x64EA432C, 0xF8A35B24, 0x018ECD81, 0x8326BEAC];
// "\x4A\x3C\xE2\x4F\x11\xC2\xBA\x80\x3A\xBD\x17\x69\xBD\x8E\x52\xF0"
pub const OTHERS_KEY: [u32; 4] = [0x4FE23C4A, 0x80BAC211, 0x6917BD3A, 0xF0528EBD];

fn le_slice_to_u32(input:&[u8]) -> u32{
	let buf:[u8;4] = [input[0], input[1], input[2], input[3]];
	u32::from_le_bytes(buf)
}

// I swear if rust make me put Wrapping everywhere one day changing rustc
fn decrypt(input:&std::vec::Vec<u8>, save:bool) -> std::vec::Vec<u8> {
	let key = if save{
		&SAVE_KEY
	}else{
		&OTHERS_KEY
	};

	let mut output = std::vec::Vec::<u8>::new();

	if input.len() < 8{
		return output;
	}

	let datalen = if save{
		input.len()
	}else{
		input.len() - 8
	};

	let mut blocks:usize = datalen >> 3;
	let mut cursor:usize = 0;
	let mut y:u32;
	let mut z:u32;

	while blocks != 0{
		if save{
			y = le_slice_to_u32(&input[cursor..cursor + 4]);
			z = le_slice_to_u32(&input[cursor + 4..cursor + 8]);
		}else{			
			y = le_slice_to_u32(&input[cursor + 8..cursor + 12]);
			z = le_slice_to_u32(&input[cursor + 12..cursor + 16]);
		}

		let mut sum = 0xc6ef3720_u32;
		for _ in 0..32{
			z = z - ((y + ((y >> 5) ^ (y << 4))) ^ (sum + key[usize::try_from((sum >> 11) & 3).unwrap()]));
			sum = sum + 0x61c88647_u32;
			y = y - ((z + ((z >> 5) ^ (z << 4))) ^ (sum + key[usize::try_from(sum & 3).unwrap()]));
		}

		if save{
			output.append(&mut y.to_le_bytes().to_vec());
			output.append(&mut z.to_le_bytes().to_vec());
		}else{
			output.append(&mut ((le_slice_to_u32(&input[cursor..cursor + 4])) ^ y).to_le_bytes().to_vec());
			output.append(&mut ((le_slice_to_u32(&input[cursor + 4..cursor + 8])) ^ z).to_le_bytes().to_vec());
		}

		blocks = blocks - 1;
		cursor = cursor + 8;
	}
	output
}

fn encrypt(input:&std::vec::Vec<u8>, save:bool) -> std::vec::Vec<u8> {
	let key = if save{
		&SAVE_KEY
	}else{
		&OTHERS_KEY
	};

	let mut output = std::vec::Vec::<u8>::new();

	if input.len() < 8{
		return output;
	}

	let mut input = input.clone();
	let padding = input.len() % 32;

	if padding != 0{
		input.append(&mut vec![0u8; padding]);
	}

	let datalen = input.len();

	if !save{
		output.append(&mut 42u32.to_le_bytes().to_vec());
		output.append(&mut (!42u32).to_le_bytes().to_vec());	
	}

	let mut blocksize:usize = datalen >> 3;
	let mut cursor:usize = 0;
	let mut y:u32;
	let mut z:u32;

	while blocksize != 0{
		if save{
			y = le_slice_to_u32(&input[cursor..cursor + 4]);
			z = le_slice_to_u32(&input[cursor + 4..cursor + 8]);
		}else{
			y = le_slice_to_u32(&output[cursor..cursor + 4]) ^ le_slice_to_u32(&input[cursor..cursor + 4]);
			z = le_slice_to_u32(&output[cursor + 4..cursor + 8]) ^ le_slice_to_u32(&input[cursor + 4..cursor + 8]);
		}

		let mut sum = 0u32;
		for _ in 0..32{
			y = y + ((z + ((z >> 5) ^ (z << 4))) ^ (sum + key[usize::try_from(sum & 3).unwrap()]));
			sum = sum - 0x61c88647_u32;
			z = z + ((y + ((y >> 5) ^ (y << 4))) ^ (sum + key[usize::try_from((sum >> 11) & 3).unwrap()]));
		}

		output.append(&mut y.to_le_bytes().to_vec());
		output.append(&mut z.to_le_bytes().to_vec());

		blocksize = blocksize - 1;
		cursor = cursor + 8;
	}
	output
}

pub fn decrypt_others(input:&std::vec::Vec<u8>) -> std::vec::Vec<u8> {
	decrypt(input, false)
}

pub fn encrypt_others(input:&std::vec::Vec<u8>) -> std::vec::Vec<u8> {
	encrypt(input, false)
}

pub fn decrypt_save(input:&std::vec::Vec<u8>) -> std::vec::Vec<u8> {
	decrypt(input, true)
}

pub fn encrypt_save(input:&std::vec::Vec<u8>) -> std::vec::Vec<u8> {
	encrypt(input, true)
}
