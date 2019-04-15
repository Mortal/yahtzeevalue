use std::fmt;
use crate::constants::*;

#[derive(Clone, Copy, PartialEq)]
pub struct State {
    pub combination_mask: u16,
    pub sides_mask: u8,
    pub score: u32,
}

impl State {
    pub fn decode(s: u32) -> Self {
        let combination_mask = (s & 0xFFF) as u16;
        let sides_mask = ((s >> 12) & 0x3F) as u8;
        let score = s >> 18;
        State {
            combination_mask: combination_mask,
            sides_mask: sides_mask,
            score: score,
        }
    }

    pub fn encode(&self) -> u32 {
        (self.combination_mask as u32)
            | ((self.sides_mask as u32) << 12)
            | (self.score << 18)
    }

    pub fn initial() -> State {
        State {
            combination_mask: 0,
            sides_mask: 0,
            score: 0,
        }
    }

    pub fn all_sides() -> State {
        State {
            combination_mask: 0,
            sides_mask: 0x3F,
            score: 0,
        }
    }

    pub fn has_side(&self, side: usize) -> bool {
        self.sides_mask & (1 << side) != 0
    }

    pub fn with_side(&self, side: usize) -> State {
        State {
            combination_mask: self.combination_mask,
            sides_mask: self.sides_mask | (1 << side),
            score: self.score,
        }
    }

    pub fn has_comb(&self, comb: Comb) -> bool {
        self.combination_mask & (1 << comb) != 0
    }

    pub fn with_comb(&self, comb: Comb) -> State {
        State {
            combination_mask: self.combination_mask | (1 << comb),
            sides_mask: self.sides_mask,
            score: self.score,
        }
    }

    pub fn with_score(&self, score: u32) -> State {
        State {
            combination_mask: self.combination_mask,
            sides_mask: self.sides_mask,
            score: score,
        }
    }

    pub fn upper_bound_points(&self) -> u32 {
        let mut ub = 0;
        let mut score = self.score;
        for d in 0..SIDES {
            if self.has_side(d) { continue; }
            let s = (d as u32 + 1) * (DICE_COUNT as u32);
            if score < BONUS_LIMIT && score + s >= BONUS_LIMIT {
                score = BONUS_LIMIT;
                ub += BONUS;
            } else {
                score += s;
            }
            ub += s;
        }
        if !self.has_comb(S2) { ub += 2 * SIDES as u32; }
        if !self.has_comb(S22) { ub += 4 * SIDES as u32 - 2; }
        if !self.has_comb(S222) { ub += 6 * SIDES as u32 - 6; }
        if !self.has_comb(S3) { ub += 3 * SIDES as u32; }
        if !self.has_comb(S4) { ub += 4 * SIDES as u32; }
        if !self.has_comb(S33) { ub += 6 * SIDES as u32 - 3; }
        if !self.has_comb(R15) { ub += 1 + 2 + 3 + 4 + 5; }
        if !self.has_comb(R26) { ub += 2 + 3 + 4 + 5 + 6; }
        if !self.has_comb(R16) { ub += 30; }
        if !self.has_comb(S23) { ub += 5 * SIDES as u32 - 2; }
        if !self.has_comb(CHANCE) { ub += DICE_COUNT as u32 * SIDES as u32; }
        if !self.has_comb(YAHTZEE) { ub += 100 + DICE_COUNT as u32 * SIDES as u32; }
        // 42 * 6 - 13 + 15 + 20 + 30 + 100 + 126 + 50 = 580
        ub
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State {{ combination_mask: 0x{:04x}, sides_mask: 0x{:02x}, score: {} }}",
            self.combination_mask, self.sides_mask, self.score)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut score = self.score as i32;
        for d in 0..SIDES {
            if self.has_side(d) {
                write!(f, "{}", d + 1)?;
                score -= 4 * (d as i32 + 1);
            } else {
                write!(f, "-")?;
            }
        }
        if score >= 0 {
            score = BONUS as i32;
        }
        write!(f, " {:+3} ", score)?;
        let symbols = "PDTVQWsSCH?!";
        for (i, c) in symbols.chars().enumerate() {
            if self.has_comb(i) {
                write!(f, "{}", c)?;
            } else {
                write!(f, "-")?;
            }
        }
        Ok(())
    }
}