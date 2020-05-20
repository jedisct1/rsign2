use clap::{Arg, SubCommand};

pub fn parse_args<'a>() -> clap::ArgMatches<'a> {
    app_from_crate!()
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate public and private keys")
                .arg(
                    Arg::with_name("pk_path")
                        .short("p")
                        .long("public-key-path")
                        .takes_value(true)
                        .value_name("PUBLIC_KEY_PATH")
                        .help("path to the new public key"),
                )
                .arg(
                    Arg::with_name("sk_path")
                        .short("s")
                        .long("secret-key-path")
                        .takes_value(true)
                        .value_name("SECRET_KEY_PATH")
                        .help("path to the new secret key"),
                )
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("force generate a new keypair"),
                )
                .arg(
                    Arg::with_name("comment")
                        .takes_value(true)
                        .help("add a one-line untrusted comment")
                        .value_name("COMMENT")
                        .short("c")
                        .long("comment"),
                ),
        )
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verify a signed file with a given public key")
                .arg(
                    Arg::with_name("public_key")
                        .short("P")
                        .long("public-key-string")
                        .takes_value(true)
                        .conflicts_with("pk_path")
                        .help("public key string")
                        .value_name("PUBLIC_KEY_STRING"),
                )
                .arg(
                    Arg::with_name("pk_path")
                        .short("p")
                        .long("public-key-path")
                        .takes_value(true)
                        .value_name("PUBLIC_KEY_PATH")
                        .help("path to public key file"),
                )
                .arg(
                    Arg::with_name("sig_file")
                        .takes_value(true)
                        .help("signature file to be verified")
                        .value_name("SIG_FILE")
                        .short("x")
                        .long("sig-file"),
                )
                .arg(
                    Arg::with_name("quiet")
                        .help("quiet mode, supress output")
                        .takes_value(false)
                        .short("q")
                        .long("quiet"),
                )
                .arg(
                    Arg::with_name("output")
                        .help("output the file content after verification")
                        .takes_value(false)
                        .short("o")
                        .long("output"),
                )
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .help("file to be verified")
                        .value_name("FILE")
                        .short("m")
                        .long("file-name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("sign")
                .about("Sign a file with a given private key")
                .arg(
                    Arg::with_name("public_key")
                        .short("P")
                        .long("public-key-string")
                        .takes_value(true)
                        .conflicts_with("pk_path")
                        .help("public key string")
                        .value_name("PUBLIC_KEY_STRING"),
                )
                .arg(
                    Arg::with_name("pk_path")
                        .short("p")
                        .long("public-key-file")
                        .takes_value(true)
                        .value_name("PUBLIC_KEY_FILE")
                        .help("path to public key file"),
                )
                .arg(
                    Arg::with_name("sk_path")
                        .short("s")
                        .long("secret-key-file")
                        .takes_value(true)
                        .value_name("SECRET_KEY_FILE")
                        .help("secret key to be used to sign"),
                )
                .arg(
                    Arg::with_name("sig_file")
                        .takes_value(true)
                        .help("signature file")
                        .value_name("SIG_FILE")
                        .short("x")
                        .long("sig-file"),
                )
                .arg(
                    Arg::with_name("data")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .help("file to sign")
                        .value_name("FILE")
                        .short("m")
                        .long("message-file"),
                )
                .arg(
                    Arg::with_name("trusted-comment")
                        .help("add a one-line trusted comment")
                        .value_name("TRUSTED_COMMENT")
                        .short("t")
                        .long("trusted-comment"),
                )
                .arg(
                    Arg::with_name("untrusted-comment")
                        .help("add a one-line untrusted comment")
                        .value_name("UNTRUSTED_COMMENT")
                        .short("c")
                        .long("untrusted-comment"),
                )
                .arg(
                    Arg::with_name("hash")
                        .required(false)
                        .short("H")
                        .long("hash")
                        .help("pre-hash in order to sign large files (>1G)"),
                ),
        )
        .get_matches()
}
