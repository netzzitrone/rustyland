fn commonCharacterCount(s1: String, s2: String) -> i32 {
    let mut v1: Vec<&str> = s1.split("").collect();
    let mut v2: Vec<&str> = s2.split("").collect();
    //workaround. remove empty entrys
    v1.remove(0);
    v2.remove(0);
    v1.remove(s1.len());
    v2.remove(s2.len());
    
    let mut cnt = 0;
    
    for i in 0..v1.len() {
        for j in 0..v2.len() {
            if v1[i] == v2[j] {
                cnt = cnt + 1;
                v2.remove(j);
                break; 
            }
        }
    }
    cnt
}
