# Recipe: check the UI (snapshots)

1. **Take a snapshot:** `cargo run -p editor-app -- --snapshot out.png --size 1200x800` (no window opens; exit code 2 means no GPU adapter).
2. **Look at it:** open `out.png` with your file/image viewer tool (Claude: the Read tool shows images) and describe what you see in the PR.
3. **Run the snapshot tests:** `cargo test -p editor-app --test snapshot`. Stored PNGs live in `crates/editor-app/tests/snapshots/`; tolerance is documented in `crates/editor-app/tests/snapshot.rs`.
4. **A test failed?** Look at `<name>.new.png` (what the app draws now) and `<name>.diff.png` next to the stored PNG.
5. **Your task changes the UI on purpose?** `UPDATE_SNAPSHOTS=1 cargo test -p editor-app --test snapshot`, look at every updated PNG, and say so in the PR (rule 5: never update to hide a bug).
6. **Test a new screen:** add a test in `crates/editor-app/tests/snapshot.rs`; click with `harness.get_by_label("File").click()`, then `harness.run()`.
7. CI runs them on Linux for every PR (software renderer) and on Windows/macOS after merge. A machine with no GPU adapter at all prints `SKIP` and passes; on Linux, install `mesa-vulkan-drivers`.
