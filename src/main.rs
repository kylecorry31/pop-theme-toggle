use std::{env, process};
use std::io::Result;

pub mod gnome_theme;
pub mod pop_theme;
mod dconf;
mod gdbus;

use pop_theme::*;

fn dark() -> Result<()> {
	println!("Setting dark theme");
	set_theme(Theme::Dark)
}

fn light() -> Result<()> {
	println!("Setting light theme");
	set_theme(Theme::Light)
}

fn slim_light() -> Result<()> {
	println!("Setting slim light theme");
	set_theme(Theme::SlimLight)
}

fn slim_dark() -> Result<()> {
	println!("Setting slim dark theme");
	set_theme(Theme::SlimDark)
}

fn toggle_to_slim() -> Result<()> {
	match get_theme() {
		Theme::Light => slim_light(),
		Theme::Dark => slim_dark(),
		_ => {
			println!("Already in slim theme");
			Ok(())
		}
	}
}

fn toggle() -> Result<()> {
	match get_theme() {
		Theme::Light => dark(),
		Theme::Dark => light(),
		Theme::SlimLight => slim_dark(),
		Theme::SlimDark => slim_light(),
	}
}

fn main() {
	let mut args = env::args().skip(1);


	if let Some(arg) = args.next() {
		match arg.as_str() {
			"light" => {
				light().unwrap();
			},
			"dark" => {
				dark().unwrap();
			},
			"slim-dark" | "dark-slim" => {
				slim_dark().unwrap();
			},
			"slim-light" | "light-slim" => {
				slim_light().unwrap();
			},
			"slim" => {
				toggle_to_slim().unwrap();
			},
			"toggle" => {
				toggle().unwrap();
			},
			_ => {
				eprintln!("pop-theme-toggle: unknown sub-command {}", arg);
				eprintln!("Usage: pop-theme-toggle (light|dark|toggle|slim|dark-slim|slim-dark|light-slim|slim-light)");
				process::exit(1);
			}
		}
	} else {
		eprintln!("Usage: pop-theme-toggle (light|dark|toggle)");
		process::exit(1);
	}

}

