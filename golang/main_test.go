package main

import (
    "encoding/json"
    "github.com/buger/jsonparser"
    "io/ioutil"
    "runtime"
    "testing"
    "fmt"
)

func readJSONFile(filename string) ([]byte, error) {
    jsonBytes, err := ioutil.ReadFile(filename)
    if err != nil {
        return nil, err
    }
    return jsonBytes, nil
}


func BenchmarkJsonParser(b *testing.B) {
    jsonData, err := readJSONFile("../benches/data.json")

    if err != nil {
        fmt.Println("Failed to read JSON file:", err)
        return
    }

    jsonBytes := []byte(jsonData)

    runtime.GC() // Run garbage collection before calculating
    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        _, _, _, err := jsonparser.Get(jsonBytes, "data", "users")
        if err != nil {
            b.Fatal(err)
        }
    }
    b.StopTimer()
}

func BenchmarkEncodingJson(b *testing.B) {
    jsonBytes, err := readJSONFile("../benches/data.json")
    if err != nil {
        b.Fatalf("Failed to read JSON file: %v", err)
    }

    var jsonData map[string]interface{}

    runtime.GC() // Run garbage collection before starting the benchmark
    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        if err := json.Unmarshal(jsonBytes, &jsonData); err != nil {
            b.Fatal(err)
        }
        // Access the nested "users" key
        _, ok := jsonData["data"].(map[string]interface{})["users"]
        if !ok {
            b.Fatal("Failed to access 'users' key")
        }
    }
}