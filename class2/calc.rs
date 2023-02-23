
use std::io;

enum OPERATOR {
    PLUS, MINUS, MULT, DIVIDE, NONE
}

fn main() {

    let mut result = 0;
    read_input_number(&mut result);
    loop {
        let op = read_input_operator().unwrap_or(OPERATOR::NONE);

        match op {
            OPERATOR::NONE => {
                println!("please enter a valid operator (+, -, * or /)");
            }
            _ => {
                let mut input_nr: i32 = 0;
                read_input_number(&mut input_nr);
                match op {
                    OPERATOR::PLUS => result += input_nr,
                    OPERATOR::MINUS => result -= input_nr,
                    OPERATOR::MULT => result *= input_nr,
                    OPERATOR::DIVIDE => result /= input_nr,
                    _ => {}
                }
            }
        }
        println!("Result: {}", result);
    }
}

fn read_input_number(number: &mut i32) {
    println!("Enter number: ");
    
    let mut buffer = String::new();
    let res = io::stdin().read_line(&mut buffer);
    if res.is_err() {
        println!("Not a valid input!");
        read_input_number(number);
        return;
    }
    buffer.pop(); // remove newline
    let my_int: i32 = buffer.parse().unwrap();

    number.clone_from(&my_int);
    
}

fn read_input_operator() -> io::Result<OPERATOR> {
    
    println!("Enter operator: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let op = match buffer.chars().next().unwrap() {
        '+' => OPERATOR::PLUS,
        '-' => OPERATOR::MINUS,
        '*' => OPERATOR::MULT,
        '/' => OPERATOR::DIVIDE,
        _ => OPERATOR::NONE,
    };
    Ok(op)
}
