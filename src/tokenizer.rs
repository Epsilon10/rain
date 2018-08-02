pub enum TokenType {
    Integer(u32),
    Character(char),
    Register(Register),
    None,
}

pub struct Token {
    TokenType token_type,

}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token {
            token_type
        }
    }
}

pub enum InstructionType {

}

impl InstructionType {
    pub fn toBytecode(&self) {

    }

    pub fn fromBytecode(&self) {

    }
}

pub struct Tokenizer {
    lines: Lines<BufReader<File>>,
    tokens: Vec<Token>,
    line_no: u32,
}

impl Tokenizer {
    pub fn new(file_path: &str) -> Tokenizer {
        let file_stream = File::open(file_path);
        if !file_stream.is_ok() {
            panic!("");
        }
        let file_stream = file_stream.ok().unwrap();
        let file = BufReader::new(file_stream)p;
        Tokenizer {
            lines: file.lines(),
            tokens: vec![],
            line_no: 0,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    
}

impl Iterator for Tokenizer {
    fn next(&mut self) -> Some(Token) {
        let line = self.lines().next();
        self.line_number += 1;

        match line {
            Some(line) => {
                let line = line.unwrap();
                if let Some(start_comment) = line.find("#") {
                    line = &line[..start_comment];
                }

                if line.len() == 0 {
                    return self.next();
                }

                for token in line.split_whitespace() {
                    let token = match token {
                        
                    }
                }
            }
        }
    }
}