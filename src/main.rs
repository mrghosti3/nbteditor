use std::env;

mod cli;
mod err;
mod state;
mod util;

fn main() {
    // Only to be used once for parsing now strucured data
    let _args = env::args().skip(1).map(|arg| arg.into_boxed_str());

    // Process passed arguments and config
    let config = match cli::Config::parse(_args) {
        Ok(args) => args,
        Err(err) => process_err(err),
    };

    let _ = match config.cmd {
        cli::Command::Compile => todo!("Run util::compile"),
        cli::Command::Decompile => todo!("Run util::decompile"),
        cli::Command::Watch => todo!("Run util::watch"),
    };
}

fn process_err(err: err::ConfigErr) -> ! {
    use err::ConfigErr;
    use std::process::exit;

    eprint!("ERROR: ");
    match err {
        ConfigErr::BadCommand(bad_cmd) => {
            eprintln!("Command '{}' is not implemented !!!", bad_cmd)
        }
        ConfigErr::ArgError(arg) => {
            eprintln!("Argument: '{}' !!!", arg)
        }
        ConfigErr::CommandMissing => {
            eprintln!("Missing command !!!")
        }
    };
    exit(1);
}

#[cfg(test)]
mod test;
