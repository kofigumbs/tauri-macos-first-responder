#![allow(unused_variables)]

use tauri::{Event, Manager};

fn main() {
  let app = tauri::Builder::default()
    .build(tauri::generate_context!())
    .expect("error while building tauri application");
  app.run(|handle, event| match event {
    #[cfg(all(target_os = "macos", feature = "make-first-responder"))]
    Event::Ready => {
      use cocoa::appkit::NSWindow;
      let handle = handle.clone();
      tauri::async_runtime::spawn(async move {
        let window = handle.get_window("main").unwrap().ns_window().unwrap() as cocoa::base::id;
        unsafe { window.makeFirstResponder_(window.contentView()) };
      });
    }
    _ => {}
  });
}
