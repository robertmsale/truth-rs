# truth-rs

This is a simple command line tool to check if a given string is true or false. It scans for unique tokens in the input and generates a truth table of every possible combination of inputs.

## Usage

```bash
$ truth-rs "!(A && B) || (C && D)"
```

Supports the following operators:
- `!` (not)
- `&&` (and)
- `||` (or)