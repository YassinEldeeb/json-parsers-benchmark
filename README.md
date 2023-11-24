# JSON Parsers Benchmark

| Benchmark | Time (in µs) |
|-----------|------|
| Go - jsonparser | 112.498 |
| Rust - simd_json | 136.41 |
| Rust - serde_json | 322.01 |
| Rust - serialize | 633.43 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B112.498%2C136.41%2C322.01%2C633.43%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Go%20-%20jsonparser%22%2C%22Rust%20-%20simd_json%22%2C%22Rust%20-%20serde_json%22%2C%22Rust%20-%20serialize%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

simd_json               time:   [136.41 µs 137.46 µs 138.76 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

serde_json              time:   [322.01 µs 323.58 µs 325.13 µs]

rustc-serialize         time:   [633.43 µs 636.14 µs 638.69 µs]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) low mild
  10 (10.00%) high mild
  1 (1.00%) high severe


```

Go Benchmark Output:

```shell
goos: linux
goarch: amd64
pkg: my-project
cpu: AMD EPYC 7763 64-Core Processor                
BenchmarkJsonParser-4   	   10000	    112498 ns/op
PASS
ok  	my-project	1.141s

```

</details>
