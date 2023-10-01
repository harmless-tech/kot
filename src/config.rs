use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    /// Print version info and exit. (Default: false)
    pub version: bool,
    /// Cache kotfile AST under .kotfile.cache. (Default: false)
    // TODO
    pub cache: bool,
    /// How many cmds to run in parallel when using .parallel. (Default: Number of logical cores)
    // TODO
    pub children: usize,
    /// Require runtime to be using a 64bit int. (Default: false)
    pub require_i64: bool,
    /// Allow .inject. (Default: false)
    // TODO
    pub unsafe_inject: bool,
}
impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_config(config_str: &str) -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.configure(config_str)?;
        Ok(config)
    }

    pub fn from_config_slice(config_slice: &[String]) -> anyhow::Result<Self> {
        let mut config = Config::new();
        for c in config_slice {
            config.configure(c)?;
        }
        Ok(config)
    }

    pub fn configure_slice(&mut self, config_slice: &[String]) -> anyhow::Result<()> {
        for c in config_slice {
            self.configure(c)?;
        }
        Ok(())
    }

    pub fn configure(&mut self, config_str: &str) -> anyhow::Result<()> {
        let options: Vec<&str> = config_str.split(',').map(|s| s.trim()).collect();
        for o in options {
            let mut opt = o;
            let mut val = "true";

            if o.contains('=') {
                let s: Vec<&str> = o.split('=').collect();
                if s.len() != 2 {
                    return Err(ConfigError::MalformedConfigString(
                        format!("Multiple equals '=' in {o}"),
                        o.to_string(),
                    )
                    .into());
                }
                opt = s.first().unwrap();
                val = s.get(1).unwrap()
            }

            match opt {
                "version" => self.version = Self::true_false(opt, val)?,
                "cache" => self.cache = Self::true_false(opt, val)?,
                "children" => self.children = Self::num_usize(opt, val)?,
                "require_i64" => self.require_i64 = Self::true_false(opt, val)?,
                "unsafe_inject" => self.unsafe_inject = Self::true_false(opt, val)?,
                _ => return Err(ConfigError::UnknownConfig(opt.to_string()).into()),
            }
        }

        Ok(())
    }

    fn true_false(name: &str, str: &str) -> anyhow::Result<bool> {
        match str {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(ConfigError::WrongConfigValue {
                name: name.to_string(),
                found: str.to_string(),
                required_type: "boolean: true/false".to_string(),
            }
            .into()),
        }
    }

    fn num_usize(name: &str, str: &str) -> anyhow::Result<usize> {
        match usize::from_str(str) {
            Ok(num) => Ok(num),
            Err(_) => Err(ConfigError::WrongConfigValue {
                name: name.to_string(),
                found: str.to_string(),
                required_type: "number: 0 - usize::MAX".to_string(),
            }
            .into()),
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: false,
            cache: false,
            children: num_cpus::get(),
            require_i64: false,
            unsafe_inject: false,
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    UnknownConfig(String),
    WrongConfigValue {
        name: String,
        found: String,
        required_type: String,
    },
    MalformedConfigString(String, String),
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::write;
        match self {
            ConfigError::UnknownConfig(name) => {
                write(f, format_args!("Unknown config option: {name}"))
            }
            ConfigError::WrongConfigValue {
                name,
                found,
                required_type,
            } => write(
                f,
                format_args!(
                    "Config option '{name}' requires type {required_type}, but found {found}"
                ),
            ),
            ConfigError::MalformedConfigString(reason, str) => write(
                f,
                format_args!("Config string was malformed. {reason}. {str}"),
            ),
        }
    }
}
impl std::error::Error for ConfigError {}

// TODO: TEST!!!!
