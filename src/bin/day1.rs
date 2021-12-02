use adventofcode2015::utils::{file, get_answerer, BoxResult};

fn main() -> BoxResult<()> {
	let input = file::read_to_string("inputs/day1.txt")?;

	let mut answer = get_answerer();
	answer(part1(&input));
	answer(part2(&input));

	Ok(())
}

fn part1(input: &String) -> BoxResult<i32> {
	let mut count = 0;
	for c in input.chars() {
		if c == '(' {
			count += 1;
		} else if c == ')' {
			count -= 1;
		}
	}
	Ok(count)
}

fn part2(input: &String) -> BoxResult<i32> {
	let mut count = 0;
	for (i, c) in input.chars().enumerate() {
		if c == '(' {
			count += 1;
		} else if c == ')' {
			count -= 1;
		}
		if count < 0 {
			return Ok(i as i32 + 1);
		}
	}
	Ok(0)
}
