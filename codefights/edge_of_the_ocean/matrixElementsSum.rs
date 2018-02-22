fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum:i32 = 0;

    let mut haunted = Vec::new();
    
    for x in 0..matrix[0].len() {
        sum = sum + matrix[0][x];
        if matrix[0][x] == 0 {
            haunted.push(x);
        }
    }
    
    if matrix.len() > 1 {
        for y in 1..matrix.len() {
            for x in 0..matrix[y].len() {
                let mut is_haunted = false;
                for tmp in 0..haunted.len() {
                    if haunted[tmp] == x {
                        is_haunted = true;
                        break;
                    }
                }
                if matrix[y-1][x] > 0 {
                    if is_haunted == false {
                        sum = sum + matrix[y][x];
                    }
                }
                else {
                    haunted.push(x);
                }
            }
        }
    }
    sum
}