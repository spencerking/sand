pub enum Actions {
    QUIT,
    PRINT,
    MOVE {line_num: i64},
    INSERT {txt: String},
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
	    while let Some(b) = iter.peek() {
		if b.is_numeric() {
		    line_num_str = line_num_str + &b.to_string();
		    iter.next();
		} else {
		    break;
		}
	    }
	    let num = line_num_str.parse::<i64>().unwrap();
	    result.push(Actions::MOVE {line_num: num});
	} else if c == 'i' {
	    // Currently this doesn't require a matching close slash
	    // which seems to be consistent with plan9port sam
	    let mut found_str = false;
	    let mut str_to_insert = String::new();

	    while let Some(b) = iter.peek() {
		if b == &'/' && !found_str {
		    found_str = true;
		    iter.next();
		} else if b == &'/' && found_str {
		    iter.next();
		    break;
		} else if found_str {
		    str_to_insert = str_to_insert + &b.to_string();
		    iter.next();
		} else {
		    iter.next();
		}
	    }
	    if str_to_insert != "" {
		result.push(Actions::INSERT {txt: str_to_insert});
	    }
	}
    }
    return result;
}
