(module
  (func $main (export "_main") call $outer)
  (func $outer call $middle)
  (func $middle call $inner)
  (func $inner
    (drop (i32.div_u (i32.const 100) (i32.const 0)))))