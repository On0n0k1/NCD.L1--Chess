// pawn: forward 1-step                             
// rook: straight line                              
// knight: L shape                                  
// bishop: diagonal                                 
// queen: straight line + diagonal max distance     
// king: straight line + diagonal 1-step can't walk into self-checkmate

// empty:           0
// white pawn:      1
// white rook:      2
// white knight:    3
// white bishop:    4
// white queen:     5
// white king:      6
// black pawn:      7
// black rook:      8
// black knight:    9
// black bishop:    10
// black queen:     11
// black king:      12

// board size 8 x 8 (values in hex)
// 
// 8 9 A B C A 9 8 
// 7 7 7 7 7 7 7 7
// 0 0 0 0 0 0 0 0 
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 0 0 0 0 0 0 0 0
// 1 1 1 1 1 1 1 1 
// 2 3 4 5 6 4 3 2

// algorithm for move order:
//  - check if game is running
//  - check if movement is valid for given player
//  - attempt to move
//  - check if the player will be under check after move, if true, cancel the move
//  - after the player move, check if rival is under check,
//  - if true, check for all possible moves rival can do,
//  - if there are no moves remaining, checkmate.
//  - if there are moves remaining, on to the next turn.

// use pieces::piece::PieceActions;
pub mod pieces;
pub mod player;
pub mod board;
pub mod movement;
pub mod game;


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};

use game::Game;

near_sdk::setup_alloc!();


pub fn log(message: &str){
    env::log(message.as_bytes());
}



#[near_bindgen]
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Chess {
    game: Game,
}

#[near_bindgen]
impl Chess{
    // Reset the game to the beginning.
    pub fn reset_game(&mut self) -> String {
        self.game.reset_game_game()
    }

    // Return the state of the game.
    pub fn get_game_status(&self) -> Game{
        self.game.get_game_status_game()
    }

    // Get the name of a piece in given position.
    pub fn get_piece_name(&self, col: u8, row: u8) -> String{
        self.game.get_piece_name_game(col, row)
    }

    // Get the name of a piece in given index.
    pub fn get_piece_name_index(&self, index: u8) -> String {
        self.game.get_piece_name_index_game(index)
    }

    // Get the entire board as a string.
    pub fn get_board(&self) -> String {
        self.game.get_board_game()
    }

    
    // Move a piece from position "current" to "target" by column and row.
    pub fn move_to(
        &mut self,
        current_col: u8,
        current_row: u8,
        target_col: u8,
        target_row: u8,
    ) -> String {
        let current_index: u8 = current_row * 8 + current_col;
        let target_index: u8 = target_row * 8 + target_col;
        return self.move_to_index(
            current_index,              // current: u8,
            target_index,               // target: u8,
        );
    }

    // Move a piece from position "current" to "target" by index.
    pub fn move_to_index(
        &mut self,
        current: u8,
        target: u8,
    ) -> String {
        let response: String = self.game.move_to_game(
            current,
            target,
        );

        log(&response);
        response
    }
}
