use board::{Board, Coord};

mod board;

fn main() {
    let width = 100;
    let height = 30;
    let mut board = Board::new(width, height);
    let seed: Vec<Coord> = (0..(width * height / 2))
        .into_iter()
        .map(|_| generate_random_coord(width, height))
        .collect();

    board.seed(seed);
    loop {
        clear_screen();
        print_board(board.cells(), board.generation(), board.population());
        wait_for_enter();
        board.update();
    }
}

fn generate_random_coord(width: usize, height: usize) -> Coord {
    (
        rand::random::<usize>() % (width - 1),
        rand::random::<usize>() % (height - 1),
    )
}

fn print_board(board: &Vec<Vec<bool>>, generation: usize, population: usize) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!(
                "{}",
                match board[i][j] {
                    true => 'O',
                    false => ' ',
                }
            );
        }
        println!("");
    }
    println!("Generation: {}", generation);
    println!("Population: {}", population);
}

fn clear_screen() {
    println!("\x1B[2J\x1B[H");
}

fn wait_for_enter() {
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
}
