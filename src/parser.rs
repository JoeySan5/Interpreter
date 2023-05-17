// Here is where the various combinators are imported. You can find all the combinators here:
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{alphanumeric1, digit1},
  };
  
  // Here are the different node types. You will use these to make your parser and your grammar.
  // You may add other nodes as you see fit, but these are expected by the runtime.
  
  #[derive(Debug, Clone)]
  pub enum Node {
    Program { children: Vec<Node> },
    Statement { children: Vec<Node> },
    FunctionReturn { children: Vec<Node> },
    FunctionDefine { children: Vec<Node> },
    FunctionArguments { children: Vec<Node> },
    FunctionStatements { children: Vec<Node> },
    Expression { children: Vec<Node> },
    MathExpression {name: String, children: Vec<Node> },
    ConditionalExpression {name: String, children: Vec<Node>},

    IfExpression {children: Vec<Node>},
    IfStatements {children: Vec<Node>},
    ElseIfExpression{children: Vec<Node>},
    ElseExpression{children: Vec<Node>},

    MathAdd {children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
    Null,
  }
  
  // Here is the grammar, for your reference:
  
  pub fn identifier(input: &str) -> IResult<&str, Node> {
    let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
    Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
  }
  
  pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
    let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
    Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
  }
  
  pub fn boolean(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?;
    let bool_value = if result == "true" {true} else {false};
    Ok((input, Node::Bool{ value: bool_value}))
  }
  
  pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("\"")(input)?;
    let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Node::String{ value: string.join("")}))
  }
  
  pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
  } 
  
  pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = alt((conditional_expression,l1))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    Ok((input, args))
  }
  pub fn l4(input: &str) -> IResult<&str, Node> {
    alt((function_call, boolean, number, identifier, parenthetical_expression))(input)
  }
  pub fn l3_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = tag("^")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l4(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }
  pub fn l3(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l4(input)?;
    let (input, tail) = many0(l3_infix)(input)?;
    for n in tail {
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }
  pub fn l2_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = alt((tag("*"),tag("/")))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l2(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }
  pub fn l2(input: &str) -> IResult<&str, Node> {
    let (input, mut head) = l3(input)?;
    let (input, tail) = many0(l2_infix)(input)?;
    for n in tail {
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }
  pub fn l1_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = alt((tag("+"),tag("-")))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = l2(input)?;
    Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
  }
  pub fn l1(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, mut head) = l2(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, tail) = many0(l1_infix)(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    for n in tail {
      match n {
        Node::MathExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::MathExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))
  }
  
  pub fn math_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    l1(input)
    
  }

  pub fn conditional_expression(input: &str) -> IResult<&str, Node>{
    let (input, mut head) = math_expression(input)?;
    let (input, tail) = many0(op_infix)(input)?;
    for n in tail {
      match n {
        Node::ConditionalExpression{name, mut children} => {
          let mut new_children = vec![head.clone()];
          new_children.append(&mut children);
          head = Node::ConditionalExpression{name, children: new_children};
        }
        _ => () 
      };
    }
    Ok((input, head))

  }
  
  pub fn op_infix(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, op) = alt((tag(">="),tag("<="),tag("<"),tag(">"),tag("=="),tag("!=")))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, args) = math_expression(input)?;
    Ok((input, Node::ConditionalExpression{name: op.to_string(), children: vec![args]}))
  }
  

  // value = boolean | number | identifier ;
pub fn value(input: &str) -> IResult<&str, Node> {
  let (input, _ ) = many0(tag(" "))(input)?;
  let (input_left, output) = alt((boolean, number, identifier))(input)?;
  let (input, _ ) = many0(tag(" "))(input)?;

  IResult::Ok((input_left,output))

}

  //expression = boolean | if_expression | math_expression | function_call | number | string | identifier ;
  pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((boolean,if_expression, conditional_expression, math_expression, function_call, number, string, identifier))(input)?;
    Ok((input, Node::Expression{ children: vec![result]}))   
  }








  //if_expression  = "if" , (conditional_expression | boolean) , "{" , {statement} , "}" , [{ else_if_expression}] , "else" , "{" {statement} "}" ;
  pub fn if_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = many0(tag(" "))(input)?;

    let (input, if_exp) = alt((conditional_expression,boolean))(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    //returns a node of IfStatements
    let (input, if_commands) = if_statement(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    //returns a vec of elseif nodes if any
    let (input, mut else_exp) = many0(else_if_expression)(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;


    //returns a node of ElseExp
    let (input, else_commands) = else_expression(input)?;


    let mut new_vec = vec![if_exp];

    new_vec.push(if_commands);
    new_vec.append(&mut else_exp);
    new_vec.push(else_commands);


    Ok((input, Node::IfExpression{children: new_vec}))
  }


  pub fn if_statement(input: &str) -> IResult<&str, Node> {
    let (input, statements) = many1(statement)(input)?;

    Ok((input, Node::IfStatements{children: statements}))
  }

  



  //else_if_expression = "else", "if", (conditional_expression | boolean), "{", {statement}, "}" ;
  pub fn else_if_expression(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("else")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, exp) = alt((conditional_expression,boolean))(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, mut commands) = (if_statement)(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    
    let mut new_vec = vec![exp];
    new_vec.push(commands);

    Ok((input, Node::ElseIfExpression{children: new_vec}))


  }

  pub fn else_expression(input: &str) -> IResult<&str, Node> {

    let (input, _) = tag("else")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;

    let (input, _) = tag("{")(input)?;

    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    //returns a vec of statements (else block)

    let (input,  else_commands) = if_statement(input)?;

    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;

    Ok((input, Node::ElseExpression{children: vec![else_commands]}))
  
    

  }






  pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, result) = alt((variable_define, function_return))(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag(";")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = many0(tag("\n"))(input)?;
    Ok((input, Node::Statement{ children: vec![result]}))   
  }
  
  pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return ")(input)?;
    let (input, return_value) = alt((function_call, expression, identifier))(input)?;
    Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
  }
  
  pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("let ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, expression) = expression(input)?;
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
  }
  
  pub fn arguments(input: &str) -> IResult<&str, Node> {
    let (input, arg) = expression(input)?;
    let (input, mut others) = many0(other_arg)(input)?;
    let mut args = vec![arg];
    args.append(&mut others);
    Ok((input, Node::FunctionArguments{children: args}))
  }
  
  pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag(",")(input)?;
    expression(input)
  }
  
  pub fn function_definition(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("fn ")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, function_name) = identifier(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, mut args) = many0(arguments)(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    
    let (input, mut statements) = many1(statement)(input)?;
    let (input, _) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = many0(alt((tag("\n"),tag(" "))))(input)?;
    let mut children = vec![function_name];
    children.append(&mut args);
    children.append(&mut statements);
    Ok((input, Node::FunctionDefine{ children: children }))   
  }
  
  // program = function_definition+ ;
  pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, result) = many1(function_definition)(input)?;
    Ok((input, Node::Program{ children: result}))
  }
  