pub enum OperatorType {
    Plus,
    Minux,
    Times,
    Divide
}

pub enum BracketType {
    Round,
    Curly,
    Square,
    Angle
}

pub enum BracketDirection {
    Left,
    Right
}

pub enum SentinelType {
    Start,
    End
}

pub enum Token {
    Sentinel(SentinelType),
    Number(usize),
    Operator(OperatorType),
    Bracket(BracketType, BracketDirection),
}

pub fn tokenize(input: &String) -> Vec<Token> {
    Vec::new()
}