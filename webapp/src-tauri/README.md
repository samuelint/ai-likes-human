# Desktop App

The desktop app (obviously) use Tauri as a native runtime.

## Core Runtime

To have the core runtime running along the app (sidecar). The core server binary must be located at `/src-tauri/binaries/ai-assistant-core-$TARGET_TRIPLE` (For apple sillicon `/src-tauri/binaries/ai-assistant-core-aarch64-apple-darwin`).

:note: the `$TARGETR_TRIPLE` can be found by executing `rustc -Vv`. Complete list of runtimes can be found [here](https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools).
