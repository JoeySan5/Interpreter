extern crate nom;
extern crate asalang;

use asalang::{program, Value, start_interpreter, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
  let result = program(r#"fn main(){let x = if false{let y = 3; return y;} else if true{re} else {return 6;} ; return x;}"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:#?}", tree);
      
       let interpret = start_interpreter(&tree);
       println!("{:?}", interpret);

    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }

  

    
  Ok(())
}
