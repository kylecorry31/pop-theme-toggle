use std::process::{Command, Output};
use std::io::Result;

pub fn write_str(key: &str, value: &str) -> Result<()> {
	let mut cmd = Command::new("dconf");
	let s = format!("'{}'", value);
	cmd.args(&["write", key, &s]);
	match cmd.output() {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}

pub fn read_str(key: &str) -> Result<String> {
	let mut cmd = Command::new("dconf");
	cmd.args(&["read", key]);
	Ok(get_stdout(cmd.output()?))
}

fn get_stdout(output: Output) -> String {
	let vs = output.stdout;
	String::from_utf8(vs).unwrap().replace("\'", "").replace("\n", "")
}