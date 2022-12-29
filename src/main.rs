use iced::widget::{
    button, checkbox, column, container, row, scrollable, text, text_input, vertical_space, Rule,
};
use iced::{Alignment, Application, Command, Element, Length, Sandbox, Settings, Theme};
use storaget::*;
use worker::*;

mod worker;

#[derive(Debug, Default)]
struct AppData {
    name_filter: String,
    data: Option<Pack<worker::Data>>,
}

pub fn main() -> iced::Result {
    AppData::run(Settings::default())
}

impl Application for AppData {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (AppData, Command<Message>) {
        (
            AppData::default(),
            Command::perform(load_data(), Message::DataLoaded),
        )
    }

    fn title(&self) -> String {
        "ÁNYK Export".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::NameFilterChange(filter_str) => {
                self.name_filter = filter_str.to_lowercase();
                Command::none()
            }
            Message::RowAction(id, row_action) => {
                if let Some(data) = &mut self.data {
                    if let Some(worker) = data.as_mut().get_worker_mut_by_id(id) {
                        match row_action {
                            RowAction::Selected(value) => worker.is_selected = value,
                        }
                    }
                }
                Command::none()
            }
            Message::DataLoaded(data) => {
                if let Ok(data) = data {
                    self.data = Some(data);
                }
                Command::none()
            }
            _ => unimplemented!(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // .style(Container::from(style::menubar as for<'r> fn(&'r _) -> _))
        // let top = container(column(
        //     vec![
        //         row![
        //             button("ÁNYK Export").padding(10).on_press(Message::Export),
        //             horizontal_space(Length::Fill),
        //             button("Új munkavállaló").padding(10).on_press(Message::NewEmployee),
        //     ].padding(3).spacing(40).into(),
        //     Rule::horizontal(3).into()
        // ])).width(Length::Fill);

        let filter = row![
            container(
                text_input("Név keresése", &self.name_filter, Message::NameFilterChange)
                    .width(Length::Units(200))
            ),
            button("Új munkavállaló")
                .padding(10)
                .on_press(Message::NewEmployee)
        ]
        .spacing(20);

        let table = match &self.data {
            Some(data) => column(
                data.workers
                    .iter()
                    .filter(|i| i.name.to_lowercase().contains(&self.name_filter))
                    .enumerate()
                    .map(|(i, item)| {
                        column(vec![
                            item.view()
                                .map(move |row_action| Message::RowAction(item.id, row_action)),
                            Rule::horizontal(2).into(),
                        ])
                        .width(Length::Shrink)
                        .into()
                    })
                    .collect(),
            )
            .spacing(10)
            .align_items(Alignment::Start),
            None => column![],
        };

        let left_panel = column(vec![filter.into(), scrollable(table).into()])
            .padding(20)
            .width(Length::FillPortion(3))
            .spacing(40);

        let selected_panel = column(vec![
            text("Kiválasztott\nmunkavállalók").size(30).into(),
            button(text("ÁNYK Export").size(20))
                .padding(10)
                .on_press(Message::Export)
                .into(),
            vertical_space(Length::Fill).into(),
            container(
                text("Nincs kiválasztva\nmunkavállaló :(")
                    .width(Length::Fill)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .into(),
        ])
        .width(Length::FillPortion(3))
        .spacing(10)
        .padding(10);

        let content = column![row![left_panel, selected_panel]]
            .spacing(20)
            .align_items(Alignment::Start);

        container(content).height(Length::Fill).padding(10).into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    DataLoading,
    DataLoaded(Result<Pack<Data>, String>),
    NameFilterChange(String),
    RowAction(u32, RowAction),
    NewEmployee,
    Export,
}

async fn load_data() -> Result<Pack<Data>, String> {
    let data = Pack::try_load_or_init(
        dirs::home_dir()
            .expect("Error while getting your home folder")
            .join(".dailyworkerdb"),
        "workersdb",
    )
    .map_err(|_| String::from("Error loading database"))?;
    Ok(data)
}

#[derive(Debug, Clone)]
enum RowAction {
    Selected(bool),
}

impl Worker {
    fn view(&self) -> Element<RowAction> {
        let checkbox = checkbox("", self.is_selected, RowAction::Selected).width(Length::Shrink);

        row![
            checkbox,
            text(&self.name).width(Length::Units(100)),
            text(&self.taj).width(Length::Units(100)),
            text(&self.birthdate).width(Length::Units(100)),
            text(format!("{} {} {}", &self.zip, &self.city, &self.street))
                .width(Length::Units(100)),
            button(text("Szerk."))
        ]
        .spacing(20)
        .into()
    }
}

mod style {
    use iced::widget::container;
    use iced::{Color, Theme};

    pub fn menubar(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.6, 0.6))),
            ..Default::default()
        }
    }
}
