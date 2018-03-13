use super::gnome_theme::*;
use std::io::Result;

static POP_LIGHT: &'static str = "Pop";
static POP_DARK: &'static str = "Pop-dark";

#[derive(PartialEq)]
pub enum Theme {
	Light,
	Dark
}

pub fn set_theme(theme_type: Theme) -> Result<()> {
	match theme_type {
		Theme::Light => {
			set_gtk_theme(POP_LIGHT)?;
			set_user_theme(POP_LIGHT)?;
		},
		Theme::Dark => {
			set_gtk_theme(POP_DARK)?;
			set_user_theme(POP_DARK)?;
		}
	};
	Ok(())
}

pub fn get_theme() -> Theme {
	let theme = get_gtk_theme();

	match theme {
		Ok(a) => {
			if a == POP_LIGHT { 
				Theme::Light
			} else {
				Theme::Dark
			}
		},
		Err(_) => Theme::Light,
	}
}