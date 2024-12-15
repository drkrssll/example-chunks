use std::process::Command;

use chunks_rs::{position::Edge, taskbar::Bar, utils::tag_button, GtkApp, Internal, Vertical};

pub struct Taskbar {}

impl Taskbar {
    pub fn bar(factory: &GtkApp) {
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

        let margins = vec![(Edge::Top, 6), (Edge::Bottom, 6), (Edge::Left, 6)];

        let anchors = vec![(Edge::Top, true), (Edge::Left, true), (Edge::Bottom, true)];

        Bar::new(
            factory.clone(),
            "Storage",
            workspaces,
            margins,
            anchors,
            Vertical,
        )
        .build();
    }

    fn switch_workspace(number: i32) -> Result<(), std::io::Error> {
        // switch hyprland workspace
        Command::new("hyprctl")
            .args(&["dispatch", "workspace", &number.to_string()])
            .output()?;

        Ok(())
    }
}
