mod util;
mod tdudec;

use rfd::FileDialog;

use iced::{Sandbox, Element, Alignment, Settings};
use iced::widget::{button, container, checkbox, pick_list, scrollable, text, text_input, column, row};

const config_file:&str = "tdu_savegame_account_editor.conf";
const log_file:&str = "tdu_savegame_account_editor.log";

use std::io::Write;

fn log(message: &String){
	let mut file = match std::fs::OpenOptions::new()
		.write(true)
		.create(true)
		.append(true)
		.open(log_file){
		Ok(f) => f,
		Err(e) => {
			println!("cannot open {} to write log, {}", log_file, e);
			println!("log message: {}", message);
			return;
		},
	};

	match file.write_all(&format!("{}\n", message).into_bytes()){
		Ok(_) => {return},
		Err(e) => {
			println!("cannot write log to {}, {}", log_file, e);
			println!("log message: {}", message);
			return;
		},
	};
}

fn save_last_used_path(path:&String) -> Result<(), String>{
	match std::fs::write(config_file, path.clone().into_bytes()){
		Ok(_) => Ok(()),
		Err(e) => Err(format!("cannot save last used savegames path to file, {}", e)),
	}
}

fn load_last_used_path() -> Result<String, String>{
	let path_bytes = match std::fs::read(config_file){
		Ok(b) => b,
		Err(e) => {return Err(format!("cannot load last used savegames path from file, {}", e));},
	};

	match String::from_utf8(path_bytes.clone()){
		Ok(s) => Ok(s),
		Err(e) => Err(format!("cannot decode last used savegame path from file, {}", e)),
	}
}

fn guess_path() -> String {
	match load_last_used_path() {
		Ok(p) => {return p;},
		Err(e) => {log(&e);},
	}

	match std::env::var("USERPROFILE") {
		Ok(p) => {
			let ret = format!("{}/Documents/Test Drive Unlimited/savegame/ProfileList.dat", p);
			let path = std::path::Path::new(&ret);
			if path.exists(){
				save_last_used_path(&ret);
				return ret;
			}
		}
		Err(e) => {log(&format!("cannot read env USERPROFILE, {}", e));},
	}

	return String::from("");
}

struct AccountChanger {
	path:String,
	selected_profile:String,
	selected_profile_valid:bool,
	nickname:String,
	email:String,
	password:String,
	profile_list:std::vec::Vec<String>,

	playersave_import_path:String,
	online:bool,
	import_playersave:bool,

	state:String,
	commondt_cache:std::vec::Vec<u8>,
	playersave_cache:std::vec::Vec<u8>,
}

#[derive(Debug, Clone)]
enum Message{
	IgnoreString(String),
	IgnoreToggle(bool),
	SelectPath,
	SelectProfile(String),
	ChangeNickName(String),
	ChangeEmail(String),
	ChangePassword(String),
	ToggleOnline(bool),
}

fn fetch_and_filter_profile_list(path:&String) -> std::vec::Vec<String>{
	if path.len() != 0 && std::path::Path::new(&path).exists(){
		let profile_list = match std::fs::read(&path){
			Ok(p) => p,
			Err(e) => {
				log(&format!("cannot read {}, {}", path, e));
				return std::vec::Vec::<String>::new();
			},
		};

		match util::read_profile_list(&profile_list){
			Ok(profile_list) => {
				let mut ret = std::vec::Vec::<String>::new();
				let basedir = format!("{}", std::path::Path::new(&path).parent().unwrap().display());
				for item in profile_list{
					match fetch_commondt_and_playersave(path, &item){
						Ok(_) => {
							ret.push(item.clone());
						},
						Err(e) => {
							log(&format!("warning: profile {} dropped, missing files, {}", item, e))
						},
					}
				}
				return ret;
			},
			Err(e) => {log(&format!("cannot parse {}, {}", path, e));},
		};
	}
	return std::vec::Vec::<String>::new();
}

fn get_file_last_modified(path:&String) -> std::time::SystemTime{
	match std::fs::metadata(path){
		Ok(m) =>{
			match m.modified() {
				Ok(m) => m,
				Err(e) =>{
					log(&format!("warning: cannot fetch last modified time, {}", e));
					std::time::SystemTime::UNIX_EPOCH
				},
			}
		},
		Err(_) => std::time::SystemTime::UNIX_EPOCH,
	}
}

fn fetch_commondt_and_playersave(path:&String, profile_name:&String) -> Result<(std::vec::Vec<u8>, std::vec::Vec<u8>), String>{
	if !std::path::Path::new(&path).exists(){
		return Err(format!("{} does not exist", path));
	}

	let profile_dir = format!("{}/{}", std::path::Path::new(&path).parent().unwrap().display(), profile_name);
	if !std::path::Path::new(&profile_dir).is_dir(){
		return Err(format!("{} is not a directory", profile_dir));
	}

	let playersave_path = format!("{}/{}", profile_dir, "playersave/playersave");
	let playersave2_path = format!("{}/{}", profile_dir, "playersave2/playersave");
	let commondt_path = format!("{}/{}", profile_dir, "playersave/commondt.sav");
	let commondt2_path = format!("{}/{}", profile_dir, "playersave2/commondt.sav");

	if !std::path::Path::new(&commondt_path).exists() && !std::path::Path::new(&commondt2_path).exists(){
		return Err(format!("commondt.sav not found under profile {}", profile_name));
	}

	let mut playersave_present = true;
	if !std::path::Path::new(&playersave_path).exists() && !std::path::Path::new(&playersave2_path).exists(){
		log(&format!("playersave not found under profile {}", profile_name));
		playersave_present = false;
	}

	let commondt_last_modified = get_file_last_modified(&commondt_path);
	let commondt2_last_modified = get_file_last_modified(&commondt2_path);

	let commondt_path = if commondt_last_modified > commondt2_last_modified{
		commondt_path
	}else{
		commondt2_path
	};

	let playersave_last_modified = get_file_last_modified(&playersave_path);
	let playersave2_last_modified = get_file_last_modified(&playersave2_path);

	let playersave_path = if playersave_last_modified > playersave2_last_modified {
		playersave_path
	}else{
		playersave2_path
	};

	let commondt = match std::fs::read(&commondt_path) {
		Ok(b) => tdudec::decrypt_save(&b),
		Err(e) => {
			return Err(format!("cannot read {}, {}", commondt_path, e));
		},
	};

	let playersave = if playersave_present{
		match std::fs::read(&playersave_path){
			Ok(b) => tdudec::decrypt_save(&b),
			Err(e) => {
				return Err(format!("cannot read {}, {}", playersave_path, e));
			},
		}
	}else{
		std::vec::Vec::<u8>::new()
	};

	return Ok((commondt, playersave));
}

impl Sandbox for AccountChanger{
	type Message = Message;

	fn new() -> Self{
		let path = guess_path();
		let mut profile_list = fetch_and_filter_profile_list(&path);
		let selected_profile = if profile_list.len() != 0{
			profile_list[0].clone()
		}else{
			String::from("")
		};

		let player_identifier = util::PlayerIdentifier {
				nickname:String::from(""),
				email:String::from(""),
				password:String::from("")
		};

		let mut selected_profile_valid = false;
		let mut commondt_cache = std::vec::Vec::<u8>::new();
		let mut playersave_cache = std::vec::Vec::<u8>::new();

		let (player_identifier, online) = if selected_profile.len() != 0{
			match fetch_commondt_and_playersave(&path, &selected_profile){
				Ok((commondt, playersave)) => {
					match util::read_commondt(&commondt){
						Ok((pi, online)) => {
							selected_profile_valid = true;
							commondt_cache.append(&mut commondt.clone());
							playersave_cache.append(&mut playersave.clone());
							(pi, online)
						},
						Err(_) => (player_identifier, false),
					}
				}
				Err(_) => (player_identifier, false),
			}
		}else{
			(player_identifier, false)
		};

		Self{
			path:path,
			selected_profile:selected_profile,
			selected_profile_valid:selected_profile_valid,
			nickname:player_identifier.nickname.clone(),
			email:player_identifier.email.clone(),
			password:player_identifier.password.clone(),
			profile_list:profile_list,

			playersave_import_path:String::from(""),
			online:online,
			import_playersave:false,

			state:String::from(""),
			commondt_cache:commondt_cache,
			playersave_cache:playersave_cache,
		}
	}

	fn title(&self) -> String{
		String::from("Test Drive Unlimited savegame account editor")
	}

	fn update(&mut self, message:Message){
		match message{
			Message::IgnoreString(_) => {},
			Message::IgnoreToggle(_) => {},
			Message::SelectPath => {
				match FileDialog::new()
					.add_filter("ProfileList.dat", &["dat"])
					.pick_file(){
						Some(p) => {
							self.path = format!("{}", p.display());
							self.profile_list = fetch_and_filter_profile_list(&self.path);
							if self.profile_list.len() != 0{
								save_last_used_path(&self.path);
								self.update(Message::SelectProfile(self.profile_list[0].clone()));
							}
						},
						None => {
							log(&format!("file dialog returned without a path"));
						},
				}
			},
			Message::SelectProfile(n) => {
				self.selected_profile = n.clone();
				match fetch_commondt_and_playersave(&self.path, &self.selected_profile){
					Ok(cp) => {
						let (commondt, playersave) = cp;
						match util::read_commondt(&commondt){
							Ok((pi, online)) => {
								self.nickname = pi.nickname.clone();
								self.email = pi.email.clone();
								self.password = pi.email.clone();
								self.selected_profile_valid = true;
								self.commondt_cache = commondt;
								self.online = online;
								self.playersave_cache = playersave;
							},
							Err(e) => {
								self.nickname = String::from("");
								self.email = String::from("");
								self.password = String::from("");
								self.selected_profile_valid = false;
								self.state = String::from("failed parsing commondt");
								log(&format!("failed parsing commondt during field population, {}", e));
							}
						}
					},
					Err(e) => {
						self.nickname = String::from("");
						self.email = String::from("");
						self.password = String::from("");
						self.selected_profile_valid = false;
						self.state = String::from("failed reading commondt");
						log(&format!("failed reading commondt during field population, {}", e));
					}
				}

			},
			Message::ChangeNickName(s) => {
				self.nickname = s;
			},
			Message::ChangeEmail(s) => {
				self.email = s;
			},
			Message::ChangePassword(s) => {
				self.password = s;
			},
			Message::ToggleOnline(b) => {
				self.online = b;
			}
		};
	}

	fn view(&self) -> Element<Message>{
		column![
			row![
				text("ProfileList.dat path:"),
				text_input("", &self.path).on_input(Message::IgnoreString),
				button("...").on_press(Message::SelectPath),
			].align_items(Alignment::Center),
			row![
				text("Profile to edit:"),
				pick_list(self.profile_list.clone(), Some(self.selected_profile.clone()), Message::SelectProfile),
			].align_items(Alignment::Center),
			row![
				text("Nickname:"),
				if self.selected_profile_valid{
					text_input("", &self.nickname).on_input(Message::ChangeNickName)
				}else{
					text_input("", "")
				},
			].align_items(Alignment::Center),
			row![
				text("Email:"),
				if self.selected_profile_valid && self.online{
					text_input("", &self.email).on_input(Message::ChangeEmail)
				}else{
					text_input("", "")
				},
			].align_items(Alignment::Center),
			row![
				text("Password:"),
				if self.selected_profile_valid && self.online{
					text_input("", &self.password).on_input(Message::ChangePassword)
				}else{
					text_input("", "")
				},
			].align_items(Alignment::Center),
			row![
				text("Online:"),
				if self.selected_profile_valid{
					checkbox("", self.online, Message::ToggleOnline)
				}else{
					checkbox("", false, Message::IgnoreToggle)
				},
			].align_items(Alignment::Center),
		]
		.align_items(Alignment::Start)
		.padding(10)
		.into()
	}
}

fn main() {
	AccountChanger::run(Settings::default());
}

fn test() {
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
	let (login, online) = util::read_commondt(&commondt_decrypted).unwrap();
	print!("nickname: {}\n", login.nickname);
	print!("email: {}\n", login.email);
	print!("password: {}\n", login.password);
	print!("is online: {}\n", online);
}

fn test_profilelist_read_write(){
	let profile_list1 = std::fs::read("ProfileList.dat").unwrap();
	let profile_list2 = std::fs::read("ProfileList.dat2").unwrap();
	let profile_list3 = std::fs::read("ProfileList.dat3").unwrap();

	let profile_list1 = util::read_profile_list(&profile_list1).unwrap();
	let profile_list2 = util::read_profile_list(&profile_list2).unwrap();
	let profile_list3 = util::read_profile_list(&profile_list3).unwrap();

	let profile_list1 = util::write_profile_list(&profile_list1);
	let profile_list2 = util::write_profile_list(&profile_list2);
	let profile_list3 = util::write_profile_list(&profile_list3);

	std::fs::write("ProfileList.dat.rewritten", &profile_list1);
	std::fs::write("ProfileList.dat2.rewritten", &profile_list2);
	std::fs::write("ProfileList.dat3.rewritten", &profile_list3);
}
