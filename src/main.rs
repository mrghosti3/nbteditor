use std::env;

mod cli;
mod cmd;
mod err;
mod xml;

#[cfg(test)]
mod test;

fn main() {
    // Process passed arguments and config
    let config = {
        // Only to be used once for parsing now strucured data
        let _args = env::args().skip(1).map(|arg| arg.into_boxed_str());

        match cli::Config::parse(_args) {
            Ok(args) => args,
            Err(err) => process_err(err::MyError::Setup(err)),
        }
    };

    let res = match config.cmd {
        cli::Command::Decompile => cmd::decompile(&config),
        cli::Command::Compile => cmd::compile(&config),
        cli::Command::Watch => todo!("Run util::watch"),
    };

    if let Err(err) = res {
        process_err(err::MyError::Runtime(err));
    }

    std::process::exit(0)
}

fn process_err(err: err::MyError) -> ! {
    use err::MyError::{Runtime, Setup};
    use err::{ConfigErr, RuntimeErr};
    use std::process::exit;

    eprint!("ERROR: ");
    match err {
        Setup(ConfigErr::CommandMissing) => {
            eprintln!("Missing command !!!")
        }
        Setup(ConfigErr::BadCommand(bad_cmd)) => {
            eprintln!("Command '{}' is not implemented !!!", bad_cmd)
        }
        Setup(ConfigErr::ArgError(arg)) => {
            eprintln!("Argument: '{}' !!!", arg)
        }
        Runtime(RuntimeErr::OSError(os_err)) => {
            eprintln!("OS Error: {}", os_err)
        }
        Runtime(RuntimeErr::NBTError(nbt_err)) => {
            eprintln!("NBT LIB Error: {}", nbt_err)
        }
        Runtime(RuntimeErr::XmlError(xml_error)) => {
            eprintln!("QUICK XML Error: {}", xml_error)
        }
    };

    exit(1);
}
