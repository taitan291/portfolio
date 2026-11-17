//! 個人ランキング（ブラウザ内だけに保存する自己ベスト一覧）。
//!
//! 保存形式は 1 行 1 記録の `total,penalties,date_ms`。外部ライブラリなしで読み書きでき、
//! 壊れた行は読み飛ばす。

/// 保存する記録の件数。
pub const RANKING_LEN: usize = 5;

/// 1 回分のクリア記録。
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Record {
    /// ペナルティ込みのタイム（秒）。小さいほど上位。
    pub total: f64,
    /// 誤射の回数。
    pub penalties: u32,
    /// 記録した日時（UNIX エポックからのミリ秒）。
    pub date_ms: f64,
}

/// タイム順（昇順）に並んだ記録。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Ranking {
    records: Vec<Record>,
}

impl Ranking {
    pub fn records(&self) -> &[Record] {
        &self.records
    }

    /// 記録を追加し、ランクインした順位（0 始まり）を返す。圏外なら `None`。
    /// 同タイムは先に出した記録を上位にする。
    pub fn insert(&mut self, record: Record) -> Option<usize> {
        if !is_valid(&record) {
            return None;
        }
        let pos = self
            .records
            .iter()
            .position(|r| record.total < r.total)
            .unwrap_or(self.records.len());
        if pos >= RANKING_LEN {
            return None;
        }
        self.records.insert(pos, record);
        self.records.truncate(RANKING_LEN);
        Some(pos)
    }

    pub fn serialize(&self) -> String {
        self.records
            .iter()
            .map(|r| format!("{},{},{}", r.total, r.penalties, r.date_ms))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 保存文字列から復元する。壊れた行は無視し、並び順と件数も整え直す。
    pub fn parse(s: &str) -> Self {
        let mut ranking = Self::default();
        for line in s.lines() {
            let mut it = line.trim().split(',');
            let (Some(total), Some(penalties), Some(date_ms), None) =
                (it.next(), it.next(), it.next(), it.next())
            else {
                continue;
            };
            let (Ok(total), Ok(penalties), Ok(date_ms)) =
                (total.parse(), penalties.parse(), date_ms.parse())
            else {
                continue;
            };
            ranking.insert(Record {
                total,
                penalties,
                date_ms,
            });
        }
        ranking
    }
}

fn is_valid(r: &Record) -> bool {
    r.total.is_finite() && r.total > 0.0 && r.date_ms.is_finite()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(total: f64, date_ms: f64) -> Record {
        Record {
            total,
            penalties: 0,
            date_ms,
        }
    }

    #[test]
    fn insert_keeps_ascending_order_and_returns_rank() {
        let mut r = Ranking::default();
        assert_eq!(r.insert(rec(10.0, 1.0)), Some(0));
        assert_eq!(r.insert(rec(8.0, 2.0)), Some(0));
        assert_eq!(r.insert(rec(12.0, 3.0)), Some(2));
        let totals: Vec<f64> = r.records().iter().map(|x| x.total).collect();
        assert_eq!(totals, vec![8.0, 10.0, 12.0]);
    }

    #[test]
    fn tie_goes_below_existing_record() {
        let mut r = Ranking::default();
        r.insert(rec(10.0, 1.0));
        assert_eq!(r.insert(rec(10.0, 2.0)), Some(1));
        assert_eq!(r.records()[0].date_ms, 1.0);
    }

    #[test]
    fn keeps_only_top_n() {
        let mut r = Ranking::default();
        for i in 0..RANKING_LEN {
            r.insert(rec(10.0 + i as f64, i as f64));
        }
        assert_eq!(r.insert(rec(99.0, 9.0)), None);
        assert_eq!(r.insert(rec(1.0, 9.0)), Some(0));
        assert_eq!(r.records().len(), RANKING_LEN);
        assert_eq!(
            r.records().last().unwrap().total,
            10.0 + (RANKING_LEN - 2) as f64
        );
    }

    #[test]
    fn rejects_invalid_records() {
        let mut r = Ranking::default();
        assert_eq!(r.insert(rec(f64::NAN, 0.0)), None);
        assert_eq!(r.insert(rec(0.0, 0.0)), None);
        assert_eq!(r.insert(rec(f64::INFINITY, 0.0)), None);
        assert!(r.records().is_empty());
    }

    #[test]
    fn serialize_round_trip() {
        let mut r = Ranking::default();
        r.insert(Record {
            total: 12.345678,
            penalties: 3,
            date_ms: 1_763_000_000_000.0,
        });
        r.insert(rec(9.5, 1.0));
        assert_eq!(Ranking::parse(&r.serialize()), r);
    }

    #[test]
    fn parse_skips_broken_lines_and_sorts() {
        let r = Ranking::parse("abc\n12,0,1\n\n9,1\n8,2,3,4\n7,1,5\n-1,0,0\n");
        let totals: Vec<f64> = r.records().iter().map(|x| x.total).collect();
        assert_eq!(totals, vec![7.0, 12.0]);
    }
}
