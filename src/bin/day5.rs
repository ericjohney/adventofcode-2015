use adventofcode2015::utils::{file, get_answerer, BoxResult};
use fancy_regex::Regex;

fn main() -> BoxResult<()> {
	let strings = file::lines::<String>("inputs/day5.txt")?;

	let mut answer = get_answerer();
	answer(part1(&strings));
	answer(part2(&strings));

	Ok(())
}

fn count_nice_strings(strings: &Vec<String>, nice_rules: &[&str]) -> usize {
	strings
		.iter()
		.filter(|s| {
			let is_nice = nice_rules
				.iter()
				.map(|r| Regex::new(r).unwrap().is_match(s).unwrap())
				.fold(true, |acc, x| acc && x);
			return is_nice;
		})
		.count()
}

fn part1(strings: &Vec<String>) -> BoxResult<usize> {
	let nice_rules = &[r"([aeiou].*){3,}", r"(\w)\1", r"^(?!.*(ab|cd|pq|xy)).*$"];

	Ok(count_nice_strings(strings, nice_rules))
}

fn part2(strings: &Vec<String>) -> BoxResult<usize> {
	let nice_rules = &[r"(\w{2}).*\1", r"(\w)\w\1"];

	Ok(count_nice_strings(strings, nice_rules))
}
