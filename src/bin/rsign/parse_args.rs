use clap::{App, Arg};

pub fn parse_args() -> (clap::ArgMatches, String) {
    let mut app = app_from_crate!()
        .subcommand(
            App::new("generate")
                .about("Generate public and private keys")
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-path")
                        .takes_value(true)
                        .value_name("PUBLIC_KEY_PATH")
                        .help("path to the new public key"),
                )
                .arg(
                    Arg::new("sk_path")
                        .short('s')
                        .long("secret-key-path")
                        .takes_value(true)
                        .value_name("SECRET_KEY_PATH")
                        .help("path to the new secret key"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("force generate a new keypair"),
                )
                .arg(
                    Arg::new("comment")
                        .takes_value(true)
                        .help("add a one-line untrusted comment")
                        .value_name("COMMENT")
                        .short('c')
                        .long("comment"),
                ),
        )
        .subcommand(
            App::new("verify")
                .about("Verify a signed file with a given public key")
                .arg(
                    Arg::new("public_key")
                        .short('P')
                        .long("public-key-string")
                        .takes_value(true)
                        .conflicts_with("pk_path")
                        .help("public key string")
                        .value_name("PUBLIC_KEY_STRING"),
                )
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-path")
                        .takes_value(true)
                        .value_name("PUBLIC_KEY_PATH")
                        .help("path to public key file"),
                )
                .arg(
                    Arg::new("sig_file")
                        .takes_value(true)
                        .help("signature file to be verified")
                        .value_name("SIG_FILE")
                        .short('x')
                        .long("sig-file"),
                )
                .arg(
                    Arg::new("quiet")
                        .help("quiet mode, supress output")
                        .takes_value(false)
                        .short('q')
                        .long("quiet"),
                )
                .arg(
                    Arg::new("allow-legacy")
                        .short('l')
                        .long("allow-legacy")
                        .help("accept legacy signatures"),
                )
                .arg(
                    Arg::new("output")
                        .help("output the file content after verification")
                        .takes_value(false)
                        .short('o')
                        .long("output"),
                )
                .arg(
                    Arg::new("file")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .help("file to be verified")
                        .value_name("FILE")
                        .short('m')
                        .long("file-name"),
                ),
        )
        .subcommand(
            App::new("sign")
                .about("Sign a file with a given private key")
                .arg(
                    Arg::new("public_key")
                        .short('P')
                        .long("public-key-string")
                        .takes_value(true)
                        .conflicts_with("pk_path")
                        .help("public key string")
                        .value_name("PUBLIC_KEY_STRING"),
                )
                .arg(
                    Arg::new("pk_path")
                        .short('p')
                        .long("public-key-file")
                        .takes_value(true)
                        .value_name("PUBLIC_KEY_FILE")
                        .help("path to public key file"),
                )
                .arg(
                    Arg::new("sk_path")
                        .short('s')
                        .long("secret-key-file")
                        .takes_value(true)
                        .value_name("SECRET_KEY_FILE")
                        .help("secret key to be used to sign"),
                )
                .arg(
                    Arg::new("sig_file")
                        .takes_value(true)
                        .help("signature file")
                        .value_name("SIG_FILE")
                        .short('x')
                        .long("sig-file"),
                )
                .arg(
                    Arg::new("data")
                        .index(1)
                        .takes_value(true)
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
                        .short('t')
                        .long("trusted-comment"),
                )
                .arg(
                    Arg::new("untrusted-comment")
                        .help("add a one-line untrusted comment")
                        .value_name("UNTRUSTED_COMMENT")
                        .short('c')
                        .long("untrusted-comment"),
                )
                .arg(
                    Arg::new("hash")
                        .required(false)
                        .short('H')
                        .long("hash")
                        .help("ignored (for backwards compatibility only"),
                ),
        );
    let help_usage = app.render_usage();
    let matches = app.get_matches();
    (matches, help_usage)
}
