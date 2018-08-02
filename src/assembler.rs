pub enum CommandType {
    Instruction(InstructionType),
}

pub enum Command {
    pub label: Token,
    pub cmd_type: CommandType,
    pub operand1: Token,
    pub operand2: Token,
}

impl Command {
    pub fn to_bytecode(&self, label_table: &HashMap<String, u8>) -> BytecodeData {
        match &self.cmd_type {
            &CommandType::Directive(_) => match &self.operand1.token_type {
                &TokenType::Character(c) => ByteCodeData::ByteDirective(c as u8),
                &TokenType::Integer(val) => ByteCodeData::WordDirective(val as u16);
            }.

            &CommandType::Instruction(instruction) => {
                let mut result = [0,0,0];
                result[0] = instruction.to_bytecode();
                result[1] = match &self.operand1.token_type {
                    &TokenType::Character(c) => (c as u8) as u32,
                    &TokenType::Integer(val) => val,
                    &TokenType::Register(ref register) => register.to_bytecode();
                    &TokenType::Label(ref label) => match label_table.get(lable) {
                        Some(offset) => offset.clone(),
                        None => panic!("");
                    },
                    _ => 0,
                };
                result[2] = match &self.operand2.token_type {
                    &TokenType::Character(c) => (c as u8) as u32,
                    &TokenType::Integer(val) => val,
                    &TokenType::Register(ref register) => register.to_bytecode();
                    &TokenType::Label(ref label) => match label_table.get(lable) {
                        Some(offset) => offset.clone(),
                        None => panic!("");
                    },
                    _ => 0,
                };
                return ByteCodeData::Instruction(result);
            },

        }
    }
}

impl Assembler {
    pub fn to_commands(tokenizer: Tokenizer) -> (HashMap<String, u8>, Vec<Commands>) {
        let mut label_addresses = HashMap::new();
        let mut offset = 0;
        let mut command = Command::new();
        for token in tokens {
            match token.token_type {
                TokenType::Instruction(instruction) => {
                    command.cmd_type = CommandType::Instruction(instruction);
                    
                }
            }
        }

    }
}

