# C++コンテナベンチマーク (C++ Container Benchmark)

これは、C++の標準ライブラリで提供される主要なシーケンスコンテナ（`std::vector`, `std::deque`, `std::list`）の基本的な性能を比較するためのベンチマークプログラムです。

コンテナの特性がパフォーマンスにどのように影響するかを、実際の実行時間を通じて直感的に理解することを目的としています。


## 主な特徴

このベンチマークでは、以下の項目について各コンテナの性能を計測します。


データコピー性能: `std::array`から各コンテナへのデータコピーにかかる時間

`std::vector`については、`reserve()`を呼び出して事前にメモリを確保した場合としない場合の比較も行います。

シーケンシャルアクセス性能: コンテナの全要素を順番に読み取る際の速度

統計計算性能: 全要素の平均値および分散を計算する時間



## 動作環境

C++17 以上をサポートするC++コンパイラ (g++, Clang など)


## 使用方法

### 1. リポジトリのクローン

```bash

git clone https://github.com/sin4auto/cpp-container-benchmark.git

cd cpp-container-benchmark

```



### 2. 動作確認

オンラインコンパイラ "https://wandbox.org/" のC++で動作確認しています。


## 実行結果の例

実行すると、各処理の実行時間がミリ秒単位で表示されます。

```text

===== C++コンテナベンチマーク =====

要素数: 1000000



● 固定長配列（元データ）に乱数を格納

実行時間 (配列生成\_乱数): 17.52 ms 



● データコピー性能

実行時間 (vector\_reserveなし): 4.10 ms 

実行時間 (vector\_reserveあり): 2.45 ms 

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

実行時間 (vector\_平均値): 1.68 ms

vectorの平均値: -0.015

実行時間 (deque\_平均値): 5.11 ms

dequeの平均値: -0.015

実行時間 (list\_平均値): 15.20 ms

listの平均値: -0.015



● 分散計算の性能

実行時間 (vector\_分散): 4.25 ms

vectorの分散: 3399.7

実行時間 (deque\_分散): 10.33 ms

dequeの分散: 3399.7

実行時間 (list\_分散): 29.81 ms

listの分散: 3399.7

```



## ベンチマークのカスタマイズ

`main.cpp`内の`BenchmarkConfig`構造体の値を変更することで、ベンチマークの条件を簡単にカスタマイズできます。



```cpp

struct BenchmarkConfig {

&nbsp;   using DataType = int;  // テスト対象のデータ型

&nbsp;   static constexpr size\_t Size = 1000000;  // 配列の要素数

&nbsp;   static constexpr size\_t ReadingRepeat = 10;  // 読み取り回数

&nbsp;   static constexpr size\_t DisplayCount = 10;  // 表示する要素数

&nbsp;   static constexpr DataType MinRandomValue = -100; // 生成する乱数の最小値

&nbsp;   static constexpr DataType MaxRandomValue = 100;  // 生成する乱数の最大値

};

```


## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細については `LICENSE` ファイルをご覧ください。


## 作者

sin4auto
