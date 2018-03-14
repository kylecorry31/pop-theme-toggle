use super::gnome_theme::*;
use std::io::Result;

static POP_LIGHT: &'static str = "Pop";
static POP_DARK: &'static str = "Pop-dark";
static POP_SLIM_LIGHT: &'static str = "Pop-slim";
static POP_SLIM_DARK: &'static str = "Pop-dark-slim";

#[derive(PartialEq)]
pub enum Theme {
	Light,
	Dark,
	SlimLight,
	SlimDark,
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
		},
		Theme::SlimLight => {
			set_gtk_theme(POP_SLIM_LIGHT)?;
			set_user_theme(POP_SLIM_LIGHT)?;
		},
		Theme::SlimDark => {
			set_gtk_theme(POP_SLIM_DARK)?;
			set_user_theme(POP_SLIM_DARK)?;
		},
	};
	Ok(())
}

pub fn get_theme() -> Theme {
	let theme = get_gtk_theme();

	match theme {
		Ok(a) => {
			if a == POP_LIGHT { 
				Theme::Light
			} else if a == POP_DARK {
				Theme::Dark
			} else if a == POP_SLIM_LIGHT {
				Theme::SlimLight
			} else if a == POP_SLIM_DARK {
				Theme::SlimDark
			} else {
				Theme::Light
			}
		},
		Err(_) => Theme::Light,
	}
}
