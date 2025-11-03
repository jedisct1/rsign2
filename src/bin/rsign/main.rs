#[macro_use]
extern crate clap;

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
    passwordless: bool,
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
    let mut pk_writer = create_file(pk_path, 0o644)?;
    let mut sk_writer = create_file(sk_path, 0o600)?;
    let kp = KeyPair::generate_and_write_encrypted_keypair(
        &mut pk_writer,
        &mut sk_writer,
        comment,
        if passwordless {
            Some(Default::default())
        } else {
            None
        },
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
    passwordless: bool,
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
    let sk = SecretKey::from_file(
        sk_path,
        if passwordless {
            Some(Default::default())
        } else {
            None
        },
    )?;
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

fn create_sk_path_or_default(sk_path_str: Option<&str>, force: bool) -> Result<PathBuf> {
    let sk_path = match sk_path_str {
        Some(path) => {
            let complete_path = PathBuf::from(path);
            let mut dir = complete_path.clone();
            dir.pop();
            create_dir(&dir)?;
            complete_path
        }
        None => match std::env::var(SIG_DEFAULT_CONFIG_DIR_ENV_VAR) {
            Ok(env_path) => {
                let mut complete_path = PathBuf::from(env_path);
                if !complete_path.exists() {
                    return Err(PError::new(
                        ErrorKind::Io,
                        format!(
                            "folder {} referenced by {} doesn't exist, you'll have to create \
                             it yourself",
                            complete_path.display(),
                            SIG_DEFAULT_CONFIG_DIR_ENV_VAR
                        ),
                    ));
                }
                complete_path.push(SIG_DEFAULT_SKFILE);
                complete_path
            }
            Err(_) => {
                let home_path =
                    home_dir().ok_or_else(|| PError::new(ErrorKind::Io, "can't find home dir"))?;
                let mut complete_path = home_path;
                complete_path.push(SIG_DEFAULT_CONFIG_DIR);
                if !complete_path.exists() {
                    create_dir(&complete_path)?;
                }
                complete_path.push(SIG_DEFAULT_SKFILE);
                complete_path
            }
        },
    };
    if sk_path.exists() {
        if !force {
            return Err(PError::new(
                ErrorKind::Io,
                format!(
                    "Key generation aborted:
{} already exists

If you really want to overwrite the existing key pair, add the -f switch to
force this operation.",
                    sk_path.display()
                ),
            ));
        } else {
            std::fs::remove_file(&sk_path)?;
        }
    }
    Ok(sk_path)
}

fn get_pk_path(explicit_path: Option<&str>) -> Result<PathBuf> {
    Ok(PathBuf::from(explicit_path.unwrap_or(SIG_DEFAULT_PKFILE)))
}

fn get_sk_path(explicit_path: Option<&str>) -> Result<PathBuf> {
    let default_file_name = SIG_DEFAULT_SKFILE;
    match explicit_path {
        Some(explicit_path) => Ok(PathBuf::from(explicit_path)),
        None => match std::env::var(SIG_DEFAULT_CONFIG_DIR_ENV_VAR) {
            Ok(env_path) => {
                let mut complete_path = PathBuf::from(env_path);
                complete_path.push(default_file_name);
                Ok(complete_path)
            }
            Err(_) => {
                let home_path =
                    home_dir().ok_or_else(|| PError::new(ErrorKind::Io, "can't find home dir"))?;
                let mut complete_path = home_path;
                complete_path.push(SIG_DEFAULT_CONFIG_DIR);
                complete_path.push(default_file_name);
                Ok(complete_path)
            }
        },
    }
}

fn run(args: clap::ArgMatches, help_usage: &str) -> Result<()> {
    if let Some(generate_action) = args.subcommand_matches("generate") {
        let force = generate_action.get_flag("force");
        let pk_path = get_pk_path(
            generate_action
                .get_one::<String>("pk_path")
                .map(|s| s.as_str()),
        )?;
        let sk_path_str = generate_action.get_one::<String>("sk_path");
        let sk_path = create_sk_path_or_default(sk_path_str.map(|s| s.as_str()), force)?;
        let comment = generate_action.get_one::<String>("comment");
        let passwordless = generate_action.get_flag("passwordless");
        let KeyPair { pk, .. } = cmd_generate(
            force,
            &pk_path,
            &sk_path,
            comment.map(|s| s.as_str()),
            passwordless,
        )?;
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
        let sk_path = get_sk_path(sign_action.get_one::<String>("sk_path").map(|s| s.as_str()))?;
        let pk = if let Some(pk_inline) = sign_action.get_one::<String>("public_key") {
            Some(PublicKey::from_base64(pk_inline)?)
        } else if let Some(pk_path) = sign_action.get_one::<String>("pk_path") {
            Some(PublicKey::from_file(get_pk_path(Some(pk_path))?)?)
        } else {
            None
        };
        let data_path = PathBuf::from(sign_action.get_one::<String>("data").unwrap()); // safe to unwrap
        let signature_path = if let Some(file) = sign_action.get_one::<String>("sig_file") {
            PathBuf::from(file)
        } else {
            PathBuf::from(format!("{}{}", data_path.display(), SIG_SUFFIX))
        };
        let trusted_comment = sign_action.get_one::<String>("trusted-comment");
        let trusted_comment = trusted_comment.map(|s| s.as_str());
        let untrusted_comment = sign_action.get_one::<String>("untrusted-comment");
        let untrusted_comment = untrusted_comment.map(|s| s.as_str());
        let passwordless = sign_action.get_flag("passwordless");
        cmd_sign(
            pk,
            sk_path,
            signature_path,
            &data_path,
            trusted_comment,
            untrusted_comment,
            passwordless,
        )
    } else if let Some(verify_action) = args.subcommand_matches("verify") {
        let pk = if let Some(pk_inline) = verify_action.get_one::<String>("public_key") {
            PublicKey::from_base64(pk_inline)?
        } else {
            PublicKey::from_file(get_pk_path(
                verify_action
                    .get_one::<String>("pk_path")
                    .map(|s| s.as_str()),
            )?)?
        };
        let data_path = verify_action.get_one::<String>("file").unwrap();
        let signature_path = if let Some(path) = verify_action.get_one::<String>("sig_file") {
            PathBuf::from(path)
        } else {
            PathBuf::from(format!("{data_path}{SIG_SUFFIX}"))
        };
        let quiet = verify_action.get_flag("quiet");
        let output = verify_action.get_flag("output");
        let allow_legacy = verify_action.get_flag("allow-legacy");
        cmd_verify(pk, data_path, signature_path, quiet, output, allow_legacy)
    } else {
        println!("{help_usage}\n");
        std::process::exit(1);
    }
}

fn main() {
    let (args, help_usage) = parse_args();
    run(args, &help_usage).map_err(|e| e.exit()).unwrap();
    std::process::exit(0);
}
