use std::cell::OnceCell;
use std::fs::File;
use std::io::{stdin, stdout};
use std::os::fd::{AsRawFd, FromRawFd};
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
    FileOutput,
}

impl FromStr for Args {
    type Err = err::ConfigErr<'static>;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "--file" | "-f" => Ok(Self::FileOutput),
            _ => Err(Self::Err::ArgError("Not recognized given argument!")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FdArgument {
    File(Box<str>),
    StdIn,
    StdOut,
}

impl FdArgument {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::StdIn => "STDIN",
            Self::StdOut => "STDOUT",
            Self::File(fname) => Box::leak(fname.clone()),
        }
    }

    pub fn to_file(&self, write: bool) -> std::io::Result<File> {
        match self {
            Self::StdIn => unsafe { Ok(File::from_raw_fd(stdin().as_raw_fd())) },
            Self::StdOut => unsafe { Ok(File::from_raw_fd(stdout().as_raw_fd())) },
            Self::File(fname) => {
                if write {
                    File::create(fname.as_ref())
                } else {
                    File::open(fname.as_ref())
                }
            }
        }
    }
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
    file_input: FdArgument,
    file_out: FdArgument,
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

        let fin: OnceCell<FdArgument> = OnceCell::new();
        let fout: OnceCell<FdArgument> = OnceCell::new();
        while let Some(arg) = args.next() {
            if arg.starts_with("-") && arg.as_ref() != "-" {
                let _ = match Args::from_str(arg.as_ref())? {
                    Args::FileOutput => fout.set(FdArgument::File(
                        args.next()
                            .ok_or(ConfigErr::ArgError("Missing output file name!"))?,
                    )),
                };
                continue;
            }

            let _ = match arg.as_ref() {
                "-" => fin.set(FdArgument::StdIn),
                _ => fin.set(FdArgument::File(arg)),
            };
        }

        fin.get_or_init(|| FdArgument::StdIn);
        fout.get_or_init(|| FdArgument::StdOut);

        Ok(Self {
            cmd,
            file_out: fout.into_inner().unwrap(),
            file_input: fin.into_inner().unwrap(),
        })
    }

    #[inline]
    pub fn get_in_file(&self) -> &FdArgument {
        &self.file_input
    }

    #[inline]
    pub fn get_out_file(&self) -> &FdArgument {
        &self.file_out
    }
}
