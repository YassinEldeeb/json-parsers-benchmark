# JSON Parsers Benchmark

| Benchmark | Time (in µs) |
|-----------|------|
| Go - JsonParser | 130.456 |
| Rust - simd_json | 138.97 |
| Rust - serde_json | 337.49 |
| Rust - serialize | 639.3 |
| Go - EncodingJson | 1189.205 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B130.456%2C138.97%2C337.49%2C639.3%2C1189.205%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Go%20-%20JsonParser%22%2C%22Rust%20-%20simd_json%22%2C%22Rust%20-%20serde_json%22%2C%22Rust%20-%20serialize%22%2C%22Go%20-%20EncodingJson%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

simd_json               time:   [138.97 µs 139.33 µs 139.97 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

serde_json              time:   [337.49 µs 338.06 µs 338.96 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

rustc-serialize         time:   [639.30 µs 641.65 µs 644.48 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe


```

Go Benchmark Output:

```shell
goos: linux
goarch: amd64
pkg: my-project
cpu: AMD EPYC 7763 64-Core Processor                
BenchmarkJsonParser   	    9174	    130456 ns/op
BenchmarkEncodingJson 	    1045	   1189205 ns/op
PASS
ok  	my-project	2.574s

```

</details>
