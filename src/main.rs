use std::{env, process};
use std::io::Result;

pub mod gnome_theme;
pub mod pop_theme;
mod dconf;

use pop_theme::*;

fn dark() -> Result<()> {
	set_theme(Theme::Dark)
}

fn light() -> Result<()> {
	set_theme(Theme::Light)
}

fn toggle() -> Result<()> {
	match get_theme() {
		Theme::Light => dark(),
		Theme::Dark => light(),
	}
}

fn main() {
	let mut args = env::args().skip(1);


	if let Some(arg) = args.next() {
		match arg.as_str() {
			"light" => {
				println!("Setting light theme");
				light().unwrap();
			},
			"dark" => {
				println!("Setting dark theme");
				dark().unwrap();
			},
			"toggle" => {
				match get_theme(){
					Theme::Light => println!("Setting dark theme"),
					Theme::Dark => println!("Setting light theme"),
				};
				toggle().unwrap();
			},
			_ => {
				eprintln!("pop-theme-toggle: unknown sub-command {}", arg);
				eprintln!("Usage: pop-theme-toggle (light|dark|toggle)");
				process::exit(1);
			}
		}
	} else {
		eprintln!("Usage: pop-theme-toggle (light|dark|toggle)");
		process::exit(1);
	}

}

