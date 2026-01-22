use std::collections::HashMap;
use std::{fs, path::PathBuf};
use calamine::{HeaderRow, open_workbook, Ods, Reader};
use iced::Alignment::Center;
use iced::widget::{button, center, column, row, text, text_input};
use iced::{event::{self, Status},keyboard::{Event::KeyPressed}, Event, Element, Theme, Length};
use iced::keyboard::key::{Key, Named};

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Error {
    InvalidImage,
}

#[derive(Debug, Clone)]
enum File {
    Parameters,
    Template,
    Output,
}

#[derive(Debug, Clone)]
enum Message {
    WriteOutput,
    OpenParameters,
    OpenTemplate,
    OpenOutputDirectory,
    OutputFilenameUpdated(String),
}

#[derive(Default, Clone)]
struct State {
    parameters_path: Option<PathBuf>,
    template_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    output_filename: String,
}

#[derive(Default)]
struct App {
    state: State,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::default(),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let write_button = if self.state.parameters_path.is_some() && self.state.template_path.is_some() && self.state.output_path.is_some() && self.state.output_filename != "" {
            button("Write Output File").on_press(Message::WriteOutput)
        } else {
            button("Write Output File")
        };
        center(
            column![
                row![
                    button("Select Parameters Spreadsheet")
                        .width(Length::FillPortion(1))
                        .on_press(Message::OpenParameters),
                    text(format!("{:?}",self.state.parameters_path))
                        .width(Length::FillPortion(1))
                    ]
                    .spacing(20)
                ].spacing(20)
                .push(
                    row![
                        button("Select LaTeX Template")
                            .width(Length::FillPortion(1))
                            .on_press(Message::OpenTemplate),
                        text(format!("{:?}",self.state.template_path))
                            .width(Length::FillPortion(1))
                    ]
                    .spacing(20)
                )
                .push(
                    row![
                    button("Select Output Directory")
                        .width(Length::FillPortion(1))
                        .on_press(Message::OpenOutputDirectory),
                    text(format!("{:?}",self.state.output_path))
                        .width(Length::FillPortion(1))
                    ]
                    .spacing(20)
                )
                .push(
                    row![
                        text("Output filename: "),
                        text_input("Filename", &self.state.output_filename)
                            .width(Length::FillPortion(1))
                            .on_input(Message::OutputFilenameUpdated)
                    ]
                    .spacing(20)
                )
                .push(write_button)
                .align_x(Center),
                
        )
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenParameters => self.open_filepath(File::Parameters),
            Message::OpenTemplate => self.open_filepath(File::Template),
            Message::OpenOutputDirectory => self.open_filepath(File::Output),
            Message::WriteOutput => {
                
            if self.state.parameters_path.is_some() && self.state.template_path.is_some() && self.state.output_path.is_some() && self.state.output_filename != "" {
                let mut output_path = self.state.output_path.clone().unwrap();
                output_path.push(self.state.output_filename.clone());
                output_path.push(".tex");
                substitute(self.state.parameters_path.clone().unwrap(), self.state.template_path.clone().unwrap(), output_path);
            }
            },
            Message::OutputFilenameUpdated(new_name) => self.state.output_filename = new_name,
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, status, _| match (event, status) {
            (
                Event::Keyboard(KeyPressed { 
                key: Key::Character(key),
                ..
            }),
            Status::Captured,
            )  => {
                match key.as_str() {
                    "p" => Some(Message::OpenParameters),
                    &_ => None
                }
            }, 
             
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Enter),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::WriteOutput),
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(Message::OpenParameters),
            _ => None,
        })
    }

    pub fn open_filepath(&mut self, filetype: File){
        match filetype {
        File::Parameters => {
            self.state.parameters_path = rfd::FileDialog::new()
                .add_filter("Spreadsheet Formats",vec!("xlsx", "xls", "ods").as_slice())
                .pick_file();
        },
        File::Template => {
            self.state.template_path = rfd::FileDialog::new()
                .add_filter("LaTeX Formats",vec!("tex").as_slice())
                .pick_file();
        },
        File::Output => {
            self.state.output_path = rfd::FileDialog::new()
                .pick_folder();
        }
    }
}
}
fn substitute(param_path: PathBuf, template_path: PathBuf, output_path: PathBuf) {

    let mut excel: Ods<_> = open_workbook(param_path).unwrap();

    let mut text_string = fs::read_to_string(template_path).expect("Unable to read file");

    let sheet1 = excel
    .with_header_row(HeaderRow::Row(1))
    .worksheet_range("Sheet1")
    .unwrap();
    let mut values: HashMap<String, String> = HashMap::new();
    for row in 0..sheet1.height() {
        let key = sheet1.get_value((row as u32, 1 as u32)); 
        let value = sheet1.get_value((row as u32, 2 as u32));
        if key.is_some() && value.is_some() {
            values.insert(
                key.unwrap().to_string(),
                value.unwrap().to_string(),
            );
        } 
    }
    for (key, value) in values.iter() {
        text_string = text_string.replace(key, value);
    }
    fs::write(output_path, text_string).expect("Unable to write file");
}
fn main() -> iced::Result {
    iced::application(  App::new, App::update, App::view).subscription(App::subscription).theme(Theme::Dark).centered().run()
}
