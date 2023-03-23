use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}};
use iced::{Sandbox, Settings, widget::{column, row, button, text}};

use crate::{sample_load, playback};

//Another Underdeveloped Tracker for Underproduced Music... Now?

pub fn setup() -> Result<(), iced::Error> {
    AutumnApp::run(Settings::default())?;
    Ok(())
}

struct AutumnApp {
    host: cpal::Host,
    device: cpal::Device,
    sample_format: cpal::SampleFormat,
    audio_config: cpal::StreamConfig,
    sample_list: Vec<sample_load::WavSample>,
}

#[derive(Debug, Clone)]
enum Message {
    LoadSound,
    UnloadSound(usize),
    PlaySound(usize),

    // PlaySong,
    // PlayBlock(usize),
    // Pause,
    // Stop,
    // Resume,
}

impl Sandbox for AutumnApp {
    type Message = Message;

    fn new() -> Self {
        let host;
        #[cfg(target_os = "windows")]
        {
            host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to init ASIO host");
        }
        #[cfg(not(target_os = "windows"))]
        {
            host = cpal::default_host();
        }
        let device = host.default_output_device().expect("no output device available");
        let mut supported_configs_range = device.supported_output_configs().expect("error while querying configs");
        let supported = supported_configs_range.next().expect("no supported configs").with_max_sample_rate();
        let sample_format = supported.sample_format();
        let audio_config = supported.into();
        let stream = playback::get_stream(&device, &audio_config, &sample_format).unwrap();
        let sample_list = vec![];

        stream.play().unwrap();

        Self { 
            host,
            device,
            sample_format,
            audio_config,
            sample_list,
        }
    }

    fn title(&self) -> String {
        "AUTUMN".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LoadSound => {
                let sample = sample_load::WavSample::new();
                if let Ok(s) = sample {
                    self.sample_list.push(s);
                }
                else {
                    eprintln!("{:?}", sample.err());
                }
            },
            Message::UnloadSound(_) => println!("not implimented"),
            Message::PlaySound(i) => {

            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut agg = iced::widget::Column::new()
            .push(text("temp stuff..."))
            .push(button("Add Sound").on_press(Message::LoadSound));
        for i in 0..self.sample_list.len() {
            agg = agg.push(button("sample").on_press(Message::PlaySound(i)));
        }
        agg.into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}