fn adjacentElementsProduct(inputArray: Vec<i32>) -> i32 {
    let mut product: i32 = 1000*1000*-1;
    for i in 0..inputArray.len()-1 {
            if inputArray[i] * inputArray[i+1] > product {
                product = inputArray[i] * inputArray[i+1];
        }
    }
    product
}
