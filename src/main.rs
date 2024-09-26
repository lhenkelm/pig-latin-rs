use std::io;

use pig_latin::translate;

fn main()->io::Result<()>{
    let input_text = read_all_stdin()?;
    let translated = translate(&input_text);
    println!("{translated}");
    Ok(())
}

fn read_all_stdin() -> io::Result<String>{
    let stdin =  io::stdin();
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