use clap::{App, Arg, SubCommand};
use std::io::{self, Read};

fn main() {
    let mut app = App::new("kaiser")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("ioc").about("Calculates the index of coincidence"))
        .subcommand(
            SubCommand::with_name("chi")
                .about("Calculates chi squared statistic against english letter distribution"),
        )
        .subcommand(SubCommand::with_name("freqs").about("Counts letter frequencies"))
        .subcommand(SubCommand::with_name("quadgrams").about("Generates a quadgram score for the text (higher is better)"))
        .subcommand(
            SubCommand::with_name("trim")
                .about(
                    "Strips all non-alphabetic characters and converts to uppercase. \
                     Optionally extracts only the characters at a specified stride and offset",
                )
                .arg(
                    Arg::with_name("stride")
                        .long("stride")
                        .takes_value(true)
                        .help("Take every nth character"),
                )
                .arg(
                    Arg::with_name("offset")
                        .long("offset")
                        .takes_value(true)
                        .help("Start taking characters from this offset"),
                ),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("ioc", Some(_)) => {
            println!("{}", kaiser::stats::index_of_coincidence(&input()));
        }
        ("chi", Some(_)) => {
            println!("{}", kaiser::stats::chi_squared(&input()));
        }
        ("quadgrams", Some(_)) => {
            println!("{}", kaiser::stats::quadgram_score(&input()));
        }
        ("freqs", Some(_)) => {
            for (i, freq) in input().letter_frequencies().iter().enumerate() {
                println!("{}: {}", (b'A' + i as u8) as char, freq);
            }
        }
        ("trim", Some(matches)) => {
            let stride = matches.value_of("stride").map_or(1, |s| {
                s.parse::<usize>()
                    .ok()
                    .filter(|&i| i > 0)
                    .expect("stride must be a positive integer")
            });

            let offset = matches.value_of("offset").map_or(0, |s| {
                s.parse::<usize>()
                    .expect("offset must be a positive integer")
            });

            let mut s = String::new();
            io::stdin()
                .read_to_string(&mut s)
                .expect("unable to read from stdin");

            let out = s
                .chars()
                .filter(|c| c.is_ascii() && c.is_alphabetic())
                .map(|c| c.to_ascii_uppercase())
                .skip(offset)
                .step_by(stride)
                .collect::<String>();

            println!("{}", out);
        }
        _ => {
            app.print_help().unwrap();
        }
    }
}

fn input() -> kaiser::Buffer {
    let mut s = String::new();

    io::stdin()
        .read_to_string(&mut s)
        .expect("unable to read from stdin");

    kaiser::Buffer::from(&s)
}
