use std::fmt;

/// Bytecode instructions for the stack VM.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Push a 64‑bit integer onto the stack.
    PushConst(i64),
    /// Pop two values, push their sum.
    Add,
    /// Pop two values, push left minus right.
    Sub,
    /// Pop two values, push their product.
    Mul,
    /// Pop two values, push left divided by right (integer division).
    Div,
    /// Negate the top of the stack.
    Neg,
    /// Duplicate the top of the stack.
    Dup,
    /// Swap the top two stack elements.
    Swap,
    /// Pop the top element and discard.
    Pop,
    /// Stop execution.
    Halt,
}

/// Errors that can arise during VM execution.
#[derive(Debug, Clone, PartialEq)]
pub enum VMError {
    /// Attempted to read past the end of the program.
    ProgramCounterOutOfBounds,
    /// Stack underflow for an operation requiring more elements.
    StackUnderflow,
    /// Division by zero.
    DivisionByZero,
    /// Encountered an unknown opcode (should never happen with the enum).
    UnknownOpcode,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::UnknownOpcode => write!(f, "Unknown opcode"),
        }
    }
}

/// The virtual machine executing a sequence of `Instruction`s.
#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    /// Program bytecode.
    pub program: Vec<Instruction>,
    /// Instruction pointer.
    pub ip: usize,
    /// Operand stack.
    pub stack: Vec<i64>,

}

impl VM {
    /// Create a new VM with the supplied program. The stack starts empty.
    pub fn new(program: Vec<Instruction>) -> Self {
        VM {
            program,
            ip: 0,
            stack: Vec::new(),
        }
    }

    /// Execute the next instruction. Returns `Ok(true)` if the VM halted,
    /// `Ok(false)` if execution should continue, or an error.
    pub fn step(&mut self) -> Result<bool, VMError> {
        if self.ip >= self.program.len() {
            return Err(VMError::ProgramCounterOutOfBounds);
        }

        let instr = self.program[self.ip].clone();
        self.ip += 1; // advance before execution to simplify jumps (none here)

        match instr {
            Instruction::PushConst(v) => {
                self.stack.push(v);
                Ok(false)
            }
            Instruction::Add => {
                let (b, a) = self.pop_two()?;
                self.stack.push(a.wrapping_add(b));
                Ok(false)
            }
            Instruction::Sub => {
                let (b, a) = self.pop_two()?;
                self.stack.push(a.wrapping_sub(b));
                Ok(false)
            }
            Instruction::Mul => {
                let (b, a) = self.pop_two()?;
                self.stack.push(a.wrapping_mul(b));
                Ok(false)
            }
            Instruction::Div => {
                let (b, a) = self.pop_two()?;
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                self.stack.push(a.wrapping_div(b));
                Ok(false)
            }
            Instruction::Neg => {
                let v = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.stack.push(-v);
                Ok(false)
            }
            Instruction::Dup => {
                let v = *self.stack.last().ok_or(VMError::StackUnderflow)?;
                self.stack.push(v);
                Ok(false)
            }
            Instruction::Swap => {
                let len = self.stack.len();
                if len < 2 {
                    return Err(VMError::StackUnderflow);
                }
                self.stack.swap(len - 1, len - 2);
                Ok(false)
            }
            Instruction::Pop => {
                self.stack.pop().ok_or(VMError::StackUnderflow)?;
                Ok(false)
            }
            Instruction::Halt => Ok(true),
        }
    }

    /// Run the program to completion (or error). Returns the final stack.
    pub fn run(&mut self) -> Result<Vec<i64>, VMError> {
        loop {
            let halted = self.step()?;
            if halted {
                return Ok(self.stack.clone());
            }
        }
    }

    /// Helper to pop two values, returning (right, left) as (b, a).
    fn pop_two(&mut self) -> Result<(i64, i64), VMError> {
        let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        Ok((b, a))
    }

    /// Expose the current stack for inspection (read‑only).
    pub fn stack(&self) -> &[i64] {
        &self.stack
    }
}