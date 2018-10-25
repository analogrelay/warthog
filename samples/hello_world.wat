(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func))
  (import "env" "memory" (memory (;0;) 256 256))
  (import "env" "print" (func (;0;) (type 0)))
  (func (;1;) (type 1)
    i32.const 1400
    i32.const 13  ;; length
    call 0)
  (export "_main" (func 1))
  (data (i32.const 1400) "Hello, world!"))