(module
	(import "wasi_snapshot_preview1" "proc_exit" (func $proc_exit (param i32)))

	(memory 1)
	(export "memory" (memory 0))

	(func $_start
		call $main
		call $proc_exit
	)
	(start $_start)

	
;; folded mode:
(func $main (result i32)
	(if (result i32)
	(i32.const 1)
	(then (i32.const 7))
	(else (i32.const 6))
)
)

;; linear mode:
(func $main (result i32)
	i32.const 0
	if (result i32)
	  i32.const 7
	else
		i32.const 6
	end
)

(func $x (result i32)
	i32.const 7
)

)