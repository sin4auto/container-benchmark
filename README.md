# C++ / Rust コンテナベンチマーク

## 概要
- C++（`std::vector` / `std::deque` / `std::list`）と Rust（`Vec` / `VecDeque` / `LinkedList`）で同一ワークロードを実行し、主要コンテナの性能を比較。
- コピー性能・シーケンシャル読み取り・統計量（平均・分散）を測定して、振る舞いを定量的に把握します。
- 単一の `Makefile` で `main` バイナリを C++ / Rust いずれからでも生成可能。

## クイックスタート
1. **ツールチェーン**: C++17 対応コンパイラ（g++ / clang++）と Rust stable (`rustc`) を用意。
2. **取得**:
   ```bash
   git clone <repo-url>
   cd container-benchmark
   ```
3. **ベンチマーク実行**:
   ```bash
   make cpp-bench   # C++ 版ビルド & 実行
   make rust-bench  # Rust 版ビルド & 実行
   ```
   - 各ターゲットは `build/<lang>/main` を生成し、最後に `./main` を上書きします。
4. **クリーンアップ**:
   ```bash
   make clean
   ```

## リポジトリ構成
- `vector_deque_list.cpp` — C++17 実装。設定値は `BenchmarkConfig` に集約。
- `vector_deque_list.rs` — Rust 実装。`config` モジュール内で設定を管理し、`std::hint::black_box` で読み取り最適化を防止。
- `Makefile` — `cpp-bench` / `rust-bench` / `clean` ターゲットを定義。
- `build/` — ビルド結果を保持（初回実行時に作成）。

## ベンチマークシナリオ
- **コピー**: 共通データから各コンテナへ投入し、割り当て動作を比較。
- **シーケンシャル読み取り**: `READ_REPEAT_COUNT` 回ループしつつ `i64` に加算、`black_box` でコード除去を防止。
- **統計量**: 平均と分散を計算し、イテレータコストを評価。
- **表示**: 先頭 `DISPLAY_COUNT` 件と経過時間（ミリ秒）をすべて出力。

## カスタマイズ
- **データサイズ**: C++ は `BenchmarkConfig::Size`、Rust は `ELEMENT_COUNT` を調整。
- **乱数範囲**: C++ の `MinRandomValue` / `MaxRandomValue`、Rust の `RANDOM_MIN` / `RANDOM_MAX` を更新。
- **ビルドフラグ**: `Makefile` 内の `g++` / `rustc` 呼び出しを編集して最適化レベルや警告を変更。

## Make ターゲット
| ターゲット | 説明 |
| ---------- | ---- |
| `make` / `make all` | `cpp-bench` → `rust-bench` の順に実行 |
| `make cpp-bench` | C++ 版をビルド・実行（成果物: `build/cpp/main`）|
| `make rust-bench` | Rust 版をビルド・実行（成果物: `build/rust/main`）|
| `make clean` | `build/` と `./main` を削除 |

> **注意**: 両言語のバイナリが必要な場合は `build/cpp/main` と `build/rust/main` を直接利用してください。

## 結果の読み方
- ミリ秒が小さいほど高速。コンテナ間および言語間で比較。
- `Vec` / `vector` と `LinkedList` の差はキャッシュヒット率やポインタ追跡コストの影響が大きい。
- Rust : 読み取り結果が `0.00 ms` になる場合は最新版に更新し、`black_box` が効いているか確認。

## トラブルシュート
- **コンパイラが見つからない**: `g++` / `rustc` が PATH にあるか確認し、必要ならインストール。
- **メモリ不足**: `Size` / `ELEMENT_COUNT` を小さくする。
- **乱数を固定したい**: 両言語ともにシードを固定値へ変更。

## ライセンス
MIT License（詳細は `LICENSE.md` を参照）。

# C++ / Rust container-benchmark

## Overview
- Compare `std::vector`, `std::deque`, `std::list` with `Vec`, `VecDeque`, `LinkedList` under identical workloads.
- Measure copy throughput, sequential reads, and statistics to understand trade-offs.
- Ship a single Makefile so `main` can be produced from either toolchain with consistent flags.

## Quickstart
1. **Toolchains**: Install a C++17 compiler (g++/clang++) and Rust stable (`rustc`).
2. **Clone**:
   ```bash
   git clone <repo-url>
   cd container-benchmark
   ```
3. **Run benchmarks**:
   ```bash
   make cpp-bench
   make rust-bench
   ```
   - Each target writes to `build/<lang>/main` and copies the binary to `./main`.
4. **Cleanup**:
   ```bash
   make clean
   ```

## Repository Layout
- `vector_deque_list.cpp` — C++17 implementation with `BenchmarkConfig`.
- `vector_deque_list.rs` — Rust implementation; configuration lives in `config`; sequential reads call `std::hint::black_box`.
- `Makefile` — Defines `cpp-bench`, `rust-bench`, `clean`.
- `build/` — Generated on demand to store compiled binaries.

## Benchmark Scenarios
- **Copy** — Load each container from the shared dataset to highlight allocation behaviour.
- **Sequential read** — Iterate `READ_REPEAT_COUNT` times, summing into `i64` while preventing optimisation removal.
- **Statistics** — Compute mean and variance to expose traversal overhead.
- **Output** — Print elapsed milliseconds plus the first `DISPLAY_COUNT` elements for sanity checks.

## Customisation
- **Workload size** — Edit `BenchmarkConfig::Size` (C++) or `ELEMENT_COUNT` (Rust).
- **Random range** — Adjust `MinRandomValue`/`MaxRandomValue` or `RANDOM_MIN`/`RANDOM_MAX`.
- **Build flags** — Modify the `g++` / `rustc` commands in `Makefile` to experiment with optimisation levels or warnings.

## Make Targets
| Target | Description |
| ------ | ----------- |
| `make` / `make all` | Run `cpp-bench` then `rust-bench`. |
| `make cpp-bench` | Build & execute the C++ benchmark (`build/cpp/main`). |
| `make rust-bench` | Build & execute the Rust benchmark (`build/rust/main`). |
| `make clean` | Remove `build/` and `./main`. |

> **Note**: To keep both binaries, use the files under `build/cpp/` and `build/rust/` directly.

## Interpreting Results
- Lower milliseconds imply faster operations; compare across containers and languages.
- Large gaps between `Vec`/`vector` and `LinkedList` often stem from cache locality and pointer traversal.
- Rust : If you still observe `0.00 ms` sequential reads, rebuild to ensure the `black_box` updates are applied.

## Troubleshooting
- **Compiler missing** — Verify `g++` and `rustc` exist on the `PATH`; install via your package manager or `rustup`.
- **High memory usage** — Reduce the element counts.
- **Deterministic RNG** — Replace the dynamic seeds with fixed values in both sources.

## License
Distributed under the MIT License (`LICENSE.md`).
