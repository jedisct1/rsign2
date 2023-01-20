use clap::{Arg, ArgAction::SetTrue, Command};

pub fn parse_args() -> (clap::ArgMatches, String) {
    let mut app = command!()
        .subcommand(
            Command::new("generate")
                .about("Generate public and private keys")
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-path")
                        .num_args(1)
                        .value_name("PUBLIC_KEY_PATH")
                        .help("path to the new public key"),
                )
                .arg(
                    Arg::new("sk_path")
                        .short('s')
                        .long("secret-key-path")
                        .num_args(1)
                        .value_name("SECRET_KEY_PATH")
                        .help("path to the new secret key"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .action(SetTrue)
                        .help("force generate a new keypair"),
                )
                .arg(
                    Arg::new("comment")
                        .short('c')
                        .long("comment")
                        .num_args(1)
                        .value_name("COMMENT")
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
                        .num_args(1)
                        .value_name("PUBLIC_KEY_STRING")
                        .conflicts_with("pk_path")
                        .help("public key string"),
                )
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-path")
                        .num_args(1)
                        .value_name("PUBLIC_KEY_PATH")
                        .help("path to public key file"),
                )
                .arg(
                    Arg::new("sig_file")
                        .short('x')
                        .long("sig-file")
                        .num_args(1)
                        .value_name("SIG_FILE")
                        .help("signature file to be verified"),
                )
                .arg(
                    Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .action(SetTrue)
                        .help("quiet mode, supress output"),
                )
                .arg(
                    Arg::new("allow-legacy")
                        .short('l')
                        .long("allow-legacy")
                        .action(SetTrue)
                        .help("accept legacy signatures"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .action(SetTrue)
                        .help("output the file content after verification"),
                )
                .arg(
                    Arg::new("file")
                        .index(1)
                        .num_args(1)
                        .required(true)
                        .value_name("FILE")
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
                        .num_args(1)
                        .value_name("PUBLIC_KEY_STRING")
                        .conflicts_with("pk_path")
                        .help("public key string"),
                )
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-file")
                        .num_args(1)
                        .value_name("PUBLIC_KEY_FILE")
                        .help("path to public key file"),
                )
                .arg(
                    Arg::new("sk_path")
                        .short('s')
                        .long("secret-key-file")
                        .num_args(1)
                        .value_name("SECRET_KEY_FILE")
                        .help("secret key to be used to sign"),
                )
                .arg(
                    Arg::new("sig_file")
                        .short('x')
                        .long("sig-file")
                        .num_args(1)
                        .value_name("SIG_FILE")
                        .help("signature file"),
                )
                .arg(
                    Arg::new("data")
                        .index(1)
                        .num_args(1)
                        .value_name("FILE")
                        .required(true)
                        .help("file to sign"),
                )
                .arg(
                    Arg::new("trusted-comment")
                        .short('t')
                        .long("trusted-comment")
                        .num_args(1)
                        .value_name("TRUSTED_COMMENT")
                        .help("add a one-line trusted comment"),
                )
                .arg(
                    Arg::new("untrusted-comment")
                        .short('c')
                        .long("untrusted-comment")
                        .num_args(1)
                        .value_name("UNTRUSTED_COMMENT")
                        .help("add a one-line untrusted comment"),
                )
                .arg(
                    Arg::new("hash")
                        .short('H')
                        .long("hash")
                        .required(false)
                        .action(SetTrue)
                        .help("ignored (for backwards compatibility only"),
                ),
        );
    let help_usage = app.render_usage().to_string();
    let matches = app.get_matches();
    (matches, help_usage)
}
