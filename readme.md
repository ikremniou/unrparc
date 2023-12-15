# Unrparc - Extract RPA('.rpa') Renpy archives 

The CLI tool library to unpack ".rpa" archives written in Rust.

## How to install

TBA

## How to use

The overall usage patten is as follows:
```bash
unrparc <COMMAND> [OPTIONS] [ARGUMENTS] 
```

There are two commands:
- `extract` - extracts the RPA archive. 
- `scan` - scans the RPA archive and outputs files it contains into standard out.
- `help` - shows this help message. Refer to it for up-to-date help.

### Extract

Extract command can be used with optional `--glob` or short `-g` flag. It allows extracting only specific files.

```bash
unrparc extract <ARCHIVE> <DESTINATION>
```

### Scan 

Scan command can be used with optional `--glob` or short `-g` flag. It allows showing only specific files.

```bash
unrparc scan <ARCHIVE>
```

## Examples

1. Scan files from the `./tests/assets/scripts.rpa` archive:

    ```bash
    unrparc scan ./tests/assets/scripts.rpa
    ```

    ```result
    gui.rpy: 16306 bytes
    gui.rpyc: 25908 bytes
    options.rpy: 7033 bytes
    options.rpyc: 5246 bytes
    screens.rpy: 43480 bytes
    screens.rpyc: 104276 bytes
    script.rpy: 842 bytes
    script.rpyc: 2098 bytes
    ```

1. Scan files from the `./tests/assets/scripts.rpa` archive using `-g *.rpy` flag:

    ```bash
    unrparc scan ./tests/assets/scripts.rpa -g *.rpy
    ```

    ```result
    gui.rpy: 16306 bytes
    options.rpy: 7033 bytes
    screens.rpy: 43480 bytes
    script.rpy: 842 bytes
    ```

1. Extract files from the `./tests/assets/scripts.rpa` archive:

    ```bash
    unrparc extract ./tests/assets/scripts.rpa ./extracted
    ```

1. Extract files from the `./tests/assets/scripts.rpa` archive using `-g *.rpy` flag:

    ```bash
    unrparc extract ./tests/assets/scripts.rpa ./extracted -g *.rpy
    ```