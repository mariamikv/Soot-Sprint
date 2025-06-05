#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScoreType {
    FirstValue = 1000,
    SecondValue = 5000,
    ThirdValue = 10000,
    FourthValue = 20000,
    FifthValue = 50000,
}

impl ScoreType {
    pub fn value(self) -> u32 {
        self as u32
    }
}