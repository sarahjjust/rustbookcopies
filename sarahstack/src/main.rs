fn main() {
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut stack = Vec::new();
            let words:Vec<_> = line.split(" ").collect();

            for word in words {
                if let Ok(parsed) = word.parse::<i32>() {
                    stack.push(parsed);
                } else {
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        _ => panic!("{word:?} could not be parsed"),
                    }
                }
            }

            println!("{stack:?}");
        }
    }
}

fn add(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a + b);
}

fn sub(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(b - a);
}

fn mul(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a * b);
}

fn div(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(b / a);
}
