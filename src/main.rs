extern crate elefren;

use std::io;
use std::io::Write;
use std::error::Error;

use elefren::prelude::*;
use elefren::UpdateCredsRequest;
use elefren::scopes::Scopes;
use elefren::helpers::toml as elefren_toml;
use elefren::helpers::cli;
// use toml;



fn main() -> Result<(), Box<dyn Error>> {
	let mastodon = if let Ok(data) = elefren_toml::from_file("mastodon-data.toml") {
		Mastodon::from(data)
	} else {
		register()?
	};

	let icons = ["\u{2705}","\u{2705}/\u{26AA}", "\u{26AA}", "\u{26AA}/\u{1F534}" ,"\u{1F534}"];

	println!("verifying credentials...");
	let you = mastodon.verify_credentials()?;
	let mut me = String::from(you.display_name);

	for i in icons.iter(){
		me = String::from(me.trim_end_matches(i));
	}

	println!("press q or c to quit");

	loop{
		let mut input = String::new();
		println!("Hi {}, How are you today?", me);
		println!("1 = I'm okay!");
		println!("2 = I'm struggling a bit");
		println!("3 = I'm struggling quite a bit more");
		println!("4 = I'm struggling a lot");
		println!("5 = I'm struggling really badly");

		io::stdin().read_line(&mut input)?;
		match input.trim(){
			"1" => {
				update_nick(mastodon, me+icons[0])?;
				break;
			},
			"2" => {
				update_nick(mastodon, me+icons[1])?;
				break;
			},
			"3" => {
				update_nick(mastodon, me+icons[2])?;
				break;
			},
			"4" => {
				update_nick(mastodon, me+icons[3])?;
				break;
			},
			"5" => {
				update_nick(mastodon, me+icons[4])?;
				break;
			},
			"q" | "c" => { break; }
			_ => {
			}
		}
	}


	Ok(())
}

fn update_nick(mastodon: Mastodon, nick: String) -> Result<(), Box<dyn Error>> {
	let mut builder = UpdateCredsRequest::new();
	builder.display_name(nick);
	mastodon.update_credentials(&mut builder)?;
	Ok(())
}

fn register() -> Result<Mastodon, Box<dyn Error>> {
	print!("Please enter your mastodon instance address: ");
	io::stdout().flush().unwrap();
	let mut instance = String::new();
	io::stdin().read_line(&mut instance)
		.expect("Failed to read line");

	let registration = Registration::new(instance.trim())
																	.client_name("mh-icon")
																	.scopes(Scopes::read_all().and(Scopes::write_all()))
																	.build()?;
	let mastodon = cli::authenticate(registration)?;

	// Save app data for using on the next run.
	elefren_toml::to_file(&*mastodon, "mastodon-data.toml")?;

	Ok(mastodon)
}
