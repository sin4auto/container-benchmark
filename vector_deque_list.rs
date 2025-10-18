//! # Rust コンテナ・ベンチマーク
//!
//! C++版の簡易ベンチマークを **標準ライブラリのみ** で Rust に移植した実装。
//!
//! - 対象コンテナ: `Vec`, `VecDeque`, `LinkedList`
//! - 計測内容: データコピー性能・シーケンシャル読み取り・平均/分散（母分散）
//! - 設計方針: イテレータ中心／各ベンチケースは新規コンテナで独立測定／RAII による計測
//! - 外部クレート: 不要（擬似乱数は MT19937 の簡易実装）
//!
//! 実行は単一ファイル `main.rs` で可能。

use std::collections::{LinkedList, VecDeque};
use std::fmt::{Display, Write};
use std::hint::black_box;
use std::time::Instant;

/// ベンチマークの設定値をまとめたモジュール。
mod config {
    /// ベンチマークで扱うデータ型。
    pub type DataType = i32;
    /// 元データの要素数。
    pub const ELEMENT_COUNT: usize = 1_000_000;
    /// シーケンシャル読み取りの繰り返し回数。
    pub const READ_REPEAT_COUNT: usize = 10;
    /// 先頭表示件数。
    pub const DISPLAY_COUNT: usize = 10;
    /// 乱数の最小値（含む）。
    pub const RANDOM_MIN: DataType = -100;
    /// 乱数の最大値（含む）。
    pub const RANDOM_MAX: DataType = 100;
}

/// スコープ生存期間で経過時間を測定し、ドロップ時に表示する簡易プロファイラ。
///
/// # 使い方
/// スコープ先頭でインスタンスを生成すると、スコープ終了時（`Drop`）に経過時間が出力される。
#[must_use]
struct ScopeProfiler {
    /// 計測対象のラベル。
    mark: String,
    /// 計測開始時刻。
    start: Instant,
}

impl ScopeProfiler {
    /// 指定ラベルで計測を開始する。
    pub fn new(mark: impl Into<String>) -> Self {
        Self { mark: mark.into(), start: Instant::now() }
    }
}

impl Drop for ScopeProfiler {
    fn drop(&mut self) {
        let ms = self.start.elapsed().as_secs_f64() * 1000.0;
        println!("実行時間 ({}): {:.2} ms", self.mark, ms);
    }
}

/// MT19937 による擬似乱数生成器（外部依存なし）。
///
/// 32bit のメルセンヌツイスタをそのまま移植し、整数範囲の一様乱数を提供する。
struct Mt19937 {
    state: [u32; Self::N],
    index: usize,
}

impl Mt19937 {
    const N: usize = 624;
    const M: usize = 397;
    const MATRIX_A: u32 = 0x9908_B0DF;
    const UPPER_MASK: u32 = 0x8000_0000;
    const LOWER_MASK: u32 = 0x7FFF_FFFF;

    /// シードから生成器を初期化する。
    pub fn new(seed: u64) -> Self {
        let mut mt = Self { state: [0; Self::N], index: Self::N };
        mt.reseed(seed);
        mt
    }

    /// シードを再設定する。
    fn reseed(&mut self, seed: u64) {
        let seed32 = (seed ^ (seed >> 32)) as u32;
        self.state[0] = seed32;
        for i in 1..Self::N {
            let prev = self.state[i - 1];
            self.state[i] = 1_812_433_253u32.wrapping_mul(prev ^ (prev >> 30)).wrapping_add(i as u32);
        }
        self.index = Self::N;
    }

    /// 内部状態を更新する。
    fn twist(&mut self) {
        for i in 0..Self::N {
            let x = (self.state[i] & Self::UPPER_MASK)
                | (self.state[(i + 1) % Self::N] & Self::LOWER_MASK);
            let mut xa = x >> 1;
            if x & 1 != 0 {
                xa ^= Self::MATRIX_A;
            }
            self.state[i] = self.state[(i + Self::M) % Self::N] ^ xa;
        }
        self.index = 0;
    }

    /// 次の 32bit 値を生成する。
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        if self.index >= Self::N {
            self.twist();
        }
        let mut y = self.state[self.index];
        self.index += 1;

        // テンパリング（符号化を安定化）
        y ^= y >> 11;
        y ^= (y << 7) & 0x9D2C_5680;
        y ^= (y << 15) & 0xEFC6_0000;
        y ^= y >> 18;
        y
    }

    /// 64bit 値を生成する（32bit の 2 回合成）。
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | self.next_u32() as u64
    }

    /// `[min, max]` の一様整数を生成する。
    #[inline]
    pub fn next_i32_range(&mut self, min: i32, max: i32) -> i32 {
        debug_assert!(min <= max);
        let span = (max as i64 - min as i64 + 1) as u64;
        debug_assert!(span > 0);
        let zone = u64::MAX - (u64::MAX % span);
        loop {
            let value = self.next_u64();
            if value < zone {
                let offset = (value % span) as i64;
                let result = min as i64 + offset;
                debug_assert!(((i32::MIN as i64)..=(i32::MAX as i64)).contains(&result));
                return result as i32;
            }
        }
    }
}

/// 指定サイズの乱数ベクタを生成する。
///
/// # 引数
/// - `size`: 生成する要素数
/// - `min_v`, `max_v`: 乱数範囲（両端含む）
///
/// # 戻り値
/// 乱数で埋めた `Vec<i32>`。
fn generate_source(size: usize, min_v: i32, max_v: i32) -> Vec<i32> {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let _profiler = ScopeProfiler::new("乱数配列生成");
    let mut rng = Mt19937::new(seed);

    (0..size).map(|_| rng.next_i32_range(min_v, max_v)).collect()
}

/// 先頭 `n` 要素を 1 行で出力する（スペース区切り）。
///
/// # 引数
/// - `name`: 見出し名（コンテナ名）
/// - `iter`: 対象イテレータ
/// - `n`: 表示件数
fn print_first_n<I, T>(name: &str, iter: I, n: usize)
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    let mut buf = String::new();
    iter.into_iter().take(n).for_each(|x| {
        let _ = write!(buf, "{} ", x);
    });
    println!("{}: {}", name, buf.trim_end());
}

/// 平均値（母平均）を求める。空入力時は `0.0`。
///
/// # 計算量
/// O(N) で 1 パス。
fn mean<I>(iter: I) -> f64
where
    I: IntoIterator<Item = i32>,
{
    let (sum, cnt) = iter.into_iter().fold((0f64, 0f64), |(s, c), v| (s + v as f64, c + 1.0));
    if cnt == 0.0 { 0.0 } else { sum / cnt }
}

/// 分散（母分散）を 1 パスで求める（Welford 法）。空入力時は `0.0`。
///
/// # 計算量
/// O(N) で 1 パス、数値安定性も高い。
fn variance<I>(iter: I) -> f64
where
    I: IntoIterator<Item = i32>,
{
    let mut n = 0f64;
    let mut mean = 0f64;
    let mut m2 = 0f64;
    for x in iter {
        n += 1.0;
        let dx = x as f64 - mean;
        mean += dx / n;
        m2 += dx * (x as f64 - mean);
    }
    if n == 0.0 { 0.0 } else { m2 / n }
}

/// イテレータのシーケンシャル読み取りを `repeats` 回行い、合計値を返す。
///
/// 最適化抑止は行わず、素直に加算するのみ。
fn read_sequential<I>(it: I, repeats: usize) -> i64
where
    I: Clone + IntoIterator<Item = i32>,
{
    std::iter::repeat(())
        .take(repeats)
        .map(|_| it.clone().into_iter().map(|v| v as i64).sum::<i64>())
        .sum()
}

/// ベンチマーク本体。各処理を独立に測定・出力する。
fn run() {
    use config::*;

    println!("===== Rust コンテナ・ベンチマーク =====");
    println!("要素数: {}\n", ELEMENT_COUNT);

    // 元データ生成
    println!("● 元データ生成");
    let src = generate_source(ELEMENT_COUNT, RANDOM_MIN, RANDOM_MAX);

    // --- データコピー性能（ケースごとに新しいコンテナを生成） ---
    println!("\n● データコピー性能");
    {
        let _profiler = ScopeProfiler::new("Vec_reserveなし");
        let mut v: Vec<DataType> = Vec::new();
        v.extend_from_slice(&src);
    }
    {
        let _profiler = ScopeProfiler::new("Vec_reserveあり");
        let mut v: Vec<DataType> = Vec::with_capacity(ELEMENT_COUNT);
        v.extend_from_slice(&src);
    }
    {
        let _profiler = ScopeProfiler::new("VecDeque");
        let mut d: VecDeque<DataType> = VecDeque::with_capacity(ELEMENT_COUNT);
        d.extend(src.iter().copied());
    }
    {
        let _profiler = ScopeProfiler::new("LinkedList");
        let mut l: LinkedList<DataType> = LinkedList::new();
        l.extend(src.iter().copied());
    }

    // --- 以降の処理（読み取り・統計）用に、計測対象外でコンテナを準備 ---
    let vec_main: Vec<DataType> = src.clone();
    let deq_main: VecDeque<DataType> = src.iter().copied().collect();
    let lis_main: LinkedList<DataType> = src.iter().copied().collect();

    // --- シーケンシャル読み取り ---
    println!("\n● シーケンシャル読み取り性能 ({}回繰り返し)", READ_REPEAT_COUNT);
    {
        let _profiler = ScopeProfiler::new("Vec");
        let sum = read_sequential(vec_main.iter().copied(), READ_REPEAT_COUNT);
        black_box(sum);
    }
    {
        let _profiler = ScopeProfiler::new("VecDeque");
        let sum = read_sequential(deq_main.iter().copied(), READ_REPEAT_COUNT);
        black_box(sum);
    }
    {
        let _profiler = ScopeProfiler::new("LinkedList");
        let sum = read_sequential(lis_main.iter().copied(), READ_REPEAT_COUNT);
        black_box(sum);
    }

    // --- 先頭確認 ---
    println!("\n● 先頭 {} 要素の確認", DISPLAY_COUNT);
    print_first_n("Vec", vec_main.iter().copied(), DISPLAY_COUNT);
    print_first_n("VecDeque", deq_main.iter().copied(), DISPLAY_COUNT);
    print_first_n("LinkedList", lis_main.iter().copied(), DISPLAY_COUNT);

    // --- 平均 ---
    println!("\n● 平均値計算の性能");
    {
        let _profiler = ScopeProfiler::new("Vec_平均値");
        println!("Vecの平均値: {:.3}", mean(vec_main.iter().copied()));
    }
    {
        let _profiler = ScopeProfiler::new("VecDeque_平均値");
        println!("VecDequeの平均値: {:.3}", mean(deq_main.iter().copied()));
    }
    {
        let _profiler = ScopeProfiler::new("LinkedList_平均値");
        println!("LinkedListの平均値: {:.3}", mean(lis_main.iter().copied()));
    }

    // --- 分散 ---
    println!("\n● 分散計算の性能");
    {
        let _profiler = ScopeProfiler::new("Vec_分散");
        println!("Vecの分散: {:.1}", variance(vec_main.iter().copied()));
    }
    {
        let _profiler = ScopeProfiler::new("VecDeque_分散");
        println!("VecDequeの分散: {:.1}", variance(deq_main.iter().copied()));
    }
    {
        let _profiler = ScopeProfiler::new("LinkedList_分散");
        println!("LinkedListの分散: {:.1}", variance(lis_main.iter().copied()));
    }

    println!("\n===== ベンチマーク終了 =====");
}

/// エントリポイント。
fn main() {
    let _profiler = ScopeProfiler::new("全体処理");
    run();
}
