# MIPS-INSTRUCTION-TOOL

> A mips instruction tool

## Usage

- show help message

```bash
mitool -h
```

- `b2c` subcommand: convert binary file to coe file
    - `-i`: input file
    - `-o`: output file

```bash
mitool b2c -i inst.bin -o inst.coe
```

- `c2i` subcommand: convert coe file to readable assembly file
    - `-s`: input hex string
    - `-i`: input coe file
    - `-o`: output assembly file

```bash
mitool c2i -s 3C010010
```

```bash
mitool c2i -i inst.coe -o inst.S
```
