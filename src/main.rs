use rfd::FileDialog;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use pyo3::prelude::*;

use iced::widget::{Button, Column, Container, Text};
use iced::{Element, Settings, Sandbox};

fn main() -> Result<(), iced::Error> {
    pyo3::prepare_freethreaded_python();
    MyApp::run(Settings::default())
}

struct MyApp {
    file_path: Option<String>,
    file_contents: String,
    metadata: Option<String>, // new field to store metadata
}

#[derive(Debug, Clone, Copy)]
enum MyAppMessage {
    OpenFile,
    SecondButton,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        MyApp {
            file_path: None,
            file_contents: String::new(),
            metadata: None, // initialize metadata as none
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

            MyAppMessage::SecondButton => {
                Python::with_gil(|py| {
                    let module = PyModule::import(py, "funcs").expect("No flying for you.");
                    let load_json = module.getattr("load_json").unwrap();
                    let get_metadata = module.getattr("get_metadata").unwrap();
                    let get_data = module.getattr("get_data").unwrap();
                    let save_func = module.getattr("save_dataframe_to_csv").unwrap();

                    // Import the student_gender variable from Python
                    let student_gender: PyObject = module.getattr("student_gender").unwrap().extract().unwrap();

                    // Import the publication type variable from Python
                    let publication_type: PyObject = module.getattr("publication_type_output").unwrap().extract().unwrap();
                    
                    if let Some(file_path) = &self.file_path {
                        // load json data
                        let data: PyObject = load_json.call1((file_path,)).unwrap().extract().unwrap();

                        // get "ShortTitle" data
                        let var1 = "ShortTitle"; // Replace with your desired variable
                        let ShortTitle: PyObject = get_metadata.call1((data.clone_ref(py), var1)).unwrap().extract().unwrap();

                        // get "Year" data
                        let var2 = "Year"; // Replace with your desired variable
                        let Year: PyObject = get_metadata.call1((data.clone_ref(py), var2)).unwrap().extract().unwrap();

                        // Call get_data function with student_gender data
                        let gender: PyObject = get_data.call1((data.clone_ref(py), student_gender)).unwrap().extract().unwrap();

                        // Call get_data function with student_gender data
                        let pub_type: PyObject = get_data.call1((data, publication_type)).unwrap().extract().unwrap();

                        // Print data
                        println!("Gender: {}", gender);
                        println!("ShortTitle: {}", ShortTitle);
                        println!("Year: {}", Year);
                        println!("Pub Type: {}", pub_type);


                    }
                });
            }
            
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let open_file_button = Button::new(Text::new("Open File"))
            .on_press(MyAppMessage::OpenFile);

        let second_button = Button::new(Text::new("Second Button"))
            .on_press(MyAppMessage::SecondButton);

        let mut col = Column::new();
        col = col.push(open_file_button);
        col = col.push(second_button);

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
