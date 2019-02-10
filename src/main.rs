use clap::{App, Arg, SubCommand};
use kaiser::ciphers::Decrypt;
use kaiser::ciphers::Encrypt;
use std::io::{self, Read};

#[macro_use]
extern crate scan_fmt;

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
        .subcommand(
            SubCommand::with_name("quadgrams")
                .about("Generates a quadgram score for the text (higher is better)"),
        )
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
        )
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Implements encryption of several common classical ciphers")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .takes_value(true)
                        .help("Select the cipher type"),
                )
                .arg(
                    Arg::with_name("key")
                        .short("k")
                        .long("key")
                        .takes_value(true)
                        .help("Provide the key to use"),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Implements decryption of several common classical ciphers")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .takes_value(true)
                        .help("Select the cipher type"),
                )
                .arg(
                    Arg::with_name("key")
                        .short("k")
                        .long("key")
                        .takes_value(true)
                        .help("Provide the key to use"),
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
            for (i, freq) in kaiser::stats::letter_frequencies(&input())
                .iter()
                .enumerate()
            {
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
        ("decrypt", Some(matches)) => {
            let key = match matches.value_of("key") {
                Some(_) => matches.value_of("key").unwrap(),
                None => {
                    println!("No key provided, defaulting to 0");
                    "0"
                }
            };
            match matches.value_of("type") {
                Some("caesar") => {
                    // if user tries to provide a string, default to 0 and don't encrypt.
                    let caesar =
                        kaiser::ciphers::Caesar::new(key.parse::<u8>().unwrap_or_else(|_| {
                            println!(
                                "Invalid key provided (must be a single integer), defaulting to 0"
                            );
                            0
                        }));
                    let buf = input();
                    let buf = caesar.decrypt(buf).unwrap();
                    print!("{}", buf.to_string());
                }
                Some("affine") => {
                    // try to read two u8 separated by "," otherwise default to the values of 1,0 (i.e., don't encrypt at all)
                    let (a, b) = scan_fmt!(key, "{},{}", u8, u8);
                    let shift = a.unwrap_or_else(| | {println!("Invalid key provided for a (must be a single integer), defaulting to 1"); 1});
                    let mult = b.unwrap_or_else(| | {println!("Invalid key provided for b (must be a single integer), defaulting to 0"); 0});
                    let affine = kaiser::ciphers::Affine::new(shift, mult);
                    let buf = input();
                    let buf = affine.decrypt(buf).unwrap();
                    print!("{}", buf.to_string());
                }
                Some("vigenere") => {
                    // TODO: Write some sane error handling if user tries to provide a numeric key (e.g. treat 1,2,3 as "ABC")
                    // At the moment it just panics.
                    let vigenere = kaiser::ciphers::Vigenere::new(key);
                    let buf = input();
                    let buf = vigenere.decrypt(buf).unwrap();
                    print!("{}", buf.to_string());
                }
                Some(_) => println!("Unknown cipher type"),
                None => println!("No cipher type provided"),
            }
        }
        ("encrypt", Some(matches)) => {
            let key = match matches.value_of("key") {
                Some(_) => matches.value_of("key").unwrap(),
                None => {
                    println!("No key provided, defaulting to 0");
                    "0"
                }
            };
            match matches.value_of("type") {
                Some("caesar") => {
                    // if user tries to provide a string, default to 0 and don't encrypt.
                    let caesar =
                        kaiser::ciphers::Caesar::new(key.parse::<u8>().unwrap_or_else(|_| {
                            println!(
                                "Invalid key provided (must be a single integer), defaulting to 0"
                            );
                            0
                        }));
                    let buf = input();
                    let buf = caesar.encrypt(buf).unwrap();
                    print!("{}", buf.to_string());
                }
                Some("affine") => {
                    // try to read two u8 separated by "," otherwise default to the values of 1,0 (i.e., don't encrypt at all)
                    let (a, b) = scan_fmt!(key, "{},{}", u8, u8);
                    let shift = a.unwrap_or_else(| | {println!("Invalid key provided for a (must be a single integer), defaulting to 1"); 1});
                    let mult = b.unwrap_or_else(| | {println!("Invalid key provided for b (must be a single integer), defaulting to 0"); 0});
                    let affine = kaiser::ciphers::Affine::new(shift, mult);
                    let buf = input();
                    let buf = affine.encrypt(buf).unwrap();
                    print!("{}", buf.to_string());
                }
                Some("vigenere") => {
                    // TODO: Write some sane error handling if user tries to provide a numeric key (e.g. treat 1,2,3 as "ABC")
                    // At the moment it just panics.
                    let vigenere = kaiser::ciphers::Vigenere::new(key);
                    let buf = input();
                    let buf = vigenere.encrypt(buf).unwrap();
                    print!("{}", buf.to_string());
                }
                Some(_) => println!("Unknown cipher type"),
                None => println!("No cipher type provided"),
            }
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
