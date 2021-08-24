// START: use
use bracket_lib::prelude::*;
// END: use

// START: state
struct State {}
// END: state

// START: gamestate
impl GameState for State {// <callout id="co.flappy.impl_trait" />
    fn tick(&mut self, ctx: &mut BTerm) { // <callout id="co.flappy.tick" />
        ctx.cls(); // <callout id="co.flappy.cls" />
        ctx.print(1, 1, "Hello, Bracket Terminal!"); // <callout id="co.flappy.print" />
    }
}
// END: gamestate

// START: main
// START: mainfn
fn main() -> BError {
// END: mainfn
// START: builder
    let context = BTermBuilder::simple80x50() // <callout id="co.flappy.builder" />
        .with_title("Flappy Dragon") // <callout id="co.flappy.title" />
        .build()?; // <callout id="co.flappy.builderror" />
// END: builder

// START: mainloop
    main_loop(context, State{})
// END: mainloop
}
// END: main