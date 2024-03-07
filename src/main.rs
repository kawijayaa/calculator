use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let operand1: String = args.nth(1).unwrap_or_else(|| panic!("First operand is not given!"));
    let operator: String = args.nth(0).unwrap_or_else(|| panic!("Operator is not given"));
    let operand2: String = args.nth(0).unwrap_or_else(|| panic!("Second operand is not given!"));

    let operand1_int: f32 = operand1.parse::<f32>().unwrap();
    let operator_char: char = operator.chars().next().unwrap();
    let operand2_int: f32 = operand2.parse::<f32>().unwrap();

    let result: Option<f32> = match operator_char {
        '+' => Some(operand1_int + operand2_int),
        '-' => Some(operand1_int - operand2_int),
        '*' => Some(operand1_int * operand2_int),
        '/' => Some(operand1_int / operand2_int),
        _ => None,
    };

    match result {
        Some(num) => println!("Result of {} {} {} is {}", operand1, operator, operand2, num),
        None => panic!("Operator is not recognized!")
    }
}
