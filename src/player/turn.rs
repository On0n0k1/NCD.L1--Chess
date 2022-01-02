use crate::{
    pieces::{
        piece::{
            Color,
        },
    },
    player::errors::ErrorResponse,
};

#[derive(Clone)]
pub struct Turn{
    // What player is currently running.
    current_player: Color,
    // How many turns this game went. If it reaches 255, calls reset.
    value: u8,
    // A list of each step taken since the start of the round.
    // It's public so we don't have to copy when retrieving value. Just access directly.
    // pub steps: Vec<Step>,
    // will implement later
    black_check: bool,
    white_check: bool,
    checkmate: bool,
}


impl Turn{
    pub fn new(
        current_player: bool,
        value: u8,
    ) -> Self {
        let current_player = match current_player{
            false => Color::WHITE,
            true => Color::BLACK,
        };

        Turn{
            current_player,
            value,
            // steps: Vec::new(),
            black_check: false,
            white_check: false,
            checkmate: false,
        }
    }

    pub fn get_current_player_color(&self) -> Color {
        self.current_player.clone()
    }

    pub fn get_current_player_boolean(&self) -> bool{
        match self.current_player {
            Color::WHITE => false,
            Color::BLACK => true,
            _ => panic!("Error in turn.get_current_player_boolean. Own color is Empty."),
        }
    }
    
    pub fn get_value(&self) -> u8 {
        self.value.clone()
    }

    pub fn is_black_check(&self) -> bool {
        self.black_check.clone()
    }

    pub fn is_white_check(&self) -> bool {
        self.white_check.clone()
    }

    pub fn is_checkmate(&self) -> bool {
        self.checkmate.clone()
    }

    pub fn set_black_check(&mut self) {
        self.black_check = true;
    }

    pub fn unset_black_check(&mut self) {
        self.black_check = false;
    }

    pub fn set_white_check(&mut self) {
        self.white_check = true;
    }

    pub fn unset_white_check(&mut self) {
        self.white_check = false;
    }

    pub fn set_checkmate(&mut self) {
        self.checkmate = true;
    }

    pub fn next_turn(&mut self) -> Result<(), ErrorResponse> {
        let current_player: Color = self.current_player;

        self.current_player = match current_player{
            Color::EMPTY => panic!("Error in Turn.next_player. Own color is Empty."),
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };

        self.value += 1;
        
        // Won't store more than 128 steps, which is two or three times the average length of games.
        // if self.turn <= 128{
        //     self.steps.push(next_step);
        // }
        
        // 255 turns is way over the top. 
        // Probably an AI that doesn't know what to do.
        if self.value == 255{
            return Result::Err(ErrorResponse::GameOver)
        }

        Result::Ok(())
    }

}