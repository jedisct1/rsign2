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
                        .num_args(1)
                        .help("add a one-line untrusted comment")
                        .value_name("COMMENT")
                        .short('c')
                        .long("comment"),
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
                        .conflicts_with("pk_path")
                        .help("public key string")
                        .value_name("PUBLIC_KEY_STRING"),
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
                        .num_args(1)
                        .help("signature file to be verified")
                        .value_name("SIG_FILE")
                        .short('x')
                        .long("sig-file"),
                )
                .arg(
                    Arg::new("quiet")
                        .help("quiet mode, supress output")
                        .action(SetTrue)
                        .short('q')
                        .long("quiet"),
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
                        .help("output the file content after verification")
                        .short('o')
                        .long("output")
                        .action(SetTrue),
                )
                .arg(
                    Arg::new("file")
                        .index(1)
                        .num_args(1)
                        .required(true)
                        .help("file to be verified")
                        .value_name("FILE"),
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
                        .conflicts_with("pk_path")
                        .help("public key string")
                        .value_name("PUBLIC_KEY_STRING"),
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
                        .num_args(1)
                        .help("signature file")
                        .value_name("SIG_FILE")
                        .short('x')
                        .long("sig-file"),
                )
                .arg(
                    Arg::new("data")
                        .index(1)
                        .num_args(1)
                        .required(true)
                        .help("file to sign")
                        .value_name("FILE")
                        .short('m')
                        .long("message-file"),
                )
                .arg(
                    Arg::new("trusted-comment")
                        .help("add a one-line trusted comment")
                        .value_name("TRUSTED_COMMENT")
                        .num_args(1)
                        .short('t')
                        .long("trusted-comment"),
                )
                .arg(
                    Arg::new("untrusted-comment")
                        .help("add a one-line untrusted comment")
                        .value_name("UNTRUSTED_COMMENT")
                        .num_args(1)
                        .short('c')
                        .long("untrusted-comment"),
                )
                .arg(
                    Arg::new("hash")
                        .required(false)
                        .short('H')
                        .long("hash")
                        .action(SetTrue)
                        .help("ignored (for backwards compatibility only"),
                ),
        );
    let help_usage = app.render_usage().to_string();
    let matches = app.get_matches();
    (matches, help_usage)
}
