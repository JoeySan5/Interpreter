extern crate asalang;
extern crate nom;

use asalang::{program, Node, Value, start_interpreter};
use nom::IResult;

macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),String> {
      match program($test) {
        Ok((input, p)) => {
          assert_eq!(input, "");
          assert_eq!(start_interpreter(&p), $expected);
          Ok(())
        },
        Err(e) => Err(format!("{:?}",e)),
      }
    }
  )
}


test!(conditional_test_1, r#"fn main(){let x = 5;
  let y = 4;
  let z = 2;
  let result = x - y * z > y / z + x != true;
  return result;
}"#, Ok(Value::Bool(true)));
// 5 - 4 * 2 > 4 / 2 + 5
// -3 > 7 evals to false
// false != true evals to true

test!(conditional_test_2, r#"fn foo(a,b,c) {
  let x = (a + b) >  3 == false;

  return x;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Bool(true)));
// 1 + 2 > 3 evals to false
// false == false evals to true

//calls 3 functions inside of one function in main to return string
test!(conditional_test_3, r#" fn main(){ return (4 ^ 2 >= 2 * 8) != (100/10 <= 15 - 2 * 3); }"#, Ok(Value::Bool(true)));
// (16 >= 16) evals to true
// (10 <= 9) evals to false
// true != false evals to true
//uses all other operators


//all if_arms are false except for the else arm
test!(if_expressions_test_1, r#"fn main(){
  let x = if false{
    let y = 3; return y;
  } else if false {
    return 4;
  } 
  else if false{
    return 7;
  } else {
    return 6;
  }; 
  return x;
  }"#, Ok(Value::Number(6)));


  //using conditional expressions combined with if_expressions
  test!(if_expressions_test_2, r#"fn main(){
    let y = 3;
    let z = 6;
    let x = if y>z{
      return y * 3;
    } else if y<z {
      return z^2;
    } 
    else {
      return 6;
    }; 
    return x;
    }"#, Ok(Value::Number(36)));

  //two line if expression returning strings instead
  test!(if_expression_test_3, r#" fn main(){ return if 4+3 > 5{ return "I like to code";} 
  else {return "I do not like to code";}; }"#, Ok(Value::String("I like to code".to_string())));

