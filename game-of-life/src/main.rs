extern crate rand;

//init with random
fn init(board: &mut[[bool; 10];10]) {
    for x in 0..10 {
        for y in 0..10 {
            let mut rnd = rand::random::<u8>();
            if rnd < 196 {
                board[x][y] = false;
            }
            else {
                board[x][y] = true;
            }
        }  
    }
}

/*fn mutate(board: &mut[[bool; 10];10]) {
    for x in 0..10 {
        for y in 0..10 {
           //implement
        }  
    }
}*/

//display board on screen
fn print (board: [[bool; 10];10]) {
    for x in 0..10 {
        for y in 0..10 {
            if board[x][y] == true {
                print!("X");
            }
            else {
                print!("_");
            }
        }  
        println!("");
    }
}

fn main() {    
    //init board
    let mut board: [[bool; 10];10] = [[false;10];10];

    init(&mut board);
    print(board);
   
}
