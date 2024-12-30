#![allow(unused_imports)]

mod bar;
mod chunks;
mod plates;
mod slabs;

use bar::Taskbar;
use chunks::Chunks;
use chunks_rs::{utils::load_css, widgets::Chunk, Factory, GtkApp};
use plates::Plates;
use slabs::Slabs;

const STYLE: &str = "
window {
    background-color: transparent;
}

* {
    font-family: feather;
    font-family: Iosevka;
}

#taskbar {
    background-color: #000000;
    color: #FFFFFF;
    border: 2px solid black;
    border-radius: 10px;
}

#bar-clock {
    margin-top: 775px;
    font-size: 24px;
}

#clock, #storage, #volume, #weather, #picture, #welcome {
    font-size: 34px;
    background-color: #000000;
    color: #FFFFFF;
    padding: 10px;
    border: 2px solid black;
    border-radius: 20px;
}

#volume, #weather, #storage {
    font-size: 24px;
}

#workspace {
    font-size: 18px;
    background-color: red;
    border-radius: 10px;
}
";

// for async functions like get_weather to work, you have to apply tokio::main
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    let factory = Factory::new("chunk.factory");

    // let weather_data = Internal::get_weather("London").await?;

    let chunks = move |factory: GtkApp| {
        // Chunks::weather(&factory, weather_data.clone());

        Chunks::network(&factory);
        Chunks::helloworld(&factory);

        // Taskbar::bar(&factory);

        // Chunks::storage(&factory);
        // Chunks::clock(&factory);

        // Slabs::volume(&factory);

        // Plates::welcome(&factory);

        load_css(STYLE);
    };

    factory.pollute(chunks);
}
