#[derive(Debug)]
pub struct Config {
    pub version: bool,
    pub list: bool,
    pub cache: bool,
    pub children: usize,
    pub pure_raw_strings: bool,
    pub require_i64: bool,
}
impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn pull_from_args(&mut self, mut args: Args) -> anyhow::Result<()> {
    //     let _exe_path = args.next();
    //     let mut kot_arg = false;
    //
    //     for a in args {
    //         if a.starts_with("--kot") {
    //             match a.get(5) {
    //                 Some(a) => {}
    //                 None => kot_arg,
    //             }
    //         }
    //         else if kot_arg {
    //             if a.starts_with("-") {
    //                 panic!("Malformed --kot args. ");
    //             }
    //         }
    //     }
    //
    //     Ok(())
    // }
    //
    // pub fn config(&mut self, ) -> anyhow::Result<()> {
    //     Ok(())
    // }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: false,
            list: false,
            cache: false,
            children: num_cpus::get(),
            pure_raw_strings: false,
            require_i64: false,
        }
    }
}

// TODO: TEST!!!!
