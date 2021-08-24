#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

// START: constants
const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;
// END: constants

// START: player
struct Player {
    x: i32, // <callout id="co.flappy.player_x" />
    y: i32, // <callout id="co.flappy.player_y" />
    velocity: f32, // <callout id="co.flappy.player_velocity" />
}
// END: player

// START: playerconstructor
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,// <callout id="co.flappy.float_assign" />
        }
    }
    // END: playerconstructor

    // START: playerrender
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set( // <callout id="co.flappy.set" />
            0, // <callout id="co.flappy.x_render" />
            self.y, // <callout id="co.flappy.y_render" />
            YELLOW, // <callout id="co.flappy.rgb" />
            BLACK,
            to_cp437('@') // <callout id="co.flappy.glyph" />
        );
    }
    // END: playerrender

    // START: gravity
    fn gravity_and_move(&mut self) {
         if self.velocity < 2.0 { // <callout id="co.flappy.terminal_velocity" />
            self.velocity += 0.2; // <callout id="co.flappy.gravity" />
        }
        self.y += self.velocity as i32; // <callout id="co.flappy.add_gravity" />
        self.x += 1; // <callout id="co.flappy.increment_x" />
        if self.y < 0 {
            self.y = 0;
        }
    }
    // END: gravity

    // START: flap
    fn flap(&mut self) {
        self.velocity = -2.0;
    }    
    // END: flap
}

enum GameMode {
    Menu,
    Playing,
    End,
}

// START: state
struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
    // END: state

    // START: restart
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
    // END: restart

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

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

    // START: play
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // <callout id="co.flappy.cls_bg" />
        self.frame_time += ctx.frame_time_ms; // <callout id="co.flappy.framerate" />
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();           
        }
        if let Some(VirtualKeyCode::Space) = ctx.key { // <callout id="co.flappy.spaceflap" />
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        if self.player.y > SCREEN_HEIGHT { // <callout id="co.flappy.arewedeadyet" />
            self.mode = GameMode::End;
        }
    }
    // END: play
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
