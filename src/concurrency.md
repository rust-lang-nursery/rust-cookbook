# Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Create a parallel data pipeline][ex-crossbeam-pipeline] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Pass data between two threads][ex-crossbeam-spsc] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Maintain global mutable state][ex-global-mut-state] | [![lazy_static-badge]][lazy_static] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Calculate SHA1 sum of *.iso files concurrently][ex-threadpool-walk]  | [![threadpool-badge]][threadpool] [![walkdir-badge]][walkdir] [![std-badge]][std] [![ring-badge]][ring] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [Sum the size of every file in a directory][ex-threadpool-size] | [![threadpool-badge]][threadpool] [![walkdir-badge]][walkdir] [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [Draw fractal dispatching work to a thread pool][ex-threadpool-fractal] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![std-badge]][std] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |
| [Lock-free counter with `AtomicUsize`][ex-sync-atomic] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] |
| [Guard compound state with `Arc<Mutex<T>>`][ex-sync-arc-mutex] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] |
| [Concurrent reads with `Arc<RwLock<T>>`][ex-sync-rwlock] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] |
| [Communicate between threads with `mpsc` channels][ex-sync-mpsc] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] |
| [Coordinate thread phases with `Barrier`][ex-sync-barrier] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] |
| [Signal a waiting thread with `Condvar`][ex-sync-condvar] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] |
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Test in parallel if any or all elements of a collection match a given predicate][ex-rayon-any-all] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Search items using given predicate in parallel][ex-rayon-parallel-search] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Sort a vector in parallel][ex-rayon-parallel-sort] | [![rayon-badge]][rayon] [![rand-badge]][rand] | [![cat-concurrency-badge]][cat-concurrency] |
| [Map-reduce in parallel][ex-rayon-map-reduce] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Generate jpg thumbnails in parallel][ex-rayon-thumbnails] | [![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [Actor Pattern with Tokio][ex-actor-pattern] | [![tokio-badge]][tokio] | [![cat-concurrency-badge]][cat-concurrency] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Custom Future (Pin, Waker, Poll)][ex-custom-future] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] [![cat-rust-patterns-badge]][cat-rust-patterns] |


[ex-crossbeam-spawn]: concurrency/threads.html#spawn-a-short-lived-thread
[ex-crossbeam-pipeline]: concurrency/threads.html#create-a-parallel-pipeline
[ex-crossbeam-spsc]: concurrency/threads.html#pass-data-between-two-threads
[ex-global-mut-state]: concurrency/threads.html#maintain-global-mutable-state
[ex-threadpool-walk]: concurrency/threads.html#calculate-sha256-sum-of-iso-files-concurrently
[ex-threadpool-size]: concurrency/threads.html#sum-the-size-of-every-file-in-a-directory
[ex-threadpool-fractal]: concurrency/threads.html#draw-fractal-dispatching-work-to-a-thread-pool
[ex-sync-arc-mutex]: concurrency/sync.html#guard-compound-state-with-arcmutext
[ex-sync-atomic]: concurrency/sync.html#lock-free-counter-with-atomicusize
[ex-sync-barrier]: concurrency/sync.html#coordinate-thread-phases-with-barrier
[ex-sync-condvar]: concurrency/sync.html#signal-a-waiting-thread-with-condvar
[ex-sync-mpsc]: concurrency/sync.html#communicate-between-threads-with-mpsc-channels
[ex-sync-rwlock]: concurrency/sync.html#concurrent-reads-with-arcrwlockt
[ex-rayon-iter-mut]: concurrency/parallel.html#mutate-the-elements-of-an-array-in-parallel
[ex-rayon-any-all]: concurrency/parallel.html#test-in-parallel-if-any-or-all-elements-of-a-collection-match-a-given-predicate
[ex-rayon-parallel-search]: concurrency/parallel.html#search-items-using-given-predicate-in-parallel
[ex-rayon-parallel-sort]: concurrency/parallel.html#sort-a-vector-in-parallel
[ex-rayon-map-reduce]: concurrency/parallel.html#map-reduce-in-parallel
[ex-rayon-thumbnails]: concurrency/parallel.html#generate-jpg-thumbnails-in-parallel
[ex-actor-pattern]: concurrency/actor.html#actor-pattern-with-tokio-handleactormessage
[ex-custom-future]: concurrency/custom_future.html#implement-a-custom-future-pin-waker-poll

{{#include links.md}}
