// OS開発時はstdが使えない
#![no_std]

// error: requires `start` lang_item エラーを回避するために no_mainを指定する。
// stdを使うRustコードは起動時にcrt0というCランタイムライブラリを使用している。
// crt0はスタックの生成や引数をレジスタに格納するなどの初期処理を行い、main関数を呼ぶ。
// no_std の場合は crt0 が使えないので main 関数を使用できない
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

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
