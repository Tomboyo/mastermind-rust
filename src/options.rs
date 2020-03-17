use clap::{App, Arg};

#[derive(Debug)]
pub struct Options {
    pub code_length: usize,
    pub code_base: usize,
}

pub fn from_stdin() -> Options {
    let matches = create_clap_app().get_matches();

    let code_length = matches.value_of("code-length")
        .unwrap().parse().unwrap();
    let code_base = matches.value_of("code-base")
        .unwrap().parse().unwrap();
    
    Options { code_length, code_base }
}

fn create_clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("mastermind")
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("code-length")
                .long("code-length")
                .takes_value(true)
                .required(true)
                .validator(validate_positive_int)
                .help("The length ('number of pegs') of each code in the \
                       universe of codes for this simulation. For example, if \
                       each code in a game of mastermind is comprised of six \
                       pegs, then code-length is 6."),
        )
        .arg(
            Arg::with_name("code-base")
                .long("code-base")
                .takes_value(true)
                .required(true)
                .validator(validate_positive_int)
                .help("The base ('number of colors per peg') of each code in \
                       the universe of codes for this simulation. For example, \
                       if each code in a game of mastermind is comprised of one \
                       of six color choices, then the code-base is 6."),
        )
}

fn validate_positive_int(value: String) -> Result<(), String> {
    value.parse::<u32>()
        .or_else(|_| Err(String::from("must be a positive integer")))
        .and(Ok(()))
}
