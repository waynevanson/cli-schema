mod deserializer;
mod seralizer;

use nonempty::NonEmpty;
use std::env::Args;

pub struct Command {}

pub struct Schema {
    commands: NonEmpty<Command>,
}

fn args_to_strings(args: Args) -> Vec<String> {
    args.into_iter().collect()
}
