# fnm-util

![GitHub last commit](https://img.shields.io/github/last-commit/kaha440/fnm-util?style=flat-square)
![GitHub Tag](https://img.shields.io/github/v/tag/kaha440/fnm-util?style=flat-square)
![GitHub License](https://img.shields.io/github/license/kaha440/fnm-util?style=flat-square)

A tool for updating Node.js using [fnm](https://github.com/Schniz/fnm)

## Installation

Ensure that fnm is already installed.

```shell
cargo install --git https://github.com/kaha440/fnm-util
```

## Usage

To check for Node.js updates and install the latest version if you need:

```shell
fnm-util update
```

To migrate global packages while installing:

```shell
fnm-util update -p <FROM>
fnm-util install -p <FROM> <VERSION>
```

## License

MIT License
