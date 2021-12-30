use crate::{
    board::Board,
    movement::report::Report,
    pieces::{
        piece_ext::PieceExt,
        bishop::Bishop,
        empty::Empty,
        knight::Knight,
        king::King,
        pawn::Pawn,
        queen::Queen,
        rook::Rook,
    },
};

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


#[derive(Clone, Copy, PartialEq)]
pub enum Color{
    EMPTY,
    WHITE,
    BLACK,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece{
    BISHOP( Bishop ),
    EMPTY(  Empty  ),
    KING(   King   ),
    KNIGHT( Knight ),
    PAWN(   Pawn   ),
    QUEEN(  Queen  ),
    ROOK(   Rook   ),
}

impl Default for Piece{
    fn default() -> Self{
        Piece::new(
            0,      // piece_number: u8, 
            0,      // column: u8, 
            0,      // row: u8,
        )
    }
}

impl Piece{
    pub fn get_piece_name(
        piece_number: u8,
    ) -> String {
        let color: &str = {
            if piece_number == 0 {
                ""
            } else if piece_number < 7{
                "White"
            } else {
                "Black"
            }
        };

        let name: &str = match piece_number{
            0 => { "Empty" },
            1 | 7  => { " Pawn" },
            2 | 8  => { " Rook" },
            3 | 9  => { " Knight" },
            4 | 10 => { " Bishop" },
            5 | 11 => { " Queen" },
            6 | 12 => { " King" },
            _      => { panic!("Invalid argument for piece_number ({}) in Piece constructor.", piece_number);   },
        };

        format!("{}{}", color, name)
    }
}


impl PieceExt for Piece{
    fn new(
        piece_number: u8,
        column: u8, 
        row: u8,
    ) -> Self {
        match piece_number{
            0      => { return Piece::EMPTY(Empty::new(piece_number, column, row));                                },
            1 | 7  => { return Piece::PAWN(Pawn::new(piece_number, column, row));                               },
            2 | 8  => { return Piece::ROOK(Rook::new(piece_number, column, row));                               },
            3 | 9  => { return Piece::KNIGHT(Knight::new(piece_number, column, row));                           },
            4 | 10 => { return Piece::BISHOP(Bishop::new(piece_number, column, row));                           },
            5 | 11 => { return Piece::QUEEN(Queen::new(piece_number, column, row));                             },
            6 | 12 => { return Piece::KING(King::new(piece_number, column, row));                               },
            _      => { panic!("Invalid argument for piece_number ({}) in Piece constructor.", piece_number);   },
        }
    }

    fn get_piece_number(&self) -> u8{                
        match &self{
            Piece::EMPTY(value)  => { return value.get_piece_number(); },
            Piece::PAWN(value)   => { return value.get_piece_number(); },
            Piece::ROOK(value)   => { return value.get_piece_number(); },
            Piece::KNIGHT(value) => { return value.get_piece_number(); },
            Piece::BISHOP(value) => { return value.get_piece_number(); },
            Piece::QUEEN(value)  => { return value.get_piece_number(); },
            Piece::KING(value)   => { return value.get_piece_number(); },
        }
    }


    fn get_column(&self) -> u8 {
        match &self{
            Piece::EMPTY(value)  => { return value.get_column(); },
            Piece::PAWN(value)   => { return value.get_column(); },
            Piece::ROOK(value)   => { return value.get_column(); },
            Piece::KNIGHT(value) => { return value.get_column(); },
            Piece::BISHOP(value) => { return value.get_column(); },
            Piece::QUEEN(value)  => { return value.get_column(); },
            Piece::KING(value)   => { return value.get_column(); },
        }
    }

    fn get_row(&self) -> u8 {
        match &self{
            Piece::EMPTY(value)  => { return value.get_row(); },
            Piece::PAWN(value)   => { return value.get_row(); },
            Piece::ROOK(value)   => { return value.get_row(); },
            Piece::KNIGHT(value) => { return value.get_row(); },
            Piece::BISHOP(value) => { return value.get_row(); },
            Piece::QUEEN(value)  => { return value.get_row(); },
            Piece::KING(value)   => { return value.get_row(); },
        }
    }

    fn get_color(&self) -> Color{
        match &self{
            Piece::EMPTY(value)  => { return value.get_color(); },
            Piece::PAWN(value)   => { return value.get_color(); },
            Piece::ROOK(value)   => { return value.get_color(); },
            Piece::KNIGHT(value) => { return value.get_color(); },
            Piece::BISHOP(value) => { return value.get_color(); },
            Piece::QUEEN(value)  => { return value.get_color(); },
            Piece::KING(value)   => { return value.get_color(); },
        }
    }

    fn is_king(&self) -> bool {
        match &self{
            Piece::EMPTY(value)  => { return value.is_king(); },
            Piece::PAWN(value)   => { return value.is_king(); },
            Piece::ROOK(value)   => { return value.is_king(); },
            Piece::KNIGHT(value) => { return value.is_king(); },
            Piece::BISHOP(value) => { return value.is_king(); },
            Piece::QUEEN(value)  => { return value.is_king(); },
            Piece::KING(value)   => { return value.is_king(); },
        }
    }

    fn get_movement_report(
        &self,
        board: &Board, 
        search_checkmate: bool,
    ) -> Report {
        match &self{
            Piece::EMPTY(value)  => { return value.get_movement_report(&board, search_checkmate); },
            Piece::PAWN(value)   => { return value.get_movement_report(&board, search_checkmate); },
            Piece::ROOK(value)   => { return value.get_movement_report(&board, search_checkmate); },
            Piece::KNIGHT(value) => { return value.get_movement_report(&board, search_checkmate); },
            Piece::BISHOP(value) => { return value.get_movement_report(&board, search_checkmate); },
            Piece::QUEEN(value)  => { return value.get_movement_report(&board, search_checkmate); },
            Piece::KING(value)   => { return value.get_movement_report(&board, search_checkmate); },
        }
    }

}
