fn almostIncreasingSequence(sequence: Vec<i32>) -> bool {
    
    let mut result = true;

    fn check (sequence: &Vec<i32>) -> bool {
        let mut result = true ;
        let mut last = sequence[0];
        for i in 1..sequence.len() {
            if last >= sequence[i] {
                result = false;
                break;
            }
            last = sequence[i];
        }
        result
    }
    
    fn truncate (mut sequence: Vec<i32>) -> Vec<i32> {
        let mut last = sequence[0];
        let mut cnt = 0;
        for i in 1..sequence.len() {
            if last >= sequence[i] {
                cnt = i; 
                break;
            }
            last = sequence[i];
        }
        if cnt > 0 {
            
            if cnt > 1 {
                if sequence[cnt] > sequence[cnt-2] {
                    sequence.remove(cnt-1);
                }
                else {
                    sequence.remove(cnt);    
                }
            }
            else {
                sequence.remove(cnt-1);
            }
            
        }
        sequence
    }
      

    if check(&sequence) == false {
        let seq_two = truncate(sequence);
        result = check(&seq_two);
    } 
    
    result
}


