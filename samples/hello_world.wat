(module
  (memory (import "env" "memory") 256 256)
  (func $print (import "env" "print") (param i32 i32))
  (func $get_offset (result i32) i32.const 1400)
  (func $get_count (result i32) i32.const 13)
  (func (export "_main")
    (call $print (call $get_offset) (call $get_count)))
  (data (i32.const 1400) "Hello, world!"))