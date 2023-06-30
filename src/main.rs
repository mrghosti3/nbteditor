pub(crate) mod err;
pub(crate) mod state;
pub(crate) mod util;

fn main() {
    let (watch, mut state, fname_out) = init().expect("Could not init proc");

    util::decompile(&mut state).unwrap();

    if !watch {
        return;
    }

    let inotif = util::inotify_init().expect("Could not get inotify instance");
    let out_fd =
        util::inotify_add_watch(&inotif, &fname_out).expect("Could not add watch for output file");

    loop {
        // TODO: test between NBT data of various block counts
        let evs = inotif.read_events().unwrap();
        evs.iter().for_each(|ev| {
            if ev.wd == out_fd {
                println!("Changes on YAML file!");
                util::compile(&mut state).expect("Could not compile NBT file");
            }
        });
    }
}

/// private function for collecting arguments and initiating exe STATE.
///
/// # Errors
///
/// This function will return an error if:
/// - input arguments are structured badly (not in a supported structure);
/// - given file name is incorrect;
/// - STATE cannot open files.
fn init() -> Result<(bool, state::State, String), err::MyError> {
    use std::env;
    use std::path::Path;

    let args: Box<[String]> = env::args().skip(1).collect();

    let (watch, fname_in, fname_out) = match &args[..] {
        [watch, file] => {
            let watch = *watch == "watch";
            if !watch {
                return Err(err::MyError::ArgError(
                    "When arguments there are 2 arguments, the first one must be 'watch'!!!",
                ));
            }

            let fname_out = util::make_fname(Path::new(&file))?;
            (true, file.as_str(), fname_out)
        }
        [file] => {
            let fname_out = util::make_fname(Path::new(&file))?;
            (false, file.as_str(), fname_out)
        }
        _ => {
            return Err(err::MyError::ArgError(
                "This structure of args is currently not supported.",
            ))
        }
    };

    let state = state::State::new(fname_in, &fname_out)?;

    Ok((watch, state, fname_out))
}
