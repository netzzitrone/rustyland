fn allLongestStrings(inputArray: Vec<String>) -> Vec<String> {
    let mut max_len = 0;
    let mut result = Vec::new();
    
    for i in 0..inputArray.len() {
        if inputArray[i].len() > max_len {
            max_len = inputArray[i].len();
        }
    }
    
    for i in 0..inputArray.len() {
        if inputArray[i].len() == max_len {
            result.push(inputArray[i].clone());
        }
    }
    result
}