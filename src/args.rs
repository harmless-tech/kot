#[derive(Debug)]
pub struct EntryArgs {
    pub kot: Vec<String>,
    pub single: Vec<(Vec<char>, Option<String>, Vec<usize>)>,
    pub double: Vec<(String, Option<String>, Vec<usize>)>,
    pub positional: Vec<String>,
    pub last: Vec<String>,
}

#[derive(Debug)]
struct EntryArgsImm {
    pub kot: Vec<String>,
    pub single: Vec<(String, Vec<usize>)>,
    pub double: Vec<(String, Vec<usize>)>,
    pub positional: Vec<String>,
    pub last: Vec<String>,
}
impl EntryArgsImm {
    fn new() -> Self {
        Self {
            kot: Vec::new(),
            single: Vec::new(),
            double: Vec::new(),
            positional: Vec::new(),
            last: Vec::new(),
        }
    }
}

pub fn collect_args() -> EntryArgs {
    let args = std::env::args().collect();
    let imm = parse_args(args);
    flatten_args(imm)
}

fn parse_args(args: Vec<String>) -> EntryArgsImm {
    #[cfg(debug_assertions)]
    dbg!(&args);

    let mut args = args.into_iter();
    let _ = args.next(); // Remove the exe arg.

    let mut entry = EntryArgsImm::new();
    let mut e_hold = None;

    while let Some(mut a) = args.next() {
        if a.eq("--") {
            // Last
            entry.last = args.collect();
            break;
        }
        else if a.starts_with("--") {
            // Double
            a.drain(0..2);

            if a.is_empty() {
                panic!("Args: Double dash (--) without any alphanumeric character after it is not allowed.");
            }

            if a.starts_with("kot ") {
                let arg = args.next().expect("Args: No arguments after --kot.");
                if arg.starts_with('-') {
                    panic!("Args: Dash (- | --) argument after --kot.");
                }
                entry.kot.push(arg);
            }
            else if a.starts_with("kot=") {
                a.drain(0..4);
                entry.kot.push(a);
            }
            else {
                entry.double.push((a.clone(), Vec::new()));
                let i = entry.double.len() - 1;
                e_hold = entry.double.get_mut(i);
            }
        }
        else if a.starts_with('-') {
            // Single
            a.remove(0);
            if a.is_empty() {
                panic!("Args: Single dash (-) without any alphanumeric character after it is not allowed.");
            }

            entry.single.push((a.clone(), Vec::new()));
            let i = entry.single.len() - 1;
            e_hold = entry.single.get_mut(i);
        }
        else {
            // Positional
            entry.positional.push(a);
            match &mut e_hold {
                Some(e) => e.1.push(entry.positional.len() - 1),
                None => {}
            }
        }
    }

    entry
}

fn flatten_args(imm: EntryArgsImm) -> EntryArgs {
    let kot = imm.kot;
    let positional = imm.positional;
    let last = imm.last;

    // Process single
    let mut single = Vec::new();
    for (mut s, p) in imm.single {
        let data = split_eq(&mut s);
        single.push((s.chars().collect(), data, p));
    }

    // Process double
    let mut double = Vec::new();
    for (mut s, p) in imm.double {
        let data = split_eq(&mut s);
        double.push((s, data, p));
    }

    EntryArgs {
        kot,
        single,
        double,
        positional,
        last,
    }
}

fn split_eq(s: &mut String) -> Option<String> {
    if let Some(e) = s.find('=') {
        let mut s: String = s.drain(e..s.len()).collect();
        s.remove(0);
        return Some(s);
    }
    None
}

// TODO: Test!!!
