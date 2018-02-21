fn makeArrayConsecutive2(statues: Vec<i32>) -> i32 {
    let mut min:i32;
    let mut max:i32;
    min = statues[0];
    max = statues[0];
    for i in 0..statues.len() {
        if statues[i] < min {
           min = statues[i];
        }
        if statues[i] > max {
           max = statues[i];
        }
    }
    let mut count_missing = 0;
    let mut found;
    for j in min+1..max {
        found = false;
        for i in 0..statues.len() {
            if j == statues[i] {
                found = true;
                break;
            }
        }
        if found == false {
            count_missing = count_missing +1;
        }
    }
    count_missing
}
