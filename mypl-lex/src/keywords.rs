#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Keyword {
    Unsigned8, Unsigned32,
    Signed8, Signed32,
    ConstDecleration, VariableDecleration,
    UnionDecleration, RecordDecleration,
    If, Else,
    For, Loop,
}

pub fn create_all() -> Vec<Keyword> {
    use Keyword::*;

    vec![
        Unsigned8, Unsigned32,
        Signed8, Signed32,
        ConstDecleration, VariableDecleration,
        UnionDecleration, RecordDecleration,
        If, Else,
        For, Loop
    ]
}

impl Keyword {
    pub fn to_code(&self) -> &'static str {
        use Keyword::*;
        match self {
            Unsigned8 => "u8",
            Unsigned32 => "u32",
            Signed8 => "i8",
            Signed32 => "i32",    
            ConstDecleration => "cst",
            VariableDecleration => "var",
            UnionDecleration => "union",
            RecordDecleration => "record",
            For => "for",
            Loop => "loop",
            If => "if",
            Else => "else",
        }
    }
    // pub fn from_code(code: &str) -> Option<Keyword> {
    //     use Keyword::*;
    //     match code {
    //         "u8" => Some(Unsigned8),
    //         "u32" => Some(Unsigned32),
    //         "i8" => Some(Signed8),
    //         "i32" => Some(Signed32),
            
    //         _ => None,
    //     }
    // }
}