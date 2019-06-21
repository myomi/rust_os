// OS開発時はstdが使えない
#![no_std]

// error: requires `start` lang_item エラーを回避するために no_mainを指定する。
// stdを使うRustコードは起動時にcrt0というCランタイムライブラリを使用している。
// crt0はスタックの生成や引数をレジスタに格納するなどの初期処理を行い、main関数を呼ぶ。
// no_std の場合は crt0 が使えないので main 関数を使用できない
#![no_main]


use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // VGAビデオメモリのアドレス割り当て
    // 0xA0000 - EGA/VGAグラフィックモード（64KB）
    // 0xB0000 - モノクロテキストモード（32KB）
    // 0xB8000 - カラーテキストモードおよびCGA互換グラフィックモード（32KB）
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// no_std の場合は必須。
/// コンパイラがpanic発生時に実行する関数。
/// 
/// ## VSCode(というかLanguage Server)のエラー対策
/// "rust.all_targets": false の設定を .vscode/settings.json に追加してエラー出力を回避している
/// 参考: https://github.com/rust-lang/rls/issues/904
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
