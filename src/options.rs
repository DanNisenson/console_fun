use std::io::{stdout, Write, Error};
use crossterm::{
    event::{read, Event, KeyCode}, 
    queue, 
    style, 
    terminal::{disable_raw_mode, enable_raw_mode}, 
};

pub fn choose_pokemon_by_number() {
    let mut stdout = stdout();
    let opts = ["Charmander", "Bulbasaur", "Squirtle"];

    queue!(stdout, style::Print("Select your pokemon")).expect("Bad queueing");
    for (i, opt) in opts.iter().enumerate() {
        queue!(stdout, style::Print(format!("\n{}: {}", i + 1, opt))).expect("Bad queueing");
    }
    queue!(stdout, style::Print("\n")).expect("Bad queueing");
    
    stdout.flush().expect("Bad flush");
    
   
    let i = handle_usr_input().expect("Bad user input");
    let i = i as usize;

    queue!(stdout, style::Print(format!("You selected: {}", opts[i - 1]))).expect("Bad queueing");
}

fn handle_usr_input() -> Result<u8, Error>  {
    loop {
        enable_raw_mode().expect("Bad enable raw mode");
        let event = read().expect("Bad user input read");
        disable_raw_mode().expect("Bad disable raw mode");

        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Char('1') {
                    return Ok(1);
                } else if key.code == KeyCode::Char('2') {
                    return Ok(2);
                } else if key.code == KeyCode::Char('3') {
                    return Ok(3);
                } else {
                    println!("Please select a valid option. Your selection: {:?}", key.code);
                }
            },
            _ => () 
        }    
    }
}
