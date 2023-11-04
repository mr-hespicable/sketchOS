# how to setup 
1. install rust nightly
```
rustup override set nightly
```

2. add stdlib component for some reason? (idk all i know is that it doesn't work without it)

- on mac:
```
rustup component add rust-src --toolchain nightly-x86_64-apple-darwin
```

- on linux:
```
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

3. add llvm-tools-preview
```
rustup component add llvm-tools-preview
```

done then just build and develop
