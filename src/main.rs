use rfd::FileDialog;
use std::fs::File;
use std::io::Read;
use serde_json::Value;

use iced::widget::{Button, Column, Container, Text};

use iced::{Element, Settings, Sandbox};

fn main() -> Result<(), iced::Error> {
    MyApp::run(Settings::default())
}

struct MyApp {
    file_path: Option<String>,
    file_contents: String,
}

#[derive(Debug, Clone, Copy)]
enum MyAppMessage {
    OpenFile,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        MyApp {
            file_path: None,
            file_contents: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::OpenFile => {
                let file = FileDialog::new()
                    .add_filter("JSON Files", &["json"])
                    .pick_file();
                match file {
                    Some(file_path) => {
                        self.file_path = Some(file_path.to_string_lossy().into_owned());

                        let mut file = File::open(&file_path).expect("Could not open file");
                        let mut contents = String::new();

                        file.read_to_string(&mut contents).expect("Could not read file");

                        let json: Value =
                            serde_json::from_str(&contents).expect("Could not parse JSON");
                        let pretty_json =
                            serde_json::to_string_pretty(&json).expect("Could not pretty print JSON");

                        self.file_contents = pretty_json.clone();
                        println!("File contents: {}", pretty_json);

                        println!("Selected file: {:?}", file_path);
                    }
                    None => println!("No file selected"),
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let open_file_button = Button::new(Text::new("Open File"))
            .on_press(MyAppMessage::OpenFile);

        let mut col = Column::new();
        col = col.push(open_file_button);

        if let Some(file_path) = &self.file_path {
            col = col.push(Text::new(file_path).width(iced::Length::Fill));
        }

        Container::new(col)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
