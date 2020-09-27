# pgm converter

Convert and clop pgm image.

## Build

[NOTE] You don't need to build. Executable files are available on github action's artifact.
1. install rustup
2. `$ cargo build --release`
3. find executable: `./target/release/build/pgm_converter`

## Usage

```
$ pgm_converter --help
```

to see help

```
pgm_converter 0.1.0

USAGE:
    pgm_converter [OPTIONS] --dst <dst> --mode <mode> --src <src>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --mode <mode>        Output PGM Type (b: Binary, a: Ascii)
    -s, --src <src>          Source file path
    -d, --dst <dst>          Output file path
    -l, --left <left>        left corrdinate [default: 0]
    -t, --top <top>          top cordinate [default: 0]
    -w, --width <width>      width (Optional)
    -h, --height <height>    height (Optional)
```

## Example
- Convert `./example.pgm` into Ascii.

    ```shell
    $ pgm_converter -m a -s ./example.pgm -d ./example_cloped_ascii.pgm
    ```

-  Convert `./example.pgm` into Ascii and clop (3,3).

    ```shell
    $ pgm_converter -m a -s ./example.pgm -d ./example_cloped_ascii.pgm -l 3 -t 3
    ```

-  Convert `./example.pgm` into Ascii and clop (3,3)-(9,9).

    ```shell
    $ pgm_converter -m a -s ./example.pgm -d ./example_cloped_ascii.pgm -l 3 -t 3 -w 6 -h 6
    ```