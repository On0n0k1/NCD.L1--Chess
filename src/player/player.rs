use crate::{
    board::Board,
    pieces::{
        piece::{
            Color,
            Piece,
        },
        piece_ext::PieceExt,
    },
    player::errors::ErrorResponse,
    movement::{
        step::Step,
        report::Report,
    }

};

pub struct Player{
    color: Color,    
    pieces: Vec<Piece>,
    latest_reports: Vec<Report>,
    check: bool,
    search_checkmate: bool,
}

impl Player{
    pub fn new(
        color: Color,
        board: &Board,
    ) -> Self{
        let mut player: Player = Player{
            color,
            pieces: Vec::with_capacity(16),
            latest_reports: Vec::new(),
            check: false,
            search_checkmate: false,
        };
        
        player.build_pieces(
            board,                                  // board: &Board,
        );

        player
    }

    fn unset_check(&mut self) {
        self.check = false;
    }

    pub fn is_check(&mut self) -> bool {
        self.check.clone()
    }

    pub fn set_check(&mut self) {
        self.check = true;
    }

    pub fn is_step_valid(&self, step: &Step) -> Result<bool, ErrorResponse> {
        // If search checkmate is true, then it won't store the positions in each report.
        // If it was a bug, then it would be a nightmare to track. That's why I'm panicking if it's set.
        if self.search_checkmate{
            panic!("Error in player.is_step_valid. search_checkmate is enabled, we will not find anything here.");
        }

        if self.color != step.current_piece.get_color() {
            return Result::Err(ErrorResponse::RivalPiece);
        }

        for report in &self.latest_reports{
            if report.is_step_here(
                step,                               // step: &Step,
            ){
                return Result::Ok(true);
            }
        }

        Result::Ok(false)
    }


    /// Check the board for all pieces of this Player's color.
    /// I don't like this function, we don't need to allocate this data. We can compute it directly without needing to store it's data.
    fn build_pieces(&mut self, board: &Board){        
        // Getting a reference (zero-copy) of color
        let color: &Color = &self.color;
        // Getting a mutable reference (zero-copy) of pieces
        let pieces: &mut Vec<Piece>= &mut self.pieces;

        match color{
            Color::BLACK => board.push_black_pieces(pieces),
            Color::WHITE => board.push_white_pieces(pieces),
            Color::EMPTY => panic!("Invalid color for player.build_pieces. Own color is Empty."),
        }
    }

    /// Build reports once (Own),
    /// If under check, search for a position that clears the check (build rival reports),
    /// Do the movement and build reports again (build rival reports)
    pub fn build_reports(
        &mut self, 
        board: &Board, 
        search_checkmate: bool,
    ){
        // If search checkmate is true, then it won't store the positions in each report.
        // If it was a bug, then it would be a nightmare to track. That's why I'm storing it.
        self.search_checkmate = search_checkmate;
        self.build_pieces(
            board,                                  // board: &Board,
        );

        self.unset_check();
        let mut check = false;
        let pieces: &Vec<Piece> = &self.pieces;
        let reports: &mut Vec<Report> = &mut self.latest_reports;

        for piece in pieces{
            let report: Report = piece.get_movement_report(
                board,                              // board: &Board,
                search_checkmate,                   // search_checkmate: bool,
            );

            if report.is_check(){
                // can't do this below due to borrow
                // self.set_check();
                // So we do this instead
                check = true;
            }

            if !search_checkmate{
                // Will not alloc any report if we are searching for checkmate
                reports.push(report);
            }
            
        }
        // Reference is not used again, so we can use mutable borrow here.
        if check {
            self.set_check();
        }
    }

    pub fn get_movement_report(&self) -> Vec<Report> {
        self.latest_reports.clone()
    }

    pub fn can_avoid_checkmate(
        &mut self,
        board: &mut Board,
    ) -> bool {
        // If search checkmate is true, then it won't store the positions in each report.
        // If it was a bug, then it would be a nightmare to track. That's why I'm panicking if it's set.
        if self.search_checkmate{
            panic!("Error in player.can_avoid_checkmate. search_checkmate is enabled, we will not find anything here.");
        }

        for report in &mut self.latest_reports {
            if report.can_avoid_checkmate(
                board,                              // board: &mut Board,
            ) {
                // This means that there's at least one movement that will save the player from check status.
                // It's not checkmate yet.
                return true;
            }
        }

        false
    }
}

