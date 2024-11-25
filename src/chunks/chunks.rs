use chrono::Local;
use chunks_rs::{
    position::{Edge, EdgeConfig, Layer},
    utils::{tag_box, tag_label},
    widgets::Chunk,
    Application, Internal,
};

pub struct Chunks {}

impl Chunks {
    pub fn clock(factory: &Application) {
        let tag = tag_label("clock");
        let margins = vec![(Edge::Top, 20), (Edge::Right, 20)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();

        let date_closure = || {
            let current_time = Local::now();

            let date = format!(
                "<span background='#000000' foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#fabbc2' size='small'>{}  </span><span foreground='#FF0110' weight='bold' size='small'>{}</span>",
                current_time.format("%a"),
                current_time.format("%b"),
                current_time.format("%d"),
            );

            date
        };

        let time_closure = || {
            let current_time = Local::now();

            let time = format!(
                "<span foreground='#FFFFFF' size='large'>{}</span><span foreground='#FF0110' weight='bold' size='small'>  {}</span>\n<span foreground='#FFFFFF' size='large'> {}</span>",
                current_time.format("%I"),
                current_time.format("%p"),
                current_time.format("%M"),
            );

            time
        };

        Internal::static_to_update(&tag, date_closure, 2, time_closure, 1);

        Chunk::new(
            factory.clone(),
            "Clock".to_string(),
            tag,
            margins,
            anchors,
            Layer::Bottom,
        )
        .build();
    }

    pub fn weather(factory: &Application, weather_data: String) {
        let tag = tag_label("weather");
        let margins = vec![(Edge::Top, 90), (Edge::Right, 160)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();

        let weather_closure = move || {
            let text = format!(
                "<span foreground='#FFFFFF' size='large'>{}</span>",
                weather_data
            );
            text
        };

        Internal::update_widget(&tag, weather_closure, 5);

        Chunk::new(
            factory.clone(),
            "Storage".to_string(),
            tag,
            margins,
            anchors,
            Layer::Bottom,
        )
        .build();
    }

    pub fn storage(factory: &Application) {
        let tag = tag_label("storage");
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
            margins,
            anchors,
            Layer::Bottom,
        )
        .build();
    }

    pub fn picture(factory: &Application) {
        let box_tag = tag_box("picture");
        let margins = vec![(Edge::Top, 20), (Edge::Left, 20)];
        let anchors = EdgeConfig::TOP_LEFT.to_vec();

        let pathname = "/path/to/example.png";

        Internal::static_picture(&box_tag, pathname);

        Chunk::new(
            factory.clone(),
            "Picture".to_string(),
            box_tag,
            margins,
            anchors,
            Layer::Bottom,
        )
        .build();
    }
}
