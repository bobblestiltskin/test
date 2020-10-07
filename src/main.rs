fn arith(numatod: &str) -> usize {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    // parse the input string and populate the numbers and operators vectors
    // operators are characters
    // numbers are strings
    let mut numchars = numatod.chars();
    let mut num:String = "".to_string();
    while let Some(ch) = numchars.next() {
        if ch.is_alphabetic() {
            operators.push(ch);
            numbers.push(num.clone());
            num.clear();
        } else {
            num.push(ch);
        }
    }
    numbers.push(num);

    // walk the operators vector
    // apply each operator to the first two numbers
    // remove the first operator and first number
    // replace the (new) first number with the result
    // until all of the operators have been used
    let mut oplen = operators.len();
    while oplen > 0 {
        let left = numbers[0].parse::<usize>().unwrap();
        let right = numbers[1].parse::<usize>().unwrap();
        let mut result = 0;
        match operators[0] {
            'a' => {result = left + right;},
            'b' => {result = left - right;},
            'c' => {result = left * right;},
            'd' => {result = left / right;},
            _ => println!("again operator {}", operators[0]),
        }
        numbers.remove(0);
        numbers[0] = result.to_string();
        operators.remove(0);
        oplen = operators.len();
    }
    numbers[0].parse::<usize>().unwrap()
}

fn parse(numatof: &str) -> usize {
    // takes a string potentially containing bracketed sections
    // if there are brackets, replace these by numeric results and recurse
    // start with most deeply nested sections and work upwards

    let retval:usize;
    if numatof.contains("f") { // a closing bracket
        let mut lbracket:usize = 0;
        let mut rbracket:usize = 0;
        let numvec: Vec<char> = numatof.chars().collect();
        // find the position of the first closing bracket
        for i in 0 .. numvec.len() {
            if numvec[i] == 'f' {
                rbracket = i;
                break;
            }
        }
        // walk backwards to find the matching opening bracket
        for i in (0 .. rbracket).rev() {
            if numvec[i] == 'e' {
                lbracket = i;
                break;
            }
        }
        // extract the bracketed substring
        let from:&str = &numatof[lbracket .. rbracket+1];
        // compute the result of the substring
        let result:usize = arith(&numatof[lbracket+1 .. rbracket]);
        // convert the result to our replacement string
        let to:&str = &result.to_string();
        // replace the original substring with our result string
        let newstring:String = numatof.replace(from, to);
        // and then recurse ...
        retval = parse(&newstring);
    } else {
        retval = arith(numatof);
    }
    retval
}

fn main() {
    let p1 = "3a2c4";
    println!("Input: {}\nResult: {}\n", p1, parse(p1));
    let p2 = "32a2d2";
    println!("Input: {}\nResult: {}\n", p2, parse(p2));
    let p3 = "500a10b66c32";
    println!("Input: {}\nResult: {}\n", p3, parse(p3));
    let p4 = "3ae4c66fb32";
    println!("Input: {}\nResult: {}\n", p4, parse(p4));
    let p5 = "3c4d2aee2a4c41fc4f";
    println!("Input: {}\nResult: {}\n", p5, parse(p5));
}
