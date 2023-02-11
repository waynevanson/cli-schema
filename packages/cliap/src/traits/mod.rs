use combine::Parser;

use crate::builder::{Argument, Command, Flag, Subcommand};
use crate::parser::flag;
use std::vec::IntoIter;

pub trait Flags {
    fn flags() -> Vec<Flag> {
        vec![]
    }
}

pub trait Subcommands {
    fn subcommands() -> Vec<Subcommand>;
}

pub trait Commands {
    fn command() -> Command;
}

pub trait Arguments {
    fn arguments() -> Vec<Argument> {
        vec![]
    }
}

type Args = Vec<String>;

pub struct FromArgs {
    args: IntoIter<String>,
    state: State,
    // probably need to create a partial that goes from one to another.
    command: Command,
}

enum Kind {
    Command(String),
    Subcomand(String),
    Flag { name: String },
    FlagArgument { name: String, argument: String },
    Argument,
}

enum State {
    Initialising,
    Initialised,
}

impl Iterator for FromArgs {
    type Item = Kind;

    fn next(&mut self) -> Option<Self::Item> {
        use Kind::*;
        use State::*;

        let arg = self.args.next()?;

        match self.state {
            Initialising => {
                self.state = Initialised;
                Command(arg)
            }
            Initialised => {
                // is it a flag? if so check all flags
                let hi = flag().parse(&arg).map(|result| result.0);
                if let Ok(((identifier, label_kind), argument)) = hi {
                    let index = self
                        .command
                        .flags
                        .into_iter()
                        .enumerate()
                        .find(|(_, flag)| {
                            flag.labels
                                .into_iter()
                                .any(|(x, y)| x == identifier && y == label_kind)
                        })
                        .map(|(index, _)| index)?;
                    let flag = self.command.flags.swap_remove(index);

                    // should I have an argument?
                    // if argument is nonflage

                    let name = flag.name;
                    if let Some(argument) = argument {
                        FlagArgument { name, argument }
                    } else {
                        Flag { name }
                    }
                } else {
                    let cmds = self.command.flags;

                    // is it a subcommand?
                    // does the flag have an argument?
                    // welp I don't think there's much else left
                }
            }
        }
        .into()
    }
}

pub trait Cli: Commands {
    fn to_fromargs(args: Args) -> FromArgs {
        FromArgs {
            args: args.into_iter(),
            state: State::Initialising,
            command: Self::command(),
        }
    }
}
