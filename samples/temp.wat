(module
  (func (export "test") (param $x i32) (param $y i32) (result i32)
    (i32.gt_u (get_local $x) (get_local $y))) 
)

(assert_return (invoke "test" (i32.const 1) (i32.const 0)) (i32.const 1))