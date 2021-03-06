#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

// START: enum
enum GameMode {// <callout id="co.flappy_state.enum" />
    Menu,// <callout id="co.flappy_state.enums" />
    Playing,
    End,
}
// END: enum

// START: state
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
// END: state

    // START: restart
    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }
    // END: restart

    // START: mainmenu
    // START: mainprint
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        // END: mainprint
        // START: mainiflet

        if let Some(key) = ctx.key {// <callout id="co.flappy_state.if_let_key" />
            match key {
                VirtualKeyCode::P => self.restart(),// <callout id="co.flappy_state.restart" />
                VirtualKeyCode::Q => ctx.quitting = true,// <callout id="co.flappy_state.quitting" />
                _ => {}
            }
        }
    } // Close the function
    // END: mainiflet
    // END: mainmenu

    // START: dead
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    // END: dead

    // START: play
    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }
    // END: play
}

// START: tick
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}
// END: tick

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    // START: mainloop
    main_loop(context, State::new())
    // END: mainloop
}
