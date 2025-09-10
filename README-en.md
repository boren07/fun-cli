[简体中文](README.md)

<div align="center">

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Stars](https://img.shields.io/github/stars/borenchan/fun-cli?logo=github)](https://github.com/borenchan/fun-cli/stargazers)
[![Forks](https://img.shields.io/github/forks/borenchan/fun-cli?logo=github)](https://github.com/borenchan/fun-cli/network/members)
[![GitHub last commit](https://img.shields.io/github/last-commit/mxsm/rocketmq-rust)](https://github.com/mxsm/rocketmq-rust/commits/main)
![GitHub repo size](https://img.shields.io/github/repo-size/borenchan/fun-cli)
![Static Badge](https://img.shields.io/badge/MSRV-1.80.0%2B-25b373)
<br/>
[![Fun CLI](https://img.shields.io/badge/fun-cli)](https://github.com/borenchan/fun-cli)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

</div>

# 🎮 `fun-cli` —— **Your Terminal Playground**

> *"Life is too long, but fun CLIs are too few"*

**`fun-cli` is a constantly growing command-line entertainment toolkit** designed for those who find work boring. Here, you can:
- ☔ Check the weather for your family and friends anytime
- 🎵 Play your favorite music with an ASCII spectrum
- 🕹️ Play Thunder Fighter and relive childhood joy (Immersive gameplay, press Q to exit)
- 💻 Monitor your computer with a cool monitoring panel
- 📝 Run on all platforms (Windows, MacOS, Linux)
- ...**or contribute your own creative ideas!**


## 📦 **Quick Start**

### Install from release package (supports all platforms)

1. Download the latest release package => [release](https://github.com/borenchan/fun-cli/releases)
2. Add to your environment variable PATH
3. Congratulations! You can start having fun now
```bash
fun -h    # View help
```


### Install from source

**Environment requirements:  Rust ≥1.80**
```bash
# Clone the repository (requires Rust ≥1.80)
git clone https://github.com/borenchan/fun-cli.git
cd fun-cli

# Build and install
cargo build --release
cargo install --path . # Install globally

# Run unit tests (we call this "happy validation")
cargo test -- --nocapture # Prevent output from being swallowed

# Congratulations! You can start having fun
fun -h # View help
```
## 🚀 **Why Join?**
- Practice programming in the happiest way **Happiness is the fundamental motivation for programming**
- No serious code reviews needed, **fun is the primary productivity**
- Get rid of repetitive and boring work cycles , **enjoy the most primitive joy of programming**


## 🤝 **How to Contribute?**
1. Submit a PR with: a description of the feature
2. While coding, chant "the borrow checker is my friend" three times

**We don't need perfect code, just interesting souls!**

```shell
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000OOkxxxddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddxxkkO0000000000
0000OkxoollloooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooolllodxO0000000
000kdlloodddddxddxxxxxddxxxxdxxxxxxdxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxdxxxxxxxxxxxxxxxxxxddddollodO00000
0OxlloddxxxxxxxxxxxxxxxxddxxxdxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxdxxxxxxxxxxxxxxxxdddoloxO000
Odllddxxxxxxxxxxxxxxxxxxxxdxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxddxxxxxxxxxxxxxxxxxdolox000
xllodxxxxxxxxxxxxdolc:::::::::codxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxdl:,''',:odxxxxxxxxxxxxxdolok00
dlodxdxxdxxxxxxxdc..............;oxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxddl,...','...;oxxxxxxxxxxxxxdllx00
oloxxxxxxxxxxxxxc. .;:::::::::'  :dxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxl' .cddxdo;. ,dxxxxxxxxxxxxdoldO0
oloxxxxxxxxxxxxd:. ,cccccccccc;. ,dxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxd:..:dxxxdxo' .lxxxxxxxxxxxxdoldO0
olodxxxxxxdddddd:. ,cccccccccc;. ,odddddxxxxxxxxxxxxxxxxxxxxxxxxxxxc. 'ldddxdc. 'oxxxxxxxxxxxxdoldO0
olodxxxdo:'...... .,cccccccccc;. ......';ldxxxxxxxxxxxxxxxxxxddddxxd:. .,;:;'..'ldxddddxxdxxxxdoldO0
olodxxxl' ..'''''';cccccccccccc;''''''.. .cdxxxxxxxxxxxxxdoc,'..',:odo:'.....,cooc;'..',:ldxxxdoldO0
olodxxd:  ,cccccccccccccccccccccccccccc;. ,dxxxxxxxxxxxxdc...',,'. .:oxddooodddc. .',,'...;oxxdoldO0
olodxxd; .;cccccccccccccccccccccccccccc:. ,dxxxxxxxxxxxdc. 'ldxddo, .:dxxxxxxdc. 'lddddo;. ;dxdoldO0
oloxxxd; .;cccccccccccccccccccccccccccc:. 'dxxxxxxxxxxxd; .cddddxdl. ,oxxxxdxd; .cddddddo. 'oxdoldO0
oloxxxd; .,cccccccccccccccccccccccccccc:. ,oxxxxxxxxxxxd:. ,oddddo;. ;dxxxxdddc. ,odxxdd:. ;dxdoldO0
olodxxd:. ':cccccccccccccccccccccccccc:,. ;dxxxxxxxxxxxxd:. .,;;,. .;oxdooloodd:. .,;;,...,odxdoldO0
olodxxxo;. .......':cccccccccc:'....... .,oxxxxxxxxxxxxxxdo:'....';ldl;'.....':oo:'....';ldxxxdoldO0
olodxxxxdl:;;;;;.  ,cccccccccc;. .,;;;;:ldxxxxxxxxxxxxxxxxxxddddddxd:. .;:c:,. .lddddoddxxxxxxdoldO0
olodxxxxxxxxxxxd:. ,cccccccccc;. ;dxxdxxxxxxxxxxxxxxxxxxxxxxxxxxxxxc. ,odddddc. 'oxxxxxxxxxxxxdoldO0
olodxdxxxxxxxxxd:. ,cccccccccc;. ;dxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx:. ;ddddddo' .lxxxxxxxxxxxxdoldO0
olodddxxxxxxxxxxl. .',,,,,,,,'. .cxxdxxdxxxxxxxxxxxxxxxxxxxxxxxxxxxl' .:odddl;. ;dxxxxxxxxxxxxdllx00
dlodxxxxxxxxxxxxdl,............,cdxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxdl,....'....:oxxxxxxxxxxxdxdllx00
xllodxxxxxxxxxxdxxddooooooooooddxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxdl:;,,,;codxxxxxxxxxxxxddoloO00
Oxllodxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxdxxxxxxxxxxxxxxxxdolok000
0Oxoloddxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxddxxxxxxxxxxxxdddollok0000
000kdollooddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddoolloxO00000
00000OkdoollllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllloodxkO0000000
00000000OOkkxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxkkOO00000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

