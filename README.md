# Lister
A lister cli written in rust.

## Usage
Input this:
```
lister COLUMN1 COLUMN2 - ROW1_COL1 ROW1_COL2 - ROW2_COL1 ROW2_COL2
```
Will output something off:
```
+---------+---------+
|COLUMN1  |COLUMN2  |
+---------+---------+
|ROW1_COL1|ROW1_COL2|
|ROW2_COL1|ROW2_COL2|
+---------+---------+
```

## Options
```
-d:
    Flip between box drawing display or ASCII display.
```

# Build and Install

## Requirement
- [Rust Development Package](https://www.rust-lang.org/tools/install)
- [git](https://git-scm.com/downloads)

## Clone
- Change directory to desired place.
- Run `git clone https://github.com/ello-guby/lister.git`.

## Build
- In folder run `cargo build`, `target` folder spawned.

## Install
- Copy binary from the `target` folder to your binary folder.
    - Common Binary Folder Directory:
        - Windows: i didnt use that, sorry.
        - Linux: `/usr/bin/ | /usr/local/bin/`
        - MacOS: nope, im too poor to own one.
- Are the binary folder in PATH variable???

## Using
- Run `lister something something something - something something something ...`