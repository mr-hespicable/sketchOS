# how to setup 
0. install rust
```
cmon man just like sudo pacman -S rustup or whatever idk
```
1. install rust nightly
```
rustup override set nightly
```

2. add stdlib component for some reason? (idk all i know is that it doesn't work without it)
```
rustup component add rust-src
```
3. add llvm-tools-preview
```
rustup component add llvm-tools-preview
```
4. install bootimage thingy so things can be made into a bootimage
```
cargo install bootimage
```
done then just build and develop

one liner because i lazy :)
```
rustup override set nightly && rustup component add rust-src llvm-tools-preview && cargo install bootimage && cargo build
```
