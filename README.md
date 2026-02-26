# img2ldr
Converts an image to an ldraw file for creating brick art.

## Installation

### Arch Linux

```
pacman -U img2ldr-0.1.0-x86_64.pkg.tar.zst
```

### Tar.gz

```
tar -xzf img2ldr-0.1.0-x86_64.tar.gz
./img2ldr
```

### Build From Source

```
git clone https://github.com/kadsendino/img2ldr.git
cd img2ldr
cargo run --release
```

## Usage

```
img2ldr -i <inputpath> -o <outputpath> -w <number of chunks width> <number of chunks height>
```
