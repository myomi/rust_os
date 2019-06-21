# Rust OS
RustでOSを書く

## Prerequirement
VSCode with "Remote - Containers" extension
Docker
Qemu (動作確認用)

## Setup
```
git clone
code ./rust_os
```

## Build
### Linux
``` sh
cargo rustc -- -C link-arg=-nostartfiles
```
### Windows
``` sh
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
```
### macOS
``` sh
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

## 参考
教科書:
https://os.phil-opp.com/


参考書:
このあたりも参考にさせてもらってる。
https://qiita.com/morimolymoly/items/182b5b4cbf6bc2837868
https://tomo-wait-for-it-yuki.hatenablog.com/entry/2018/11/17/152359

