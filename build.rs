fn main() {
    relm4_icons_build::bundle_icons(
        "icon_names.rs",
        Some("dev.lennoxlotl.ShareShot"),
        None::<&str>,
        None::<&str>,
        ["plus", "floppy", "cross-large"],
    );
}
