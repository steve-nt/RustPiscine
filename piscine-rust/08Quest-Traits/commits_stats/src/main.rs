use commits_stats::{commits_per_week, commits_per_author};

fn main() {
	let contents = include_str!("commits.json");
	let serialized = json::parse(&contents).unwrap();
	println!("{:?}", commits_per_week(&serialized));
	println!("{:?}", commits_per_author(&serialized));
}