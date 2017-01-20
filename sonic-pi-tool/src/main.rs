#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate lib;

fn main() {
    let cli_app = App::new("sonic-pi-tool")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequired)
        .version(crate_version!());

    let eval_stdin = SubCommand::with_name("eval-stdin")
        .about("Reads Sonic Pi code from stdin and sends it to the server");

    let eval_file = SubCommand::with_name("eval-file")
        .about("Reads Sonic Pi code from a file and sends it to the server")
        .arg(Arg::with_name("PATH")
            .help("Path to the file of Sonic Pi code")
            .required(true)
            .index(1));

    let stop = SubCommand::with_name("stop")
        .about("Stops all currently playing music on the server");

    let matches = cli_app.subcommand(stop)
        .subcommand(eval_stdin)
        .subcommand(eval_file)
        .get_matches();

    match matches.subcommand_name() {
        Some("stop") => lib::stop(),
        Some("eval-stdin") => lib::eval_stdin(),
        Some("eval-file") => do_eval_file(matches),
        _ => panic!("This should be unreachable."),
    }
}

fn do_eval_file(matches: clap::ArgMatches) {
    let path = matches.subcommand_matches("eval-file")
        .unwrap()
        .value_of("PATH")
        .unwrap()
        .to_string();
    lib::eval_file(path);

}