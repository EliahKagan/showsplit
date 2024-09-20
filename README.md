# showsplit - Show what `std::env::split_paths` does

This is a simple command-line wrapper for `env::split_paths` in the Rust
standard library.

## Motivation

The behavior of `env::split_paths` may seem trivial but actually has some
subtleties, and its specific relationship to the path search behavior of
`std::process::Command` is non-obvious. In particular, its behavior differs by
operating system, which this can be used to check and experiment with.

For example, on Windows, `env::split_paths` recognizes quoting with literal `"`
characters that remove the special meaning from `;`, which is otherwise the
separator. (Such values of the `PATH` environment variable work as intended in
some but not all context in Windows itself, and that `std::env::split_paths`
and `std::process::Command` support them does not mean one should use this
feature. But exploring it is nonetheless one of the motivating uses of this
little utility.)

## Usage

With a command-line argument, the value of the `PATH` environment variable is
used:

```sh
showsplit
```

With a command-line argument, the argument provides the value to be split. For
example, on Unix-like systems:

```sh
showsplit '/bin:/usr/bin/:/sbin:/usr/sbin'
```

Or on Windows:

```pwsh
showsplit 'C:\Windows\System32;C:\Windows;C:\Users\me\bin'
```

Passing more than one argument is not supported.

`--help` is supported and shows usage information.

## Output format

The individual pathnames are shown readably when possible, as literal text
inside `[` `]` with no other quoting and no escaping. But they fall back to
their quoted debugging representation in the rare case that they are not valid
Unicode.

For example, on Windows, from PowerShell:

```text
> cargo run -- "`"foo;bar`";baz;ab`u{D800}cd;qu ux"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target\debug\showsplit.exe "\"foo;bar\";baz;ab�cd;qu ux"`
  0: [foo;bar]
  1: [baz]
  2: "ab\u{d800}cd"
  3: [qu ux]
```

## License

[0BSD](LICENSE)
