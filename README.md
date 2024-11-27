# Rocks Text Editor

Rocks is a simple TUI text editor made in Rust using [Ratatui](https://ratatui.rs/).

## Roadmap

- [x] Simple terminal user interface
- [x] Open files by passing an argument
- [x] Edit file contents
- [x] Save file
- [ ] Cursor behavior
  - [ ] Cursor movement
  - [ ] Insert/delete text at cursor position
  - [ ] Cursor selection
  - [ ] Multiple cursors
- [ ] Copy and paste
- [ ] Syntax highlighting
- [ ] Undo/Redo

## Usage

Rocks is a terminal-based text editor, in order to run it you need to run:

Windows:

```./rocks.exe [PATH_TO_FILE]```

Linux:

```rocks [PATH_TO_FILE]```

## Building

> A working [Rust environment](https://www.rust-lang.org/en-US/learn/get-started) is needed.

```
cargo build --release
```

## License

Rocks is licensed under the [GNU GPLv3](LICENSE.txt).
