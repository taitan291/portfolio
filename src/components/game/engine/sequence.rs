//! 撃つ順番（シーケンス）と、的に書かれる文字。

use super::config::SEQUENCE_LEN;

/// 的に書かれる文字。数字の `0`（`D0`）とアルファベットの `O` は別の値。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Glyph {
    D4,
    D0,
    N,
    O,
    T,
    F,
    U,
    D,
    E,
    H,
    K,
    M,
    R,
    S,
    W,
    X,
    Y,
    Z,
}

/// 的の種類。描画色の決定に使う。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GlyphKind {
    /// 数字（`4`, `0`）。
    Digit,
    /// シーケンス内のアルファベット。
    SeqLetter,
    /// シーケンスに含まれないダミー。
    Decoy,
}

impl Glyph {
    pub fn kind(self) -> GlyphKind {
        use Glyph::*;
        match self {
            D4 | D0 => GlyphKind::Digit,
            N | O | T | F | U | D => GlyphKind::SeqLetter,
            E | H | K | M | R | S | W | X | Y | Z => GlyphKind::Decoy,
        }
    }

    /// フォントの検索に使う文字。数字の `0` は `'0'`、オーは `'O'`。
    pub fn as_char(self) -> char {
        use Glyph::*;
        match self {
            D4 => '4',
            D0 => '0',
            N => 'N',
            O => 'O',
            T => 'T',
            F => 'F',
            U => 'U',
            D => 'D',
            E => 'E',
            H => 'H',
            K => 'K',
            M => 'M',
            R => 'R',
            S => 'S',
            W => 'W',
            X => 'X',
            Y => 'Y',
            Z => 'Z',
        }
    }
}

use Glyph::*;

/// 撃つ順番: `404` × 3 → `NOTFOUND`。
pub const SEQUENCE: [Glyph; SEQUENCE_LEN] =
    [D4, D0, D4, D4, D0, D4, D4, D0, D4, N, O, T, F, O, U, N, D];
/// シーケンスに含まれる文字（重複なし）。正解以外が「紛らわし」になる。
pub const SEQ_DECOYS: [Glyph; 8] = [D4, D0, N, O, T, F, U, D];
/// シーケンスに含まれない、`0/O/4/D` と見分けやすい文字。
pub const UNRELATED_DECOYS: [Glyph; 10] = [E, H, K, M, R, S, W, X, Y, Z];

/// `404` の 1 セットの長さと、セット数。
pub const SET_LEN: usize = 3;
pub const SET_COUNT: usize = 3;

/// 進捗が `progress` になった瞬間に `404` のセットを撃ち終えたなら、そのセット番号（1 始まり）。
pub fn completed_set(progress: usize) -> Option<u8> {
    (progress > 0 && progress <= SET_LEN * SET_COUNT && progress.is_multiple_of(SET_LEN))
        .then_some((progress / SET_LEN) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_spells_404x3_notfound() {
        let s: String = SEQUENCE.iter().map(|g| g.as_char()).collect();
        assert_eq!(s, "404404404NOTFOUND");
    }

    #[test]
    fn decoy_pools_are_disjoint_and_cover_sequence() {
        for g in SEQUENCE {
            assert!(SEQ_DECOYS.contains(&g));
            assert!(!UNRELATED_DECOYS.contains(&g));
        }
        for g in UNRELATED_DECOYS {
            assert_eq!(g.kind(), GlyphKind::Decoy);
        }
        assert_ne!(D0, O);
        assert_ne!(D0.kind(), O.kind());
    }

    #[test]
    fn set_boundaries() {
        let sets: Vec<_> = (0..=SEQUENCE_LEN).filter_map(completed_set).collect();
        assert_eq!(sets, vec![1, 2, 3]);
        assert_eq!(completed_set(9), Some(3));
        assert_eq!(completed_set(12), None);
    }
}
