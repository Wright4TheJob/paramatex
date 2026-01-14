use std::collections::HashMap;
use std::{fs, path::PathBuf};
use calamine::{HeaderRow, open_workbook, Ods, Reader};
use iced::Alignment::Center;
use iced::widget::{button, center, column};
use iced::{Element, Theme};

fn substitute() {

    let path = format!("{}/example-parameters.ods", env!("CARGO_MANIFEST_DIR"));
    let mut excel: Ods<_> = open_workbook(path).unwrap();

    let mut text_string = fs::read_to_string("example-source.tex").expect("Unable to read file");

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
    fs::write("example-output.tex", text_string).expect("Unable to write file");
}

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
    FileCancelled,
    FileWritten,
    FileSelected,
}

#[derive(Default, Clone)]
struct State {
    parameters_path: Option<PathBuf>,
    template_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    output_filename: Option<String>,
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
        let write_button = if self.state.parameters_path.is_some() && self.state.template_path.is_some() && self.state.output_path.is_some() && self.state.output_filename.is_some() {
            button("Write Output File").on_press(Message::WriteOutput)
        } else {
            button("Write Output File")
        };
        center(
            column![button("Select Parameters Spreadsheet").on_press(Message::OpenParameters),]
                .push(button("Select LaTeX Template").on_press(Message::OpenTemplate))
                .push(write_button)
                .align_x(Center),
        )
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OpenParameters => self.open_filepath(File::Parameters),
            Message::OpenTemplate => self.open_filepath(File::Template),
            Message::WriteOutput => substitute(),
            Message::FileCancelled => (),
            Message::FileWritten => (),
            Message::FileSelected => (),
        }
        
    }


pub fn open_filepath(&mut self, filetype: File){
    let types = match filetype {
        File::Parameters => vec!("xlsx", "xls", "ods"),
        File::Template => vec!("tex"),
        File::Output => vec!("tex"),
    };

    let path_buf = rfd::FileDialog::new()
        .add_filter(
            "Spreadsheet Formats",
                types.as_slice()
            )
            .pick_file();
    match filetype {
        File::Parameters => {
            self.state.parameters_path = path_buf.clone();
        },
        File::Template => {
            self.state.template_path = path_buf.clone();
        },
        File::Output => {
            self.state.output_path = path_buf.clone();
        },
    } 
}
}
fn main() -> iced::Result {
    iced::application(  App::new, App::update, App::view).theme(Theme::Dark).centered().run()
}
