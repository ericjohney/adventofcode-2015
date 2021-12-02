use super::BoxResult;
use simple_error::bail;
use std::fs;
use std::str::FromStr;

pub fn read_to_string(filename: &'static str) -> BoxResult<String> {
	let string = fs::read_to_string(filename)?;

	Ok(string.trim().to_string())
}

pub fn lines<T>(filename: &'static str) -> BoxResult<Vec<T>>
where
	T: FromStr,
{
	let input = fs::read_to_string(filename)?;
	let lines: Result<_, _> = input.lines().into_iter().map(|s| s.parse()).collect();

	match lines {
		Ok(res) => Ok(res),
		Err(_e) => bail!("failed to parse lines from file: {}", filename),
	}
}
