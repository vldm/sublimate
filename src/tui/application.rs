use ncurses::*;

use core::Core;
use core::bindings::Key;
use super::view::window::Window;
use super::toolkit::*;
use super::view::theme::PALETTE;

use traits::Application;

use std::ops::Drop;

pub struct TuiApplication
{
    window: Window,
}

impl Application for TuiApplication {
    type Error = ();
    fn new(core: Core) -> Result<TuiApplication, Self::Error>{
        setlocale(LcCategory::all, "en_US.utf-8");

        initscr();

        noecho();
        keypad(stdscr, true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        raw();

        start_color();
        use_default_colors();

        for (i, &(ref fg, ref bg)) in PALETTE.iter().enumerate() {
            init_pair(i as i16, fg.to_term(), bg.to_term());
        }
        
        Ok(TuiApplication {
                window: Window::new(core)
        })
    }
    
    fn run(&mut self){

        self.window.render(Canvas::screen());
        loop {
            if let Some(key) = Key::from_keycode(getch()) {
                if key == Key::Enter {
                    break;
                }
                self.window.on_keypress(Canvas::screen(), key);
            }
        }

    }
}

impl Drop for TuiApplication{
    fn drop (&mut self) {
        // Terminate ncurses.
        endwin();
    }
}


