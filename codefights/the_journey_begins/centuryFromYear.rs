fn centuryFromYear(year: i32) -> i32 {
    let mut result;
    result = year / 100;
    if (year % 100 != 0) {
            result = result + 1;
    }
    result
}