# C_OS

This is my attempt at writing my first operating system. It is written in rust, and my first steps are coming from https://os.phil-opp.com

## Build

This is how you build this thing. First install cargo-xbuild:

``` 
cargo install cargo-xbuild
```

Then install the rust source:

````
rustup component add rust-src
````

Then build(cross compile) the kernel:

````
cargo xbuild --target x86_64-c_os.json
````

To actually make a bootable kernel-image you need to install "bootimage" first:

```
cargo install bootimage --version "^0.5.0"
````

after this, create the bootimage with:

```
bootimage build
````
