pub enum Actions {
    QUIT,
    PRINT,
    MOVE {line_num: i64},
}

pub fn parse(cmd_str: &String) -> Vec<Actions> {
    //println!("{:?}", cmd_str.chars());
    let mut result: Vec<Actions> = Vec::new();
    let mut iter = cmd_str.chars().peekable();
    
    while let Some(c) = iter.next() {
	//println!("{:?}", c);
	if c == 'q' {
	    result.push(Actions::QUIT);
	} else if c == 'p' {
	    result.push(Actions::PRINT);
	} else if c.is_numeric() {
	    let mut line_num_str = c.to_string();
	    if let Some(b) = iter.peek() {
		if b.is_numeric() {
		    line_num_str = line_num_str + &b.to_string();
		    iter.next();
		}
	    }
	    let num = line_num_str.parse::<i64>().unwrap();
	    result.push(Actions::MOVE {line_num: num});
	}
    }
    return result;
}
