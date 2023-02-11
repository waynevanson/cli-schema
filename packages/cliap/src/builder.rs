use crate::nonempty_map::NonEmptyHashMap;
use enum_as_inner::EnumAsInner;

pub enum Argumental {
    Arguments(Vec<Argument>),
    Subcommands(Vec<Subcommand>),
}

pub struct Command {
    pub flags: Vec<Flag>,
    pub argumental: Argumental,
}

impl Command {
    fn to_subcommand<A>(self, name: A) -> Subcommand
    where
        A: AsRef<String>,
    {
        Subcommand {
            name: name.as_ref().to_string(),
            flags: self.flags,
            argumental: self.argumental,
        }
    }

    // fn find_short_flag(&self) -> Option<(char)> {
    //     self.flags.into_iter().map(|x|x.labels.)
    // }
}

pub struct Subcommand {
    name: String,
    flags: Vec<Flag>,
    argumental: Argumental,
}

impl From<Subcommand> for Command {
    fn from(value: Subcommand) -> Self {
        Self {
            flags: value.flags,
            argumental: value.argumental,
        }
    }
}

pub struct Argument {
    pub optional: bool,
}

#[derive(EnumAsInner, PartialEq, Debug)]
pub enum LabelKind {
    Long,
    Short,
}

pub struct Flag {
    pub name: String,
    pub optional: bool,
    pub labels: NonEmptyHashMap<String, LabelKind>,
    pub argument: Option<Argument>,
}

impl Flag {
    /// Adds a short alias to the flag
    fn short(&mut self, short: char) -> &Self {
        self.labels.insert(short.to_string(), LabelKind::Short);
        self
    }

    /// Adds a short alias to the flag
    fn long(&mut self, long: String) -> &Self {
        self.labels.insert(long, LabelKind::Long);
        self
    }
}
