use std::cell::OnceCell;
use std::str::FromStr;

use crate::err::{self, ConfigErr};

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

pub enum Args {
    StdOut,
}

impl FromStr for Args {
    type Err = err::ConfigErr<'static>;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--stdout" | "-s" | "-" => Ok(Self::StdOut),
            _ => Err(Self::Err::ArgError("Not recognized given argument!")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FdArgument {
    File(Box<str>),
    StdIO,
}

impl From<Box<str>> for FdArgument {
    #[inline(always)]
    fn from(value: Box<str>) -> Self {
        Self::File(value)
    }
}

#[derive(Debug)]
pub struct Config {
    pub cmd: Command,
    file_input: Box<str>,
    file_out: OnceCell<FdArgument>,
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
        let mut fout = None;
        for arg in args {
            if arg.starts_with("-") {
                match Args::from_str(arg.as_ref())? {
                    Args::StdOut => fout = Some(FdArgument::StdIO),
                }
                continue;
            }

            if fin.is_none() {
                fin = Some(arg);
            }
        }

        let file_out = OnceCell::new();
        if let Some(fout) = fout {
            file_out.set(fout).unwrap();
        }

        if fin.is_none() {
            return Err(ConfigErr::ArgError("Missing file_input argument"));
        }

        Ok(Self {
            cmd,
            file_out,
            file_input: fin.unwrap(),
        })
    }

    #[inline]
    pub fn get_in_file(&self) -> &str {
        &self.file_input
    }

    #[inline]
    pub fn get_out_file(&self) -> &FdArgument {
        self.file_out
            .get_or_init(|| format!("{}.xml", self.file_input).into_boxed_str().into())
    }
}
