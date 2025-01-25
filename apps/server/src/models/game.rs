use rand::random;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    pub fn random() -> Self {
        if random::<i8>() % 2 == 0 {
            return Self::White;
        }

        Self::Black
    }

    pub fn choose(color_preference: Option<Self>) -> Self {
        match color_preference {
            Some(color) => color,
            None => PlayerColor::random(),
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Self::White => String::from("white_player"),
            Self::Black => String::from("black_player"),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Default)]
pub struct Game {
    pub id: Uuid,
    pub white_player: Option<Player>,
    pub black_player: Option<Player>,
    pub bet_value: i32,
    pub moves: Vec<String>,
}

impl Game {
    pub fn new_empty() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }

    pub fn to_game_record(self) -> GameRecord {
        GameRecord {
            id: self.id,
            white_player: self.white_player.map(|player| player.id),
            black_player: self.black_player.map(|player| player.id),
            bet_value: self.bet_value,
            moves: self.moves,
        }
    }
}

#[derive(Default)]
pub struct GameRecord {
    pub id: Uuid,
    pub white_player: Option<Uuid>,
    pub black_player: Option<Uuid>,
    pub bet_value: i32,
    pub moves: Vec<String>,
}

impl GameRecord {
    fn new_empty() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }

    pub fn new(player_id: Uuid, color_preference: PlayerColor) -> Self {
        let mut game_record = Self::new_empty();

        match color_preference {
            PlayerColor::White => game_record.white_player = Some(player_id),
            PlayerColor::Black => game_record.black_player = Some(player_id),
        };

        game_record
    }
}
