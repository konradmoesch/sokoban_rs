use std::fmt;
use std::fmt::Display;
use ggez::event::KeyCode;
use specs::World;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

pub enum GameplayState {
    Playing,
    Won
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
}