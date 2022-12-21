#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RockKind {
    Horizontal,
    Plus,
    Ell,
    Vertical,
    Square,
}

#[derive(Debug, Clone)]
pub struct Rock {
    x: i64,
    y: i64,
    kind: RockKind,
}

impl Rock {
    pub fn rock(&self) -> u16 {
        match self.kind {
            RockKind::Horizontal => 0xF000,
            RockKind::Plus => 0x2720,
            RockKind::Ell => 0x7440,
            RockKind::Vertical => 0x1111,
            RockKind::Square => 0x3300,
        }
    }
    pub fn jet(&mut self, jet: &Jet) {
        match jet {
            Jet::Left => self.x -= 1,
            Jet::Right => self.x += 1,
        }
    }
    pub fn fall(&mut self) {
        self.y -= 1;
    }
    pub fn next_kind(&self) -> RockKind {
        match self.kind {
            RockKind::Horizontal => RockKind::Plus,
            RockKind::Plus => RockKind::Ell,
            RockKind::Ell => RockKind::Vertical,
            RockKind::Vertical => RockKind::Square,
            RockKind::Square => RockKind::Horizontal,
        }
    }
}

/**
 * Struct representing the game state.
 */
#[derive(Debug, Clone)]
pub struct GameState {
    column: Vec<u8>,
    rock: Rock,
    floor: usize,
    height: usize,
}
impl GameState {
    pub fn new() -> GameState {
        GameState {
            column: vec![],
            rock: Rock {
                x: 2,
                y: 3,
                kind: RockKind::Horizontal,
            },
            floor: 0,
            height: 0,
        }
    }
    pub fn tower_size(&self) -> i64 {
        (self.height + self.column.iter().filter(|r| r != &&0).count()) as i64
    }
    pub fn record(&mut self) {
        let mut new_column = self.column.clone();
        new_column.resize(self.floor + 4 + 1, 0);
        let rock = self.rock.rock();
        for y in 0..4 {
            let y_offset = (3 - y) * 4;
            let row = ((rock & (0xf << y_offset)) >> y_offset) << self.rock.x;
            new_column[self.rock.y as usize + y] |= row as u8;
        }
        // trim column
        let mut acc = 0;
        let mut trim_at = 0;
        for (i, row) in new_column.iter().enumerate().rev() {
            acc |= row;
            if acc == 0x7f {
                trim_at = i;
                break;
            }
        }

        new_column = new_column[trim_at..].to_vec();
        self.height += trim_at;

        self.floor = new_column
            .iter()
            .enumerate()
            .find(|(_, &row)| row == 0)
            .unwrap_or((new_column.len(), &0))
            .0
            - 1;
        self.column = new_column[0..=self.floor].to_vec();
        let next_kind = self.rock.next_kind();
        self.rock = Rock {
            x: 2,
            y: 3 + 1 + self.floor as i64,
            kind: next_kind,
        };
    }
}

/**
 * Render the game state.
 * The column is seven characters wide. Ignoring the most significant bit of the row, print "#" for
 * 1 and " " for 0.
 * The rock is a 4x4 tile represented by 16 bits. Its x position is relative to the left side of the column,
 * and the y position is relative to the bottom of the column.
 */
#[allow(dead_code)]
pub fn render_game_state(state: &GameState) {
    let max_height = (state.column.len() as i64).max(state.rock.y + 4);
    for total_y in 0..max_height {
        let y = max_height - total_y - 1;
        let row = state.column.get(y as usize).unwrap_or(&0);
        print!("{:02} ", y);
        for x in 0..7 {
            print!(
                "{}",
                if x >= state.rock.x
                    && x < state.rock.x + 4
                    && y >= state.rock.y
                    && y < state.rock.y + 4
                {
                    if state.rock.rock()
                        & (1 << ((3 - (y - state.rock.y)) * 4 + (x - state.rock.x)))
                        != 0
                    {
                        "@"
                    } else {
                        " "
                    }
                } else if row & (1 << x) != 0 {
                    "#"
                } else {
                    "."
                }
            );
        }
        println!();
    }
    println!("   -------\n")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Jet {
    Left,
    Right,
}

pub fn test_collision(column: &[u8], rock: &Rock) -> bool {
    if rock.y < 0 || rock.x < 0 {
        // out of bounds of column
        return true;
    }

    let fill = &vec![0u8; 7][0..];

    let rows = &[column, fill].concat()[(rock.y as usize)..(rock.y as usize) + 4];
    for (y, row) in rows.iter().enumerate() {
        let row_bits = (0xff << rock.x) & row;
        let y_offset = (3 - y) * 4;
        let rock_bits = (((rock.rock() & (0xf << y_offset)) >> y_offset) << rock.x) as u8;
        let overflow_rock_bit = (rock_bits & 0x80) >> 7;
        if row_bits & rock_bits != 0 || overflow_rock_bit != 0 {
            return true;
        }
    }
    false
}

pub fn step_game_state(state: &mut GameState, jet: &Jet) {
    // handle jet
    let mut new_rock = state.rock.clone();
    new_rock.jet(jet);
    if !test_collision(&state.column, &new_rock) {
        state.rock = new_rock;
    }
    // handle fall
    let mut new_rock = state.rock.clone();
    new_rock.fall();
    if !test_collision(&state.column, &new_rock) {
        state.rock = new_rock;
    } else {
        // rock has landed
        state.record();
    }
}

pub fn step_game_state_until_rock_lands<'a>(
    state: &'a mut GameState,
    jets: &mut [Jet],
) -> &'a mut GameState {
    loop {
        let old_rock = state.rock.kind.clone();
        step_game_state(state, &jets[0]);
        jets.rotate_left(1);
        if old_rock != state.rock.kind {
            break;
        }
    }
    state
}
