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
``` sh
cargo xbuild
```

## Create boot image
``` sh
cargo bootimage
```

## Run
これはDocker上でなく、ホストOS上で試す
``` cmd
qemu-system-x86_64.exe -drive format=raw,file=./target/x86_64-rust_os/debug/bootimage-rust_os.bin
```

## Memo: Boot
- PCの電源ON
- マザーボードROMに焼かれているファームウェアを実行
    - power-on self-test を実行
    - RAMが利用可能なことを検知
    - CPUとハードウェアのpre-initialize
    - boot可能なディスクを検知
    - boot

## ファームウェア(x86)
- BIOS
- UEFI

## 参考
教科書:
https://os.phil-opp.com/


参考書:
このあたりも参考にさせてもらってる。
https://qiita.com/morimolymoly/items/182b5b4cbf6bc2837868
https://tomo-wait-for-it-yuki.hatenablog.com/entry/2018/11/17/152359

