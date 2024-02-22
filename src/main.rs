use clap::{command, Arg};

fn main() {
    let match_results = command!()
        .arg(
            Arg::new("action")
                .short('a')
                .long("action")
                .value_name("ACTION")
                .required(true),
        )
        .get_matches();

    match match_results {}
}
