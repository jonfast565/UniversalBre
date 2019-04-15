#[derive(Debug, Clone)]
pub struct AtomStatus {
    atom: char,
}

impl AtomStatus {
    pub fn init(atom: char) -> AtomStatus {
        AtomStatus { atom: atom }
    }

    pub fn is_dot(&self) -> bool {
        self.atom == '.'
    }

    pub fn is_digit(&self) -> bool {
        self.atom.is_digit(10)
    }

    pub fn is_whitespace(&self) -> bool {
        let whitespace_chars: Vec<char> = vec![' ', '\r', '\n', '\t'];
        whitespace_chars.contains(&self.atom)
    }

    pub fn is_newline_char(&self) -> bool {
        let newline_chars: Vec<char> = vec!['\r', '\n'];
        newline_chars.contains(&self.atom)
    }

    pub fn is_break_char(&self) -> bool {
        let breaking_chars: Vec<char> = vec![
            '(', ')', '+', '-', '*', '/', '~', '.', '<', '>', '=', '&', '|', ';', '{', '}', ',',
            ']', '[', ':',
        ];
        breaking_chars.contains(&self.atom)
    }

    pub fn is_integer_break_char(&self) -> bool {
        let integer_breaking_chars: Vec<char> = vec![
            '(', ')', '+', '-', '*', '/', '~', '<', '>', '=', '&', '|', ';', '{', '}', ',', ']',
            '[', ':',
        ];
        integer_breaking_chars.contains(&self.atom)
    }

    pub fn is_alpha(&self) -> bool {
        self.atom.is_alphabetic()
    }

    pub fn is_alpha_digit(&self) -> bool {
        self.is_alpha() || self.is_digit()
    }

    pub fn is_alpha_digit_underscore(&self) -> bool {
        self.is_alpha_digit() || self.is_underscore()
    }

    pub fn is_empty(&self) -> bool {
        self.atom == '\0'
    }

    pub fn is_empty_or_whitespace(&self) -> bool {
        self.is_empty() || self.is_whitespace()
    }

    pub fn breaks_any(&self) -> bool {
        self.is_empty_or_whitespace() || self.is_break_char()
    }

    pub fn breaks_any_integer(&self) -> bool {
        self.is_empty_or_whitespace() || self.is_integer_break_char()
    }

    pub fn breaks_any_string(&self) -> bool {
        self.atom == '"'
    }

    pub fn is_underscore(&self) -> bool {
        self.atom == '_'
    }

    pub fn is_identifier_char(&self) -> bool {
        self.is_alpha_digit_underscore()
    }

    pub fn get_atom(&self) -> char {
        self.atom
    }
}
