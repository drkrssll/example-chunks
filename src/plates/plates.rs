#![allow(unused_imports)]

use chunks_rs::{
    position::{Edge, EdgeConfig},
    utils::tag_label,
    widgets::Plate,
    Builder, GtkApp, Internal,
};

pub struct Plates {}

impl Plates {
    pub fn welcome(factory: &GtkApp) {
        let tag = tag_label("welcome");
        let margins = vec![(Edge::Bottom, 0), (Edge::Left, 0)];
        let anchors = EdgeConfig::CENTER.to_vec();

        let text = format!(
            "<span foreground='#FFFFFF' size='large'>Example</span>\n<span foreground='#FF0000' size='large'>Chunks</span>",
        );

        Internal::static_widget(&tag, &text);

        let plate = Plate::new(factory.clone(), "Welcome", tag, margins, anchors, 2);

        plate.build();
    }
}
