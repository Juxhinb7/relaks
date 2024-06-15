/* use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct Track<'track_struct_lifetime> {
    pub file_path: &'track_struct_lifetime str
}

impl Track<'_> {
    pub fn new(file_path: &str) -> Track<'_> {
        Track { file_path }
    }

    fn _get_output_stream(&self) -> (OutputStream, OutputStreamHandle) {
        OutputStream::try_default().unwrap()
    }

    fn _load_file(&self) -> BufReader<File> {
        BufReader::new(File::open(self.file_path).unwrap())
    }

    fn _decode_into_source(&self) -> Decoder<BufReader<File>> {
        Decoder::new(self._load_file()).unwrap()
    }

    pub fn play(&self, stream_handle: OutputStreamHandle) {
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(self._decode_into_source());
        sink.sleep_until_end(); 
    }
} */
use crate::ui_util::{self};
use crate::cli_util::cli;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use rand::Rng;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use clap::ArgMatches;
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen}, ExecutableCommand};
use ratatui::prelude::*;

const LOGO: &str = r#"
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~██████╗ ███████╗██╗      █████╗ ██╗  ██╗███████╗~
~██╔══██╗██╔════╝██║     ██╔══██╗██║ ██╔╝██╔════╝~
~██████╔╝█████╗  ██║     ███████║█████╔╝ ███████╗~
~██╔══██╗██╔══╝  ██║     ██╔══██║██╔═██╗ ╚════██║~
~██║  ██║███████╗███████╗██║  ██║██║  ██╗███████║~
~╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~                                                                                
"#;

pub static CURRENT_TRACK: Mutex<String> = Mutex::new(String::new());
pub static DESC: Mutex<String> = Mutex::new(String::new());
pub static SHOULD_QUIT: Mutex<bool> = Mutex::new(false);
pub static IS_ANIMATED: Mutex<bool> = Mutex::new(true);

pub enum MusicMode {
    Playlist,
    Specific
}

pub struct Application {
    pub tracks: Arc<Mutex<Vec<&'static str>>>,
    pub infos: Arc<Mutex<Vec<&'static str>>>,
}

impl Application {
    pub fn new<'a>(tracks: Vec<&'static str>, infos: Vec<&'static str>) -> Application {
        Application { 
            tracks: Arc::new(Mutex::new(tracks)),
            infos: Arc::new(Mutex::new(infos)),
        }
    } 
    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        
        let queue = Arc::clone(&self.tracks);

        let info_queue = Arc::clone(&self.infos);

        
        let handle = thread::spawn(move || {

            while let Some(track) = get_next_track(&queue, &info_queue) {
                play_track(&stream_handle, &track[0], &track[1], MusicMode::Playlist);
            }
            if let None = get_next_track(&queue, &info_queue) {
                *SHOULD_QUIT.lock().unwrap() = true;
            }

        });

        draw_ui(handle)?;
            
        Ok(())

    }
    pub fn get_matches(&self) -> ArgMatches {
        cli(LOGO).get_matches()
    }
}

pub fn get_next_track(queue: &Arc<Mutex<Vec<&'static str>>>, info_queue: &Arc<Mutex<Vec<&'static str>>>) -> Option<[String; 2]> {
    let mut queue = queue.lock().unwrap();
    let mut info_queue = info_queue.lock().unwrap();
    if queue.is_empty() {
        None
    } else {
        Some([queue.remove(0).to_string(), info_queue.remove(0).to_string()])
    }
}

pub fn play_track(stream_handle: &OutputStreamHandle, filename: &str, info: &str, mode: MusicMode) {
    let file = File::open(format!("sounds/{filename}.mp3")).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::try_new(stream_handle).unwrap();
    sink.append(source);
    *CURRENT_TRACK.lock().unwrap() = filename.to_string();
    *DESC.lock().unwrap() = info.to_string();
    

    while !sink.empty() {
        thread::sleep(Duration::from_millis(50));
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Enter {
                if sink.is_paused() {
                    sink.play();
                    *IS_ANIMATED.lock().unwrap() = true;

                } else {
                    sink.pause();
                    *IS_ANIMATED.lock().unwrap() = false;
                } 
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {

                *SHOULD_QUIT.lock().unwrap() = true;
            }

            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Right {
                
                match mode {
                    MusicMode::Specific => {
                        *SHOULD_QUIT.lock().unwrap() = true;
                    },
                    MusicMode::Playlist => {
                        if sink.is_paused() {
                            *IS_ANIMATED.lock().unwrap() = true;
                        }
                        sink.skip_one()
                    }
                }

            }
        }
    }
}

pub fn draw_ui(handle: JoinHandle<()>,) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    while !*SHOULD_QUIT.lock()? {
        thread::sleep(std::time::Duration::from_millis(50));

        let mut rng = rand::thread_rng();

        let visual_data: Vec<u64> = (0..6).map(|_| rng.gen_range(0..100)).collect();

        if *IS_ANIMATED.lock().unwrap() {
            terminal.draw(|f| ui_util::build_ui(f, visual_data))?;
        }


    }

    if *SHOULD_QUIT.lock()? {
        disable_raw_mode()?;
        std::io::stdout().execute(LeaveAlternateScreen)?;
        return Ok(());
    }


    handle.join().unwrap();
    Ok(())
}