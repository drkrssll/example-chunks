#![allow(unused_imports)]

use chrono::Local;
use chunks_rs::{
    position::{Edge, EdgeConfig, Layer},
    utils::{tag_box, tag_container, tag_label, tag_scroller},
    widgets::Chunk,
    GtkApp, Internal, Orientation,
};

pub struct Chunks {}

impl Chunks {
    pub fn helloworld(factory: &GtkApp) {
        let tag_label_hello = tag_label("storage");
        let tag_label_world = tag_label("storage");
        let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();

        Internal::static_widget(&tag_label_hello, "hello");
        Internal::static_widget(&tag_label_world, "world");

        let tag = tag_container(
            "storage",
            Orientation::Vertical,
            9,
            vec![tag_label_hello, tag_label_world],
        );

        Chunk::new(
            factory.clone(),
            "Storage",
            tag,
            margins,
            anchors,
            Layer::Top,
            true,
        )
        .build();
    }

    pub fn scroller_text(factory: &GtkApp) {
        let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();
        let mut labels = vec![];

        for _i in 1..=8 {
            let text = tag_label("storage");

            Internal::static_widget(&text, "This is an example.");

            labels.push(text);
        }

        let boxx = tag_container("storage", Orientation::Vertical, 20, labels);

        let chunk = Chunk::new(
            factory.clone(),
            "Storage",
            tag_scroller("Storage", boxx, false, false),
            margins,
            anchors,
            Layer::Top,
            true,
        );

        chunk.build().set_dimensions(400, 200);
    }

    pub fn clock(factory: &GtkApp) {
        let tag = tag_label("clock");
        let margins = vec![(Edge::Top, 20), (Edge::Right, 20)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();

        let date_closure = || {
            let current_date = Local::now();

            let date = format!(
                "<span background='#000000' foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#fabbc2' size='small'>{}  </span><span foreground='#FF0110' weight='bold' size='small'>{}</span>",
                current_date.format("%a"),
                current_date.format("%b"),
                current_date.format("%d"),
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

        let chunk = Chunk::new(
            factory.clone(),
            "Clock",
            tag,
            margins,
            anchors,
            Layer::Bottom,
            false,
        );

        chunk.build();
    }

    pub fn network(factory: &GtkApp) {
        let tag = tag_label("network");
        let margins = vec![(Edge::Top, 20), (Edge::Left, 20)];
        let anchors = EdgeConfig::TOP_LEFT.to_vec();

        let network_closure = || {
            let text = format!(
                "<span foreground='#FFFFFF' size='large'>{}</span>",
                Internal::get_network().unwrap(),
            );

            text
        };

        Internal::update_widget(&tag, network_closure, 5);

        let chunk = Chunk::new(
            factory.clone(),
            "Network",
            tag,
            margins,
            anchors,
            Layer::Bottom,
            true,
        );

        chunk.build();
    }

    pub fn weather(factory: &GtkApp, weather_data: String) {
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

        let chunk = Chunk::new(
            factory.clone(),
            "Storage",
            tag,
            margins,
            anchors,
            Layer::Bottom,
            false,
        );

        chunk.build();
    }

    pub fn storage(factory: &GtkApp) {
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

        let chunk = Chunk::new(
            factory.clone(),
            "Storage",
            tag,
            margins,
            anchors,
            Layer::Bottom,
            false,
        );

        chunk.build();
    }

    // tag_box should be used for images
    pub fn picture(factory: &GtkApp) {
        let box_tag = tag_box("picture");
        let margins = vec![(Edge::Top, 20), (Edge::Left, 20)];
        let anchors = EdgeConfig::TOP_LEFT.to_vec();

        let pathname = "/path/to/example.png";

        Internal::static_picture(&box_tag, pathname);

        let chunk = Chunk::new(
            factory.clone(),
            "Picture",
            box_tag,
            margins,
            anchors,
            Layer::Bottom,
            false,
        );

        chunk.build();
    }
}
