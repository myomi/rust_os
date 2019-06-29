use core::fmt;
use volatile::Volatile;

/// VGA text mode のカラーパレット
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// VGA text buffer のカラーコード
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]    // u8のメモリレイアウトを保証する
struct ColorCode(u8);

impl ColorCode {
    /// 1 - 4bit: forground
    /// 5 - 8bit: background     
    fn new(forground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (forground as u8))
    }
}

/// VGA text buffer の文字
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]  // field のメモリレイアウト順を保証する
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;    // VGA text buffer の行数
const BUFFER_WIDTH: usize = 80;     // VGA text buffer の列数

/// text buffer２次元配列
///  0 -  7bit ASCII code point
///  8 - 11bit Foreground color (4つ目のbitはbright colorのフラグ)
/// 12 - 14bit Background color
///      15bit Blink
/// 
/// Volatileを使って、VGA text bufferが最適化されてしまうのを防ぐ
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// VGA text bufferのライター
pub struct Writer {
    column_position: usize,         // 現在のカーソル位置
    color_code: ColorCode,          // カラーコード
    buffer: &'static mut Buffer,    // text buffer ２次元配列
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    // 最右端まで書き込んでいたら改行する
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;    // 最下端に書く
                let col = self.column_position; // 続きに書く

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ASCII
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not ASCII
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                // １行上に書き直す
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // 最下行をクリア
        self.clear_row(BUFFER_HEIGHT - 1);
        // カーソルを再左端に移動
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

/// write! マクロでWriterを使えるようにする
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Writeのテスト
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // VGAビデオメモリのアドレス割り当て
        // 0xA0000 - EGA/VGAグラフィックモード（64KB）
        // 0xB0000 - モノクロテキストモード（32KB）
        // 0xB8000 - カラーテキストモードおよびCGA互換グラフィックモード（32KB）
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    // VGA text bufferへの書き込みは絶対に失敗しない前提でunwrapしている
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    writer.new_line();
    writer.write_string("Hello, Rust OS!");
}