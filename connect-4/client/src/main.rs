#![recursion_limit="512"]

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
use yew::prelude::*;

// MODEL
pub struct Model {
    link: ComponentLink<Self>,
    tab: Tab,
    player_name: String
}

pub enum Tab {
    HowToConnect4,
    Connect4Computer,
    Connect4Human,
    HowToToot,
    TootOttoComputer,
    TootOttoHuman,
    ScoreBoard,
    Scores,
    Nothing
}

// CONTROLLER
pub enum Msg {
    ClickedHowToC4,
    ClickedC4Comp,
    ClickedC4Human,
    ClickedHowToToot,
    ClickedTootComp,
    ClickedTootHuman,
    ClickedScoreBoard,
    ClickedScores,
    GotPlayerName(String),
    StartConnect4,
    StartTootOtto
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            tab: Tab::Nothing,
            player_name: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickedHowToC4 => self.tab = Tab::HowToConnect4,
            Msg::ClickedC4Comp => self.tab = Tab::Connect4Computer,
            Msg::ClickedC4Human => self.tab = Tab::Connect4Human,
            Msg::ClickedHowToToot => self.tab = Tab::HowToToot,
            Msg::ClickedTootComp => self.tab = Tab::TootOttoComputer,
            Msg::ClickedTootHuman => self.tab = Tab::TootOttoHuman,
            Msg::ClickedScoreBoard => self.tab = Tab::ScoreBoard,
            Msg::ClickedScores => self.tab = Tab::Scores,
            Msg::GotPlayerName(name) => self.player_name = name,
            Msg::StartConnect4 => {
                // i think we call callback emit here idk
            },
            Msg::StartTootOtto => {
                // i think we call callback emit here idk
            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        let view_tab = || match self.tab {
            Tab::HowToConnect4 => self.view_howto_connect4(),
            Tab::Connect4Computer => self.view_connect4_computer(),
            Tab::Connect4Human => self.view_connect4_human(),
            Tab::HowToToot => self.view_howto_toototto(),
            Tab::TootOttoComputer => self.view_toototto_computer(),
            Tab::TootOttoHuman => self.view_toototto_human(),
            Tab::ScoreBoard => self.view_scoreboard(),
            Tab::Scores => self.view_scores(),
            Tab::Nothing => self.view_main()
        };
        html! {
            <div>
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding menu" style="z-index:3;height:100%;width:350px;font-weight:bold" id="mySidenav">
                    <div class="w3-container">
                        <h3 class="w3-padding-64"><b>{"Play"}<br></br>{"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>
                    <a href="#/HowToConnect4" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedHowToC4)>{"How to Play Connect4"}</a><br></br>
                    <a href="#/Connect4Computer" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedC4Comp)>{"Play Connect4 with Computer"}</a><br></br>
                    <a href="#/Connect4Human" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedC4Human)>{"Play Connect4 with Another Human"}</a><br></br>
                    <a href="#/HowToToot" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedHowToToot)>{"How to Play TOOT-OTTO"}</a><br></br>
                    <a href="#/TootOttoComputer" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedTootComp)>{"Play Toot-Otto with Computer"}</a><br></br>
                    <a href="#/TootOttoHuman" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedTootHuman)>{"Play Toot-Otto With Another Human"}</a><br></br>
                    <a href="#/ScoreBoard" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedScoreBoard)>{"View Game History"}</a><br></br>
                    <a href="#/Scores" class="w3-padding w3-hover-white" onclick=self.link.callback(|_| Msg::ClickedScores)>{"Score Board"}</a>
                </nav>
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                    {view_tab()}
                </div>
            </div>
        }
    }
}


impl Model {
    fn view_howto_connect4(&self) -> Html {
        html! {
            <div>
                <h1><b>{"How to Play Connect 4"}</b></h1>
                <p>
                    {"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
                </p>
                <h2>{"To play Connect 4 follow the following steps:"}</h2>
                <ul>
                    <li>{"A new game describes discs of which color belongs to which player"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
                </ul>
                {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
            </div>
        }
    }

    fn view_connect4_computer(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Enter Your Name"}</b></h1>
                <div>
                    <input
                        type="text"
                        value=&self.player_name
                        oninput=self.link.callback(|e: InputData| Msg::GotPlayerName(e.value))
                        placeholder="Your Name"/>
                    <button onclick=self.link.callback(|_| Msg::StartConnect4)>{"Start Game"}</button>
                </div>
            </div>
        }
    }

    fn view_connect4_human(&self) -> Html {
        html! {
            <p>{"Connect 4 Human"}</p>
        }
    }

    fn view_howto_toototto(&self) -> Html {
        html! {
            <div>
                <h1><b>{"How to Play TOOT-OTTO"}</b></h1>
                <p>
                    {"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
                </p>
                <h2>{"To play TOOT-OTTO follow the following steps:"}</h2>
                <ul>
                    <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                    <li>{"Select the disc type T or O that you want to place"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
                </ul>
                {"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
            </div>
        }
    }

    fn view_toototto_computer(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Enter Your Name"}</b></h1>
                <div>
                    <input
                        type="text"
                        value=&self.player_name
                        oninput=self.link.callback(|e: InputData| Msg::GotPlayerName(e.value))
                        placeholder="Your Name"/>
                    <button onclick=self.link.callback(|_| Msg::StartTootOtto)>{"Start Game"}</button>
                </div>
            </div>
        }
    }

    fn view_toototto_human(&self) -> Html {
        html! {
            <p>{"Toot Otto Human"}</p>
        }
    }

    fn view_scoreboard(&self) -> Html {
        html! {
            <p>{"Game History"}</p>
        }
    }

    fn view_scores(&self) -> Html {
        html! {
            <p>{"Score Board"}</p>
        }
    }

    fn view_main(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Welcome"}</b></h1>
                <p>{"This application contains the following two board games, both in human vs. human and human vs. computer versions."}</p>
                <ul>
                    <li>{"Connect 4"}</li>
                    <li>{"TOOT-OTTO"}</li>
                </ul>
                <p>{"Select the game of your choice above, and start playing. Enjoy!"}</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}