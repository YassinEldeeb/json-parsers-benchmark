package main

import (
    "testing"
    "runtime"
    "github.com/buger/jsonparser"
    "io/ioutil"
    "fmt"
)

func readJSONFile(filename string) (string, error) {
    jsonBytes, err := ioutil.ReadFile(filename)
    if err != nil {
        return "", err
    }
    return string(jsonBytes), nil
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