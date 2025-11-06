// Go dashboard server for Solana MEV Bot.
// Provides a minimal REST API and static file serving for a lightweight web UI.

package main

import (
    "encoding/json"
    "log"
    "net/http"
)

type HealthResponse struct {
    Status string `json:"status"`
    Uptime int64  `json:"uptime_seconds"`
}

func main() {
    http.HandleFunc("/api/health", func(w http.ResponseWriter, r *http.Request) {
        resp := HealthResponse{Status: "ok", Uptime: 123}
        w.Header().Set("Content-Type", "application/json")
        json.NewEncoder(w).Encode(resp)
    })

    http.Handle("/", http.FileServer(http.Dir("./static")))

    log.Println("Solana MEV Bot dashboard listening on :8080")
    if err := http.ListenAndServe(":8080", nil); err != nil {
        log.Fatal(err)
    }
}
