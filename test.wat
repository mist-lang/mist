(module
	(import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

	(memory 1)
	(export "memory" (memory 0))

	(func $_start (export "_start")
		(call $proc_exit (call $main))
	)

	
(func $main (result i32)
	(if (result i32)
	(i32.const 1)
	(then (i32.const 8))
	(else (i32.const 6))
)

)

(func $x (result i32)
	(i32.const 7)
)

)