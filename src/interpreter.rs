use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

#[derive(Debug)]
pub struct Runtime {
  //holds function name, and function statements
  functions: HashMap<String, Vec<Node>>,

  //holds stack of stack frames
  //stack is used when entering a function and holding variables from statements (e.g let x = 4)
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      
      stack: Vec::new(),
      
    }
  }

  pub fn extract_val(node: &Node) -> Vec<Node> {
    let mut v = Vec::new();
    
    match node{
      Node::FunctionArguments{children} =>{
        //children will be vec of expressions, a child will be a single expression
          for child in children {
              match child{
                Node::Expression{children} => {
                  match &children[0] {
                    Node::MathExpression{name,children} => v.push(Node::MathExpression{name: name.to_string(),children: children.to_vec()}),
                    Node::Number{value} => v.push(Node::Number{value: *value}),
                    Node::Identifier{value} => v.push(Node::Identifier{value: value.to_string()}),
                    _ => ()
                  }
                }
                _ => ()
              }
          }
      }
      _ => ()
    };

    v

  }

  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    match node {
      Node::Program{children} => {
        // the children in program only consist of funcdefinitons, or things that make up functions (look at grammar)
        for child in children {
          
          match child {
            
            Node::FunctionDefine{..} => {
              self.run(child);
            },
            //this is only if program is just an expression
            Node::Expression{..} => {
              self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![child.clone()]}]);
            },
            //this is if program holds just statements
            Node::Statement{..} => {
              self.functions.insert("main".to_string(), vec![child.clone()]);
            },

            x => {return Err("Unimplemeneted 1");},
          }
        }
        Ok(Value::Bool(true))
      },
      // Evaluates a mathematical expression based on the elements in the children argument. 
      //If the expression is valid, the code evaluates it and returns a new Value object with the resulting value.
      // If the expression is not valid, the code returns an error message.
      Node::MathExpression{name, children} => {
        
        let lhs = self.run(&children[0]);
        let rhs = self.run(&children[1]);



        let result = match (lhs,rhs) {
          (Ok(Value::Number(value1)), Ok(Value::Number(value2))) => match name.as_str() {
            "+" => Ok(Value::Number(value1 + value2)),
            "-" => Ok(Value::Number(value1 - value2)),
            "*" => Ok(Value::Number(value1 * value2)),
            "/" => Ok(Value::Number(value1 / value2)),
            "^" => Ok(Value::Number(value1.pow(value2.try_into().unwrap()))),
            _ => Err("Unimplemented math exp")
        },
          _=> {Err("Unimplemeneted math exp")}
        };

        result
        
      },
      // Defines a function that takes some arguments and executes a program based on those arguments. 
      //The code first checks if the function exists, and if it does, it creates a new scope in which to execute the function's statements. 
      //The code then executes each statement in the function's statements list and returns the result of the function's execution.
      Node::FunctionCall{name, children} => {
        let mut temp = Vec::new();
        let mut func_statements = match self.functions.get(name){
          Some(val) => val.to_owned(),
          None => temp,
         };

         if (func_statements.is_empty()){
          return Err("Undefined function");
         }

         


        let mut func_call_args = Vec::new();
        let mut counter = 0;
         //This extracts funcargs node from children in func call
         for args in children{
          match args {
             Node::FunctionArguments{children} => {
              func_call_args = Self::extract_val(&args);
              
            },
            _=> ()

          }
        }  

        // val will contain a vec of Values, which correspond to the arguments passed in. 
        //This is done to avoid any stack that go out of frame, especially for evaluating ID nodes w/ run method
        let mut val = Vec::new();
        for i in func_call_args.iter_mut(){
          let x = self.run(i);
          match x {
            Ok(correct) => val.push(correct),
            Err(incorrect) => ()
          }
        };



        //returns false if there is nothing returned
        let mut result = Ok(Value::Bool(false));
        self.stack.push(HashMap::new());

          for statement in func_statements.iter_mut(){
          result = match statement{

            Node::Identifier{value} => {
               
              let mut x = match self.stack.last_mut(){
                Some(n) => n.insert(value.to_string(),val[counter].to_owned()),
                None => {return Err("no scope has been init");}
               };
               Ok(Value::Bool(true))
            },
            Node::Statement{children} => self.run(statement),
             Node::FunctionReturn{children} => {
                self.run(&Node::FunctionReturn{children: children.to_vec()})
              },
            _=> Err("node is not a variableDefine or functionReturn")
          };
          counter +=1;
            
         };
        
              
         result
        
      },
      // Defines a new function based on the elements in the children argument. 
      //The name of the function is retrieved from the first element of the children, 
      //and the statements that define the function are retrieved from rest of the children (head/tail). 
      //A new key-value pair is then inserted into the functions field of the current runtime object. 
      //If the function was successfully defined, the code returns a Value object with a boolean value of true, otherwise an error is returned.
      Node::FunctionDefine{children} => {
        let node_id = children[0].to_owned();
        let var_name = match node_id{
          Node::Identifier{value} => {value},
          _=> {"error not ID".to_string()}
        };

        let mut x = Vec::new();
        for statements in children{
          match statements {
             Node::FunctionArguments{children} => {
              let mut args_id = Self::extract_val(&Node::FunctionArguments{children: children.to_owned()});

              x.append(&mut args_id)
            },
            Node::Statement{children} => x.push(Node::Statement{children: children.to_owned()}),
            _=> ()

        }
      }

      if (x.len() == 0){
        return Err("Incorrectly defined function")
      }

        self.functions.insert(
          var_name,
          x
        );

        Ok(Value::Bool(true))

      },
      // Calls the run method on the first element in the children argument, 
      //which recursively evaluates the AST of the program being executed and returns the resulting value or error message.
      Node::FunctionReturn{children} => {
        let result = self.run(&children[0]);
        self.stack.pop();
        result
      },

      // Retrieves the value of a variable from the current frame on the stack. If the variable is defined in the current frame, 
      //the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
       
        //expect() is similar to an unwrap. Unwrap  allows program to send an return value or send an error
        // depending if Result is Ok() or Err(),, if no hash map is found (no current stack available)
        //however, expect allows for an additional panic error message to appear if Err()
        let  result = match self.stack.last_mut(){
          Some(n) => n.get(value),
          None => {return Err("Undefined var");}
         };

        let ret_result = match result {
          Some(val) => Ok(val.to_owned()),
          None => Err("Undefined variable"),
        };

        ret_result


      },

      // Checks the type of the first element in the children argument and deciding what to do based on that type.
      // If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => {
        //will equal statement{w/ children varDefine or funcReturn}
        let node_first = children[0].to_owned();
        //check if children is varDefine or funcReturn, then do run method on the valid node, which returns a result type
        let result = match node_first{
          Node::VariableDefine{children} => self.run(&Node::VariableDefine{children}),
          Node::FunctionReturn{children} => self.run(&Node::FunctionReturn{children}),
          _=> Err("node is not a variableDefine or functionReturn")
        };

        result

      },
      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, 
      //and the value is retrieved by running the run method on the second element of the children argument. 
      //The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        //this gets the name of the var
        let node_id = children[0].to_owned();
        let var_name = match node_id{
          Node::Identifier{value} => {value},
          _=> {"error not ID".to_string()}
        };

        if var_name == "error not ID".to_string(){
          return Err("Not valid ID (variable define)")
        }

        //this gets the value of the var
        let var_val = self.run(&children[1]);
        let result = match var_val {
          Ok(Value) =>{
            Value
          },
          _=> { return Err("Undefined function");
          }
        };



        let ret_result = result.clone();


        //will return Option<T> // some or none
       let mut x = match self.stack.last_mut(){
        Some(n) => n.insert(var_name,result),
        None => {return Err("no scope has been init");}
       };

       
        Ok(ret_result)

      },
      //for expression we want to return the children value of type: vec![nodes]
      Node::Expression{children} => {
        for child in children{
        return self.run(child);
        }
        
        Err("did not evaluate expression")
        
      },
      Node::Number{value} => {
        Ok(Value::Number(*value))
      },
      Node::String{value} => {
        Ok(Value::String(value.to_string()))
      },
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      },
      x => {
        Err("Unimplemented 2")
      },
    }
  }

}

pub fn start_interpreter(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  //when we run this the first time, we are collecting all the function definitions into functions data strcutre
  runtime.run(node);

  //this will begin the main program, by going to check if main is a function, and then going from there (goes into main function and checks for other func calls)
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}
