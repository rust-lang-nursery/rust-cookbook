# 並列性

| Recipe | Crates | Categories |
|--------|--------|------------|
| [短命のスレッドの生成][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [グローバルな可変状態を維持する][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [* .isoファイルのSHA1合計を同時に計算する][ex-threadpool-walk]  | [![threadpool-badge]][threadpool] [![walkdir-badge]][walkdir] [![num_cpus-badge]][num_cpus] [![ring-badge]][ring] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [フラクタル図形をスレッドプールに描画する][ex-threadpool-fractal] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |
| [配列の要素を並列に変換する][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [コレクションの要素の一部または全部が特定の述語に一致するかどうかを並列にテストする][ex-rayon-any-all] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [与えられた述語を使って並列にアイテムを探す][ex-rayon-parallel-search] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [並列にベクタをソートする][ex-rayon-parallel-sort] | [![rayon-badge]][rayon] [![rand-badge]][rand] | [![cat-concurrency-badge]][cat-concurrency] |
| [並列なMap-reduce][ex-rayon-map-reduce] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [jpgのサムネイルを並列に生成する][ex-rayon-thumbnails] | [![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |


[ex-crossbeam-spawn]: concurrency/threads.html#spawn-a-short-lived-thread
[ex-global-mut-state]: concurrency/threads.html#maintain-global-mutable-state
[ex-threadpool-walk]: concurrency/threads.html#calculate-sha1-sum-of-iso-files-concurrently
[ex-threadpool-fractal]: concurrency/threads.html#draw-fractal-dispatching-work-to-a-thread-pool
[ex-rayon-iter-mut]: concurrency/parallel.html#mutate-the-elements-of-an-array-in-parallel
[ex-rayon-any-all]: concurrency/parallel.html#test-in-parallel-if-any-or-all-elements-of-a-collection-match-a-given-predicate
[ex-rayon-parallel-search]: concurrency/parallel.html#search-items-using-given-predicate-in-parallel
[ex-rayon-parallel-sort]: concurrency/parallel.html#sort-a-vector-in-parallel
[ex-rayon-map-reduce]: concurrency/parallel.html#map-reduce-in-parallel
[ex-rayon-thumbnails]: concurrency/parallel.html#generate-jpg-thumbnails-in-parallel

{{#include links.md}}
