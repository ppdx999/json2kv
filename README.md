# json2kv

A Rust program that converts JSON data from stdin to KeyValue format.

## Overview

This tool reads JSON data and converts it to a flat KeyValue format. Nested JSON structures are represented by concatenating keys with dots (`.`).

## KeyValue Format Specification

- Each JSON key-value pair is stored on a single line
- Keys and values are separated by a space
- Keys and values are identified by the first space that appears
- Spaces in keys are converted to `-` (hyphen)
- Tabs in keys are converted to `-` (hyphen)
- Values can contain spaces as-is
- Neither keys nor values should contain newlines
  - If a key contains newlines (`\n`, `\r`, `\r\n`), the program exits with an error
  - If a value contains newlines, they are processed as follows:
    - `\r\n` (Windows format) is normalized to `\n`
    - `\r` (old Mac format) is normalized to `\n`
    - Normalized `\n` is escaped to `\n`
- The program exits with an error if dangerous characters such as null characters are present
- Nested JSON structures are represented by concatenating keys with `.`

## Installation

### Download Binary (Recommended)

You can download binaries for each platform from the latest release:

```bash
# Linux (x86_64)
curl -LO https://github.com/ppdx999/json2kv/releases/latest/download/json2kv-linux-x86_64
chmod +x json2kv-linux-x86_64
sudo mv json2kv-linux-x86_64 /usr/local/bin/json2kv

# Linux (ARM64)
curl -LO https://github.com/ppdx999/json2kv/releases/latest/download/json2kv-linux-aarch64
chmod +x json2kv-linux-aarch64
sudo mv json2kv-linux-aarch64 /usr/local/bin/json2kv

# macOS (Intel)
curl -LO https://github.com/ppdx999/json2kv/releases/latest/download/json2kv-macos-x86_64
chmod +x json2kv-macos-x86_64
sudo mv json2kv-macos-x86_64 /usr/local/bin/json2kv

# macOS (Apple Silicon)
curl -LO https://github.com/ppdx999/json2kv/releases/latest/download/json2kv-macos-aarch64
chmod +x json2kv-macos-aarch64
sudo mv json2kv-macos-aarch64 /usr/local/bin/json2kv

# Windows (PowerShell)
# Invoke-WebRequest -Uri "https://github.com/ppdx999/json2kv/releases/latest/download/json2kv-windows-x86_64.exe" -OutFile "json2kv.exe"
```

### Build from Source

```bash
# Build
cargo build --release

# Binary is generated at target/release/json2kv
```

## Usage

```bash
# Read JSON from stdin
echo '{"name": "Alice", "age": 30}' | json2kv

# Read from file
cat input.json | json2kv

# Specify output file
cat input.json | json2kv > output.kv
```

## Input/Output Examples

### Example 1: Simple JSON

**Input:**
```json
{
  "name": "Alice",
  "age": 30,
  "city": "Tokyo"
}
```

**Output:**
```
age 30
city Tokyo
name Alice
```

### Example 2: Nested JSON

**Input:**
```json
{
  "user": {
    "name": "Bob",
    "address": {
      "city": "Osaka",
      "zip": "530-0001"
    }
  },
  "active": true
}
```

**Output:**
```
active true
user.address.city Osaka
user.address.zip 530-0001
user.name Bob
```

### Example 3: Keys with Spaces

**Input:**
```json
{
  "user name": "Charlie",
  "email address": "charlie@example.com"
}
```

**Output:**
```
email-address charlie@example.com
user-name Charlie
```

### Example 4: Values with Newlines

**Input:**
```json
{
  "description": "This is\na multi-line\ntext"
}
```

**Output:**
```
description This is\na multi-line\ntext
```

**Note:** Regardless of newline format (`\r\n`, `\r`, `\n`), all are normalized to `\n` before being escaped. This allows uniform handling of newline formats from different platforms (Windows, Mac, Linux).

### Example 5: JSON with Arrays

**Input:**
```json
{
  "tags": ["go", "json", "cli"],
  "count": 3
}
```

**Output:**
```
count 3
tags.0 go
tags.1 json
tags.2 cli
```

## Error Handling

The program exits with an error in the following cases:

- Invalid JSON format
- Keys contain newlines (`\n`, `\r`, `\r\n`)
- Null characters (`\0`) are present
- Other control characters are inappropriately included

**Note:** Values containing newlines do not cause an error and are automatically escaped.

## License

MIT License
