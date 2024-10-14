use std::{error::Error, process::Command};

use chunks_rs::{
    load_css, tag, Application, Chunk, Edge, EdgeConfig, Factory, Internal, Layer, Local, Slab,
};
use regex::Regex;

const STYLE: &str = "
window {
    background-color: transparent;
}

#clock, #storage, #volume, #weather {
    font-size: 34px;
    font-family: feather;
    font-family: Iosevka;
    background-color: #000000;
    color: #FFFFFF;
    padding: 10px;
    border: 2px solid black;
    border-radius: 20px;
}

#storage, #volume, #weather{
    font-size: 24px;
}
";

// for async functions like get_weather to work, you have to apply tokio::main
// and set main to be async as well
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
fn main() -> Result<(), Box<dyn Error>> {
    let factory = Factory::new("chunk.factory");

    // let weather_data = get_weather("Buena Vista GA").await?;

    let chunks = move |factory: Application| {
        storage(&factory);
        clock(&factory);

        // weather(&factory, weather_data.clone());

        load_css(STYLE);
    };

    factory.pollute(chunks);

    Ok(())
}

/*
async fn get_weather(location: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://wttr.in/{}?format=3", location);
    let response = reqwest::get(&url).await?.text().await?;

    let re = Regex::new(r"\s*([\d]+°F)")?;

    if let Some(caps) = re.captures(&response) {
        let emoji = caps.get(1).map_or("", |m| m.as_str());
        let temperature = caps.get(2).map_or("", |m| m.as_str());

        Ok(format!("{} {}", emoji, temperature).trim().to_string())
    } else {
        Ok("Weather data not available".to_string())
    }
}

fn weather(factory: &Application, weather_data: String) {
    let tag = tag("weather");
    let margins = vec![(Edge::Top, 90), (Edge::Right, 160)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let text = format!(
        "<span foreground='#FFFFFF' size='large'>{}</span>",
        weather_data
    );

    Chunk::new(
        factory.clone(),
        "Storage".to_string(),
        tag,
        anchors,
        margins,
        Layer::Bottom,
    )
    .build();
}
*/
fn storage(factory: &Application) {
    let tag = tag("storage");
    let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let storage_closure = || {
        let text = format!(
            "<span foreground='#FF0000' size='large'>/ </span><span foreground='#FFFFFF' size='large'>{:.0}%</span>",
            Internal::get_storage(),
        );

        text
    };

    Internal::update_storage(&tag, storage_closure);

    Chunk::new(
        factory.clone(),
        "Storage".to_string(),
        tag,
        anchors,
        margins,
        Layer::Bottom,
    )
    .build();
}

fn volume(factory: &Application) {
    let tag = tag("volume");
    let margins = vec![(Edge::Top, 20), (Edge::Left, 20)];
    let anchors = EdgeConfig::TOP_LEFT.to_vec();

    let volume = || {
        let output = Command::new("pactl")
            .args(&["get-sink-volume", "@DEFAULT_SINK@"])
            .output()
            .expect("Failed to execute pactl command");

        let output_str = String::from_utf8_lossy(&output.stdout);

        if let Some(volume) = output_str.split_whitespace().find(|&s| s.ends_with('%')) {
            volume.to_string()
        } else {
            "Unknown".to_string()
        }
    };

    let text = format!(
        "<span foreground='#FFFFFF' size='large'>📢 {}</span>",
        volume()
    );

    Internal::static_widget(&tag, text);

    Slab::new(
        factory.clone(),
        "Volume".to_string(),
        tag,
        anchors,
        margins,
        2,
    )
    .build();
}

fn clock(factory: &Application) {
    let tag = tag("clock");
    let margins = vec![(Edge::Top, 20), (Edge::Right, 20)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let date_closure = || {
        let current_time = Local::now();

        let date = format!(
            "<span background='#000000' foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#fabbc2' size='small'>{}  </span><span foreground='#FF0110' weight='bold' size='small'>{}</span>",
            current_time.format("%a").to_string(),
            current_time.format("%b").to_string(),
            current_time.format("%d").to_string(),
        );

        date
    };

    let time_closure = || {
        let current_time = Local::now();

        let time = format!(
            "<span foreground='#FFFFFF' size='large'>{}</span><span foreground='#FF0110' weight='bold' size='small'>  {}</span>\n<span foreground='#FFFFFF' size='large'> {}</span>",
            current_time.format("%I").to_string(),
            current_time.format("%p").to_string(),
            current_time.format("%M").to_string(),
        );

        time
    };

    Internal::static_to_update(&tag, date_closure, 2, time_closure, 1);

    Chunk::new(
        factory.clone(),
        "Clock".to_string(),
        tag,
        anchors,
        margins,
        Layer::Overlay,
    )
    .build();
}
