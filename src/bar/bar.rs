#![allow(unused_imports)]

use std::process::Command;

use chrono::Local;
use chunks_rs::{
    position::Edge,
    taskbar::Bar,
    utils::{tag_button, tag_label},
    GtkApp, Internal, Vertical,
};

pub struct Taskbar {}

impl Taskbar {
    pub fn bar(factory: &GtkApp) {
        let margins = vec![(Edge::Top, 6), (Edge::Bottom, 6), (Edge::Left, 6)];
        let anchors = vec![(Edge::Top, true), (Edge::Left, true), (Edge::Bottom, true)];
        let mut workspaces = vec![];

        for i in 0..5 {
            let workspace = tag_button("workspace");
            let num = i + 1;

            Internal::static_button(&workspace, move || {
                Self::switch_workspace(num).expect("Failed to switch workspace")
            });

            Internal::static_widget(&workspace, &(num).to_string());

            workspaces.push(workspace);
        }

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

        workspaces.push(label);

        let bar = Bar::new(
            factory.clone(),
            "Storage",
            workspaces,
            margins,
            anchors,
            Vertical,
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
