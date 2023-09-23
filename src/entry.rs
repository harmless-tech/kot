use std::env::Args;

pub struct EntryArgs {}

pub fn collect_args() -> EntryArgs {
    let args = std::env::args().collect();
    parse_args(args)
}

fn parse_args(mut args: Vec<String>) -> EntryArgs {
    todo!()
}

// TODO: Test!!!
