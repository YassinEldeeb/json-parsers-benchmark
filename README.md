# JSON Parsers Benchmark

| Benchmark | Time (in µs) |
|-----------|------|
| Rust - simd_json | 140.77 |
| Rust - serde_json | 336.47 |
| Rust - serialize | 637.91 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B140.77%2C336.47%2C637.91%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Rust%20-%20simd_json%22%2C%22Rust%20-%20serde_json%22%2C%22Rust%20-%20serialize%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

simd_json               time:   [140.77 µs 140.97 µs 141.32 µs]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

serde_json              time:   [336.47 µs 336.73 µs 337.07 µs]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  5 (5.00%) high severe

rustc-serialize         time:   [637.91 µs 641.24 µs 646.53 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe


```

Go Benchmark Output:

```shell
goos: linux
goarch: amd64
pkg: my-project
cpu: AMD EPYC 7763 64-Core Processor                
BenchmarkJsonParser   	    9142	    130601 ns/op
BenchmarkEncodingJson 	    1017	   1185459 ns/op
PASS
ok  	my-project	2.537s

```

</details>
