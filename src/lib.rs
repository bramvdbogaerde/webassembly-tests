#[cfg(test)]
mod tests {
    use indoc::indoc;
    use wast::parser::{parse, ParseBuffer};
    use wast::Wast;

    const FORWARD_WAST: &'static str = indoc! { r#"
    (module
      (func $even (export "even") (param $n i32) (result i32)
        (if (result i32) (i32.eq (local.get $n) (i32.const 0))
          (then (i32.const 1))
          (else (call $odd (i32.sub (local.get $n) (i32.const 1))))))
    
      (func $odd (export "odd") (param $n i32) (result i32)
        (if (result i32) (i32.eq (local.get $n) (i32.const 0))
          (then (i32.const 0))
          (else (call $even (i32.sub (local.get $n) (i32.const 1)))))))
    (assert_return (invoke "even" (i32.const 13)) (i32.const 0))
    (assert_return (invoke "even" (i32.const 20)) (i32.const 1))
    (assert_return (invoke "odd" (i32.const 13)) (i32.const 1))
    (assert_return (invoke "odd" (i32.const 20)) (i32.const 0))
    "# };

    #[test]
    fn example() {
        let buf = ParseBuffer::new(FORWARD_WAST).unwrap();
        let module = parse::<Wast>(&buf).unwrap();
        let Wast { directives } = module;
        for directive in directives {
            match directive {
                // The WAT part, the regular module
                wast::WastDirective::Wat(_) => todo!(),
                // An example of one of the assertions
                wast::WastDirective::AssertReturn { .. } => todo!(),
                _ => todo!(),
            }
        }
    }
}
