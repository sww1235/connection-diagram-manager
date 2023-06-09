use simple_logger::SimpleLogger;

use log::LevelFilter;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .with_colors(true)
        .init()
        .unwrap();

    #[cfg(feature = "cli")]
    use std::str::FromStr;
    #[cfg(feature = "cli")]
    let text = "this is a line of text, which is very long and needs to be broken into lines.";
    #[cfg(feature = "cli")]
    let font_path = std::path::PathBuf::from("/usr/share/fonts/TTF/DejaVuSans.ttf");
    #[cfg(feature = "cli")]
    let face = rustybuzz::Face::from_slice(std::fs::read(font_path).unwrap().leak(), 0).unwrap();
    #[cfg(feature = "cli")]
    let textbox_width = 1.0 * dimensioned::ucum::IN_US;
    #[cfg(feature = "cli")]
    let direction = rustybuzz::Direction::LeftToRight;
    #[cfg(feature = "cli")]
    let language = rustybuzz::Language::from_str("English").unwrap();

    #[cfg(feature = "cli")]
    let lines =
        cdm_core::paragraph_breaking::to_lines(text, &face, 24, textbox_width, direction, language);

    #[cfg(feature = "cli")]
    println!("test: {:?}", lines);
}
