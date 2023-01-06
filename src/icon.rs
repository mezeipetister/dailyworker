use iced::{
    alignment,
    widget::{text, Text},
    Font, Length,
};

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../font/icons.ttf"),
};

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}

pub fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}
