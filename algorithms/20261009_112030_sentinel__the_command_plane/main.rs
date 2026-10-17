mod core;
use core::core::{CommandPlane};

fn main() {
    // Basic sanity check executed at runtime.
    let mut plane = CommandPlane::new();
    plane.add_move(2, 0);
    plane.add_turn(90);
    plane.add_move(0, 3);
    let (x, y, orientation) = plane.execute();
    assert_eq!((x, y, orientation), (2, 3, 90));
    // If the assertion passes, the program exits normally.
}

#[cfg(test)]
mod integration_tests {
    use super::core::core::CommandPlane;

    #[test]
    fn integration_execute() {
        let mut cp = CommandPlane::new();
        cp.add_move(1, 0);
        cp.add_turn(90);
        cp.add_move(0, 1);
        let (x, y, orient) = cp.execute();
        assert_eq!((x, y, orient), (1, 1, 90));
    }
}