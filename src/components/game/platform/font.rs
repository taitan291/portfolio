//! 5×7 ドットの自作ビットマップフォント。
//!
//! 各文字は 1 行 5 ビット（上位ビットが左）の `u8` × 7 行。
//! 数字の `0` は斜線入り、アルファベットの `O` は角を太らせた丸で、見分けられるようにしている。

use web_sys::CanvasRenderingContext2d;

pub const GLYPH_W: u32 = 5;
pub const GLYPH_H: u32 = 7;

pub type Bitmap = [u8; 7];

#[rustfmt::skip]
const DIGITS: [Bitmap; 10] = [
    [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110], // 0（斜線入り）
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110], // 1
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111], // 2
    [0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110], // 3
    [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010], // 4
    [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110], // 5
    [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110], // 6
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000], // 7
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110], // 8
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100], // 9
];

#[rustfmt::skip]
const LETTERS: [Bitmap; 26] = [
    [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001], // A
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110], // B
    [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110], // C
    [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110], // D
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111], // E
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000], // F
    [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111], // G
    [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001], // H
    [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110], // I
    [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100], // J
    [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001], // K
    [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111], // L
    [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001], // M
    [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001], // N
    [0b01110, 0b11011, 0b10001, 0b10001, 0b10001, 0b11011, 0b01110], // O（太めの丸、斜線なし）
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000], // P
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101], // Q
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001], // R
    [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110], // S
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100], // T
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // U
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100], // V
    [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010], // W
    [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001], // X
    [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100], // Y
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111], // Z
];

#[rustfmt::skip]
const SYMBOLS: [(char, Bitmap); 7] = [
    (' ', [0, 0, 0, 0, 0, 0, 0]),
    ('.', [0, 0, 0, 0, 0, 0b01100, 0b01100]),
    ('+', [0, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0]),
    ('-', [0, 0, 0, 0b11111, 0, 0, 0]),
    ('!', [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0, 0b00100]),
    (':', [0, 0b01100, 0b01100, 0, 0b01100, 0b01100, 0]),
    ('*', [0b00100, 0b00100, 0b11111, 0b01110, 0b01010, 0b10001, 0]), // 星
];

/// 文字のビットマップ。フォントにない文字は `None`。
pub fn bitmap(ch: char) -> Option<&'static Bitmap> {
    match ch {
        '0'..='9' => DIGITS.get(ch as usize - '0' as usize),
        'A'..='Z' => LETTERS.get(ch as usize - 'A' as usize),
        _ => SYMBOLS.iter().find(|(c, _)| *c == ch).map(|(_, b)| b),
    }
}

/// ドットのある位置を `(列, 行)` で列挙する。
pub fn dots(ch: char) -> impl Iterator<Item = (u32, u32)> {
    bitmap(ch).into_iter().flat_map(|b| {
        b.iter().enumerate().flat_map(|(row, bits)| {
            (0..GLYPH_W)
                .filter(move |col| bits & (1 << (GLYPH_W - 1 - col)) != 0)
                .map(move |col| (col, row as u32))
        })
    })
}

/// 1 文字を左上 `(x, y)` に `scale` 倍で描く（塗り色は事前に設定しておく）。
pub fn fill_char(ctx: &CanvasRenderingContext2d, ch: char, x: f64, y: f64, scale: f64) {
    for (col, row) in dots(ch) {
        ctx.fill_rect(
            x + f64::from(col) * scale,
            y + f64::from(row) * scale,
            scale,
            scale,
        );
    }
}

/// 文字列の幅（1 文字 5 ドット + 字間 1 ドット）。
pub fn text_width(text: &str, scale: f64) -> f64 {
    let n = text.chars().count() as f64;
    if n == 0.0 {
        0.0
    } else {
        (n * f64::from(GLYPH_W + 1) - 1.0) * scale
    }
}

/// 中心 `(cx, cy)` に、1 ドットの縁取り付きで文字列を描く。
pub fn draw_text_outlined(
    ctx: &CanvasRenderingContext2d,
    text: &str,
    cx: f64,
    cy: f64,
    scale: f64,
    fill: &str,
    outline: &str,
) {
    let left = (cx - text_width(text, scale) / 2.0).floor();
    let top = (cy - f64::from(GLYPH_H) * scale / 2.0).floor();
    let advance = f64::from(GLYPH_W + 1) * scale;
    let draw = |dx: f64, dy: f64| {
        for (i, ch) in text.chars().enumerate() {
            fill_char(ctx, ch, left + i as f64 * advance + dx, top + dy, scale);
        }
    };
    ctx.set_fill_style_str(outline);
    for (dx, dy) in [(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0)] {
        draw(dx, dy);
    }
    ctx.set_fill_style_str(fill);
    draw(0.0, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::game::engine::sequence::{SEQUENCE, UNRELATED_DECOYS};

    #[test]
    fn all_game_glyphs_exist_and_fit() {
        let chars = SEQUENCE
            .iter()
            .chain(UNRELATED_DECOYS.iter())
            .map(|g| g.as_char())
            .chain("+2.0 404! *".chars());
        for ch in chars {
            let b = bitmap(ch).unwrap_or_else(|| panic!("no glyph {ch:?}"));
            assert!(b.iter().all(|row| *row < 32), "{ch:?}");
        }
    }

    #[test]
    fn zero_and_oh_differ() {
        assert_ne!(bitmap('0'), bitmap('O'));
        // 斜線: 0 の中央付近にはドットがあり、O にはない。
        assert!(dots('0').any(|d| d == (2, 3)));
        assert!(!dots('O').any(|d| d == (2, 3)));
    }

    #[test]
    fn text_width_counts_spacing() {
        assert_eq!(text_width("", 1.0), 0.0);
        assert_eq!(text_width("A", 2.0), 10.0);
        assert_eq!(text_width("+2.0", 1.0), 23.0);
    }
}
