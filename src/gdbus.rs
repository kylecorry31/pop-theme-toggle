use std::process::{Command, Output};
use std::io::Result;

pub fn exec_shell_command(command: &str) -> Result<String> {
	let mut cmd = Command::new("gdbus");
	let c = format!("'{}'", command);
	cmd.args(&["call", "--session", "--dest", "org.gnome.Shell", "--object-path", "/org/gnome/Shell", "--method", "org.gnome.Shell.Eval", &c]);
	match cmd.output() {
		Ok(o) => Ok(get_stdout(o)),
		Err(e) => Err(e),
	}
}

fn get_stdout(output: Output) -> String {
	let vs = output.stdout;
	String::from_utf8(vs).unwrap().replace("\'", "").replace("\n", "")
}