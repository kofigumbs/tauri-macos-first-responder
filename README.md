Demonstrates that the WKWebView is not properly set as the window's first responder.

- `cargo tauri dev` -- required a click before webview is active
- `CARGO_FEATURE_MAKE_FIRST_RESPONDER=1 cargo tauri dev` -- works as expected
