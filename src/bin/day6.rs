use adventofcode2015::utils::{file, get_answerer, BoxResult};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{command} {x1},{y1} through {x2},{y2}")]
struct Command {
	#[from_str(regex = "turn on|turn off|toggle")]
	command: String,
	x1: usize,
	y1: usize,
	x2: usize,
	y2: usize,
}

fn main() -> BoxResult<()> {
	let commands = file::lines::<Command>("inputs/day6.txt")?;

	let mut answer = get_answerer();
	answer(part1(&commands));
	answer(part2(&commands));

	Ok(())
}

fn part1(commands: &Vec<Command>) -> BoxResult<u32> {
	let mut lights = vec![vec![false; 1000]; 1000];

	for command in commands {
		let (x1, y1, x2, y2) = (command.x1, command.y1, command.x2, command.y2);
		for x in x1..=x2 {
			for y in y1..=y2 {
				match command.command.as_str() {
					"turn on" => lights[x][y] = true,
					"turn off" => lights[x][y] = false,
					"toggle" => lights[x][y] = !lights[x][y],
					_ => (),
				}
			}
		}
	}

	let mut count = 0;
	for x in 0..1000 {
		for y in 0..1000 {
			if lights[x][y] {
				count += 1;
			}
		}
	}

	Ok(count)
}

fn part2(commands: &Vec<Command>) -> BoxResult<u32> {
	let mut lights = vec![vec![0; 1000]; 1000];

	for command in commands {
		let (x1, y1, x2, y2) = (command.x1, command.y1, command.x2, command.y2);
		for x in x1..=x2 {
			for y in y1..=y2 {
				match command.command.as_str() {
					"turn on" => lights[x][y] += 1,
					"turn off" => {
						if lights[x][y] > 0 {
							lights[x][y] -= 1;
						}
					}
					"toggle" => lights[x][y] += 2,
					_ => (),
				}
			}
		}
	}

	let mut brightness = 0;
	for x in 0..1000 {
		for y in 0..1000 {
			brightness += lights[x][y];
		}
	}

	Ok(brightness)
}
