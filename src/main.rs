use std::fs::File;
use std::io::Write;

use chrono::{Datelike, Local, NaiveDate, Timelike};
use iced::widget::{self, button, checkbox, row, text, Text};
use iced::{
    alignment, event, keyboard, subscription, Application, Command, Element, Event, Font, Length,
    Settings, Subscription, Theme,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use uuid::Uuid;
use worker::*;

// mod import;
mod icon;
mod worker;
mod xml;

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Main,
    NewEmployee,
    EditEmployee(Option<Worker>),
    Modal(String),
}

#[derive(Debug, Default)]
pub struct AppData {
    name_filter: String,
    data: Option<worker::Data>,
    view: View,
    worker_selected: Option<Worker>,
}

pub fn main() -> iced::Result {
    AppData::run(Settings {
        default_font: Some(include_bytes!("../font/roboto.ttf")),
        ..Default::default()
    })
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
                    match row_action {
                        RowAction::Selected(value) => {
                            data.set_worker_selected_by_id(id, value);
                        }
                        RowAction::Edit(worker) => {
                            self.view = View::EditEmployee(Some(worker.clone()));
                            self.worker_selected = Some(worker);
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
            Message::ChangeView(view) => {
                self.view = view;
                match &self.view {
                    View::Main => {
                        self.worker_selected = None;
                    }
                    View::NewEmployee => self.worker_selected = Some(Worker::default()),
                    View::EditEmployee(worker) => self.worker_selected = worker.clone(),
                    _ => (),
                }
                Command::none()
            }
            Message::SetWorkerName(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.name = value;
                }
                Command::none()
            }
            Message::SetWorkerTaj(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.taj = value;
                }
                Command::none()
            }
            Message::SetWorkerTaxnumber(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.taxnumber = value;
                }
                Command::none()
            }
            Message::SetWorkerMothersname(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.mothersname = value;
                }
                Command::none()
            }
            Message::SetWorkerBirthdate(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.birthdate = value;
                }
                Command::none()
            }
            Message::SetWorkerBirthplace(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.birthplace = value;
                }
                Command::none()
            }
            Message::SetWorkerZip(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    if let Ok(zip) = value.parse::<u32>() {
                        worker.zip = zip.to_string();
                    }
                }
                Command::none()
            }
            Message::SetWorkerCity(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.city = value;
                }
                Command::none()
            }
            Message::SetWorkerStreet(value) => {
                if let Some(worker) = &mut self.worker_selected {
                    worker.street = value;
                }
                Command::none()
            }
            Message::CreateWorker(worker) => {
                if let Some(data) = &mut self.data {
                    let _ = data.add_new_worker(worker);
                    self.view = View::Main;
                }
                Command::none()
            }
            Message::UpdateWorker(worker) => {
                if let Some(data) = &mut self.data {
                    let _ = data.update_worker(worker);
                    self.view = View::Main;
                }
                Command::none()
            }
            Message::Export => {
                if let Some(data) = &self.data {
                    let path = FileDialog::new()
                        .set_location("~/")
                        .add_filter("XML", &["xml"])
                        .show_open_single_dir()
                        .unwrap();

                    if let Some(path) = path {
                        let workers_selected = data.get_workers_selected();
                        let file_path = path.join(&format!(
                            "{}-{}-{} {} óra {} perc.xml",
                            Local::now().year(),
                            Local::now().month(),
                            Local::now().day(),
                            Local::now().hour(),
                            Local::now().minute()
                        ));

                        let content = xml::render_xml(&workers_selected);
                        let mut file =
                            File::create(file_path).expect("Could not create file to export");
                        file.write_all(content.as_bytes())
                            .expect("Error while writing xml data to export file");
                    }
                }
                Command::none()
            }
            Message::TabPressed { shift } => {
                if shift {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                }
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
        match &self.view {
            View::Main => view::main_view(self),
            View::NewEmployee => view::new_employee_view(self),
            View::EditEmployee(worker) => view::edit_employee_view(self),
            View::Modal(msg) => view::modal_view(self),
            _ => unimplemented!(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                    ..
                }),
                event::Status::Ignored,
            ) => Some(Message::TabPressed {
                shift: modifiers.shift(),
            }),
            _ => None,
        })
    }
}

mod view {
    use crate::{AppData, Message, View};
    use iced::widget::text::Appearance;
    use iced::widget::{
        button, column, container, row, scrollable, text, text_input, vertical_space, Column, Row,
        Rule, Text,
    };
    use iced::{Alignment, Element, Length, Renderer};

    fn window(title: &str) -> Column<Message> {
        column(vec![text(title).size(40).into()])
            .padding(20)
            .spacing(20)
    }

    pub fn main_view(d: &AppData) -> Element<'_, Message> {
        let filter = row![
            container(
                text_input("Név keresése", &d.name_filter, Message::NameFilterChange)
                    .width(Length::Units(200))
            ),
            button("Új munkavállaló")
                .padding(10)
                .on_press(Message::ChangeView(View::NewEmployee))
        ]
        .spacing(20);

        let table = match &d.data {
            Some(data) => column(
                data.get_workers()
                    .into_iter()
                    .filter(|i| i.name.to_lowercase().contains(&d.name_filter))
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

        let selected = match &d.data {
            Some(data) => data.get_workers_selected(),
            None => vec![],
        };

        let selected_panel = column(vec![
            text("Kiválasztott\nmunkavállalók").size(30).into(),
            button(text("ÁNYK Export").size(20))
                .padding(10)
                .on_press(Message::Export)
                .into(),
            match selected.len() > 0 {
                true => {
                    let a: Vec<Element<'_, Message, Renderer>> = selected
                        .into_iter()
                        .map(move |item| {
                            column![
                                item.view_checkbox()
                                    .map(move |row_action| Message::RowAction(item.id, row_action)),
                                vertical_space(Length::Units(20))
                            ]
                            .into()
                        })
                        .collect();
                    column(a).into()
                }
                false => container(
                    text("Nincs kiválasztva\nmunkavállaló :(")
                        .width(Length::Fill)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            },
        ])
        .width(Length::FillPortion(3))
        .spacing(10)
        .padding(10);

        let content = column![row![left_panel, selected_panel]]
            .spacing(20)
            .align_items(Alignment::Start);

        window("Munkavállalók bejelentése")
            .push(content)
            .height(Length::Fill)
            .into()
    }

    pub fn new_employee_view(d: &AppData) -> Element<'_, Message> {
        if let Some(worker) = &d.worker_selected {
            let mut buttons = vec![button(text("Vissza"))
                .padding(10)
                .on_press(Message::ChangeView(View::Main))
                .into()];
            if worker.has_valid_birthdate() {
                buttons.push(
                    button("Létrehozás")
                        .padding(10)
                        .on_press(Message::CreateWorker(worker.clone()))
                        .into(),
                )
            } else {
                buttons.push(text("Érvénytelen születési dátum!").into());
            }
            let mut w = window("Új munkavállaló").push(row(buttons).spacing(20));

            let w = w
                .push(row![
                    text("Név").width(Length::Units(100)),
                    text_input("Név", worker.name.as_ref(), Message::SetWorkerName)
                        .width(Length::Units(200))
                ])
                .push(row![
                    text("TAJ").width(Length::Units(100)),
                    text_input("TAJ", worker.taj.as_ref(), Message::SetWorkerTaj)
                        .width(Length::Units(200))
                ])
                .push(row![
                    text("Adószám").width(Length::Units(100)),
                    text_input(
                        "Adószám",
                        worker.taxnumber.as_ref(),
                        Message::SetWorkerTaxnumber
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Anyja neve").width(Length::Units(100)),
                    text_input(
                        "Anyja neve",
                        worker.mothersname.as_ref(),
                        Message::SetWorkerMothersname
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Születési dátum").width(Length::Units(100)),
                    text_input(
                        "Születési dátum",
                        &worker.birthdate.to_string(),
                        Message::SetWorkerBirthdate
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Születési helye").width(Length::Units(100)),
                    text_input(
                        "Születési helye",
                        worker.birthplace.as_ref(),
                        Message::SetWorkerBirthplace
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Irányítószám").width(Length::Units(100)),
                    text_input(
                        "Irányítószám",
                        worker.zip.to_string().as_str(),
                        Message::SetWorkerZip
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Település").width(Length::Units(100)),
                    text_input("Település", worker.city.as_ref(), Message::SetWorkerCity)
                        .width(Length::Units(200))
                ])
                .push(row![
                    text("Utca, házszám").width(Length::Units(100)),
                    text_input(
                        "Utca, házszám",
                        worker.street.as_ref(),
                        Message::SetWorkerStreet
                    )
                    .width(Length::Units(200))
                ]);

            w.into()
        } else {
            unimplemented!()
        }
    }

    pub fn edit_employee_view(d: &AppData) -> Element<'_, Message> {
        if let Some(worker) = &d.worker_selected {
            let mut buttons = vec![button(text("Vissza"))
                .padding(10)
                .on_press(Message::ChangeView(View::Main))
                .into()];
            if worker.has_valid_birthdate() {
                buttons.push(
                    button("Mentés")
                        .padding(10)
                        .on_press(Message::UpdateWorker(worker.clone()))
                        .into(),
                )
            } else {
                buttons.push(text("Érvénytelen születési dátum!").into());
            }
            let mut w = window("Munkavállaló szerkesztése").push(row(buttons).spacing(20));

            let w = w
                .push(row![
                    text("Név").width(Length::Units(100)),
                    text_input("Név", worker.name.as_ref(), Message::SetWorkerName)
                        .width(Length::Units(200))
                ])
                .push(row![
                    text("TAJ").width(Length::Units(100)),
                    text_input("TAJ", worker.taj.as_ref(), Message::SetWorkerTaj)
                        .width(Length::Units(200))
                ])
                .push(row![
                    text("Adószám").width(Length::Units(100)),
                    text_input(
                        "Adószám",
                        worker.taxnumber.as_ref(),
                        Message::SetWorkerTaxnumber
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Anyja neve").width(Length::Units(100)),
                    text_input(
                        "Anyja neve",
                        worker.mothersname.as_ref(),
                        Message::SetWorkerMothersname
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Születési dátum").width(Length::Units(100)),
                    text_input(
                        "Születési dátum",
                        &worker.birthdate.to_string(),
                        Message::SetWorkerBirthdate
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Születési helye").width(Length::Units(100)),
                    text_input(
                        "Születési helye",
                        worker.birthplace.as_ref(),
                        Message::SetWorkerBirthplace
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Irányítószám").width(Length::Units(100)),
                    text_input(
                        "Irányítószám",
                        worker.zip.to_string().as_str(),
                        Message::SetWorkerZip
                    )
                    .width(Length::Units(200))
                ])
                .push(row![
                    text("Település").width(Length::Units(100)),
                    text_input("Település", worker.city.as_ref(), Message::SetWorkerCity)
                        .width(Length::Units(200))
                ])
                .push(row![
                    text("Utca, házszám").width(Length::Units(100)),
                    text_input(
                        "Utca, házszám",
                        worker.street.as_ref(),
                        Message::SetWorkerStreet
                    )
                    .width(Length::Units(200))
                ]);

            w.into()
        } else {
            unimplemented!()
        }
    }
    pub fn modal_view(d: &AppData) -> Element<'_, Message> {
        let window = Column::new();
        if let View::Modal(msg) = &d.view {
            let window = window.push(
                text(msg)
                    .size(40)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            );
            return window.into();
        }
        window.into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DataLoading,
    DataLoaded(Result<Data, String>),
    NameFilterChange(String),
    RowAction(Uuid, RowAction),
    CreateWorker(Worker),
    UpdateWorker(Worker),
    SetWorkerName(String),
    SetWorkerTaj(String),
    SetWorkerTaxnumber(String),
    SetWorkerMothersname(String),
    SetWorkerBirthdate(String),
    SetWorkerBirthplace(String),
    SetWorkerZip(String),
    SetWorkerCity(String),
    SetWorkerStreet(String),
    ChangeView(View),
    Export,
    TabPressed { shift: bool },
}

async fn load_data() -> Result<Data, String> {
    let ctx = Context::new()?;
    let data = Data::init(ctx)?;
    Ok(data)
}

#[derive(Debug, Clone)]
pub enum RowAction {
    Selected(bool),
    Edit(Worker),
}

impl Worker {
    fn view(&self) -> Element<RowAction> {
        let checkbox = checkbox("", self.is_selected, RowAction::Selected).width(Length::Shrink);

        row![
            checkbox,
            text(&self.name).width(Length::Units(100)),
            text(&self.taj).width(Length::Units(50)),
            text(&self.birthdate).width(Length::Shrink),
            text(format!("{} {} {}", &self.zip, &self.city, &self.street))
                .width(Length::Units(100)),
            button(icon::edit_icon()).on_press(RowAction::Edit(self.clone())),
        ]
        .spacing(20)
        .into()
    }

    fn view_checkbox(&self) -> Element<RowAction> {
        let checkbox = checkbox("", self.is_selected, RowAction::Selected).width(Length::Shrink);
        row![
            checkbox,
            text(&format!(
                "{} ({} {}, {}) - {}",
                &self.name, &self.zip, &self.city, &self.street, &self.taj
            ))
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
