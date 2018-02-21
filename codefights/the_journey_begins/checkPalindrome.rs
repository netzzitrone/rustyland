fn checkPalindrome(inputString: String) -> bool {
    let mut result = false;
    let mut reverseString: String;
    
    if inputString.len() == 1 {
        result = true;
    } 
    reverseString = inputString.chars().rev().collect::<String>();
    if inputString == reverseString {
        result = true;
    }
    result
}