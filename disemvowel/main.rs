use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::env;

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn disemvowel(s: &str) -> String {
    s.chars().filter(|&c| !is_vowel(c)).collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut stdout = io::stdout();

    for line in reader.lines() {
        let line = line?;
        let result = disemvowel(&line);
        writeln!(stdout, "{}", result)?;
    }

    Ok(())
}