# nural
![build](https://img.shields.io/crates/v/nural) ![language](https://img.shields.io/badge/language-Rust-orange) ![license](https://img.shields.io/github/license/benj-8229/nural) ![downloads](https://img.shields.io/crates/d/nural)

**nural** is a fast, minimal, and context aware CLI note tool


It's designed to deal with the common pain points of CLI note tools; fatigue remembering note names, cumbersome to use, and a cluttered global note store.
With nural, notes are scoped to directories (called "contexts"), meaning you can only access notes in the context you're under. In addition, all commands will attempt to select notes by (in order of preference) searching for exact matches, searching for fuzzy matches, or opening a backup UI for manual selection.

![demo gif](https://raw.githubusercontent.com/benj-8229/nural/6a1574422f22337010a9c1add36fa21f4375867f/repo/demo.svg)

------------

## Quickstart
To set up a note context run `nural init` in your project root.

You can use exact note names in commands
```
nural create todo
nural append "example text" todo
```

You can also use partial note names. The below example will first check for a note named "to", and fall back to a fuzzy match. If there are multiple notes that match (e.g. "**to**do" and "s**to**res") it will open a search in the TUI where you can select a note by typing, using the arrow keys, and pressing enter.
```
nural open to
```

You can also completely leave out the note name to directly open the search UI and select a note
```
nural read
```


## Installation
Currently, the only methods of installation are cargo or building the source yourself.

1. `cargo install nural`
2. ensure `~/.cargo/bin` is in your system path.
3. (optional) add an alias for nural, e.g. `alias n=nural`


## Future Plans
I'm planning to continue development and implement more features

- Different storage modes (stored in project folder vs inside nural installation folder)
- Metadata for notes to allow more intelligent CLI parsing, e.g commands automatically selecting the MRU note
- Piping text into append command
- Global flag to access a more traditional global note store


## Suggestions
I'm eager for feedback and feature requests, just open an issue or reach out directly if you like the tool but wish it had something.
