
#![allow(non_snake_case)]


struct Calculator<T> 

    where T: std::marker::Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::ops::Div<Output = T>,
{

    desiredOperation: char,
    a: T,
    b: T
}

impl<T> Calculator<T> 
    where T: std::marker::Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::ops::Div<Output = T> + std::cmp::PartialEq + From<u8>,

{


    pub fn new(operation: char, a: T, b: T) -> Self {

        Calculator {

            desiredOperation: operation,
            a: a,
            b: b
        }
    }

    fn operation(&mut self) ->Result<T, String> 
   
    {   
        match self.desiredOperation {

            '+' => Ok(self.a + self.b),
            '-' => Ok(self.a - self.b),
            '*' => Ok(self.a * self.b),
            '/' => {

                if self.b != T::from(0) {

                    Ok(self.a / self.b)
                }
                else {

                    Err("Division by zero is not allowed!".to_string())
                }
            }          
            _ => Err("Operation not allowed!".to_string()),
            
            }
        
        }

    }


fn main() {

    let mut calculator: Calculator<f64> = Calculator::new('/', 10.0, 0.0);
    match calculator.operation() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
