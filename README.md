[日本語](#日本語) | [English](#english)

# C++コンテナベンチマーク (C++ Container Benchmark)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<br>

## <a name="日本語"></a> 🇯🇵 日本語

これは、C++の標準ライブラリで提供される主要なシーケンスコンテナ（`std::vector`, `std::deque`, `std::list`）の基本的な性能を比較するためのベンチマークプログラムです。

コンテナの特性がパフォーマンスにどのように影響するかを、実際の実行時間を通じて直感的に理解することを目的としています。

### 主な特徴

このベンチマークでは、以下の項目について各コンテナの性能を計測します。

*   **データコピー性能**:
    `std::array`から各コンテナへのデータコピーにかかる時間。
    *   `std::vector`については、`reserve()`を呼び出して事前にメモリを確保した場合としない場合の比較も行います。
*   **シーケンシャルアクセス性能**:
    コンテナの全要素を順番に読み取る際の速度。
*   **統計計算性能**:
    全要素の平均値および分散を計算する時間。

### 動作環境

*   C++17 以上をサポートするC++コンパイラ (g++, Clang など)

### 使用方法

#### 1. リポジトリのクローン
```bash
git clone https://github.com/sin4auto/cpp-container-benchmark.git
cd cpp-container-benchmark
```

#### 2. 動作確認
オンラインコンパイラ「[Wandbox](https://wandbox.org/)」の C++ (gcc) で動作確認しています。
ローカル環境でコンパイルする場合は、以下のようなコマンドを使用してください。

```bash
g++ -o benchmark main.cpp -std=c++17 -O2
```

### 実行結果の例
実行すると、各処理の実行時間がミリ秒単位で表示されます。
```text
===== C++コンテナベンチマーク =====
要素数: 1000000

● 固定長配列（元データ）に乱数を格納
実行時間 (配列生成_乱数): 17.52 ms 

● データコピー性能
実行時間 (vector_reserveなし): 4.10 ms 
実行時間 (vector_reserveあり): 2.45 ms 
実行時間 (deque): 15.61 ms 
実行時間 (list): 23.88 ms 

● シーケンシャル読み取り性能 (10回繰り返し)
実行時間 (vector): 18.23 ms 
実行時間 (deque): 48.95 ms 
実行時間 (list): 145.77 ms 

● 先頭 10 要素の確認
vector: 52 -92 23 85 -36 99 7 -15 67 -4 
deque: 52 -92 23 85 -36 99 7 -15 67 -4 
list: 52 -92 23 85 -36 99 7 -15 67 -4 

● 平均値計算の性能
実行時間 (vector_平均値): 1.68 ms
vectorの平均値: -0.015
実行時間 (deque_平均値): 5.11 ms
dequeの平均値: -0.015
実行時間 (list_平均値): 15.20 ms
listの平均値: -0.015

● 分散計算の性能
実行時間 (vector_分散): 4.25 ms
vectorの分散: 3399.7
実行時間 (deque_分散): 10.33 ms
dequeの分散: 3399.7
実行時間 (list_分散): 29.81 ms
listの分散: 3399.7
```

### ベンチマークのカスタマイズ
`main.cpp`内の`BenchmarkConfig`構造体の値を変更することで、ベンチマークの条件を簡単にカスタマイズできます。
```cpp
struct BenchmarkConfig {
    using DataType = int;  // データ型
    static constexpr size_t Size = 1000000;  // 要素数
    static constexpr size_t ReadingRepeat = 10;  // 読み取り繰り返し回数
    static constexpr size_t DisplayCount = 10;  // 表示する要素数
    static constexpr DataType MinRandomValue = -100; // 乱数の最小値
    static constexpr DataType MaxRandomValue = 100;  // 乱数の最大値
};
```

---
<br>

## <a name="english"></a> 🇬🇧 English

This is a benchmark program to compare the basic performance of major C++ Standard Library sequence containers: `std::vector`, `std::deque`, and `std::list`.

The purpose is to provide an intuitive understanding of how the characteristics of each container affect performance, demonstrated through actual execution times.

### Key Features

This benchmark measures the performance of each container on the following tasks:

*   **Data Copy Performance**:
    The time it takes to copy data from `std::array` to each container.
    *   For `std::vector`, it also compares the performance with and without pre-allocating memory using `reserve()`.
*   **Sequential Access Performance**:
    The speed of reading all elements in the container sequentially.
*   **Statistical Calculation Performance**:
    The time required to calculate the average and variance of all elements.

### System Requirements

*   A C++ compiler that supports C++17 or later (e.g., g++, Clang).

### Usage

#### 1. Clone the Repository
```bash
git clone https://github.com/sin4auto/cpp-container-benchmark.git
cd cpp-container-benchmark
```

#### 2. Tested Environment
The operation has been confirmed using C++ (gcc) on the online compiler "[Wandbox](https://wandbox.org/)".
To compile in a local environment, use a command like the following:
```bash
g++ -o benchmark main.cpp -std=c++17 -O2
```

### Example of Execution Results
When executed, the program displays the execution time for each process in milliseconds.
(The output is the same as the Japanese example above)

### Customizing the Benchmark
You can easily customize the benchmark conditions by modifying the values in the `BenchmarkConfig` struct within `main.cpp`.```cpp
struct BenchmarkConfig {
    using DataType = int;  // The data type to be tested
    static constexpr size_t Size = 1000000;  // Number of elements in the array
    static constexpr size_t ReadingRepeat = 10;  // Number of times to repeat the read test
    static constexpr size_t DisplayCount = 10;  // Number of elements to display
    static constexpr DataType MinRandomValue = -100; // Minimum value for random numbers
    static constexpr DataType MaxRandomValue = 100;  // Maximum value for random numbers
};
```

---

## License
This project is released under the MIT License. See the `LICENSE` file for details.

## Author
sin4auto
