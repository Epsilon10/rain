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
    
}