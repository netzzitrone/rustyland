fn sortByHeight(a: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = a.clone();
    let mut sorted_treeles: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();
    let mut people_index = 0;
    
    //sort order ascending
    sorted.sort();
    
    //remove trees from sorted vec
    for j in 0..sorted.len() { 
        if sorted[j] > -1 {
            sorted_treeles.push(sorted[j]);
        }
    }
    
    //merge original tree position with sorted people position
    for i in 0..a.len() {
        if a[i] == -1 {
           result.push(-1);     
        }
        else {
            result.push(sorted_treeles[people_index]);
            people_index = people_index + 1;
        }
    }
    
    result
}