use chunks_rs::{
    position::{Edge, EdgeConfig},
    utils::tag,
    widgets::Slab,
    Application, Internal,
};

pub struct Slabs {}

impl Slabs {
    pub fn volume(factory: &Application) {
        let tag = tag("volume");
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

        Slab::new(
            factory.clone(),
            "Volume".to_string(),
            tag,
            margins,
            anchors,
            2,
        )
        .build();
    }
}
