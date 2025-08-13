# XBF

An ARM64 compiler for Brainfuck

## Installation and usage

```sh
git clone https://voidwyrm-2/xbf.git
cd xbf
cargo build --release
./target/release/xbf -o hello.s examples/hello.bf
as -o hello.o hello

# If on MacOS
ld -o hello hello.o -macos_version_min 15.0 -lSystem -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib

# If on Linux
ld -o hello hello.o

./hello
```
