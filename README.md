# PoC-neotest-rust-issue-20
PoC of rouge8/neotest-rust/issues/20

## Error message

When run `:lua require('neotest').run.run()` show this message.

```bash
Error executing vim.schedule lua callback: ...share/nvim/lazy/plenary.nvim/lua/plenary/async/async.lua:18: The coroutine failed with this message: ...e/nvim/lazy/plenary.nvim/lua/plenary/context_manager.lua:47: /tmp/nvim.who/YyddkB/1.junit.xml: No such file or directory
stack traceback:
	[C]: in function 'error'
	...share/nvim/lazy/plenary.nvim/lua/plenary/async/async.lua:18: in function 'callback_or_next'
	...share/nvim/lazy/plenary.nvim/lua/plenary/async/async.lua:45: in function <...share/nvim/lazy/plenary.nvim/lua/plenary/async/async.lua:44>
```

## When CLI

When run `cargo nextest run` show this messages, this is expected behavior.

```bash
   Compiling PoC-neotest-rust-issue-20 v0.1.0 (/home/who/Documents/GitHub/PoC_neotest_rust_issue_20)
error[E0308]: mismatched types
  --> src/main.rs:14:5
   |
14 |     assert_eq!(result.unwrap(), Some(2));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found enum `Option`
   |
   = note: expected type `i32`
              found enum `Option<{integer}>`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0308`.
error: could not compile `PoC-neotest-rust-issue-20` due to previous error
error: command `/home/who/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo test --no-run --message-format json-render-diagnostics` exited with code 101
```
