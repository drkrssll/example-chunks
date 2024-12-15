#![allow(unused_imports)]

use chunks_rs::{
    position::{Edge, EdgeConfig},
    utils::tag_label,
    widgets::Slab,
    GtkApp, Internal,
};

pub struct Slabs {}

impl Slabs {
    pub fn volume(factory: &GtkApp) {
        let tag = tag_label("volume");
        let margins = vec![(Edge::Bottom, 20), (Edge::Left, 20)];
        let anchors = EdgeConfig::BOTTOM_LEFT.to_vec();

        let volume_closure = || {
            let text = format!(
                "<span foreground='#FFFFFF' size='large'>📢 {}</span>",
                Internal::get_pactl_vol()
            );
            text
        };

        Internal::update_widget(&tag, volume_closure, 0);

        let slab = Slab::new(factory.clone(), "Volume", tag, margins, anchors, 2);

        slab.build();
    }
}
