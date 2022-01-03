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
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};

use game::Game;

near_sdk::setup_alloc!();

#[cfg(not(test))]
pub fn log(message: &str){
    env::log(message.as_bytes());
}

#[cfg(test)]
pub fn log(message: &str){
    println!("{}", message);
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



#[cfg(test)]
mod tests {
    use super::*;
    // use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, MockedBlockchain};
    use crate::board::Board;

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        // VMContextBuilder::new()
        //     .signer_account_id("tester_near".to_string())
        //     .is_view(is_view)
        //     .build()

        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn board_equals(board: Vec<u8>, other: [u8; 64]) -> bool {
        assert_eq!(board.len(), 64, "Board Length is {}.", board.len());
        for index in 0..board.len() {
            if board[index] != other[index] {
                println!("{}", Board::get_board_string(
                    &board[..],
                ));

                return false;
            }
        }
        true
    }

    fn assert_move(
        contract: &mut Chess,
        start_col: u8,
        start_row: u8,
        end_col: u8,
        end_row: u8,
        counter: u8,
        squares: [u8; 64]
    ) {
        let context = get_context(vec![], false);
        testing_env!(context);

        println!("Step: {}", counter);
        println!("{}",
            &contract
                .move_to(
                    start_col, 
                    start_row, 
                    end_col,
                    end_row,
                )[..]
        );

        let context = get_context(vec![], true);
        testing_env!(context);

        let game_status: Game = contract.get_game_status();

        let board: Vec<u8> = game_status
            .get_squares();

        println!("{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n",
            "turn: ", game_status.get_turn(),
            "player_turn: ", game_status.get_player_turn(),
            "game_over: ", game_status.is_game_over(),
            "black_check: ", game_status.is_black_check(),
            "white_check: ", game_status.is_white_check(),
        );
        
        assert!(board_equals(board, squares));
    }

    fn assert_black_checkmate(
        contract: &mut Chess,
    ){
        let context = get_context(vec![], true);
        testing_env!(context);

        let game_status: Game = contract.get_game_status();
        let black_check: bool = game_status.is_black_check();
        let board: Vec<u8> = game_status.get_squares();
        println!("{}", Board::get_board_string(
            &board[..],
        ));
        
        assert!(black_check, "Black was supposed to be under checkmate here");
    }


    #[test]
    fn full_game() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Chess::default();
        let game: Game = contract.get_game_status();
        let board: Vec<u8> =  game.get_game_status_game()
            .get_squares();

        assert!(board_equals(board, 
            [
                8, 9, 10, 11, 12, 10, 9, 8,
                7, 7,  7,  7,  7,  7, 7, 7,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 0,  0,  0,  0,  0, 0, 0,
                1, 1,  1,  1,  1,  1, 1, 1,
                2, 3,  4,  5,  6,  4, 3, 2,
            ])
        );

        assert_move(
            &mut contract, 
            1, 
            6, 
            1, 
            4, 
            1,
            [
                8, 9, 10, 11, 12, 10, 9, 8,
                7, 7,  7,  7,  7,  7, 7, 7,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 1,  0,  0,  0,  0, 0, 0,
                0, 0,  0,  0,  0,  0, 0, 0,
                1, 0,  1,  1,  1,  1, 1, 1,
                2, 3,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            1, 
            1, 
            1, 
            3, 
            2,
            [
                8, 9, 10, 11, 12, 10, 9, 8,
                7, 0,  7,  7,  7,  7, 7, 7,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 7,  0,  0,  0,  0, 0, 0,
                0, 1,  0,  0,  0,  0, 0, 0,
                0, 0,  0,  0,  0,  0, 0, 0,
                1, 0,  1,  1,  1,  1, 1, 1,
                2, 3,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            1, 
            7, 
            2, 
            5, 
            3,
            [
                8, 9, 10, 11, 12, 10, 9, 8,
                7, 0,  7,  7,  7,  7, 7, 7,
                0, 0,  0,  0,  0,  0, 0, 0,
                0, 7,  0,  0,  0,  0, 0, 0,
                0, 1,  0,  0,  0,  0, 0, 0,
                0, 0,  3,  0,  0,  0, 0, 0,
                1, 0,  1,  1,  1,  1, 1, 1,
                2, 0,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            3, 
            1, 
            3, 
            2, 
            4,
            [
                8, 9, 10, 11, 12, 10, 9, 8,
                7, 0,  7,  0,  7,  7, 7, 7,
                0, 0,  0,  7,  0,  0, 0, 0,
                0, 7,  0,  0,  0,  0, 0, 0,
                0, 1,  0,  0,  0,  0, 0, 0,
                0, 0,  3,  0,  0,  0, 0, 0,
                1, 0,  1,  1,  1,  1, 1, 1,
                2, 0,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            0, 
            6, 
            0, 
            5, 
            5,
            [
                8, 9, 10, 11, 12, 10, 9, 8,
                7, 0,  7,  0,  7,  7, 7, 7,
                0, 0,  0,  7,  0,  0, 0, 0,
                0, 7,  0,  0,  0,  0, 0, 0,
                0, 1,  0,  0,  0,  0, 0, 0,
                1, 0,  3,  0,  0,  0, 0, 0,
                0, 0,  1,  1,  1,  1, 1, 1,
                2, 0,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            2, 
            0, 
            0, 
            2, 
            6,
            [
                8,  9,  0, 11, 12, 10, 9, 8,
                7,  0,  7,  0,  7,  7, 7, 7,
                10, 0,  0,  7,  0,  0, 0, 0,
                0,  7,  0,  0,  0,  0, 0, 0,
                0,  1,  0,  0,  0,  0, 0, 0,
                1,  0,  3,  0,  0,  0, 0, 0,
                0,  0,  1,  1,  1,  1, 1, 1,
                2,  0,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            7, 
            6, 
            7, 
            4, 
            7,
            [
                8,  9,  0, 11, 12, 10, 9, 8,
                7,  0,  7,  0,  7,  7, 7, 7,
                10, 0,  0,  7,  0,  0, 0, 0,
                0,  7,  0,  0,  0,  0, 0, 0,
                0,  1,  0,  0,  0,  0, 0, 1,
                1,  0,  3,  0,  0,  0, 0, 0,
                0,  0,  1,  1,  1,  1, 1, 0,
                2,  0,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            6, 
            1, 
            6, 
            2, 
            8,
            [
                8,  9,  0, 11, 12, 10, 9, 8,
                7,  0,  7,  0,  7,  7, 0, 7,
                10, 0,  0,  7,  0,  0, 7, 0,
                0,  7,  0,  0,  0,  0, 0, 0,
                0,  1,  0,  0,  0,  0, 0, 1,
                1,  0,  3,  0,  0,  0, 0, 0,
                0,  0,  1,  1,  1,  1, 1, 0,
                2,  0,  4,  5,  6,  4, 3, 2,
            ],
        );

        assert_move(
            &mut contract, 
            7, 
            7, 
            7, 
            5, 
            9,
            [
                8,  9,  0, 11, 12, 10, 9,  8,
                7,  0,  7,  0,  7,  7, 0,  7,
                10, 0,  0,  7,  0,  0, 7,  0,
                0,  7,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  0,  0,  0, 0,  1,
                1,  0,  3,  0,  0,  0, 0,  2,
                0,  0,  1,  1,  1,  1, 1,  0,
                2,  0,  4,  5,  6,  4, 3,  0,
            ],
        );

        assert_move(
            &mut contract, 
            5, 
            0, 
            7, 
            2, 
            10,
            [
                8,  9,  0, 11, 12,  0, 9,  8,
                7,  0,  7,  0,  7,  7, 0,  7,
                10, 0,  0,  7,  0,  0, 7, 10,
                0,  7,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  0,  0,  0, 0,  1,
                1,  0,  3,  0,  0,  0, 0,  2,
                0,  0,  1,  1,  1,  1, 1,  0,
                2,  0,  4,  5,  6,  4, 3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            3, 
            6, 
            3, 
            4, 
            11,
            [
                8,  9,  0, 11, 12,  0, 9,  8,
                7,  0,  7,  0,  7,  7, 0,  7,
                10, 0,  0,  7,  0,  0, 7, 10,
                0,  7,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  3,  0,  0,  0, 0,  2,
                0,  0,  1,  0,  1,  1, 1,  0,
                2,  0,  4,  5,  6,  4, 3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            3, 
            0, 
            3, 
            1, 
            12,
            [
                8,  9,  0,  0, 12,  0, 9,  8,
                7,  0,  7, 11,  7,  7, 0,  7,
                10, 0,  0,  7,  0,  0, 7, 10,
                0,  7,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  3,  0,  0,  0, 0,  2,
                0,  0,  1,  0,  1,  1, 1,  0,
                2,  0,  4,  5,  6,  4, 3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            2, 
            5, 
            1, 
            3, 
            13,
            [
                8,  9,  0,  0, 12,  0, 9,  8,
                7,  0,  7, 11,  7,  7, 0,  7,
                10, 0,  0,  7,  0,  0, 7, 10,
                0,  3,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  0,  0,  0,  0, 0,  2,
                0,  0,  1,  0,  1,  1, 1,  0,
                2,  0,  4,  5,  6,  4, 3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            0, 
            2, 
            1, 
            3, 
            14,
            [
                8,  9,  0,  0, 12,  0, 9,  8,
                7,  0,  7, 11,  7,  7, 0,  7,
                0,  0,  0,  7,  0,  0, 7, 10,
                0, 10,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  0,  0,  0,  0, 0,  2,
                0,  0,  1,  0,  1,  1, 1,  0,
                2,  0,  4,  5,  6,  4, 3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            3, 
            7, 
            3, 
            5, 
            15,
            [
                8,  9,  0,  0, 12,  0, 9,  8,
                7,  0,  7, 11,  7,  7, 0,  7,
                0,  0,  0,  7,  0,  0, 7, 10,
                0, 10,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  0,  5,  0,  0, 0,  2,
                0,  0,  1,  0,  1,  1, 1,  0,
                2,  0,  4,  0,  6,  4, 3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            1, 
            3, 
            3, 
            5, 
            16,
            [
                8,  9,  0,  0, 12,  0, 9,  8,
                7,  0,  7, 11,  7,  7, 0,  7,
                0,  0,  0,  7,  0,  0, 7, 10,
                0,  0,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  0, 10,  0,  0, 0,  2,
                0,  0,  1,  0,  1,  1, 1,  0,
                2,  0,  4,  0,  6,  4, 3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            4, 
            6, 
            3, 
            5, 
            17,
            [
                8,  9,  0,  0, 12,  0, 9,  8,
                7,  0,  7, 11,  7,  7, 0,  7,
                0,  0,  0,  7,  0,  0, 7, 10,
                0,  0,  0,  0,  0,  0, 0,  0,
                0,  1,  0,  1,  0,  0, 0,  1,
                1,  0,  0,  1,  0,  0, 0,  2,
                0,  0,  1,  0,  0,  1, 1,  0,
                2,  0,  4,  0,  6,  4, 3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            3, 
            1, 
            7, 
            5, 
            18,
            [
                8,  9,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  7, 10,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0, 11,
                0,  0,  1,  0,  0,  1,  1,  0,
                2,  0,  4,  0,  6,  4,  3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            6, 
            6, 
            7, 
            5, 
            19,
            [
                8,  9,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  7, 10,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                2,  0,  4,  0,  6,  4,  3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            7, 
            2, 
            2, 
            7, 
            20,
            [
                8,  9,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  7,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                2,  0, 10,  0,  6,  4,  3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            0, 
            7, 
            2, 
            7, 
            21,
            [
                8,  9,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  7,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  0,  2,  0,  6,  4,  3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            1, 
            0, 
            2, 
            2, 
            22,
            [
                8,  0,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  7,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  0,  2,  0,  6,  4,  3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            2, 
            7, 
            1, 
            7, 
            23,
            [
                8,  0,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  7,  0,
                0,  0,  0,  0,  0,  0,  0,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  3,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            6, 
            2, 
            6, 
            3, 
            24,
            [
                8,  0,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  0,
                0,  0,  0,  0,  0,  0,  7,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  3,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            6, 
            7, 
            5, 
            5, 
            25,
            [
                8,  0,  0,  0, 12,  0,  9,  8,
                7,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  0,
                0,  0,  0,  0,  0,  0,  7,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  3,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            0, 
            1, 
            0, 
            3, 
            26,
            [
                8,  0,  0,  0, 12,  0,  9,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  0,
                7,  0,  0,  0,  0,  0,  7,  0,
                0,  1,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  3,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            1, 
            4, 
            0, 
            3, 
            27,
            [
                8,  0,  0,  0, 12,  0,  9,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  0,
                1,  0,  0,  0,  0,  0,  7,  0,
                0,  0,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  3,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            6, 
            0, 
            7, 
            2, 
            28,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  9,
                1,  0,  0,  0,  0,  0,  7,  0,
                0,  0,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  3,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            5, 
            5, 
            4, 
            3, 
            29,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  9,
                1,  0,  0,  0,  3,  0,  7,  0,
                0,  0,  0,  1,  0,  0,  0,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            6, 
            3, 
            6, 
            4, 
            30,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  9,
                1,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  1,  0,  0,  7,  1,
                1,  0,  0,  1,  0,  0,  0,  1,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            7, 
            5, 
            6, 
            4, 
            31,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  9,
                1,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  1,  0,  0,  1,  1,
                1,  0,  0,  1,  0,  0,  0,  0,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            7, 
            2, 
            6, 
            4, 
            32,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  0,
                1,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  1,  0,  0,  9,  1,
                1,  0,  0,  1,  0,  0,  0,  0,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  4,  0,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            5, 
            7, 
            7, 
            5, 
            33,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  9,  7,  0,  0,  0,  0,
                1,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  1,  0,  0,  9,  1,
                1,  0,  0,  1,  0,  0,  0,  4,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  0,  0,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            2, 
            2, 
            3, 
            4, 
            34,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  0,  0,
                1,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  9,  0,  0,  9,  1,
                1,  0,  0,  1,  0,  0,  0,  4,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  0,  0,  0,
            ],
        );

        // white
        assert_move(
            &mut contract, 
            7, 
            5, 
            6, 
            4, 
            35,
            [
                8,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  0,  0,
                1,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  9,  0,  0,  4,  1,
                1,  0,  0,  1,  0,  0,  0,  0,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  0,  0,  0,
            ],
        );

        // black
        assert_move(
            &mut contract, 
            0, 
            0, 
            0, 
            3, 
            36,
            [
                0,  0,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  0,  0,
                8,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  9,  0,  0,  4,  1,
                1,  0,  0,  1,  0,  0,  0,  0,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  2,  0,  0,  6,  0,  0,  0,
            ],
        );

        // white
        // checkmate
        assert_move(
            &mut contract, 
            1, 
            7, 
            1, 
            0, 
            36,
            [
                0,  2,  0,  0, 12,  0,  0,  8,
                0,  0,  7,  0,  7,  7,  0,  7,
                0,  0,  0,  7,  0,  0,  0,  0,
                8,  0,  0,  0,  3,  0,  0,  0,
                0,  0,  0,  9,  0,  0,  4,  1,
                1,  0,  0,  1,  0,  0,  0,  0,
                0,  0,  1,  0,  0,  1,  0,  0,
                0,  0,  0,  0,  6,  0,  0,  0,
            ],
        );

        assert_black_checkmate(&mut contract);

        
    }
}