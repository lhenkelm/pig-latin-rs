use std::io;

use pig_latin::translate;

/// # Translate all lines recieved from stdin
///
/// Reads from stdin until end of file (EOF) then translates all of it,
/// printing the translation to stdout. Useful to translate large volumes
/// of text quickly.
///
/// ## Usage
///
/// ```shell
/// $ echo 'Hello World!
/// This is a second line. This is still the second line.
/// Even more lines. I wonder if there is a limit ...' | cargo run
/// ```
///
/// Which gives
/// ```output
/// Ellohay Orldway!
/// Isthay ishay ahay econdsay inelay. Isthay ishay illstay ethay econdsay inelay.
/// Evenhay oremay ineslay. Ihay onderway ifhay erethay ishay ahay imitlay ...
/// ```
/// ## Translation details
///
/// See the library crate [`pig_latin`]
fn main() -> io::Result<()> {
    let input_text = read_all_stdin()?;
    let translated = translate(&input_text);
    println!("{translated}");
    Ok(())
}

/// # Read every line from stdin into a new string buffer and return it
///
/// The read loop only ends when reaching EOF (ctrl-Z on windows)
fn read_all_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut input_text = String::new();
    loop {
        match stdin.read_line(&mut input_text) {
            Ok(0) => break,
            Err(error) => return Err(error),
            Ok(_) => continue,
        }
    }
    Ok(input_text)
}
