# git-shclone

[![Rust](https://github.com/joshburnsxyz/git-shclone/actions/workflows/ci.yml/badge.svg)](https://github.com/joshburnsxyz/git-shclone/actions/workflows/ci.yml)

Git plugin that provides a shortcut to clone repos from github using SSH urls. In the future I plan on refactoring
the interface to make things not github specific.

__BEFORE__:

```
$ git clone git@github.com:joshburnsxyz/git-shclone
```

__AFTER__:

```
$ git shclone joshburnsxyz/git-shclone
```

## Usage

```
USAGE:
    git-shclone [ARGS]

ARGS:
    <REPO>
    <DEST>    [default: ./]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Installation

> Linux package managers, Homebrew, etc. coming soon 

### Build From Source

> Requires working Rust toolchain and Cargo installation

1. Clone repo

```
git clone git@github.com:joshburnsxyz/git-shclone
cd git-shclone
```

2. Build the release binary with `cargo`
```
cargo build --release
```

3. Move the binary into your `$PATH`
```
mv target/release/git-shclone /usr/bin/git-shclone
```

4. Test the `--help` message works.
```
git shclone --help
```

## Todo

- [ ] If supplied with a HTTP(S) URL, transform it into the SSH version
- [ ] Refactor to make things not Github specific
- [ ] Implement toolchain / scripts to build for all 3 major platforms to provide release binaries
- [ ] Packaging for (major desktop) Linux distros, Homebrew, etc.
- [ ] Use a `Makefile` to automate build steps and installation
- [ ] Use `.gitconfig` files to assume default host choice, etc.
