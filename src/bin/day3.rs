use adventofcode2015::utils::{file, get_answerer, BoxResult};
use std::collections::HashMap;

fn main() -> BoxResult<()> {
	let input = file::read_to_string("inputs/day3.txt")?;

	let mut answer = get_answerer();
	answer(part1(&input));
	answer(part2(&input));

	Ok(())
}

fn part1(input: &String) -> BoxResult<usize> {
	let mut location = (0, 0);

	let mut locations = HashMap::new();
	locations.insert(location, 1);

	for c in input.chars() {
		match c {
			'^' => location.1 += 1,
			'v' => location.1 -= 1,
			'<' => location.0 -= 1,
			'>' => location.0 += 1,
			_ => (),
		};
		locations.insert(location, 1);
	}
	println!("{:?}", locations);
	Ok(locations.len())
}

fn part2(input: &String) -> BoxResult<usize> {
	let mut location = (0, 0);
	let mut robolocation = (0, 0);

	let mut locations = HashMap::new();
	locations.insert(location, 1);

	for (i, c) in input.chars().enumerate() {
		if i % 2 == 0 {
			match c {
				'^' => location.1 += 1,
				'v' => location.1 -= 1,
				'<' => location.0 -= 1,
				'>' => location.0 += 1,
				_ => (),
			};
			locations.insert(location, 1);
		} else {
			match c {
				'^' => robolocation.1 += 1,
				'v' => robolocation.1 -= 1,
				'<' => robolocation.0 -= 1,
				'>' => robolocation.0 += 1,
				_ => (),
			};
			locations.insert(robolocation, 1);
		}
	}
	println!("{:?}", locations);
	Ok(locations.len())
}
