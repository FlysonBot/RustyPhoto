# Recipe: add a command

Every feature is added the same way. The reference example is
`crates/editor-engine/src/cmd/adjust_invert.rs`.

1. **Find the id** in `catalog/catalog.toml`. If the feature isn't there,
   stop: adding catalog entries is the architect's job (ask in the task).
2. **Put the algorithm in the lowest crate that fits.** Today that's the
   command file itself; once `editor-algo` (L2) exists, pure pixel math goes
   there and the command just calls it.
3. **Create one new file** `crates/editor-engine/src/cmd/<area>_<name>.rs`
   with a `specs()` function returning the `CommandSpec`. Read parameters
   only through `Params` getters (they validate and never panic). Document
   every parameter in `params`.
4. **Register it** with one line in `command_specs()` in
   `crates/editor-engine/src/cmd/mod.rs` (a hot spot: re-read first, one-line edit),
   plus the `mod` line above it.
5. **Write tests** in the same file:
   - the behavior, with exact expected pixel values (from `docs/behavior/` when they exist)
   - undo restores the exact previous document
   - bad parameters return an error and change nothing
   The fuzz test covers hostile input automatically.
6. **Regenerate coverage:** `cargo xtask catalog`, then raise
   `catalog/floor.txt` to the new live count it prints.
7. **Run the gate:** `cargo xtask ci`. Then open a PR with the template.

The menu item goes live by itself: the app greys out only ids without a command.
