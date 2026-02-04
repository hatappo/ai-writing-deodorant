[日本語](README_ja.md)

# ai-writing-deodorant

A CLI tool to remove AI-like formatting from text files.

## Features

- Remove `**` (bold markers) from text
- Optionally remove emoji characters with `--emoji` flag

## Installation

Download the binary from [Releases](https://github.com/hatappo/ai-writing-deodorant/releases) or build from source:

```bash
cargo install --path .
```

## Usage

```
$ deo --help
Remove AI-like formatting from text files

Usage: deo [OPTIONS] <FILE>

Arguments:
  <FILE>  Input file path (use '-' for stdin)

Options:
      --emoji    Remove emoji characters
  -h, --help     Print help
  -V, --version  Print version
```

```bash
# Process a file
deo input.txt

# Read from stdin
echo "**bold** text" | deo -

# Remove emojis as well
deo input.txt --emoji
```

## License

MIT
