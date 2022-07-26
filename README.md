## usci
a little tool of usci code

### Usage
```
USAGE:
    usci <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    generate    Generate some usci codes
    help        Print this message or the help of the given subcommand(s)
    info        Show the information of codes
    verify      Check if codes are valid
```

### Example
1. Generate usci code
```bash
// just give me one
usci generate

// wait, i need more, how about .. nine
usci generate -c 9
```
2. Check if the code is valid, `TRUE` is yes and `FALSE` is no
```bash
// tell me if that code is cute
usci verify 31803427L13Q5WFN13
```
3. Show detail of the code
```bash
// show me the puzzle
usci info 31803427L13Q5WFN13
```