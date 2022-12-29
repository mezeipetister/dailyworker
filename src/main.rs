use std::fs::File;
use std::io::Write;

use chrono::{Datelike, Local, NaiveDate, Timelike};
use iced::widget::{button, checkbox, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use storaget::*;
use worker::*;

mod worker;
mod xml;

#[derive(Debug, Default, Clone)]
pub enum View {
    #[default]
    Main,
    NewEmployee,
    EditEmployee(Option<Worker>),
}

#[derive(Debug, Default)]
pub struct AppData {
    name_filter: String,
    data: Option<Pack<worker::Data>>,
    view: View,
    worker_selected: Option<Worker>,
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
                            RowAction::Edit(worker) => {
                                self.view = View::EditEmployee(Some(worker.clone()));
                                self.worker_selected = Some(worker);
                            }
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
                    if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                        worker.birthdate = date;
                    }
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
                        worker.zip = zip;
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
                    let _ = data.as_mut().add_new_worker(worker);
                    self.view = View::Main;
                }
                Command::none()
            }
            Message::UpdateWorker(worker) => {
                if let Some(data) = &mut self.data {
                    let _ = data.as_mut().update_worker(worker);
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
            _ => unimplemented!(),
        }
    }
}

mod view {
    use crate::{AppData, Message, View};
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
                data.workers
                    .iter()
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
                    let a: Vec<Element<'_, Message, Renderer>> =
                        selected.iter().map(|i| text(&i.name).into()).collect();
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
            let mut w = window("Új munkavállaló").push(
                row(vec![
                    button(text("Vissza"))
                        .padding(10)
                        .on_press(Message::ChangeView(View::Main))
                        .into(),
                    button("Létrehozás")
                        .padding(10)
                        .on_press(Message::CreateWorker(worker.clone()))
                        .into(),
                ])
                .spacing(20),
            );

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
                ]);

            w.into()
        } else {
            unimplemented!()
        }
    }

    pub fn edit_employee_view(d: &AppData) -> Element<'_, Message> {
        if let Some(worker) = &d.worker_selected {
            let mut w = window("Munkavállaló szerkesztése").push(
                row(vec![
                    button(text("Vissza"))
                        .padding(10)
                        .on_press(Message::ChangeView(View::Main))
                        .into(),
                    button("Mentés")
                        .padding(10)
                        .on_press(Message::UpdateWorker(worker.clone()))
                        .into(),
                ])
                .spacing(20),
            );

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
                ]);

            w.into()
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DataLoading,
    DataLoaded(Result<Pack<Data>, String>),
    NameFilterChange(String),
    RowAction(u32, RowAction),
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
pub enum RowAction {
    Selected(bool),
    Edit(Worker),
}

impl Worker {
    fn view(&self) -> Element<RowAction> {
        let checkbox = checkbox("", self.is_selected, RowAction::Selected).width(Length::Shrink);

        row![
            checkbox,
            text(&self.name).width(Length::Units(50)),
            text(&self.taj).width(Length::Units(50)),
            text(&self.birthdate).width(Length::Shrink),
            text(format!("{} {} {}", &self.zip, &self.city, &self.street))
                .width(Length::Units(100)),
            button(text("Szerk.")).on_press(RowAction::Edit(self.clone())),
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
