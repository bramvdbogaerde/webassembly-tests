use wast::core::{Expression, Instruction, Module, NanPattern, V128Const};
use wast::parser::{parse, ParseBuffer};
use wast::token::{Id, Index};
use wast::{QuoteWat, Wast, WastArg, WastDirective, WastExecute, WastInvoke, WastRet, Wat};

/// Transform Wast statements to Wat expressions
trait ExecuteToExpression<'t> {
    fn to_execute(&'t self) -> Expression<'t>;
}

impl<'t> ExecuteToExpression<'t> for WastRet<'t> {
    fn to_execute(&'t self) -> Expression<'t> {
        // NOTE: This translation is not entirely correct
        // and should probably refactored. This is because
        // the WastRet expressions can actually be patterns that
        // are not part of the real value domain. For example,
        // the `Either` expressions allows for one of the values
        // to be used in the equality. This means that we will have
        // to encode some value checking into this translation unit
        // as well at some point.
        let instr: Instruction = match &self {
            Self::Core(v) => match v {
                wast::core::WastRetCore::I32(v) => Instruction::I32Const(*v),
                wast::core::WastRetCore::I64(v) => Instruction::I64Const(*v),
                wast::core::WastRetCore::F32(NanPattern::Value(v)) => Instruction::F32Const(*v),
                wast::core::WastRetCore::F64(NanPattern::Value(v)) => Instruction::F64Const(*v),
                wast::core::WastRetCore::V128(v) => todo!(),
                wast::core::WastRetCore::RefNull(_) => todo!(),
                wast::core::WastRetCore::RefExtern(_) => todo!(),
                wast::core::WastRetCore::RefHost(_) => todo!(),
                wast::core::WastRetCore::RefFunc(_) => todo!(),
                wast::core::WastRetCore::RefAny => todo!(),
                wast::core::WastRetCore::RefEq => todo!(),
                wast::core::WastRetCore::RefArray => todo!(),
                wast::core::WastRetCore::RefStruct => todo!(),
                wast::core::WastRetCore::RefI31 => todo!(),
                wast::core::WastRetCore::Either(_) => todo!(),
                _ => panic!("unsupported ret"),
            },
            // TODO: support component model
            _ => todo!(),
        };

        Expression {
            instrs: vec![instr].into(),
            branch_hints: vec![],
        }
    }
}

impl<'t> ExecuteToExpression<'t> for WastArg<'t> {
    fn to_execute(&'t self) -> Expression<'t> {
        let instr: Instruction = match &self {
            Self::Core(v) => match v {
                wast::core::WastArgCore::I32(v) => Instruction::I32Const(*v),
                wast::core::WastArgCore::I64(v) => Instruction::I64Const(*v),
                wast::core::WastArgCore::F32(v) => Instruction::F32Const(*v),
                wast::core::WastArgCore::F64(v) => Instruction::F64Const(*v),
                wast::core::WastArgCore::V128(v) => Instruction::V128Const(v.clone()),
                wast::core::WastArgCore::RefNull(v) => Instruction::RefNull(*v),
                wast::core::WastArgCore::RefExtern(v) => todo!(),
                wast::core::WastArgCore::RefHost(v) => todo!(),
            },
            _ => panic!("do not support component model"),
        };

        Expression {
            instrs: vec![instr].into(),
            branch_hints: vec![],
        }
    }
}

impl<'t> ExecuteToExpression<'t> for WastExecute<'t> {
    fn to_execute(&'t self) -> Expression<'t> {
        match &self {
            Self::Invoke(WastInvoke {
                span: _,
                module: _,
                name,
                args,
            }) =>
            // TODO: allow the more complicated case were a function
            // is invoked in a multi-module WAST file, in that case
            // the function should be imported first before it can be executed?
            {
                let mut instrs = Vec::new();
                for arg in args {
                    instrs.extend_from_slice(&arg.to_execute().instrs)
                }

                instrs.push(Instruction::Call(Index::Id(Id::new(name, self.span()))));

                Expression {
                    instrs: instrs.into(),
                    branch_hints: vec![],
                }
            }
            // TODO: allow the other expressions here as well
            _ => unreachable!(),
        }
    }
}

/// Transform assertions in the form of directives
/// to `if` statements.
/// Unfortunately, the `assert_` statements are
/// records in the directive enum which makes it
/// unfeasible to match statically over them.
impl<'t> ExecuteToExpression<'t> for WastDirective<'t> {
    fn to_execute(&'t self) -> Expression<'t> {
        match &self {
            &Self::AssertReturn {
                span: _,
                exec,
                results,
            } => {
                let mut instrs = Vec::new();

                instrs.extend_from_slice(&exec.to_execute().instrs);
                
                // TODO: support more than one return value
                assert!(results.len() < 1);
                instrs.extend_from_slice(&results.get(0).expect("value").to_execute().instrs);

                // TODO: need to know the return type for generating the correct 
                // equality function
                todo!()
            }
            _ => panic!("unexpected directive"),
        }
    }
}

struct AssertTransformer<'t> {
    buf: ParseBuffer<'t>,
}

impl<'t> AssertTransformer<'t> {
    pub fn from_str<'s>(contents: &'s str) -> AssertTransformer<'s> {
        let buf = ParseBuffer::new(contents.as_ref()).unwrap(); // TODO: proper error handling
        AssertTransformer { buf: buf }
    }

    /// Transform a single assertion
    pub fn transform_assert(&self, n: &mut WastDirective) {}

    /// Transform the AST embedded in the parse buffe
    pub fn transform(&self) -> Wat<'t> {
        let module: Wast = parse(&self.buf).unwrap();
        let Wast { directives } = module;

        let mut modules = Vec::new();
        let mut assertions = Vec::new();

        for directive in directives {
            match directive {
                wast::WastDirective::Wat(QuoteWat::Wat(w)) => modules.push(w),
                // An example of one of the assertions
                wast::WastDirective::AssertReturn { .. } => assertions.push(directive),
                _ => panic!("unhandled case"), // TODO: proper error handling
            }
        }
        // transform all assertions to the appropriate expressions
        let assert_expressions = assertions
            .iter_mut()
            .for_each(|ass| self.transform_assert(ass));

        // For simplicity: assert that there is only one module in the program
        // might be extended later
        assert_eq!(modules.len(), 1);

        // Use that module to insert an entrypoint
        let main_module = modules.get_mut(0).unwrap();

        todo!()
    }
}

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
