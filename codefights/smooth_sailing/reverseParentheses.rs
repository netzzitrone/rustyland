fn reverseParentheses(s: String) -> String {
    //TODO: actualy it reverts each part separatly eg. ab(dc(fe)hg)ij -> abcdefghij but it should be abghfecdij
    let mut result = String::new();
    let mut s_vec: Vec<&str> = s.split("").collect();
    s_vec.remove(0);
    s_vec.remove(s.len());

    //TODO realize dynamic stack
    let mut stack:Vec<String> = vec![String::from(""),String::from(""),String::from(""),String::from(""),String::from("")];
    let mut inBracket = false;
    let mut bracket_count = 0;
    
    for i in 0..s_vec.len() {
        if inBracket == false {
            if s_vec[i] != String::from("(") {
                result.push_str(s_vec[i]);
            }
            else {
                inBracket = true;
                bracket_count = 1;
            }
        }
        else {
            if (s_vec[i] == String::from(")")) || (s_vec[i] == String::from("(")) {
                let mut stack_full = stack.clone();
                let mut stack_vec: Vec<&str> = stack_full[bracket_count].split("").collect();
                stack_vec.remove(0);
                stack_vec.remove(stack[bracket_count].len());

                for j in 0..stack_full[bracket_count].len() {
                    let mut rev_index = stack_full[bracket_count].len()-1-j;
                    result.push_str(stack_vec[rev_index]);
                }
            
                stack[bracket_count].clear();
                
                if s_vec[i] == String::from("(") {
                    bracket_count = bracket_count + 1;    
                }    
                else {
                    bracket_count = bracket_count - 1; 
                }
            }
            else {
                stack[bracket_count].push_str(s_vec[i]);  
            }
            if bracket_count  == 0 {
                inBracket = false;
            }
        }
    }
    result
}
