use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error");
        return;
    }

    let expr = &args[1];

    let mut stack: Vec<i64> = Vec::new();

    for token in expr.split_whitespace(){
        match token {
          "+" | "-" | "*" | "/" | "%" => {
              if stack.len() < 2 {
                  println!("Error");
                  return;
              }
              let b = stack.pop().unwrap();
              let a = stack.pop().unwrap();

              let result = match token {
                  "+" => a.checked_add(b),
                  "-" => a.checked_sub(b),
                  "*" => a.checked_mul(b),
                  "/" => a.checked_div(b),
                  "%" => a.checked_rem(b),
                  _ => unreachable!(),
              };

              match result {
                  Some(val) => stack.push(val),
                  None => { println!("Error"); return; }
              }
          }

          _ => {
              match token.parse::<i64>() {
                  Ok(num) => stack.push(num),
                  Err(_) => { println!("Error"); return; }
                
              }
            }
          }
        }
        if stack.len() == 1{
            println!("{}",stack[0]);
        } else {
            println!("Error");
        }
}


/*
use std::env;

fn main() {
    // STEP 1: Capture command-line arguments
    // `env::args()` returns an iterator of arguments. We collect them into a Vector of Strings.
    let args: Vec<String> = env::args().collect();
    
    // The first argument (args[0]) is always the path to the executable.
    // The problem asks for exactly one argument containing the expression.
    // Therefore, the total length of `args` must be exactly 2.
    if args.len() != 2 {
        println!("Error");
        return;
    }

    // Extract the RPN expression string.
    let expr = &args[1];
    
    // STEP 2: Initialize the Stack
    // RPN is evaluated using a Last-In-First-Out (LIFO) stack.
    let mut stack: Vec<i64> = Vec::new();

    // STEP 3: Process the string token by token
    // `split_whitespace()` automatically handles multiple spaces and ignores them,
    // yielding only the actual characters/numbers.
    for token in expr.split_whitespace() {
        match token {
            // If the token is a mathematical operator...
            "+" | "-" | "*" | "/" | "%" => {
                // An operator requires exactly two operands. 
                // If the stack has fewer than 2 items, the expression is invalid.
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                
                // Pop the last two operands from the stack.
                // NOTE: The first pop is the RIGHT operand, the second is the LEFT operand.
                // E.g., for "4 2 /", we pop 2 (b), then 4 (a), and do 4 / 2.
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                
                // STEP 4: Safely evaluate the operation
                // We use `.checked_*` methods instead of standard operators (+, -) 
                // to prevent the program from panicking (crashing) on division by zero 
                // or if the number gets too big (integer overflow).
                let result = match token {
                    "+" => a.checked_add(b),
                    "-" => a.checked_sub(b),
                    "*" => a.checked_mul(b),
                    "/" => a.checked_div(b),
                    "%" => a.checked_rem(b),
                    _ => unreachable!(), // We already verified it's one of these operators
                };
                
                // If the operation succeeds, push the result back onto the stack.
                // If it fails (e.g., divide by zero), print Error and exit.
                match result {
                    Some(val) => stack.push(val),
                    None => {
                        println!("Error");
                        return;
                    }
                }
            }
            // If the token is not an operator, it must be a number...
            _ => {
                // Try to parse the string token into a 64-bit integer (i64).
                match token.parse::<i64>() {
                    // If parsing is successful, push it to the stack.
                    Ok(num) => stack.push(num),
                    // If parsing fails (e.g., "ksd", "1.5"), it's invalid.
                    Err(_) => {
                        println!("Error");
                        return;
                    }
                }
            }
        }
    }

    // STEP 5: Final Validation
    // A perfectly valid RPN expression will leave exactly ONE number on the stack: the final answer.
    // If there is more than one number, the user provided too many operands (e.g., "1 2 3 +").
    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}
*/