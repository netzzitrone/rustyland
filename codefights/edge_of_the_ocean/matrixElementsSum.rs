fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum:i32 = 0;

    for x in 0..matrix[0].len() {
        sum = sum + matrix[0][x];
    }
    if matrix.len() > 1 {
        for y in 1..matrix.len() {
            for x in 0..matrix[y].len() {
                //works unly with direct below empty rooms
                if matrix[y-1][x] > 0 {
                    sum = sum + matrix[y][x];
                }
            }
        }
    }
    sum
}