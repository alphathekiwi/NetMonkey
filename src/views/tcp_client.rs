use iced::Alignment::Center;
use iced::widget::{Column, text};

use crate::Msg;

pub fn view<'a>() -> Column<'a, Msg> {
    let items = vec![
        text("TCP Client/Server").size(24).into(),
        // text_input("Ip Address", "").size(24).into(),
        text("Coming soon...").size(16).into(),
    ];

    Column::with_children(items).align_x(Center).spacing(10)
}
