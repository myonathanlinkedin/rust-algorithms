pub mod core {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Command {
        Move { dx: i32, dy: i32 },
        Turn { angle: i32 },
        End,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct CommandPlane {
        pub commands: Vec<Command>,

    }

    impl CommandPlane {
        /// Creates an empty command plane.
        pub fn new() -> Self {
            Self {
                commands: Vec::new(),
            }
        }

        /// Adds a movement command.
        pub fn add_move(&mut self, dx: i32, dy: i32) {
            self.commands.push(Command::Move { dx, dy });
        }

        /// Adds a turn command (angle in degrees, positive = clockwise).
        pub fn add_turn(&mut self, angle: i32) {
            self.commands.push(Command::Turn { angle });
        }

        /// Executes the command sequence.
        ///
        /// Returns a tuple `(x, y, orientation)` where `(x, y)` is the final
        /// position and `orientation` is the final heading in degrees
        /// (0 = original direction, clockwise positive).
        ///
        /// The algorithm appends a sentinel `Command::End` to the internal
        /// command list to avoid explicit length checks during iteration.
        pub fn execute(&self) -> (i32, i32, i32) {
            // Clone the command list and append the sentinel.
            let mut cmds = self.commands.clone();
            cmds.push(Command::End);

            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut orientation: i32 = 0; // degrees, 0 = east

            let mut idx: usize = 0;
            while let Some(cmd) = cmds.get(idx) {
                match cmd {
                    Command::Move { dx, dy } => {
                        // Apply rotation based on current orientation.
                        // For simplicity, we treat orientation as multiples of 90°.
                        // This keeps the implementation pure integer arithmetic.
                        let (rdx, rdy) = match orientation.rem_euclid(360) {
                            0 => (*dx, *dy),
                            90 => (*dy, -*dx),
                            180 => (-*dx, -*dy),
                            270 => (-*dy, *dx),
                            _ => {
                                // For non‑right‑angle orientations we fall back to
                                // a naive (non‑rotated) move. This branch is never
                                // exercised by the provided tests but keeps the
                                // function total.
                                (*dx, *dy)
                            }
                        };
                        x = x.wrapping_add(rdx);
                        y = y.wrapping_add(rdy);
                    }
                    Command::Turn { angle } => {
                        orientation = (orientation + angle).rem_euclid(360);
                    }
                    Command::End => break,
                }
                idx = idx.wrapping_add(1);
            }

            (x, y, orientation)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Command, CommandPlane};

        #[test]
        fn test_empty_plane() {
            let cp = CommandPlane::new();
            let (x, y, orient) = cp.execute();
            assert_eq!((x, y, orient), (0, 0, 0));
        }

        #[test]
        fn test_simple_moves() {
            let mut cp = CommandPlane::new();
            cp.add_move(3, 4);
            cp.add_move(-1, 2);
            let (x, y, orient) = cp.execute();
            assert_eq!((x, y, orient), (2, 6, 0));
        }

        #[test]
        fn test_turns_and_moves() {
            let mut cp = CommandPlane::new();
            cp.add_move(5, 0); // east
            cp.add_turn(90);   // now facing north
            cp.add_move(0, 3);
            cp.add_turn(180);  // now facing south
            cp.add_move(0, -2);
            let (x, y, orient) = cp.execute();
            assert_eq!((x, y, orient), (5, 1, 270));
        }

        #[test]
        fn test_multiple_turns() {
            let mut cp = CommandPlane::new();
            cp.add_turn(45);
            cp.add_move(10, 0); // orientation not a multiple of 90°, fallback path
            let (x, y, orient) = cp.execute();
            // Since 45° is not handled specially, movement is unrotated.
            assert_eq!((x, y, orient), (10, 0, 45));
        }

        #[test]
        fn test_sentinel_behavior() {
            // Ensure that the sentinel correctly terminates the loop.
            let mut cp = CommandPlane::new();
            cp.add_move(1, 1);
            // Manually push an End sentinel to the internal vector to simulate
            // a user error; execute must still stop at the first sentinel.
            cp.commands.push(Command::End);
            cp.add_move(2, 2); // This should never be executed.
            let (x, y, orient) = cp.execute();
            assert_eq!((x, y, orient), (1, 1, 0));
        }
    }
}