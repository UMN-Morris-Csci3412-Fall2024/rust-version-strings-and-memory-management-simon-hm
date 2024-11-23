use std::io::{self, BufRead, Write};

fn str_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn is_palindrome(s: &str) -> bool {
    let rev = str_reverse(s);
    s == rev
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdin.lock();

    let size = 100;

    loop {
        let mut buffer = String::with_capacity(size);
        let bytes_read = handle.read_line(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        // Remove the newline character
        if let Some('\n') = buffer.chars().last() {
            buffer.pop();
        }

        let answer = if is_palindrome(&buffer) { "Yes" } else { "No" };

        writeln!(stdout, "Is the string <{}> a palindrome? {}", buffer, answer).unwrap();
    }
}