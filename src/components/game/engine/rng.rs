//! xorshift64* による小さな乱数生成器。seed を固定すればテストで再現できる。

#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // 状態が 0 だと 0 を出し続けるので、別の値に置き換える。
        Self(if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        })
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// [0, 1) の一様乱数。
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// [lo, hi) の一様乱数。
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }

    /// [lo, hi] の整数一様乱数。
    pub fn range_usize(&mut self, lo: usize, hi: usize) -> usize {
        lo + self.below(hi - lo + 1)
    }

    /// [0, n) の整数一様乱数。`n == 0` のときは 0。
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_f64() * n as f64) as usize % n
        }
    }

    /// 確率 `p` で true。
    pub fn chance(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// スライスから 1 つ選ぶ。空なら `None`。
    pub fn pick<T: Copy>(&mut self, items: &[T]) -> Option<T> {
        (!items.is_empty()).then(|| items[self.below(items.len())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_in_range() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..10_000 {
            let v = a.next_f64();
            assert_eq!(v, b.next_f64());
            assert!((0.0..1.0).contains(&v));
            let n = a.range_usize(2, 5);
            assert_eq!(n, b.range_usize(2, 5));
            assert!((2..=5).contains(&n));
        }
        let mut z = Rng::new(0);
        assert_ne!(z.next_u64(), z.next_u64());
    }
}
