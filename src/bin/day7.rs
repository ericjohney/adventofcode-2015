use adventofcode2015::utils::{file, get_answerer, BoxResult};
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{input} -> {output}")]
struct Instruction {
	input: String,
	output: String,
}

fn main() -> BoxResult<()> {
	let instructions = file::lines::<Instruction>("inputs/day7.txt")?;

	let mut wires = HashMap::new();
	for instruction in instructions {
		wires.insert(instruction.output.clone(), instruction.input.clone());
	}

	let mut answer = get_answerer();
	answer(part1(&wires));
	answer(part2(&wires));

	Ok(())
}

fn get_signal(
	wires: &HashMap<String, String>,
	wire: &str,
	signals: &mut HashMap<String, u16>,
) -> u16 {
	if let Some(signal) = signals.get(wire) {
		return *signal;
	}

	let instruction = wires.get(wire).unwrap();
	let output;

	let mut lookup = |signal: &str| {
		let is_numeric = signal.bytes().all(|c| c.is_ascii_digit());
		match is_numeric {
			true => signal.parse::<u16>().unwrap(),
			false => get_signal(wires, signal, signals),
		}
	};

	let parts = instruction.split(" ").collect::<Vec<_>>();
	if parts.len() == 1 {
		output = lookup(parts[0]);
	} else if parts.len() == 2 {
		let input = lookup(parts[1]);
		output = !input;
	} else {
		let input1 = lookup(parts[0]);
		let input2 = lookup(parts[2]);
		output = match parts[1] {
			"AND" => input1 & input2,
			"OR" => input1 | input2,
			"LSHIFT" => input1 << parts[2].parse::<u16>().unwrap(),
			"RSHIFT" => input1 >> parts[2].parse::<u16>().unwrap(),
			_ => 0,
		};
	}

	signals.insert(wire.to_string(), output);
	return output;
}

fn part1(wires: &HashMap<String, String>) -> BoxResult<u16> {
	let signal = get_signal(&wires, "a", &mut HashMap::new());

	Ok(signal)
}

fn part2(wires: &HashMap<String, String>) -> BoxResult<u16> {
	let signal = get_signal(&wires, "a", &mut HashMap::new());

	let mut signals = HashMap::new();
	signals.insert("b".to_string(), signal);
	let new_signal = get_signal(&wires, "a", &mut signals);

	Ok(new_signal)
}
