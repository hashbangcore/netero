use std::io::{self, IsTerminal, Read};

/// Reads all stdin content when input is piped, otherwise returns empty string.
pub fn get_stdin() -> String {
    let mut input = String::new();

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    input
}

/// Returns true when stdin comes from a pipe.
pub fn stdin_is_piped() -> bool {
    !io::stdin().is_terminal()
}
