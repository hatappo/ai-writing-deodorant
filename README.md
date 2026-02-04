[日本語](README_ja.md)

# ai-deodorant

A CLI tool to remove AI-like formatting from text files.

## Features

- Remove `**` (bold markers) from text
- Optionally remove emoji characters with `--emoji` flag

## Installation

Download the binary from [Releases](https://github.com/fumi/ai-writing-deodorant/releases) or build from source:

```bash
cargo install --path .
```

## Usage

```bash
# Process a file
ai-deodorant input.txt

# Read from stdin
echo "**bold** text" | ai-deodorant -

# Remove emojis as well
ai-deodorant input.txt --emoji
```

## License

MIT
