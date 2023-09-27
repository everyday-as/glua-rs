#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Break,
    Do,
    Else,
    ElseIf,
    End,
    For,
    Function,
    If,
    In,
    Local,
    Repeat,
    Return,
    Then,
    Until,
    While,
    // GMod specific
    Continue,
    Goto,
}
