mod core;

use core::{Instruction, VM, VMError};

fn main() {
    // Simple sanity check: compute (3 + 4) * 5 = 35
    let program = vec![
        Instruction::PushConst(3),
        Instruction::PushConst(4),
        Instruction::Add,
        Instruction::PushConst(5),
        Instruction::Mul,
        Instruction::Halt,
    ];
    let mut vm = VM::new(program);
    let result = vm.run().expect("VM should not error");
    assert_eq!(result, vec![35]);

    // Verify that a division by zero yields an error.
    let div_by_zero = vec![
        Instruction::PushConst(10),
        Instruction::PushConst(0),
        Instruction::Div,
        Instruction::Halt,
    ];
    let mut vm = VM::new(div_by_zero);
    let err = vm.run().unwrap_err();
    assert_eq!(err, VMError::DivisionByZero);

    // Verify stack manipulation instructions.
    let stack_ops = vec![
        Instruction::PushConst(1),
        Instruction::Dup,          // [1,1]
        Instruction::PushConst(2), // [1,1,2]
        Instruction::Swap,         // [1,2,1]
        Instruction::Add,          // [1,3]
        Instruction::Neg,          // [1,-3]
        Instruction::Pop,          // [1]
        Instruction::Halt,
    ];
    let mut vm = VM::new(stack_ops);
    let result = vm.run().expect("stack ops should succeed");
    assert_eq!(result, vec![1]);

    // Ensure that an empty program errors on step.
    let empty = vec![];
    let mut vm = VM::new(empty);
    let err = vm.step().unwrap_err();
    assert_eq!(err, VMError::ProgramCounterOutOfBounds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_sequence() {
        // (7 - 2) * (3 + 4) = 5 * 7 = 35
        let prog = vec![
            Instruction::PushConst(7),
            Instruction::PushConst(2),
            Instruction::Sub,
            Instruction::PushConst(3),
            Instruction::PushConst(4),
            Instruction::Add,
            Instruction::Mul,
            Instruction::Halt,
        ];
        let mut vm = VM::new(prog);
        let stack = vm.run().unwrap();
        assert_eq!(stack, vec![35]);
    }

    #[test]
    fn test_underflow_add() {
        let prog = vec![Instruction::Add, Instruction::Halt];
        let mut vm = VM::new(prog);
        let err = vm.run().unwrap_err();
        assert_eq!(err, VMError::StackUnderflow);
    }

    #[test]
    fn test_negation() {
        let prog = vec![
            Instruction::PushConst(42),
            Instruction::Neg,
            Instruction::Halt,
        ];
        let mut vm = VM::new(prog);
        let stack = vm.run().unwrap();
        assert_eq!(stack, vec![-42]);
    }

    #[test]
    fn test_multiple_halts() {
        // Halt before the second push; second push never executed.
        let prog = vec![
            Instruction::PushConst(1),
            Instruction::Halt,
            Instruction::PushConst(2),
        ];
        let mut vm = VM::new(prog);
        let stack = vm.run().unwrap();
        assert_eq!(stack, vec![1]);
    }

    #[test]
    fn test_swap_and_dup() {
        let prog = vec![
            Instruction::PushConst(10),
            Instruction::PushConst(20),
            Instruction::Swap, // [20,10]
            Instruction::Dup,  // [20,10,10]
            Instruction::Add,  // [20,20]
            Instruction::Halt,
        ];
        let mut vm = VM::new(prog);
        let stack = vm.run().unwrap();
        assert_eq!(stack, vec![20, 20]);
    }
}