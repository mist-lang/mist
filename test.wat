(module
  (start $_start)
  (import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param  i32)))
  (export "memory" (memory 0))
  (memory 1)
  (func $_start
    call $main
    call $proc_exit)
  (func $main (result i32)
    i32.const 1
    if (result i32)
      call $x
    else
      i32.const 6
    end)
  (func $x (result i32)
    i32.const 7)
)
