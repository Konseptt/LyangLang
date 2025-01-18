#[derive(Debug, Clone)]
pub enum Token {
    OiMug,
    BolMug,
    Mug,
    Identifier(String),
    Number(i32),
    String(String),
    Plus,
    Bhan,
    Jod,
    Ghata,  // New: for subtraction
    Lai,
    Equals,
    Comma,
    IsEquals,
    NotEquals,
    Yedi,     // Replace If with Yedi
    Bhane,    // Replace Then with Bhane
    Sakiyo,   // Replace End with Sakiyo
    Babaal, // For true
    Laamo,  // For false
    Aile,  // New: for else if
    Feri,  // New: for "feri" keyword
}
