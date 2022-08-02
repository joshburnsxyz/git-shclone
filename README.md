# git-shclone

Git plugin that provides a shortcut to clone repos from github using SSH urls.

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
