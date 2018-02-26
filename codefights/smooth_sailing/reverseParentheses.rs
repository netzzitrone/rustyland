fn reverseParentheses(s: String) -> String {
    let mut result = String::new();
    let mut s_vec: Vec<&str> = s.split("").collect();
    s_vec.remove(0);
    s_vec.remove(s.len());
    // let mut result:String;
    let mut stack = String::new();
    let mut inBracket = false;
    
    for i in 0..s_vec.len() {
        if inBracket == false {
            if s_vec[i] != String::from("(") {
                result.push_str(s_vec[i]);
            }
            else {
                inBracket = true;
            }
        }
        else {
            inBracket = true;
            if s_vec[i] != String::from(")") {
                stack.push_str(s_vec[i]);
            }
            else {
                let mut stack_full = stack.clone();
                let mut stack_vec: Vec<&str> = stack_full.split("").collect();
                stack_vec.remove(0);
                stack_vec.remove(stack.len());

                for j in 0..stack_full.len() {
                    let mut rev_index = stack_full.len()-1-j;
                    result.push_str(stack_vec[rev_index]);
                }
               
                stack.clear();
                inBracket = false;
            }
        }
    }
    result
}
