use adventofcode2015::utils::{file, get_answerer, BoxResult};
use md5;

fn main() -> BoxResult<()> {
	let input = file::read_to_string("inputs/day4.txt")?;

	let mut answer = get_answerer();
	answer(part1(&input));
	answer(part2(&input));

	Ok(())
}

fn part1(input: &String) -> BoxResult<usize> {
	let mut i = 0;
	let mut hash = "".to_owned();
	while !hash.starts_with("00000") {
		i += 1;
		let digest = md5::compute(format!("{}{}", input, i));
		hash = format!("{:x}", digest);
	}
	Ok(i)
}

fn part2(input: &String) -> BoxResult<usize> {
	let mut i = 0;
	let mut hash = "".to_owned();
	while !hash.starts_with("000000") {
		i += 1;
		let digest = md5::compute(format!("{}{}", input, i));
		hash = format!("{:x}", digest);
	}
	Ok(i)
}
