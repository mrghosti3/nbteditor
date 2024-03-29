use std::cell::OnceCell;

use crate::err::ConfigErr;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Compile,
    Decompile,
    Watch,
}

impl Command {
    #[inline]
    fn from_str(cmd: &str) -> Option<Self> {
        match cmd {
            "compile" | "c" => Some(Self::Compile),
            "decompile" | "d" => Some(Self::Decompile),
            "watch" | "w" => Some(Self::Watch),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub cmd: Command,
    file_input: Box<str>,
    file_out: OnceCell<Box<str>>,
}

impl Config {
    pub fn parse<'a, T>(mut args: T) -> Result<Self, ConfigErr<'a>>
    where
        T: Iterator<Item = Box<str>>,
    {
        let cmd: Command = {
            let cmd = match args.next() {
                Some(arg) => arg,
                None => return Err(ConfigErr::CommandMissing),
            };

            match Command::from_str(cmd.as_ref()) {
                Some(cmd) => cmd,
                None => return Err(ConfigErr::BadCommand(Box::leak(cmd))),
            }
        };

        let mut fin = None;
        for arg in args {
            if arg.starts_with("-") {
                eprintln!("WARNING: optional args not implemented yet: {}", arg);
                continue;
            }

            if fin.is_none() {
                fin = Some(arg);
            }
        }

        if fin.is_none() {
            return Err(ConfigErr::ArgError("Missing file_input argument"));
        }

        Ok(Self {
            cmd,
            file_input: fin.unwrap(),
            file_out: OnceCell::new(),
        })
    }

    #[inline]
    pub fn get_in_file(&self) -> &str {
        &self.file_input
    }

    #[inline]
    pub fn get_out_file(&self) -> &str {
        self.file_out
            .get_or_init(|| format!("{}.xml", self.file_input).into_boxed_str())
    }
}
