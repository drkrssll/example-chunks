use chunks_rs::{
    position::{Edge, EdgeConfig},
    utils::tag,
    widgets::Plate,
    Application, Internal,
};

pub struct Plates {}

impl Plates {
    pub fn welcome(factory: &Application) {
        let tag = tag("welcome");
        let margins = vec![(Edge::Bottom, 0), (Edge::Left, 0)];
        let anchors = EdgeConfig::CENTER.to_vec();

        let text = format!(
            "<span foreground='#FFFFFF' size='large'>Hello</span>\n<span foreground='#FF0000' size='large'>Derek</span>",
        );

        Internal::static_widget(&tag, text);

        Plate::new(
            factory.clone(),
            "Welcome".to_string(),
            tag,
            margins,
            anchors,
            2,
        )
        .build();
    }
}
