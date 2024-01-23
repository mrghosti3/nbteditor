use crate::cli::*;

#[test]
fn cli_compile() {
    let args = [
        "c".to_string().into_boxed_str(),
        "ghosti3.dat".to_string().into_boxed_str(),
    ]
    .into_iter();
    let cli = Config::parse(args).unwrap();

    assert_eq!(cli.cmd, Command::Compile);
    assert_eq!(cli.file_input.as_ref(), "ghosti3.dat");
}
