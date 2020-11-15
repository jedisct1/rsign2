extern crate clap;
#[cfg(any(windows, unix))]
extern crate dirs;
extern crate hex;
extern crate minisign;

mod helpers;
mod parse_args;

use crate::helpers::*;
use crate::parse_args::*;
use hex::decode;
use minisign::*;
use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(any(windows, unix))]
use dirs::home_dir;

#[cfg(not(any(windows, unix)))]
fn home_dir() -> Option<PathBuf> {
    Some(PathBuf::from("."))
}

pub fn cmd_generate<P, Q>(
    force: bool,
    pk_path: P,
    sk_path: Q,
    comment: Option<&str>,
    seed: Option<Vec<u8>>,
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
            std::fs::remove_file(&pk_path)?;
        }
    }
    let mut pk_writer = create_file(&pk_path, 0o644)?;
    let mut sk_writer = create_file(&sk_path, 0o600)?;
    let kp = KeyPair::generate_and_write_encrypted_keypair(
        &mut pk_writer,
        &mut sk_writer,
        comment,
        None,
        seed,
    )?;

    pk_writer.flush()?;
    sk_writer.flush()?;
    Ok(kp)
}

pub fn cmd_convert_to_onion_keys<P, Q, Z, W>(
    force: bool,
    tor_sk_path: P,
    tor_pk_path: Q,
    tor_onion_path: Z,
    sk_path: W,
) -> Result<bool>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    Z: AsRef<Path>,
    W: AsRef<Path>,
{
    let tor_sk_path = tor_sk_path.as_ref();
    let tor_pk_path = tor_pk_path.as_ref();
    let tor_onion_path = tor_onion_path.as_ref();

    if tor_sk_path.exists() || tor_pk_path.exists() || tor_onion_path.exists() {
        if !force {
            return Err(PError::new(
                ErrorKind::Io,
                "Exporting keys aborted:\n
If you want to overwrite the existing onion keys and hostname file, add the -f switch to\n
force this operation."
                    .to_string(),
            ));
        } else {
            if tor_sk_path.exists() {
                std::fs::remove_file(&tor_sk_path)?;
            }
            if tor_pk_path.exists() {
                std::fs::remove_file(&tor_pk_path)?;
            }
            if tor_onion_path.exists() {
                std::fs::remove_file(&tor_onion_path)?;
            }
        }
    }

    let mut sk_writer = create_file(&tor_sk_path, 0o666)?;
    let mut pk_writer = create_file(&tor_pk_path, 0o666)?;
    let mut hostname_writer = create_file(&tor_onion_path, 0o666)?;

    let sk = SecretKey::from_file(sk_path, None)?;
    convert_secret_to_onion_keys(&mut sk_writer, &mut pk_writer, &mut hostname_writer, sk)?;

    pk_writer.flush()?;
    sk_writer.flush()?;
    hostname_writer.flush()?;
    Ok(true)
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
    signature_box_writer.write_all(&signature_box.to_bytes())?;
    signature_box_writer.flush()?;
    Ok(())
}

pub fn cmd_verify<P, Q>(
    pk: Option<PublicKey>,
    onion_address: Option<&str>,
    data_path: P,
    signature_path: Q,
    quiet: bool,
    output: bool,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let pubkey: PublicKey;
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

    if pk == None {
        pubkey = PublicKey::from_onion_address(
            onion_address.unwrap(),
            signature_box.get_sig_alg(),
            signature_box.get_keynum(),
        )?;
    } else {
        pubkey = pk.unwrap();
    }

    let (data_reader, _should_be_prehashed) = open_data_file(&data_path).map_err(|err| {
        PError::new(
            ErrorKind::Io,
            format!(
                "could not read data file {}: {}",
                data_path.as_ref().display(),
                err
            ),
        )
    })?;
    verify(&pubkey, &signature_box, data_reader, quiet, output)
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
                                "folder {} referenced by {} doesn't exists, you'll have to create yourself",
                                complete_path.display(), SIG_DEFAULT_CONFIG_DIR_ENV_VAR
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

fn run(args: clap::ArgMatches) -> Result<()> {
    if let Some(generate_action) = args.subcommand_matches("generate") {
        let force = generate_action.is_present("force");
        let pk_path = get_pk_path(generate_action.value_of("pk_path"))?;
        let sk_path_str = generate_action.value_of("sk_path");
        let sk_path = create_sk_path_or_default(sk_path_str, force)?;
        let comment = generate_action.value_of("comment");
        let seed = {
            let seed_opt: Option<Vec<u8>>;
            if let Some(seed) = generate_action.value_of("seed") {
                let seed_vec = decode(seed).unwrap();
                println!("seed: {:?}", seed_vec);
                seed_opt = Some(seed_vec);
            } else {
                seed_opt = None;
            }
            seed_opt
        };

        let KeyPair { pk, .. } = cmd_generate(force, &pk_path, &sk_path, comment, seed)?;
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
    } else if let Some(onion) = args.subcommand_matches("export-to-onion-keys") {
        let force = onion.is_present("force");
        let sk_path = get_sk_path(onion.value_of("sk_path"))?;

        let mut tor_sk_path = sk_path.clone();
        tor_sk_path.pop();
        tor_sk_path.push(SIG_DEFAULT_TORSKFILE);

        let mut tor_onion_path = sk_path.clone();
        tor_onion_path.pop();
        tor_onion_path.push(SIG_DEFAULT_TORONIONFILE);

        let mut tor_pk_path = sk_path.clone();
        tor_pk_path.pop();
        tor_pk_path.push(SIG_DEFAULT_TORPKFILE);

        cmd_convert_to_onion_keys(force, &tor_sk_path, &tor_pk_path, &tor_onion_path, &sk_path)?;

        Ok(())
    } else if let Some(sign_action) = args.subcommand_matches("sign") {
        let sk_path = get_sk_path(sign_action.value_of("sk_path"))?;
        let pk = if let Some(pk_inline) = sign_action.value_of("public_key") {
            Some(PublicKey::from_base64(pk_inline)?)
        } else if let Some(pk_path) = sign_action.value_of("pk_path") {
            Some(PublicKey::from_file(get_pk_path(Some(pk_path))?)?)
        } else {
            None
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
        let pk = if let Some(pk_inline) = verify_action.value_of("public_key") {
            Some(PublicKey::from_base64(pk_inline)?)
        } else if let Some(path) = verify_action.value_of("pk_path") {
            Some(PublicKey::from_file(get_pk_path(Some(path))?)?)
        } else {
            let pk: Option<PublicKey>;
            if !verify_action.is_present("onion_address") {
                pk = Some(PublicKey::from_file(get_pk_path(
                    verify_action.value_of("pk_path"),
                )?)?);
                pk
            } else {
                None
            }
        };

        let onion_address = verify_action.value_of("onion_address");

        let data_path = verify_action.value_of("file").unwrap();
        let signature_path = if let Some(path) = verify_action.value_of("sig_file") {
            PathBuf::from(path)
        } else {
            PathBuf::from(format!("{}{}", data_path, SIG_SUFFIX))
        };
        let quiet = verify_action.is_present("quiet");
        let output = verify_action.is_present("output");
        cmd_verify(
            pk,
            onion_address,
            &data_path,
            &signature_path,
            quiet,
            output,
        )
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
