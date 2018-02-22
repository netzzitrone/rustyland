fn isLucky(n: i32) -> bool {
    let mut first_sum:i32 = 0;
    let mut second_sum:i32 = 0;
    let s = n.to_string();
    
    let mut v: Vec<&str> = s.split("").collect();
    //workaround. remove empty entrys
    v.remove(0);
    v.remove(s.len());
    
    let mid = (s.len() / 2);
    for i in 0..mid {
        first_sum = first_sum + v[i].parse::<i32>().unwrap();
    }
    for i in mid..s.len() {
        second_sum = second_sum + v[i].parse::<i32>().unwrap();
    }
    (first_sum == second_sum)
}