use iced::{
    executor::Default,
    theme::Theme,
    widget::{button, column, container, row, text, text_input},
    Application, Command, Element,
    Length::Fill,
};
use std::{
    fs,
    process,
};

#[derive(Debug, Clone)]
pub struct Gui {
    page: Page,
    err: String,
    code: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Main,
    Code,
    Confirm,
    Installing,
    Done,
    Err,
}

#[derive(Debug, Clone)]
pub enum Message {
    Page(Page),
    Err(String),
    Install,
    Quit,
    Check,
    Code(String),
}

impl Application for Gui {
    type Message = Message;
    type Executor = Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut commd = process::Command::new("mkdir");
        commd.arg(format!(
            "/Users/{}/Library/Application Support/cursed_pong",
            whoami::username()
        ));
        commd.status().expect("Failed to run command pre");
        (
            Self {
                page: Page::Main,
                err: String::new(),
                code: String::new(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        "Cursed Pong Installler".to_string()
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Page(page) => self.page = page,
            Message::Code(code) => self.code = code,
            Message::Err(err) => {
                self.err = err;
                self.page = Page::Err;
            }
            Message::Quit => process::exit(0),
            Message::Check => {
                let username = whoami::username();
                let mut commds = [process::Command::new("wget"), process::Command::new("unzip"), process::Command::new("rm"), process::Command::new("rm")];
                commds[0].arg("https://cursedpongdevs.github.io/cursed_pong/db/scores.zip");
                commds[1].arg("scores.zip");
                commds[2].arg("scores.zip");
                commds[3].arg("scores.scff");
                commds[0].current_dir(format!("/Users/{}/Library/Application Support/cursed_pong", username));
                commds[1].current_dir(format!("/Users/{}/Library/Application Support/cursed_pong", username));
                commds[2].current_dir(format!("/Users/{}/Library/Application Support/cursed_pong", username));
                commds[3].current_dir(format!("/Users/{}/Library/Application Support/cursed_pong", username));
                commds[0].status().expect("Failed to get scores");
                commds[1].status().expect("Failed to unzip scores");
                commds[2].status().expect("Failed to remove zip file");
                let text = fs::read_to_string(format!("/Users/{}/Library/Application Support/cursed_pong/scores.scff", username)).expect("Failed to read from strings");
                let mut parts = vec![];
                let split: Vec<&str> = text.split(" ").collect();
                let mut cont = false;
                for part in &split {
                    if *part != &self.code {
                        parts.push(part.to_string());
                    } else {
                        cont = true;
                    }
                }
                if cont == false && parts.len() == split.len() {
                    self.err = "Invalid code".to_string();
                    self.page = Page::Err;
                } else {
                    self.page = Page::Confirm;
                }
                commds[3].status().expect("Failed to remove scores file");
            }
            Message::Install => {
                self.page = Page::Installing;
                let username = whoami::username();
                let mut commds = [
                    process::Command::new("wget"),
                    process::Command::new("unzip"),
                    process::Command::new("rm"),
                    process::Command::new("wget"),
                    process::Command::new("unzip"),
                    process::Command::new("rm"),
                ];
                commds[0].arg("https://cursedpongdevs.github.io/cursed_pong/files/files.zip");
                commds[0].current_dir(format!(
                    "/Users/{}/Library/Application Support/cursed_pong",
                    username
                ));
                commds[1].arg("files.zip");
                commds[1].current_dir(format!(
                    "/Users/{}/Library/Application Support/cursed_pong",
                    username
                ));
                commds[2].arg("files.zip");
                commds[2].current_dir(format!(
                    "/Users/{}/Library/Application Support/cursed_pong",
                    username
                ));
                commds[3].arg("https://cursedpongdevs.github.io/cursed_pong/files/macos.zip");
                commds[3].current_dir(format!("/Users/{}/Applications", username));
                commds[4].arg("macos.zip");
                commds[4].current_dir(format!("/Users/{}/Applications", username));
                commds[5].arg("macos.zip");
                commds[5].current_dir(format!("/Users/{}/Applications", username));
                let mut count = 1;
                for mut commd in commds {
                    commd
                        .status()
                        .expect(&format!("Failed to run command {}", count));
                    count += 1;
                }
                self.page = Page::Done;
            }
        }

        Command::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let main = container(column![
            text("Cursed Pong Installer").size(50),
            button("Start").on_press(Message::Page(Page::Code)),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();
        let code = container(column![
            text("Plese enter your code").size(50),
            row![text("Code: "), text_input(" ", &self.code, Message::Code)],
            button("Check").on_press(Message::Check),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();
        let confirm = container(column![
            text("Are you sure you would like to install Cursed Pong?").size(50),
            row![
                button("Yes").on_press(Message::Install),
                button("No").on_press(Message::Quit)
            ],
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();
        let installing = container(column![text("Installing...").size(50)])
            .center_x()
            .center_y()
            .height(Fill)
            .width(Fill)
            .into();
        let done = container(column![
            text("Done!").size(50),
            button("Finish").on_press(Message::Quit),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();
        let err = container(column![
            text(&self.err).size(50),
            button("Exit").on_press(Message::Quit),
        ])
        .center_x()
        .center_y()
        .height(Fill)
        .width(Fill)
        .into();

        match self.page {
            Page::Main => main,
            Page::Code => code,
            Page::Confirm => confirm,
            Page::Installing => installing,
            Page::Done => done,
            Page::Err => err,
        }
    }
}
