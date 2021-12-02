use adventofcode2015::utils::{file, get_answerer, BoxResult};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{w}x{h}x{l}")]
struct Gift {
	w: u32,
	h: u32,
	l: u32,
}

fn main() -> BoxResult<()> {
	let gifts = file::lines::<Gift>("inputs/day2.txt")?;

	let mut answer = get_answerer();
	answer(part1(&gifts));
	answer(part2(&gifts));

	Ok(())
}

fn part1(gifts: &Vec<Gift>) -> BoxResult<u32> {
	let result = gifts
		.iter()
		.map(|g| {
			let sides = vec![g.w * g.h, g.h * g.l, g.l * g.w];
			let min = sides.iter().min().unwrap();
			let area = sides.iter().map(|s| 2 * s).sum::<u32>();
			return Ok(area + min);
		})
		.sum::<BoxResult<_>>()?;
	Ok(result)
}

fn part2(gifts: &Vec<Gift>) -> BoxResult<u32> {
	let result: u32 = gifts
		.iter()
		.map(|g| {
			let mut sides = vec![g.w, g.h, g.l];
			sides.sort();

			let ribbon = 2 * sides[0] + 2 * sides[1];
			let bow = sides.iter().product::<u32>();
			return ribbon + bow;
		})
		.sum();
	Ok(result)
}
