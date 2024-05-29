use crate::cli::*;

#[test]
fn cli_compile() {
    let args = [
        Box::from("c"),
        Box::from("ghosti3.dat"),
    ]
    .into_iter();
    let cli = Config::parse(args).unwrap();

    assert_eq!(cli.cmd, Command::Compile);
    assert_eq!(cli.get_in_file().to_str(), "ghosti3.dat");
}
