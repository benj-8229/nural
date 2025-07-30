# nural
![build](https://img.shields.io/crates/v/nural) ![language](https://img.shields.io/badge/language-Rust-orange) ![license](https://img.shields.io/github/license/benj-8229/nural) ![downloads](https://img.shields.io/crates/d/nural)

**nural** is a fast, minimal, and context aware CLI note tool


nural is designed to deal with all the common pain points of CLI note tools; high cognitive load, cumbersome note access, and a cluttered note store.
With nural, notes are scoped to directories (called "contexts"), meaning notes are only visible where they're relevant. In addition, all commands do their best to infer your intent, despite partial or even missing information.

![demo gif](https://raw.githubusercontent.com/benj-8229/nural/6a1574422f22337010a9c1add36fa21f4375867f/repo/demo.svg)

------------

## Quickstart
To set up a note context run `nural init` in your project root. This context spans all child directories of your project root.

You can use exact note names in commands
```
nural create todo
nural append "example text" todo
```

You can also use partial note names. The below example will first check for a note named "to", and fall back to a fuzzy match. If there are multiple notes that match (e.g. "**to**do" and "s**to**res") it will open a search in the TUI where you can select a note by typing, using the arrow keys, and pressing enter
```
nural open to
```

You can also completely leave out the note name to directly open the search UI and select a note
```
nural read
```

All commands have aliases (visible in their help messages). This, in combination with partial note name parsing, lets you access notes with minimal input and thought. For example, the below command would append the example text to the end of the newly created todo note
```
nural c todo
nural a "Update readme" to
```

## Installation
Currently, the only methods of installation are cargo or building the source yourself.

1. `cargo install nural`
2. ensure `~/.cargo/bin` is in your system path.
3. (optional) add an alias for nural, e.g. `alias n=nural`

### Platforms
nural is currently only tested on Linux. Expanding support to Windows and Mac is my number one priority and should be here soon.

### Setup
Following installation and a dry run of `nural` from the terminal to generate files, you can make some optional configuration tweaks (`~/.config/nural/nural.conf`)

- The `note_extension` setting lets you change your preferred note file extension, I would recommend leaving this default at .md.
- The `editor` setting is the command to be run to open notes. Default is `nano`.
- The `reader` setting is the command to be run to print notes to the terminal. Default is `cat`.
- The `lister` setting is the command to be run to print all available notes to the terminal. Default is `tree`.

All other settings in the config are currently unimplemented.

## Future Plans
I'm planning to continue development and implement more features

- Different storage modes (stored in project folder vs inside nural installation folder)
- Metadata for notes to allow more intelligent CLI parsing, e.g commands automatically selecting the MRU note
- Piping text into append command
- Global flag to access a more traditional global note store


## Suggestions
I'm eager for feedback and feature requests, just open an issue or reach out directly if you like the tool but wish it had something.
