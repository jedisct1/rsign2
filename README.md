# Blockchain Commons torgap-sig-cli-rust

`torgap-sig-cli-rust` is a fork of [rsign2](https://github.com/jedisct1/rsign2), a Rust implementation of [Minisign](https://jedisct1.github.io/minisign/), with support for
Tor onion v3 hidden service keys.

## Why is torgap-sig-cli-rust relevant?

Torgap-sig-cli builds on the fact that Minisig appears to be a universal donor for ed25519 signatures, usable anywhere, by a variety of different programs. This includes Tor, which uses ed25519 keys for its hidden services. Using Tor with Minisig allows for the issuance of digital signatures that enable the Tor owner to prove control, without requiring any external authorities.

This has allowed for the experimental creation of `did:onion` DIDs, whoch can be used to enable verifiable decentralized digital identities and associated verifiable claims (VCs).

### Why Tor?

Tor is private and decentralized by design: no centralized registries or identity providers are required to access or offer onion services.

When used with DIDs, Tor can provide two strong advantages not available in most other DID methods: censorship-resistance and non-correlation.

The *censorship-resistance* means that it's difficult to block the lookup or usage of Tor-enabled DID.

The *non-correlation* means that: the controller of a DID can be pseudonymous; the signer of a claim can be pseudonymous; and, perhaps most importantly, an end-user investigating a claim or a DID is not correlatable. In other words, the non-correlation of Tor is bidirectional.

A use case would be for a user to look up the signature of a software package without revealing to the signer that they are interested in the package.

### What is Torgap?

Torgap is an architectural feature created by Blockchain Commons that allows for apps and microservices to be separated by a Tor connection, creating advantages of anonymity and privacy. It can also created additional advantages of stability and availability in distributed quorum computing situations.

The `did:onion` DID experiment is an example of such a Torgap, creating DID and VC microservices across the gap.

See the [torgap repo](https://github.com/BlockchainCommons/torgap) for more info.

## Additional Information

* Still puzzled? Have a look at our [plain people's language](Torgap-made-easy.md) interview-like explanation of why our work is relevant, with lots of analogies.
* [Manual](docs/MANUAL.md) on how to use `torgap-sig-cli-rust`. This is an operational guide with example instructions.
* [Signal protocol](https://en.wikipedia.org/wiki/Signal_Protocol#Properties) properties explained.
* [Opentimestamps](opentimestamps.org) aims to be a standard format for blockchain timestamping, includingbitcoin. 

## Status - Late Alpha

`torgap-sig-cli-rust` is currently under active development and in the late alpha testing phase. It should not be used for production tasks until it has had further testing and auditing.

### Ongoing Progress

Details of this experiment are still being discussed with people in the W3C DID community. We are for a lean and mean approach.

It is unlikely we’ll support `DIDcomm` as is, more likely choosing something closer to the protocol used by Signal. We might favor the inclusion of some fundamental elements like the [perfect forward secrecy](https://en.wikipedia.org/wiki/Forward_secrecy) that Signal does.

 The torgap-sig-cli-rust experiment is aimed at the trustless & censorship resistant side of the digital identity spectrum. It is envisioned as a DID that is anti-correlation and  works with `Lightning Network`. It is not intended for personal or legal identity.

## Installation Instructions

Install with:

`cargo build`

The resulting executable is `target/debug/rsign`

You can alternatively run and build automatically with a single command: `cargo run`.

For more information: `cargo run help`

Note that `cargo run help` is equivalent to `target/debug/rsign help` and when using `cargo run`, `cargo build` can be omitted, as it's already automatically executed.

## Usage Instructions

`torgap-sig-cli-rust` allows a file to be signed and its signature to be verified with an onion v3 address.

Any file can be used:

```sh
echo "Mr. Watson--come here--I want to see you." > myfile.txt
```

Sign the file with the `rsign` command:

```sh
rsign sign myfile.txt
Password: 
**********
Deriving a key from the password and decrypting the secret key... done
```

A detached signature is generated:

```sh
untrusted comment: signature from rsign secret key
RWTFh+S84tByDXDoISKOL7c1WOCON2DbmQjFuRbJH1XTcr/79yX2gHw8L3YvMoxQU+RjbWrQz5UjVCrKGWu/AlWx1AH0xskHSQc=
trusted comment: timestamp:1605303858 file:myfile.txt addr:fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid.onion
pp15vK04ItsRBNooW9K/BAxyCVZyrp4vxiJ7draB2bKyzm3w3ChWwnLFAuzKPHqG0ZL6Am39Xde9aFQ+rebzBA==
```

`rsign` can then be used to verify this signature with an onion address:

```sh
rsign verify myfile.txt --onion-address fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid.onion
Signature and comment signature verified
Trusted comment: timestamp:1605303858 file:myfile.txt addr:fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid.onion
```

Minisign keys can be generated using `rsign generate` and exported to Tor hidden service keys via `rsign export-to-onion-keys`, i.e. `hs_ed25519_secret_key`, `hs_ed25519_public_key`, `hostname`. 
The reason you import signing-capable keys in `Tor` hidden service, is because you can’t go in the other direction and use exported Tor private keys, outside of Tor. Tor keys simply cannot be converted back to minisign key format. In fact Tor private keys can't used anywhere else, expect within Tor.


For more information on setting this up, see the [Manual](docs/MANUAL.md).

### Demo

You can try it out yourself: `http://fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid.onion/`

## Origin, Authors, Copyright & Licenses

Unless otherwise noted (either in this [/README.md](./README.md) or in the file's header comments) the contents of this repository are Copyright © 2020 by Blockchain Commons, LLC, and are [licensed](./LICENSE) under the [spdx:BSD-2-Clause Plus Patent License](https://spdx.org/licenses/BSD-2-Clause-Patent.html).

In most cases, the authors, copyright, and license for each file reside in header comments in the source code. When it does not, we have attempted to attribute it accurately in the table below.

This table below also establishes provenance (repository of origin, permalink, and commit id) for files included from repositories that are outside of this repo. Contributors to these files are listed in the commit history for each repository, first with changes found in the commit history of this repo, then in changes in the commit history of their repo of their origin.

[No external files included]

### Dependencies

To build `torgap-sig-cli-rust` you'll need to use the following tools:

- [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [torgap-sig crate](https://github.com/BlockchainCommons/torgap-sig)

### Derived from…

`torgap-sig-cli-rust` is either derived from or was inspired by:

- [rsign2](https://github.com/jedisct1/rsign2) — A command-line tool to sign files and verify signatures in pure Rust, by [Frank Denis](https://github.com/jedisct1).

### Used with…

These are other projects that work with or leverage `torgap-sig-cli-rust`:

- N/A

## Financial Support

`torgap-sig-cli-rust` is a project of [Blockchain Commons](https://www.blockchaincommons.com/). We are proudly a "not-for-profit" social benefit corporation committed to open source & open development. Our work is funded entirely by donations and collaborative partnerships with people like you. Every contribution will be spent on building open tools, technologies, and techniques that sustain and advance blockchain and internet security infrastructure and promote an open web.

To financially support further development of torgap-sig-cli-rust and other projects, please consider becoming a Patron of Blockchain Commons through ongoing monthly patronage as a [GitHub Sponsor](https://github.com/sponsors/BlockchainCommons). You can also support Blockchain Commons with bitcoins at our [BTCPay Server](https://btcpay.blockchaincommons.com/).

## Contributing

We encourage public contributions through issues and pull requests! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for details on our development process. All contributions to this repository require a GPG signed [Contributor License Agreement](./CLA.md).

### Discussions

The best place to talk about Blockchain Commons and its projects is in our GitHub Discussions areas.

[**Wallet Standard Discussions**](https://github.com/BlockchainCommons/AirgappedSigning/discussions). For standards and open-source developers who want to talk about wallet standards, please use the Discussions area of the [Airgapped Signing repo](https://github.com/BlockchainCommons/AirgappedSigning). This is where you can talk about projects like our [LetheKit](https://github.com/BlockchainCommons/bc-lethekit) and command line tools such as [seedtool](https://github.com/BlockchainCommons/bc-seedtool-cli), both of which are intended to testbed wallet technologies, plus the libraries that we've built to support your own deployment of wallet technology such as [bc-bip39](https://github.com/BlockchainCommons/bc-bip39), [bc-slip39](https://github.com/BlockchainCommons/bc-slip39), [bc-shamir](https://github.com/BlockchainCommons/bc-shamir), [Shamir Secret Key Recovery](https://github.com/BlockchainCommons/bc-sskr), [bc-ur](https://github.com/BlockchainCommons/bc-ur), and the [bc-crypto-base](https://github.com/BlockchainCommons/bc-crypto-base). If it's a wallet-focused technology or a more general discussion of wallet standards,discuss it here.

[**Blockchain Commons Discussions**](https://github.com/BlockchainCommons/Community/discussions). For developers, interns, and patrons of Blockchain Commons, please use the discussions area of the [Community repo](https://github.com/BlockchainCommons/Community) to talk about general Blockchain Commons issues, the intern program, or topics other than the [Gordian System](https://github.com/BlockchainCommons/Gordian/discussions) or the [wallet standards](https://github.com/BlockchainCommons/AirgappedSigning/discussions), each of which have their own discussion areas.

### Other Questions & Problems

As an open-source, open-development community, Blockchain Commons does not have the resources to provide direct support of our projects. Please consider the discussions area as a locale where you might get answers to questions. Alternatively, please use this repository's [issues](./issues) feature. Unfortunately, we can not make any promises on response time.

If your company requires support to use our projects, please feel free to contact us directly about options. We may be able to offer you a contract for support from one of our contributors, or we might be able to point you to another entity who can offer the contractual support that you need.

### Credits

The following people directly contributed to this repository. You can add your name here by getting involved. The first step is learning how to contribute from our [CONTRIBUTING.md](./CONTRIBUTING.md) documentation.

| Name              | Role                | Github                                            | Email                                 | GPG Fingerprint                                    |
| ----------------- | ------------------- | ------------------------------------------------- | ------------------------------------- | -------------------------------------------------- |
| Christopher Allen | Principal Architect | [@ChristopherA](https://github.com/ChristopherA) | \<ChristopherA@LifeWithAlacrity.com\> | FDFE 14A5 4ECB 30FC 5D22  74EF F8D3 6C91 3574 05ED |
| Gorazd Kovacic | Maintainer | [@gorazdko](https://github.com/gorazdko) | \<gorazdko@gmail.com\> | 41F0 EA16 99A7 4C1E 2FA4 1B53 8CF9 6BC3 FF9D BBCE |

## Responsible Disclosure

We want to keep all of our software safe for everyone. If you have discovered a security vulnerability, we appreciate your help in disclosing it to us in a responsible manner. We are unfortunately not able to offer bug bounties at this time.

We do ask that you offer us good faith and use best efforts not to leak information or harm any user, their data, or our developer community. Please give us a reasonable amount of time to fix the issue before you publish it. Do not defraud our users or us in the process of discovery. We promise not to bring legal action against researchers who point out a problem provided they do their best to follow the these guidelines.

### Reporting a Vulnerability

Please report suspected security vulnerabilities in private via email to ChristopherA@BlockchainCommons.com (do not use this email for support). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

The following keys may be used to communicate sensitive information to developers:

| Name              | Fingerprint                                        |
| ----------------- | -------------------------------------------------- |
| Christopher Allen | FDFE 14A5 4ECB 30FC 5D22  74EF F8D3 6C91 3574 05ED |

You can import a key by running the following command with that individual’s fingerprint: `gpg --recv-keys "<fingerprint>"` Ensure that you put quotes around fingerprints that contain spaces.
