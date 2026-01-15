use std::collections::HashMap;
use std::{fs, path::PathBuf};
use calamine::{HeaderRow, open_workbook, Ods, Reader};
use iced::Alignment::Center;
use iced::widget::{button, center, column, row, text, text_input};
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
    OpenOutputDirectory,
    FileCancelled,
    OutputFilenameUpdated(String),
    FileWritten,
    FileSelected,
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
                    button("Select Parameters Spreadsheet").on_press(Message::OpenParameters),
                    text(format!("{:?}",self.state.parameters_path))]]
                .push(
                    row![
                        button("Select LaTeX Template").on_press(Message::OpenTemplate),
                        text(format!("{:?}",self.state.template_path))])
                .push(
                    row![
                    button("Select Output Directory").on_press(Message::OpenOutputDirectory),
                    text(format!("{:?}",self.state.output_path))]
                )
                .push(
                    row![
                        text("Output filename: "),
                        text_input("Filename", &self.state.output_filename).on_input(Message::OutputFilenameUpdated)
                    ]
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
            Message::WriteOutput => substitute(),
            Message::OutputFilenameUpdated(new_name) => self.state.output_filename = new_name,
            Message::FileCancelled => (),
            Message::FileWritten => (),
            Message::FileSelected => (),
        }
        
    }


pub fn open_filepath(&mut self, filetype: File){
    let types = match filetype {
        File::Parameters => vec!("xlsx", "xls", "ods"),
        File::Template => vec!("tex"),
        File::Output => vec!(),
    };

    let fd = rfd::FileDialog::new()
        .add_filter(
            "Format",
                types.as_slice()
            );
    let path_buf = match filetype {
        File::Parameters | File::Template => fd.pick_file(),
        File::Output => fd.pick_folder(),
    };
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
