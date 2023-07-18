# Rust CV Sample

## Required

---

- OpenCV

```bash
brew install opencv
```

- pkg-config

```bash
brew install pkg-config
```

- vcpkg

```bash
brew install vcpkg
```

## Setup

---

synbolic link: 
```bash
ln -s /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libclang.dylib /Users/${user_name}/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib
```

## Debug Run

---

```bash
cargo run
```