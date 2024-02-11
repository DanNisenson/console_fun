use std::io::{Write, stdout};
use crossterm::{cursor, queue, style::{self, Print, Stylize}, terminal, ExecutableCommand, QueueableCommand};

pub fn print_rect_w_macros() {
    let mut stdout = stdout();

    for y in 0..10 {
        queue!(stdout, Print("\n",)).expect("Bad queue add");
        for x in 0..20 {
            if (y == 0 || y == 10 - 1) || (x == 0 || x == 20 - 1) {
                queue!(stdout, style::PrintStyledContent( "█".magenta())).expect("Bad queue add");
            } else {
                queue!(stdout, style::PrintStyledContent( "█".black())).expect("Bad queue add");
            }
      }
    }
    stdout.flush().expect("Bad flush");
}

pub fn print_rect() {
    let mut stdout = stdout();

    match stdout.execute(terminal::Clear(terminal::ClearType::All)) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e); 
            return
        },
    }

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                let str = if x == 0 || x == 150 - 1 {
                    "██"
                } else {
                    "█"
                };
                stdout
                    .queue(cursor::MoveTo(x,y)).expect("Bad queue")
                    .queue(style::PrintStyledContent(str.magenta())).expect("Bad queue");
            }
        }
    }
    match stdout.flush() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e); 
            return
        },
    }
}
