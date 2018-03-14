use super::gdbus::{exec_shell_command};
use super::dconf::{read_str, write_str};
use std::io::Result;

static GTK_THEME: &'static str = "/org/gnome/desktop/interface/gtk-theme";
static USER_THEME: &'static str = "/org/gnome/shell/extensions/user-theme/name";

pub fn set_gtk_theme(theme: &str) -> Result<()> {
	write_str(GTK_THEME, theme)
}

pub fn get_gtk_theme() -> Result<String> {
	read_str(GTK_THEME)
}

pub fn set_user_theme(theme: &str) -> Result<()> {
	write_str(USER_THEME, theme)?;
	let cmd = format!("Main.setThemeStylesheet(\"/usr/share/themes/{}/gnome-shell/gnome-shell.css\");", theme);
	exec_shell_command(&cmd)?;
	match exec_shell_command("Main.loadTheme();") {
		Ok(_) => Ok(()),
		Err(e) => Err(e) 
	}
}

pub fn get_user_theme() -> Result<String> {
	read_str(USER_THEME)
}
