use crate::Error::{GameComplete, NotEnoughPinsLeft};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
enum FrameType {
    Strike,
    Spare,
    Open,
}

impl FrameType {
    fn bonus_rolls(self) -> usize {
        match self {
            Self::Strike => 2,
            Self::Spare => 1,
            Self::Open => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Frame(Vec<u16>);

impl Frame {
    fn new() -> Self {
        Self(vec![])
    }

    fn add(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.pins_left() {
            return Err(NotEnoughPinsLeft);
        }

        self.0.push(pins);
        Ok(())
    }

    fn pins_left(&self) -> u16 {
        match self.0[..] {
            [] | [.., 10] => 10,
            [.., a, b] if a + b == 10 => 10,
            [.., last] => 10 - last,
        }
    }

    fn is_complete(&self, is_last: bool) -> bool {
        match (is_last, self.frame_type()) {
            (true, FrameType::Strike | FrameType::Spare) => self.0.len() == 3,
            (_, FrameType::Strike) => true,
            _ => self.0.len() == 2,
        }
    }

    fn pins(&self) -> u16 {
        self.0.iter().sum()
    }

    fn frame_type(&self) -> FrameType {
        match self.0[..] {
            [10, ..] => FrameType::Strike,
            [a, b, ..] if a + b == 10 => FrameType::Spare,
            _ => FrameType::Open,
        }
    }
}

#[derive(Default)]
pub struct BowlingGame {
    sheet: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_complete(&self) -> bool {
        self.sheet.len() == 10 && self.sheet.last().unwrap().is_complete(true)
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(GameComplete);
        }

        let is_last = self.sheet.len() == 10;

        match self.sheet.last_mut() {
            Some(current) if !current.is_complete(is_last) => current.add(pins)?,
            _ => {
                let mut frame = Frame::new();
                frame.add(pins)?;
                self.sheet.push(frame);
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let total = self
            .sheet
            .iter()
            .enumerate()
            .map(|(i, frame)| {
                let bonus: u16 = self.sheet[i + 1..]
                    .iter()
                    .flat_map(|f| f.0.iter())
                    .take(frame.frame_type().bonus_rolls())
                    .sum();
                frame.pins() + bonus
            })
            .sum();

        Some(total)
    }
}
