mod bytes;

use anyhow::Result;

fn main() -> Result<()> {
	let s = "1024 mib";
	let n = bytes::parse(s)?;
	println!("{} bytes", n.0);
	println!("{}", n);

	Ok(())
}
