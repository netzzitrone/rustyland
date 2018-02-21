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

//create a new generation
fn mutate(board: &mut[[bool; 10];10]) {
    //just do random stuff for testing
    let rand_init = rand::random::<u8>();
    for x in 0..10 {
        for y in 0..10 {
            let mut rnd = rand::random::<u8>();
            if rnd < rand_init {
                board[x][y] = false;
            }
            else {
                board[x][y] = true;
            }
        }  
    }
}

//render board
fn render(board: [[bool; 10];10]) -> String {
    let mut output = String::new();
    for x in 0..10 {
        for y in 0..10 {
            if board[x][y] == true {
                output.push_str("X");
            }
            else {
                output.push_str("_");
            }
        }  
        output.push_str("\n");
    }
    output
}

//display board on screen
fn print (board: [[bool; 10];10]) {
    let output = render(board);
    //@see http://ascii-table.com/ansi-escape-sequences-vt-100.php
    //move cursor to top left corner
    print!("\x1B[f");
    //Hide cursor while printing to avoid flickering
    print!("\x1B[?25l{}\x1B[?25h\n", output); 
}

fn main() {    
    //init board
    let mut board: [[bool; 10];10] = [[false;10];10];
    init(&mut board);
    //inital clear screen
    print!("\x1B[2J");
    for _loop_counter in 0..100000 {
        mutate(&mut board);
        print(board);
    }
}
