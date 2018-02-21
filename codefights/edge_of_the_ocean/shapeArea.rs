fn shapeArea(n: i32) -> i32 {
    let mut squares:i32 = 1;
    for i in 1..n+1 {
        squares = squares + (4*i)-4;
    }
    squares
}
