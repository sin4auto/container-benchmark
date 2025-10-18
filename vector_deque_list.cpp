// Path: vector_deque_list.cpp
// What: Benchmark for vector/deque/list + helpers (avg/variance)
// Why : Measure copy/read/statistics performance; keep code simple & clear
// RELEVANT FILES: Makefile, README.md, vector_deque_list.rs
#include <algorithm>    // std::generate, std::copy, std::copy_n, std::min
#include <array>        // std::array
#include <chrono>       // std::chrono::steady_clock, std::chrono::duration, std::chrono::duration_cast, std::chrono::time_point
#include <deque>        // std::deque
#include <iomanip>      // std::setprecision, std::fixed
#include <iostream>     // std::cout, std::cin, std::endl
#include <iterator>     // std::back_inserter, std::ostream_iterator
#include <list>         // std::list
#include <numeric>      // std::accumulate
#include <random>       // std::random_device, std::mt19937, std::uniform_int_distribution
#include <string>       // std::string
#include <type_traits>  // std::decay_t
#include <vector>       // std::vector

/**
 * @brief 実行時間を計測するクラス
 *
 * このクラスはRAIIパターンを使用して、スコープの開始から終了までの時間を自動的に計測します。
 * コンストラクタで計測を開始し、デストラクタでその結果を出力します。
 */
class CScopeProfiler final {
public:
    /**
     * @brief コンストラクタ - 時間計測を開始
     * @param mark 計測対象の名前
     */
    explicit CScopeProfiler(const std::string& mark)
        : m_start_time(std::chrono::steady_clock::now()), m_mark(mark) {}

    /**
     * @brief デストラクタ - 経過時間を計算して出力
     */
    ~CScopeProfiler() {
        auto elapsed_time = std::chrono::steady_clock::now() - m_start_time;
        double elapsed_time_milli = std::chrono::duration_cast<std::chrono::duration<double, std::milli>>(elapsed_time).count();
        std::cout << std::fixed << std::setprecision(2) << "実行時間 (" << m_mark << "): " << elapsed_time_milli << " ms " << std::endl;
    }

private:
    std::chrono::time_point<std::chrono::steady_clock> m_start_time; // 計測開始時刻
    std::string m_mark;  // 計測対象のマーク（ラベル）
};

// ===== ベンチマーク設定 =====
// ベンチマーク全体で使用する設定値を構造体に集約
struct BenchmarkConfig {
    using DataType = int;  // テスト対象のデータ型
    static constexpr size_t Size = 1000000;  // 配列の要素数
    static constexpr size_t ReadingRepeat = 10;  // 読み取り回数
    static constexpr size_t DisplayCount = 10;  // 表示する要素数
    static constexpr DataType MinRandomValue = -100; // 生成する乱数の最小値
    static constexpr DataType MaxRandomValue = 100;  // 生成する乱数の最大値
};

// ===== ヘルパー関数群 =====
/**
 * @brief ベンチマークの元データとなる固定長配列に乱数を格納する
 * @tparam T 格納するデータの型
 * @tparam N 配列の要素数
 * @param src_array データを格納する配列（出力）
 * @param min_val 乱数の最小値
 * @param max_val 乱数の最大値
 */
template<typename T, size_t N>
void generate_source_data(std::array<T, N>& src_array, T min_val, T max_val) {
    // ----- 乱数生成器の初期化 -----
    std::random_device seed_gen;  // 真の乱数生成器（シード用）
    std::mt19937 random_engine(seed_gen());  // メルセンヌツイスタ乱数エンジン
    std::uniform_int_distribution<T> dist(min_val, max_val);  // 一様分布
    // ----- 固定長配列に乱数値を格納 -----
    {
        CScopeProfiler profiler("配列生成_乱数");
        // 配列の各要素に乱数を格納
        std::generate(src_array.begin(), src_array.end(), [&]() { return dist(random_engine); });
    }
}

/**
 * @brief コンテナの先頭n個の要素を出力する関数
 *
 * @tparam Container コンテナの型
 * @param container 対象のコンテナ
 * @param n 表示する要素数
 * @param container_name コンテナの名前（表示用）
 */
template <typename Container>
void print_first_n_elements(const Container& container, size_t n, const std::string& container_name) {
    std::cout << container_name << ": ";
    // コンテナの要素数より多く表示しようとするのを防ぐ
    size_t elements_to_print = std::min(n, container.size());
    // STLアルゴリズムを使い、ループを明示的に書かずに出力する
    std::copy_n(container.begin(), elements_to_print, std::ostream_iterator<typename Container::value_type>(std::cout, " "));
    std::cout << std::endl;
}

/**
 * @brief コンテナの平均値を計算して返すヘルパー関数
 */
template<typename Container>
double average(const Container& container) {
    if (container.empty()) {
        return 0.0;
    }
    const double sum = std::accumulate(container.begin(), container.end(), 0.0);
    return sum / static_cast<double>(container.size());
}

/**
 * @brief コンテナの分散を計算して返すヘルパー関数
 *
 * Welford法（1パス・数値安定）で母分散を計算します。
 * 空コンテナは 0.0 を返します。サンプル分散が必要な場合は
 * `M2 / (n - 1)` に変更してください（n >= 2 前提）。
 */
template<typename Container>
double variance(const Container& container) {
    if (container.empty()) {
        return 0.0;
    }
    // Welford法: 1パスで母分散を算出
    double count = 0.0;
    double mean = 0.0;
    double m2 = 0.0;
    for (const auto& value : container) {
        const double x = static_cast<double>(value);
        count += 1.0;
        const double delta = x - mean;
        mean += delta / count;
        m2 += delta * (x - mean);
    }
    // count > 0 が保証されるのでそのまま割る（population variance）
    return m2 / count;
}

/**
 * @brief ベンチマークのメイン処理
 *
 * 各種コンテナに対して、データコピー、シーケンシャル読み取り、統計計算の性能を計測します。
 */
void run() {
    std::cout << "===== C++コンテナベンチマーク =====\n";
    std::cout << "要素数: " << BenchmarkConfig::Size << "\n\n";

    // 元データとなる固定長配列
    static std::array<BenchmarkConfig::DataType, BenchmarkConfig::Size> src_array;

    // 各種コンテナの定義
    std::vector<BenchmarkConfig::DataType> vector;  // 動的配列
    std::deque<BenchmarkConfig::DataType> deque;   // 両端キュー
    std::list<BenchmarkConfig::DataType> list;    // 双方向リスト

    // ----- 各ベンチマークの実行 -----

    // 元データの生成
    std::cout << "● 固定長配列（元データ）に乱数を格納\n";
    generate_source_data(src_array, BenchmarkConfig::MinRandomValue, BenchmarkConfig::MaxRandomValue);

    // データコピー性能の計測
    std::cout << "\n● データコピー性能\n";
    // vector（メモリ予約なし）へのコピー
    vector.clear();
    {
        CScopeProfiler profiler("vector_reserveなし");
        std::copy(src_array.begin(), src_array.end(), std::back_inserter(vector));
    }
    // vector（メモリ予約あり）へのコピー
    // これ以降のベンチマークで使用する`vec`はこの状態で初期化される
    vector.clear();
    vector.reserve(BenchmarkConfig::Size);
    {
        CScopeProfiler profiler("vector_reserveあり");
        std::copy(src_array.begin(), src_array.end(), std::back_inserter(vector));
    }
    // dequeへのコピー
    deque.clear();
    {
        CScopeProfiler profiler("deque");
        std::copy(src_array.begin(), src_array.end(), std::back_inserter(deque));
    }
    // listへのコピー
    list.clear();
    {
        CScopeProfiler profiler("list");
        std::copy(src_array.begin(), src_array.end(), std::back_inserter(list));
    }

    // シーケンシャル読み取り性能の計測
    std::cout << "\n● シーケンシャル読み取り性能 (" << BenchmarkConfig::ReadingRepeat << "回繰り返し)\n";
    // 各コンテナのデータを指定回数読み取るラムダ関数。
    // 注意:
    // 単純に要素を読み取るだけのループは、コンパイラの最適化（デッドコード削除）によって
    // 処理全体が削除されてしまう可能性があります。
    // これを防ぎ、確実に読み取り処理を実行させるため、読み取った値を`volatile`修飾子を
    // 付けた変数(`sink`)に代入しています。`volatile`変数へのアクセスは、コンパイラが
    // 無視できない副作用と見なすため、ループが維持され、純粋な読み取り性能を計測できます。
    auto read_container = [](const auto& container) {
        volatile typename std::decay_t<decltype(container)>::value_type sink{};
        for (size_t n = 0; n < BenchmarkConfig::ReadingRepeat; ++n) {
            for (const auto& element : container) {
                sink = element;
            }
        }
        (void)sink; // sinkが未使用であるというコンパイラ警告を抑制
    };
    // vectorのシーケンシャル読み取り
    {
        CScopeProfiler profiler("vector");
        read_container(vector);
    }
    // dequeのシーケンシャル読み取り
    {
        CScopeProfiler profiler("deque");
        read_container(deque);
    }
    // listのシーケンシャル読み取り
    {
        CScopeProfiler profiler("list");
        read_container(list);
    }

    // 先頭要素の表示
    std::cout << "\n● 先頭 " << BenchmarkConfig::DisplayCount << " 要素の確認\n";
    print_first_n_elements(vector, BenchmarkConfig::DisplayCount, "vector");
    print_first_n_elements(deque, BenchmarkConfig::DisplayCount, "deque");
    print_first_n_elements(list, BenchmarkConfig::DisplayCount, "list");

    // 統計計算（平均値）の性能を計測
    std::cout << "\n● 平均値計算の性能\n";
    {
        CScopeProfiler profiler("vector_平均値");
        const double avg_vec = average(vector);
        std::cout << std::fixed << std::setprecision(3) << "vectorの平均値: " << avg_vec << std::endl;
    }
    {
        CScopeProfiler profiler("deque_平均値");
        const double avg_deq = average(deque);
        std::cout << std::fixed << std::setprecision(3) << "dequeの平均値: " << avg_deq << std::endl;
    }
    {
        CScopeProfiler profiler("list_平均値");
        const double avg_lis = average(list);
        std::cout << std::fixed << std::setprecision(3) << "listの平均値: " << avg_lis << std::endl;
    }

    // 統計計算（分散）の性能を計測
    std::cout << "\n● 分散計算の性能\n";
    {
        CScopeProfiler profiler("vector_分散");
        const double var_vec = variance(vector);
        std::cout << std::fixed << std::setprecision(1) << "vectorの分散: " << var_vec << std::endl;
    }
    {
        CScopeProfiler profiler("deque_分散");
        const double var_deq = variance(deque);
        std::cout << std::fixed << std::setprecision(1) << "dequeの分散: " << var_deq << std::endl;
    }
    {
        CScopeProfiler profiler("list_分散");
        const double var_lis = variance(list);
        std::cout << std::fixed << std::setprecision(1) << "listの分散: " << var_lis << std::endl;
    }
    
    std::cout << "\n===== ベンチマーク終了 =====\n";
}

// ===== エントリポイント =====
int main() {
    CScopeProfiler profiler("全体処理");
    run();
    return 0;
}
