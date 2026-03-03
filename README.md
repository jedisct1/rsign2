# rsign2

A Rust implementation of [Minisign](https://jedisct1.github.io/minisign/).

All signatures produced by `rsign` can be verified with `minisign` and `minizign` including trusted comments, and vice versa.

In Rust, signatures can also be verified with the [minisign-verify](https://docs.rs/minisign-verify) crate.

`rsign2` is a maintained fork of [`rsign`](https://docs.rs/crate/rsign/), originally written by Daniel Rangel.

Main differences with rsign:

- `rsign2` is written in pure Rust.
- `rsign2` has way less dependencies.
- `rsign2` includes bug fixes and improvements.
- `rsign2` supports WebAssembly.

## API documentation

`rsign2` is only a command-line tool. It relies on the Minisign crate for all cryptographic operations, which can also be embedded in any application:

[API documentation on docs.rs](https://docs.rs/minisign)

## Usage

### Generating a key pair

```sh
rsign generate
```

Generates a new key pair. The public key is stored in `rsign.pub` by default. The secret key will be written at `~/.rsign/rsign.key`. You can change the default paths with `-p` and `-s` respectively.

Use `-W` to create a key without a password, or `--unencrypted` to store the secret key without any encryption at all (useful for CI pipelines).

Use `-f` to overwrite an existing key pair.

### Signing

```sh
rsign sign myfile.txt
```

Sign `myfile.txt` with your secret key. Files are prehashed automatically, so there is no size limit.

You can add a signed trusted comment with:

```sh
rsign sign myfile.txt -t "my trusted comment"
```

### Verifying

```sh
rsign verify myfile.txt -p rsign.pub
```

Or use a public key string directly:

```sh
rsign verify myfile.txt -P <public key string>
```

If the signature file has a custom name (other than `myfile.txt.minisig`):

```sh
rsign verify myfile.txt -P <public key string> -x mysignature.file
```

Use `-q` for quiet mode (no output on success), or `-o` to output the file content after successful verification.

### Full help

```text
Usage: rsign [COMMAND]

Commands:
  generate  Generate public and private keys
  verify    Verify a signed file with a given public key
  sign      Sign a file with a given private key
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
