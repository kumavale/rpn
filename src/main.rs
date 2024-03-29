use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let exp = &args[1];
    let ans = rpn(exp);

    println!("'{}' = {} => {}", exp, ans,
             if ans.to_string() == args[2] {
                 "\x1b[32mOK\x1b[0m"
             } else {
                 "\x1b[31mNG\x1b[0m"
             });
}

fn rpn(exp: &str) -> f64 {
    let mut stack = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                _ => panic!("Unknown operator: {}", token),
            }
        }
    }

    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}

