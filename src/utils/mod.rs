pub mod file;
pub mod runner;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_answerer<T, E>() -> impl FnMut(Result<T, E>)
where
	T: std::fmt::Display,
	E: std::fmt::Display,
{
	let mut part: usize = 1;

	return move |result: Result<T, E>| {
		match result {
			Ok(answer) => println!("Part {}: {}", part, answer.to_string()),
			Err(err) => println!("Part {} Error: {}", part, err.to_string()),
		};
		part += 1;
	};
}
