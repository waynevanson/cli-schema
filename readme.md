# Cliap

## Goals

Given an applicable derivable data structure, we should be able to compose structures together to create a type safe, predicable and safe CLI.

CLAP has some problems for the flexibility that I sometimes require, so I'd rather use something else.

## How it works

- Your data is converted to a schema.
- Reads CLI using the schema as validation structure (cli schema) that can be serialized/deserialised by Serde.
- Use serde to deserialize from schema to structure.
- Enjoy!

## Notes

```
command [options] subcommand_one [options] [argument]
                  subcommand_two <argument_one> <argument_two>

    -v[v][v][v] --verbose[={1,2,3,4}]   2           Verbosity
    -f          --force                 false       Force

subcommand_one
    -p=PORT     --port=PORT             undefined   Port

    argument

subcommand_two
    argument_one
    argument_two

```

Maybe we also have differences between SE and DE?

```rust
#[derive(Serialize, Deserialize)]
struct Args {
    argument_one: String
};

#[derive(Serialize, Deserialize)]
struct Flags {
    #[serde(rename = "verbose", alias = 'v', default = 2)]
    verbosity: usize

    #[serde(alias = 'v', default)]
    force: bool
}

#[derive(Serialize, Deserialize)]
enum Subcommands {
    #[serde(rename = "subcommand_one", alias = "one")]
    One(one::Command),

    #[serde(rename = "subcommand_two", alias = "two")]
    Two(two::Command)
}

#[derive(Serialize, Deserialize)]
struct Command(Flags, Args, Subcommands)

mod one {
    #[derive(Serialize, Deserialize)]
    struct Args {
        argument_one: String
    };

    #[derive(Serialize, Deserialize)]
    struct Flags {
        #[serde(rename = "", alias = 'p', default)]
        port: Option<String>
    };

    #[derive(Serialize, Deserialize)]
    struct Command(Flags, Args)
}

mod two {
    #[derive(Serialize, Deserialize)]
    struct Args {
        argument_one: String
        argument_two: String
    };

    #[derive(Serialize, Deserialize)]
    struct Command((), Args)
}
```
