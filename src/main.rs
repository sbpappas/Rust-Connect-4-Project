use std::io;

#[derive(Clone, PartialEq, Copy, Debug)]
enum Player {
    Red,
    Black,
}

struct Move {
    player: Player,
    column: i32,
}

impl Move {
    // reads a move from a column string
    fn read_move(c: String, player: &Player) -> Option<Move>{
        let trimmed_c = c.trim().parse::<i32>(); //trim the newline character and parse the int

        match trimmed_c{
            // if int parsed, return the move
            Ok(num) => {
                Some(Move{
                    player: player.clone(),
                    column: num,
                })
            },

            // return None
            Err(_) => {
                println!("Failed to parse the column!");
                None
            }
        }
    }
}

struct Board {
    gameBoard: Vec<Vec<Option<Player>>>,
}

impl Board {
    fn display(&self) -> (){ //prints out the board to the screen
        for i in 0..6{ //iterate through rows
            print!(" | ");
            for j in 0..7{//iterate thru cols
                match self.gameBoard[i][j] {
                    Some(Player::Red) => print!("X | "),
                    Some(Player::Black) => print!("O | "),
                    None => print!("- | "),
                }
            }
            println!("\n");
        }
    }
    
    fn new_board() -> Board {
        Board { 
            gameBoard: vec![vec![None; 7]; 6]
        }
    }

    fn update_board(&mut self, m: Move) {
        for i in (0..6).rev() {
            // use m.column-1 because user inputs a num from 1-7, we need 0-6
            let j: usize = usize::try_from(m.column - 1).unwrap(); 
            if self.gameBoard[i][j] == None {
                self.gameBoard[i][j] = Some(m.player);
                break;
            }
        }
    }
    

    fn is_full(&self) -> bool {
        for j in 0..=6{//move horizontally
            if self.gameBoard[0][j] == None { //go through the top row and find if any spot is open
                return false
            }
        }
        true
    }
    
    
    fn check_winner(&self) -> Option<Player> {
        let mut winner: Option<Player> = None;
        //horizontal check
        
        for i in 0..=3{
            for j in 0..=5{
                if self.gameBoard[j][i]== self.gameBoard[j][i+1] && self.gameBoard[j][i]== self.gameBoard[j][i+2] && self.gameBoard[j][i]== self.gameBoard[j][i+3] {
                    if self.gameBoard[j][i] != None {
                        winner = self.gameBoard[j][i]
                    }
                }
            }
        }
    
        //vertical check

        for i in 0..=2{
            for j in 0..=6{
                if self.gameBoard[i][j]== self.gameBoard[i+1][j] && self.gameBoard[i][j]== self.gameBoard[i+2][j] && self.gameBoard[i][j]== self.gameBoard[i+3][j] {
                    if self.gameBoard[i][j] != None {
                        winner = self.gameBoard[i][j]
                    }
                }
            }
        }

        //ascending diagonal check

        for i in 3..=5{
            for j in 0..=3{
                if self.gameBoard[i][j]== self.gameBoard[i-1][j+1] && self.gameBoard[i][j]== self.gameBoard[i-2][j+2] && self.gameBoard[i][j]== self.gameBoard[i-3][j+3] {
                    if self.gameBoard[i][j] != None {
                        winner = self.gameBoard[i][j]
                    }
                }
            }
        }

        //descending diagonal check

        for i in 3..=5{
            for j in 3..=6{
                if self.gameBoard[i][j]== self.gameBoard[i-1][j-1] && self.gameBoard[i][j]== self.gameBoard[i-2][j-2] && self.gameBoard[i][j]== self.gameBoard[i-3][j-3] {
                    if self.gameBoard[i][j] != None {
                        winner = self.gameBoard[i][j]
                    }
                }
            }
        }
        
        winner
    }

}


fn main() {
    // initialize a new game
    let mut game = Board::new_board();
    println!("Let's play Connect 4\n");
    game.display();
    // playing the game
    let mut current_player = Player::Red;
    let mut winner: Option<Player> = None;
    
    // start the main loop
    while winner == None && !game.is_full() {
        // read input
        let mut move_column = String::new();
        //if current_player == Player::Red {
            //println!("Input move for Red");
        //}
        //else println!("Input move for Black");

        // println!("Input move for {}: ", &current_player);

        // repeatedly scan input until valid
        let mut current_move: Option<Move> = None;
        while current_move.is_none() {
            let mut move_column = String::new();
            io::stdin().read_line(&mut move_column).expect("Failed to read line");
            current_move = Move::read_move(move_column, &current_player);
        }
        //unwrap the optional
        let current_move = current_move.unwrap();
        
        // update the board and display the board
        game.update_board(current_move);
        game.display();

        // change the current player
        current_player = match current_player{
            Player::Red => Player::Black,
            Player::Black => Player::Red,
        };

        // check to see if there is a winner
        winner = game.check_winner();
        println!("{:?}",winner);
    }

    // check to see if there is a winner or if there was a tie
    match winner {
        Some(player) => {println!("Winner is {:?}!", player)},
        None=> {println!("Tie!")}
    };   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_board() {
        let mut my_board = Board::new_board();
        let player_move = Move {
            column: 3,  // Replace with the desired column value
            player: Player::Red,  
        };
        let player_move2 = Move {
            column: 3,  
            player: Player::Black,  
        };

        my_board.update_board(player_move);
        my_board.update_board(player_move2);

        let expected_board = Board {
            gameBoard: vec![
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None],
                vec![None, None, Some(Player::Black), None, None, None, None],
                vec![None, None, Some(Player::Red), None, None, None, None],
            ],
        };
        assert_eq!(my_board.gameBoard, expected_board.gameBoard);
    }

    #[test]
    fn test_winner() {
        let mut my_board = Board::new_board();
        let player_move0 = Move {
            column: 7,
            player: Player::Red,  
        };
        let player_move1 = Move {
            column: 6,
            player: Player::Red,  
        };
        let player_move2 = Move {
            column: 5,
            player: Player::Red,  
        };
        let player_move3 = Move {
            column: 4,
            player: Player::Red,  
        };

        let player_move10 = Move {
            column: 7,
            player: Player::Black,  
        };
        let player_move11 = Move {
            column: 7,
            player: Player::Black,  
        };
        let player_move12 = Move {
            column: 7,
            player: Player::Black,  
        };
        let player_move13 = Move {
            column: 6,
            player: Player::Black,  
        };

        let player_move22 = Move {
            column: 6,
            player: Player::Black,  
        };
        let player_move23 = Move {
            column: 5,
            player: Player::Black,  
        };

        my_board.update_board(player_move10);
        my_board.update_board(player_move11);
        my_board.update_board(player_move12);
        my_board.update_board(player_move13);

        my_board.update_board(player_move22);
        my_board.update_board(player_move23);

        my_board.update_board(player_move0);
        my_board.update_board(player_move1);
        my_board.update_board(player_move2);
        my_board.update_board(player_move3);

        let expected_board = Board {
            gameBoard: vec![
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, Some(Player::Red)],
                vec![None, None, None, None, None, Some(Player::Red), Some(Player::Black)],
                vec![None, None, None, None, Some(Player::Red), Some(Player::Black), Some(Player::Black)],
                vec![None, None, None, Some(Player::Red), Some(Player::Black), Some(Player::Black), Some(Player::Black)],
            ],
        };

        let mut winner: Option<Player> = None;

        winner = my_board.check_winner();

        assert_eq!(my_board.gameBoard, expected_board.gameBoard);
        assert_eq!(expected_board.check_winner(), winner);
    }
}
