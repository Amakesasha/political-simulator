use crate::*;

#[derive(Debug, Clone)]
pub struct WebS {
	link: LincS,
}

pub type FactsWeb = [Option<&str>; 3];

impl Create for WebS {
	type Output = WebS;
	type Facts = FactsWeb;
}