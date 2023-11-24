# JSON Parsers benchmark Results

| Benchmark | Time (in µs) |
|-----------|------|
| Go - jsonparser | 122.774 |
| Rust - simd_json | 125.16 |
| Rust - serde_json | 385.95 |
| Rust - serialize | 728.36 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B122.774%2C125.16%2C385.95%2C728.36%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Go%20-%20jsonparser%22%2C%22Rust%20-%20simd_json%22%2C%22Rust%20-%20serde_json%22%2C%22Rust%20-%20serialize%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

simd_json               time:   [125.16 µs 125.29 µs 125.43 µs]
                        change: [-0.8637% -0.7362% -0.6081%] (p = 0.00 < 0.05)
                        Change within noise threshold.

serde_json              time:   [385.95 µs 386.34 µs 386.74 µs]
                        change: [-0.2240% -0.0778% +0.0630%] (p = 0.28 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

rustc-serialize         time:   [728.36 µs 729.14 µs 729.96 µs]
                        change: [-0.0132% +0.1247% +0.2730%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe


```

Go Benchmark Output:

```shell
goos: darwin
goarch: arm64
pkg: my-project
BenchmarkJsonParser-10    	    9771	    122774 ns/op
PASS
ok  	my-project	2.262s

```

</details>
