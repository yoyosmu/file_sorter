# File Sorter

A simple Rust program that automatically organizes files in a folder.

## Features

* Sorts images
* Sorts documents
* Sorts audio files
* Sorts archives

## ARGUMENTS

```bash
file_sorter ~/Documents
```

## Usage

Run with:

```bash
cargo run
```

Or build release version:

```bash
cargo build --release
```

## Example

Before:

```text
Downloads/
├── image.png
├── music.mp3
├── file.pdf
```

After:

```text
Downloads/
├── PDFs/
├── IMGs/
├── AUDIOs/
├── ARCHIVEs/
```

## Built With

* Rust
* std::fs
* dirs crate
