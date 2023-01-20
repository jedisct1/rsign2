use clap::{builder::StyledStr, command, value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;

pub fn parse_args() -> (clap::ArgMatches, StyledStr) {
    let mut app = command!()
        .subcommand(
            Command::new("generate")
                .about("Generate public and private keys")
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-path")
                        .value_name("PUBLIC_KEY_PATH")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("path to the new public key"),
                )
                .arg(
                    Arg::new("sk_path")
                        .short('s')
                        .long("secret-key-path")
                        .value_name("SECRET_KEY_PATH")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("path to the new secret key"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("force generate a new keypair"),
                )
                .arg(
                    Arg::new("comment")
                        .short('c')
                        .long("comment")
                        .value_name("COMMENT")
                        .action(ArgAction::Set)
                        .help("add a one-line untrusted comment"),
                ),
        )
        .subcommand(
            Command::new("verify")
                .about("Verify a signed file with a given public key")
                .arg(
                    Arg::new("public_key")
                        .short('P')
                        .long("public-key-string")
                        .conflicts_with("pk_path")
                        .value_name("PUBLIC_KEY_STRING")
                        .action(ArgAction::Set)
                        .help("public key string"),
                )
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-path")
                        .value_name("PUBLIC_KEY_PATH")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("path to public key file"),
                )
                .arg(
                    Arg::new("sig_file")
                        .short('x')
                        .long("sig-file")
                        .value_name("SIG_FILE")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("signature file to be verified"),
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .action(ArgAction::SetTrue)
                        .help("quiet mode, supress output"),
                )
                .arg(
                    Arg::new("allow-legacy")
                        .short('l')
                        .long("allow-legacy")
                        .action(ArgAction::SetTrue)
                        .help("accept legacy signatures"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .action(ArgAction::SetTrue)
                        .help("output the file content after verification"),
                )
                .arg(
                    Arg::new("file")
                        .index(1)
                        .required(true)
                        .value_name("FILE")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("file to be verified"),
                ),
        )
        .subcommand(
            Command::new("sign")
                .about("Sign a file with a given private key")
                .arg(
                    Arg::new("public_key")
                        .short('P')
                        .long("public-key-string")
                        .conflicts_with("pk_path")
                        .value_name("PUBLIC_KEY_STRING")
                        .action(ArgAction::Set)
                        .help("public key string"),
                )
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-file")
                        .value_name("PUBLIC_KEY_FILE")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("path to public key file"),
                )
                .arg(
                    Arg::new("sk_path")
                        .short('s')
                        .long("secret-key-file")
                        .value_name("SECRET_KEY_FILE")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("secret key to be used to sign"),
                )
                .arg(
                    Arg::new("sig_file")
                        .short('x')
                        .long("sig-file")
                        .value_name("SIG_FILE")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("signature file"),
                )
                .arg(
                    Arg::new("data")
                        .index(1)
                        .required(true)
                        .value_name("FILE")
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set)
                        .help("file to sign"),
                )
                .arg(
                    Arg::new("trusted-comment")
                        .short('t')
                        .long("trusted-comment")
                        .value_name("TRUSTED_COMMENT")
                        .action(ArgAction::Set)
                        .help("add a one-line trusted comment"),
                )
                .arg(
                    Arg::new("untrusted-comment")
                        .short('c')
                        .long("untrusted-comment")
                        .value_name("UNTRUSTED_COMMENT")
                        .action(ArgAction::Set)
                        .help("add a one-line untrusted comment"),
                )
                .arg(
                    Arg::new("hash")
                        .short('H')
                        .long("hash")
                        .action(ArgAction::SetTrue)
                        .help("ignored (for backwards compatibility only"),
                ),
        );
    let help_usage = app.render_usage();
    let matches = app.get_matches();
    (matches, help_usage)
}
