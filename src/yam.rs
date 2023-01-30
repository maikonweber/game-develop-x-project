use yew::{html, Component, ComponentLink, Html, ShouldRender};
use bevy::prelude::*;

struct Game {
    link: ComponentLink<Self>,
    game_state: GameState,
}

impl Component for Game {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let game_state = GameState::new();

        Game { link, game_state }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| ())>{ "Start game" }</button>
                <canvas id="game-canvas"></canvas>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.game_state.start();
        }
    }
}

struct GameState {
    // game state here
}

impl GameState {
    fn new() -> Self {
        GameState { /* initialize game state */ }
    }

    fn start(&mut self) {
        // start game loop
    }
}
