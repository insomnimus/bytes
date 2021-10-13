use std::fmt;

use anyhow::{
	anyhow,
	bail,
	Result,
};

pub type Unit = u64;

pub const KB: Unit = 1000;
pub const KIB: Unit = 1024;
pub const MB: Unit = 1000000;
pub const MIB: Unit = 1024 * 1024;
pub const GB: Unit = 1000000000;
pub const GIB: Unit = 1024 * 1024 * 1024;
pub const TB: Unit = 1000000000000;
pub const TIB: Unit = 1024 * 1024 * 1024 * 1024;
pub const PB: Unit = 1000000000000000;
pub const PIB: Unit = 1024 * 1024 * 1024 * 1024 * 1024;

#[inline]
fn unit_str(n: Unit) -> &'static str {
	match n {
		KB => "KB",
		KIB => "KiB",
		MB => "MB",
		MIB => "MiB",
		GB => "GB",
		GIB => "GiB",
		TB => "TB",
		TIB => "TiB",
		PB => "PB",
		PIB => "PiB",
		_ => unreachable!(),
	}
}

pub fn parse_unit(s: &str) -> Result<Unit> {
	Ok(match &s.to_lowercase()[..] {
		"" | "b" => 1,
		"kb" => KB,
		"kib" => KIB,
		"mb" => MB,
		"mib" => MIB,
		"gb" => GB,
		"gib" => GIB,
		"tb" => TB,
		"tib" => TIB,
		"pb" => PB,
		"pib" => PIB,
		_ => bail!("'{}' is not a valid storage unit", s),
	})
}

pub fn parse(s: &str) -> Result<Pretty> {
	let mut chars = s.trim().chars().peekable();
	if !chars
		.peek()
		.ok_or_else(|| anyhow!("value is empty"))?
		.is_digit(10)
	{
		bail!("value must start with a number");
	}

	let mut buf = String::with_capacity(s.len());
	while let Some(c) = chars.next_if(|c| c.is_digit(10) || *c == '.') {
		buf.push(c);
	}

	let n = buf
		.parse::<f64>()
		.map_err(|_| anyhow!("{} is not a valid number", &buf))?;
	buf.clear();

	buf.extend(chars.map(char::to_lowercase).flatten());
	let buf = buf.trim();
	parse_unit(buf).map(|u| (n * u as f64) as u64).map(Pretty)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Pretty(pub u64);

impl fmt::Display for Pretty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let n = self.0;

		let unit = if n >= PIB {
			PIB
		} else if n >= TIB {
			TIB
		} else if n >= GIB {
			GIB
		} else if n >= MIB {
			MIB
		} else if n >= KIB {
			KIB
		} else {
			return write!(f, "{}b", n);
		};

		let val = n as f64 / unit as f64;
		write!(f, "{}{}", val, unit_str(unit))
	}
}
