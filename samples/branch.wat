(module
  (memory (import "env" "memory") 256 256)
  (func $print (import "env" "print") (param i32 i32))
  (func $main (export "_main") (local $offset i32) (local $count i32)
    (block $myBlock
        br $myBlock
        (call $print (i32.const 0) (i32.const 13))
    )
    (call $print (i32.const 50) (i32.const 13))
  )
  (data (i32.const 0) "Hello, from A")
  (data (i32.const 50) "Hello, from B"))