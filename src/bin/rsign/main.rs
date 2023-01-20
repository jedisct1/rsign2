mod helpers;
mod parse_args;

use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(any(windows, unix))]
use dirs::home_dir;
use minisign::*;

use crate::helpers::*;
use crate::parse_args::*;

#[cfg(not(any(windows, unix)))]
fn home_dir() -> Option<PathBuf> {
    Some(PathBuf::from("."))
}

pub fn cmd_generate<P, Q>(
    force: bool,
    pk_path: P,
    sk_path: Q,
    comment: Option<&str>,
) -> Result<KeyPair>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let pk_path = pk_path.as_ref();
    let sk_path = sk_path.as_ref();
    if pk_path.exists() {
        if !force {
            return Err(PError::new(
                ErrorKind::Io,
                format!(
                    "Key generation aborted:\n
{} already exists\n
If you really want to overwrite the existing key pair, add the -f switch to\n
force this operation.",
                    pk_path.display()
                ),
            ));
        } else {
            std::fs::remove_file(pk_path)?;
        }
    }
    let mut pk_writer = create_file(pk_path, 0o644, force)?;
    let mut sk_writer = create_file(sk_path, 0o600, force)?;
    let kp = KeyPair::generate_and_write_encrypted_keypair(
        &mut pk_writer,
        &mut sk_writer,
        comment,
        None,
    )?;
    pk_writer.flush()?;
    sk_writer.flush()?;
    Ok(kp)
}

pub fn cmd_sign<P, Q, R>(
    pk: Option<PublicKey>,
    sk_path: P,
    signature_path: Q,
    data_path: R,
    trusted_comment: Option<&str>,
    untrusted_comment: Option<&str>,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    if !sk_path.as_ref().exists() {
        return Err(PError::new(
            ErrorKind::Io,
            format!(
                "can't find secret key file at {}, try using -s",
                sk_path.as_ref().display()
            ),
        ));
    }
    let mut signature_box_writer = create_sig_file(&signature_path)?;
    let sk = SecretKey::from_file(sk_path, None)?;
    let trusted_comment = if let Some(trusted_comment) = trusted_comment {
        trusted_comment.to_string()
    } else {
        format!(
            "timestamp:{}\tfile:{}\tprehashed",
            unix_timestamp(),
            data_path.as_ref().display()
        )
    };
    let data_reader = open_data_file(data_path)?;
    let signature_box = sign(
        pk.as_ref(),
        &sk,
        data_reader,
        Some(trusted_comment.as_str()),
        untrusted_comment,
    )?;
    signature_box_writer.write_all(&signature_box.to_bytes())?;
    signature_box_writer.flush()?;
    Ok(())
}

pub fn cmd_verify<P, Q>(
    pk: PublicKey,
    data_path: P,
    signature_path: Q,
    quiet: bool,
    output: bool,
    allow_legacy: bool,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let signature_box = SignatureBox::from_file(&signature_path).map_err(|err| {
        PError::new(
            ErrorKind::Io,
            format!(
                "could not read signature file {}: {}",
                signature_path.as_ref().display(),
                err
            ),
        )
    })?;
    let data_reader = open_data_file(&data_path).map_err(|err| {
        PError::new(
            ErrorKind::Io,
            format!(
                "could not read data file {}: {}",
                data_path.as_ref().display(),
                err
            ),
        )
    })?;
    verify(
        &pk,
        &signature_box,
        data_reader,
        quiet,
        output,
        allow_legacy,
    )
}

fn get_pk_path(explicit_path: Option<&PathBuf>) -> Result<PathBuf> {
    match explicit_path {
        Some(explicit_path) => Ok(explicit_path.clone()),
        None => match std::env::var(SIG_DEFAULT_CONFIG_DIR_ENV_VAR) {
            Ok(env_path) => {
                let mut complete_path = PathBuf::from(env_path);
                complete_path.push(SIG_DEFAULT_PKFILE);
                Ok(complete_path)
            }
            Err(_) => {
                let home_path =
                    home_dir().ok_or_else(|| PError::new(ErrorKind::Io, "can't find home dir"))?;
                let mut complete_path = home_path;
                complete_path.push(SIG_DEFAULT_CONFIG_DIR);
                complete_path.push(SIG_DEFAULT_PKFILE);
                Ok(complete_path)
            }
        },
    }
}

fn get_sk_path(explicit_path: Option<&PathBuf>) -> Result<PathBuf> {
    match explicit_path {
        Some(explicit_path) => Ok(explicit_path.clone()),
        None => match std::env::var(SIG_DEFAULT_CONFIG_DIR_ENV_VAR) {
            Ok(env_path) => {
                let mut complete_path = PathBuf::from(env_path);
                complete_path.push(SIG_DEFAULT_SKFILE);
                Ok(complete_path)
            }
            Err(_) => {
                let home_path =
                    home_dir().ok_or_else(|| PError::new(ErrorKind::Io, "can't find home dir"))?;
                let mut complete_path = home_path;
                complete_path.push(SIG_DEFAULT_CONFIG_DIR);
                complete_path.push(SIG_DEFAULT_SKFILE);
                Ok(complete_path)
            }
        },
    }
}

fn run(args: clap::ArgMatches, help_usage: &clap::builder::StyledStr) -> Result<()> {
    match args.subcommand() {
        Some(("generate", action)) => run_generate(action),
        Some(("sign", action)) => run_sign(action),
        Some(("verify", action)) => run_verify(action),
        _ => {
            println!("{help_usage}\n");
            std::process::exit(1);
        }
    }
}

fn run_generate(matches: &clap::ArgMatches) -> Result<()> {
    let force = matches.get_flag("force");
    let sk_path = get_sk_path(matches.get_one::<PathBuf>("sk_path"))?;
    let pk_path = get_pk_path(matches.get_one::<PathBuf>("pk_path"))?;
    let comment = matches.get_one::<String>("comment").map(|s| s.as_str());
    let KeyPair { pk, .. } = cmd_generate(force, &pk_path, &sk_path, comment)?;
    println!(
        "\nThe secret key was saved as {} - Keep it secret!",
        sk_path.display()
    );
    println!(
        "The public key was saved as {} - That one can be public.\n",
        pk_path.display()
    );
    println!("Files signed using this key pair can be verified with the following command:\n");
    println!("rsign verify <file> -P {}", pk.to_base64());
    Ok(())
}

fn run_sign(matches: &clap::ArgMatches) -> Result<()> {
    let sk_path = get_sk_path(matches.get_one::<PathBuf>("sk_path"))?;
    let pk = if let Some(pk_inline) = matches.get_one::<String>("public_key") {
        Some(PublicKey::from_base64(pk_inline)?)
    } else if let Some(pk_path) = matches.get_one::<PathBuf>("pk_path") {
        Some(PublicKey::from_file(get_pk_path(Some(pk_path))?)?)
    } else {
        None
    };
    let data_path = matches
        .get_one::<PathBuf>("data")
        .expect("'file' is required");
    let signature_path = if let Some(file) = matches.get_one::<PathBuf>("sig_file") {
        file.clone()
    } else {
        let mut sig_path = data_path.clone().into_os_string();
        sig_path.push(SIG_SUFFIX);
        sig_path.into()
    };
    let trusted_comment = matches
        .get_one::<String>("trusted-comment")
        .map(|s| s.as_str());
    let untrusted_comment = matches
        .get_one::<String>("untrusted-comment")
        .map(|s| s.as_str());
    cmd_sign(
        pk,
        sk_path,
        signature_path,
        data_path,
        trusted_comment,
        untrusted_comment,
    )
}

fn run_verify(matches: &clap::ArgMatches) -> Result<()> {
    let pk = if let Some(pk_inline) = matches.get_one::<String>("public_key") {
        PublicKey::from_base64(pk_inline)?
    } else {
        PublicKey::from_file(get_pk_path(matches.get_one::<PathBuf>("pk_path"))?)?
    };
    let data_path = matches
        .get_one::<PathBuf>("file")
        .expect("'file' is required");
    let signature_path = if let Some(path) = matches.get_one::<PathBuf>("sig_file") {
        path.clone()
    } else {
        let mut sig_path = data_path.clone().into_os_string();
        sig_path.push(SIG_SUFFIX);
        sig_path.into()
    };
    let quiet = matches.get_flag("quiet");
    let output = matches.get_flag("output");
    let allow_legacy = matches.get_flag("allow-legacy");
    cmd_verify(pk, data_path, signature_path, quiet, output, allow_legacy)
}

fn main() {
    let (args, help_usage) = parse_args();
    run(args, &help_usage).map_err(|e| e.exit()).unwrap();
    std::process::exit(0);
}
