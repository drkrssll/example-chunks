#![allow(unused_imports)]

use std::process::Command;

use chrono::Local;
use chunks_rs::{
    position::Edge,
    taskbar::Bar,
    utils::{tag_button, tag_label},
    GtkApp, Internal, Orientation,
};

pub struct Taskbar {}

impl Taskbar {
    pub fn bar(factory: &GtkApp) {
        let margins = vec![(Edge::Top, 6), (Edge::Bottom, 6), (Edge::Left, 6)];
        let anchors = vec![(Edge::Top, true), (Edge::Left, true), (Edge::Bottom, true)];
        let mut tags = vec![];

        let label = tag_label("bar-clock");

        let time_closure = || {
            let current_time = Local::now();

            let time = format!(
                "<span foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#FF0110' weight='bold' size='large'>{}</span>",
                current_time.format("%I"),
                current_time.format("%M"),
                current_time.format("%p"),
            );

            time
        };

        Internal::update_time(&label, time_closure);

        tags.push(label);

        let bar = Bar::new(
            factory.clone(),
            "Storage",
            tags,
            margins,
            anchors,
            Orientation::Vertical,
        );

        bar.build();
    }

    fn switch_workspace(number: i32) -> Result<(), std::io::Error> {
        Command::new("hyprctl")
            .args(&["dispatch", "workspace", &number.to_string()])
            .output()?;

        Ok(())
    }
}
