// OS開発時はstdが使えない
#![no_std]

// error: requires `start` lang_item エラーを回避するために no_mainを指定する。
// stdを使うRustコードは起動時にcrt0というCランタイムライブラリを使用している。
// crt0はスタックの生成や引数をレジスタに格納するなどの初期処理を行い、main関数を呼ぶ。
// no_std の場合は crt0 が使えないので main 関数を使用できない
#![no_main]

// test attribute が使えないので」代わりにcustom_test_frameworksを使う
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// no_std の場合は必須。
/// コンパイラがpanic発生時に実行する関数。
/// 
/// ## VSCode(というかLanguage Server)のエラー対策
/// "rust.all_targets": false の設定を .vscode/settings.json に追加してエラー出力を回避している
/// 参考: https://github.com/rust-lang/rls/issues/904
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests.", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}