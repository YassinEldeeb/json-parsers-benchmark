# JSON Parsers Benchmark

| Benchmark | Time (in µs) |
|-----------|------|
| Go - JsonParser | 122.779 |
| Rust - simd_json | 124.9 |
| Rust - serde_json | 385.57 |
| Rust - serialize | 729.49 |
| Go - EncodingJson | 1025.872 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B122.779%2C124.9%2C385.57%2C729.49%2C1025.872%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Go%20-%20JsonParser%22%2C%22Rust%20-%20simd_json%22%2C%22Rust%20-%20serde_json%22%2C%22Rust%20-%20serialize%22%2C%22Go%20-%20EncodingJson%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

simd_json               time:   [124.90 µs 125.19 µs 125.53 µs]
                        change: [-0.1896% +0.0307% +0.2855%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

serde_json              time:   [385.57 µs 386.25 µs 386.94 µs]
                        change: [-0.4922% -0.2049% +0.1571%] (p = 0.24 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

rustc-serialize         time:   [729.49 µs 730.37 µs 731.26 µs]
                        change: [-0.5076% -0.3563% -0.2120%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild


```

Go Benchmark Output:

```shell
goos: darwin
goarch: arm64
pkg: my-project
BenchmarkJsonParser   	    9774	    122779 ns/op
BenchmarkEncodingJson 	    1231	   1025872 ns/op
PASS
ok  	my-project	3.765s

```

</details>
