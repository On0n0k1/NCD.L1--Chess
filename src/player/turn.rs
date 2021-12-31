use crate::{
    pieces::{
        piece::{
            Color,
        },
    },
    player::errors::ErrorResponse,
};


pub struct Turn{
    // What player is currently running.
    current_player: Color,
    // How many turns this game went. If it reaches 255, calls reset.
    turn: u8,
    // A list of each step taken since the start of the round.
    // It's public so we don't have to copy when retrieving value. Just access directly.
    // pub steps: Vec<Step>,
    // will implement later
}


impl Turn{
    pub fn new(
        current_player: bool,
        turn: u8,
    ) -> Self {
        let current_player = match current_player{
            false => Color::WHITE,
            true => Color::BLACK,
        };

        Turn{
            current_player,
            turn,
            // steps: Vec::new(),
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
    
    pub fn get_turn(&self) -> u8 {
        self.turn.clone()
    }

    pub fn next_turn(&mut self) -> Result<(), ErrorResponse> {
        let current_player: Color = self.current_player;

        self.current_player = match current_player{
            Color::EMPTY => panic!("Error in Turn.next_player. Own color is Empty."),
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        };

        self.turn += 1;
        
        // Won't store more than 128 steps, which is two or three times the average length of games.
        // if self.turn <= 128{
        //     self.steps.push(next_step);
        // }
        
        // 255 turns is way over the top. 
        // Probably an AI that doesn't know what to do.
        if self.turn == 255{
            return Result::Err(ErrorResponse::GameOver)
        }

        Result::Ok(())
    }

}