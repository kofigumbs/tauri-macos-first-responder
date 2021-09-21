Demonstrates that the WKWebView is not properly set as the window's first responder.

- `cd src-tauri && cargo run` -- required a click before webview is active
- `cd src-tauri && cargo run --features make-first-responder` -- works as expected
