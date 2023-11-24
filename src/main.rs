use regex::Regex;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::{thread, time};
use urlencoding::encode;

fn main() {
    let rust_output = get_command_output("cargo", vec!["bench"], None);

    // cooldown
    thread::sleep(time::Duration::from_secs(10));
    let go_output = get_command_output("go", vec!["test", "-bench=."], Some("./golang"));

    let mut benchmarks = extract_benchmarks(&rust_output, &go_output);
    benchmarks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let chart_url = generate_quickchart_url(&benchmarks);
    let readme_content = generate_readme(&benchmarks, &chart_url, &rust_output, &go_output);
    write_to_file("README.md", &readme_content).expect("Failed to write README.md");
}

fn get_command_output(command: &str, args: Vec<&str>, dir: Option<&str>) -> String {
    let mut command = Command::new(command);
    command.args(args);
    if let Some(directory) = dir {
        command.current_dir(directory);
    }

    if command.get_program() == "go" {
        command.env("GOMAXPROCS", "1"); // use only 1 thread like rust's criterion
    }
    let output = command.output().expect("Failed to execute command").stdout;
    String::from_utf8_lossy(&output).into_owned()
}

fn extract_benchmarks(rust_output: &str, go_output: &str) -> Vec<(String, f64)> {
    let mut benchmarks = Vec::new();
    extract_time_from_rust_output(rust_output, &mut benchmarks);
    if let Some(time) = extract_time_from_go_output(go_output) {
        let time = convert_nanoseconds_to_microseconds(&time);
        benchmarks.push(("Go - jsonparser".to_string(), time));
    }
    benchmarks
}

fn generate_readme(
    benchmarks: &[(String, f64)],
    chart_url: &str,
    rust_output: &str,
    go_output: &str,
) -> String {
    format!(
        "# JSON Parsers Benchmark\n\n\
        | Benchmark | Time (in µs) |\n\
        |-----------|------|\n\
        {}\n\n\
        ![Benchmark Bar Chart]({})\n\n\
        <details><summary>Click to expand logs</summary>\n\n\
        Rust Benchmark Output:\n\n\
        ```shell\n\
        {}\n\
        ```\n\n\
        Go Benchmark Output:\n\n\
        ```shell\n\
        {}\n\
        ```\n\n\
        </details>\n",
        benchmarks
            .iter()
            .map(|(benchmark, time)| format!("| {} | {} |", benchmark, time))
            .collect::<Vec<String>>()
            .join("\n"),
        chart_url,
        rust_output,
        go_output
    )
}

fn write_to_file(file_name: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())
}

fn generate_quickchart_url(benchmarks: &[(String, f64)]) -> String {
    // Collect the labels and data from the benchmarks
    let labels: Vec<String> = benchmarks.iter().map(|(name, _)| name.clone()).collect();
    let data: Vec<f64> = benchmarks.iter().map(|&(_, time)| time).collect();

    // Serialize the chart data to JSON
    let chart_data = json!({
        "type": "bar",
        "data": {
            "labels": labels,
            "datasets": [{
                "label": "Benchmark Results",
                "data": data
            }]
        },
        "options": {
            "title": {
                "display": true,
                "text": "Lower is Better"
            },
            "scales": {
                "yAxes": [{
                    "ticks": {
                        "beginAtZero": true
                    }
                }]
            }
        }
    });

    // URL-encode the JSON string
    let data_str = chart_data.to_string();
    let encoded_chart_data = encode(&data_str);

    // Generate the full QuickChart URL
    format!(
        "https://quickchart.io/chart?bkg=white&c={}",
        encoded_chart_data
    )
}
fn extract_time_from_rust_output(output: &str, benchmarks: &mut Vec<(String, f64)>) {
    // extract time in microseconds (µs) from Rust output
    let re = Regex::new(r"(\w+)\s+time:\s+\[([\d.]+)\s+µs").unwrap();

    for captures in re.captures_iter(output) {
        let benchmark = captures.get(1).unwrap().as_str();
        let time = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse::<f64>()
            .unwrap_or(f64::MAX);
        benchmarks.push((format!("Rust - {}", benchmark), time));
    }
}

fn extract_time_from_go_output(output: &str) -> Option<String> {
    let re = Regex::new(r"BenchmarkJsonParser-\d+\s+\d+\s+([\d.]+)\s+ns/op").unwrap();
    if let Some(captures) = re.captures(output) {
        let time = captures.get(1).unwrap().as_str();
        return Some(format!("{} ns/op", time));
    }
    None
}

fn convert_nanoseconds_to_microseconds(input: &str) -> f64 {
    let cleaned_input = input.trim_end_matches(" ns/op");

    if let Ok(ns) = cleaned_input.parse::<f64>() {
        return ns / 1000.0; // Convert ns to µs
    }
    f64::MAX // Invalid input
}
