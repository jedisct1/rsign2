extern crate clap;
extern crate dirs;
extern crate minisign;

mod helpers;
mod parse_args;

use crate::helpers::*;
use crate::parse_args::*;
use minisign::*;
use std::io::Write;
use std::path::{Path, PathBuf};

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
            Err(PError::new(
                ErrorKind::Io,
                format!(
                    "Key generation aborted:\n
{} already exists\n
If you really want to overwrite the existing key pair, add the -f switch to\n
force this operation.",
                    pk_path.display()
                ),
            ))?;
        } else {
            std::fs::remove_file(&pk_path)?;
        }
    }
    let pk_writer = create_file(&pk_path, 0o644)?;
    let sk_writer = create_file(&sk_path, 0o600)?;
    KeyPair::generate_and_write_encrypted_keypair(pk_writer, sk_writer, comment, None)
}

pub fn cmd_sign<P, Q, R>(
    pk: Option<PublicKey>,
    sk_path: P,
    signature_path: Q,
    data_path: R,
    prehashed: bool,
    trusted_comment: Option<&str>,
    untrusted_comment: Option<&str>,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    if !sk_path.as_ref().exists() {
        Err(PError::new(
            ErrorKind::Io,
            format!(
                "can't find secret key file at {}, try using -s",
                sk_path.as_ref().display()
            ),
        ))?;
    }
    let mut signature_box_writer = create_sig_file(&signature_path)?;
    let sk = SecretKey::from_file(sk_path, None)?;
    let trusted_comment = if let Some(trusted_comment) = trusted_comment {
        trusted_comment.to_string()
    } else {
        format!(
            "timestamp:{}\tfile:{}",
            unix_timestamp(),
            data_path.as_ref().display()
        )
    };
    let (data_reader, should_be_prehashed) = open_data_file(data_path)?;
    let signature_box = sign(
        pk.as_ref(),
        &sk,
        data_reader,
        prehashed | should_be_prehashed,
        Some(trusted_comment.as_str()),
        untrusted_comment,
    )?;
    signature_box_writer.write_all(&signature_box.to_vec())?;
    Ok(())
}

pub fn cmd_verify<P, Q>(
    pk: PublicKey,
    data_path: P,
    signature_path: Q,
    quiet: bool,
    output: bool,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let signature_box = SignatureBox::from_file(signature_path)?;
    let (data_reader, _should_be_prehashed) = open_data_file(data_path)?;
    verify(&pk, &signature_box, data_reader, quiet, output)
}

fn sk_path_or_default(sk_path_str: Option<&str>, force: bool) -> Result<PathBuf> {
    let sk_path = match sk_path_str {
        Some(path) => {
            let complete_path = PathBuf::from(path);
            let mut dir = complete_path.clone();
            dir.pop();
            create_dir(&dir)?;
            complete_path
        }
        None => {
            let env_path = std::env::var(SIG_DEFAULT_CONFIG_DIR_ENV_VAR);
            match env_path {
                Ok(env_path) => {
                    let mut complete_path = PathBuf::from(env_path);
                    if !complete_path.exists() {
                        Err(PError::new(
                            ErrorKind::Io,
                            format!(
                                "folder {} referenced by {} doesn't exists, you'll have to create yourself",
                                complete_path.display(), SIG_DEFAULT_CONFIG_DIR_ENV_VAR
                            ),
                        ))?;
                    }
                    complete_path.push(SIG_DEFAULT_SKFILE);
                    complete_path
                }
                Err(_) => {
                    let home_path = dirs::home_dir()
                        .ok_or_else(|| PError::new(ErrorKind::Io, "can't find home dir"));
                    let mut complete_path = home_path.unwrap();
                    complete_path.push(SIG_DEFAULT_CONFIG_DIR);
                    if !complete_path.exists() {
                        create_dir(&complete_path)?;
                    }
                    complete_path.push(SIG_DEFAULT_SKFILE);
                    complete_path
                }
            }
        }
    };
    if sk_path.exists() {
        if !force {
            Err(PError::new(
                ErrorKind::Io,
                format!(
                    "Key generation aborted:
{} already exists

If you really want to overwrite the existing key pair, add the -f switch to
force this operation.",
                    sk_path.display()
                ),
            ))?;
        } else {
            std::fs::remove_file(&sk_path)?;
        }
    }
    Ok(sk_path)
}

fn run(args: clap::ArgMatches) -> Result<()> {
    if let Some(generate_action) = args.subcommand_matches("generate") {
        let force = generate_action.is_present("force");
        let pk_path = match generate_action.value_of("pk_path") {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from(SIG_DEFAULT_PKFILE),
        };
        let sk_path_str = generate_action.value_of("sk_path");
        let sk_path = sk_path_or_default(sk_path_str, force)?;
        let comment = generate_action.value_of("comment");
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
    } else if let Some(sign_action) = args.subcommand_matches("sign") {
        let sk_path = match sign_action.value_of("sk_path") {
            Some(path) => PathBuf::from(path),
            None => {
                let home_path = dirs::home_dir()
                    .ok_or_else(|| PError::new(ErrorKind::Io, "can't find home dir"));
                let mut complete_path = home_path.unwrap();
                complete_path.push(SIG_DEFAULT_CONFIG_DIR);
                complete_path.push(SIG_DEFAULT_SKFILE);
                complete_path
            }
        };
        let mut pk = None;
        if sign_action.is_present("pk_path") {
            if let Some(filename) = sign_action.value_of("pk_path") {
                pk = Some(PublicKey::from_file(filename)?);
            }
        } else if sign_action.is_present("public_key") {
            if let Some(string) = sign_action.value_of("public_key") {
                pk = Some(PublicKey::from_base64(string)?);
            }
        };
        let prehashed = sign_action.is_present("hash");
        let data_path = PathBuf::from(sign_action.value_of("data").unwrap()); // safe to unwrap
        let signature_path = if let Some(file) = sign_action.value_of("sig_file") {
            PathBuf::from(file)
        } else {
            PathBuf::from(format!("{}{}", data_path.display(), SIG_SUFFIX))
        };
        let trusted_comment = sign_action.value_of("trusted-comment");
        let untrusted_comment = sign_action.value_of("untrusted-comment");
        cmd_sign(
            pk,
            &sk_path,
            &signature_path,
            &data_path,
            prehashed,
            trusted_comment,
            untrusted_comment,
        )
    } else if let Some(verify_action) = args.subcommand_matches("verify") {
        let pk_path_str = verify_action
            .value_of("pk_path")
            .or_else(|| verify_action.value_of("public_key"));
        let pk = match pk_path_str {
            Some(path_or_string) => {
                if verify_action.is_present("pk_path") {
                    PublicKey::from_file(path_or_string)?
                } else {
                    PublicKey::from_base64(path_or_string)?
                }
            }
            None => PublicKey::from_file(SIG_DEFAULT_PKFILE)?,
        };
        let data_path = verify_action.value_of("file").unwrap();
        let signature_path = if let Some(path) = verify_action.value_of("sig_file") {
            PathBuf::from(path)
        } else {
            PathBuf::from(format!("{}{}", data_path, SIG_SUFFIX))
        };
        let quiet = verify_action.is_present("quiet");
        let output = verify_action.is_present("output");
        cmd_verify(pk, &data_path, &signature_path, quiet, output)
    } else {
        println!("{}\n", args.usage());
        std::process::exit(1);
    }
}

fn main() {
    let args = parse_args();
    run(args).map_err(|e| e.exit()).unwrap();
    std::process::exit(0);
}
