mod util;
mod model;
mod api;
mod downloader;
mod player;
mod ui;

use std::borrow::Borrow;
use std::process::Command;
use std::thread;
use std::io;
use std::collections::HashMap;
use std::time::Duration;
use qrcode_generator::QrCodeEcc;
use crate::api::*;
use crate::model::login::*;
use crate::model::user::*;
use crate::model::playlist::*;
use crate::util::Encrypt;
use crate::util::http::CloudMusic;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend}, Terminal,
};

use rodio::{Decoder, OutputStream, source::Source};
use tokio::runtime::Builder;
use tui::widgets::ListState;
use crate::player::{MusicPlayer, Task};
use crate::ui::main_ui;

#[derive(Clone, Debug)]
pub struct App {
    pub menuType: MenuType,
    pub user_profile: Option<UserProfile>,
    pub playlists: Vec<PlayList>,
    pub current_playlist: ListState,
    pub local_tracks: Vec<Track>,
    pub current_track: ListState,
    pub music_player: MusicPlayer,
}

#[derive(Clone, Debug)]
pub enum MenuType {
    Playlists, Tracks
}

impl App {
    pub async fn default () -> App {
        let music_player = player::MusicPlayer::default();
        let mut  current_playlist = ListState::default();
        current_playlist.select(Some(0));
        let mut app = App {
            menuType: MenuType::Playlists,
            user_profile: None,
            playlists: vec![],
            current_playlist,
            local_tracks: vec![],
            current_track: ListState::default(),
            music_player
        };
        let client = CloudMusic::default();
        let needLogin = false;
        if needLogin {
            let mut params= HashMap::<String, String>::new();
            params.insert("type".to_string(), "1".to_string());
            let res = client.post("/weapi/login/qrcode/unikey", &mut params).await;
            let qrkey: QrcodeUnikey = serde_json::from_str(&res).unwrap();

            // Encode some data into bits.
            let code = format!("/login?codekey={}", &qrkey.unikey);
            qrcode_generator::to_png_to_file(code, QrCodeEcc::Low, 1024, "qrcode.png").unwrap();
            Command::new("sh").arg("-c").arg("xdg-open tests/data/qrcode.png").output().expect("sh exec error!");


            for i in 1..20 {
                thread::sleep(Duration::from_secs(2));
                let mut params= HashMap::<String, String>::new();
                params.insert("type".to_string(), "1".to_string());
                params.insert("key".to_string(), qrkey.unikey.clone());
                let res = client.post("/weapi/login/qrcode/client/login", &mut params).await;
                let qr_check: QrcodeCheck = serde_json::from_str(&res).unwrap();
                println!("check[{}], message: {:?}", i, &qr_check.message);
                if qr_check.code == 803 {
                    break
                }
            }
        }

        let user_profile = user_profile().await;
        app.user_profile = Some(user_profile);

        let mut playlist = user_playlist(app.user_profile.clone().unwrap().userId.to_string().borrow()).await;
        app.playlists.append(&mut playlist);

        let playlist_detail = playlist_detail(&app.playlists[0].id.to_string()).await;

        app.local_tracks.append(&mut playlist_detail.tracks.unwrap());

        /*;
        */
        app
    }

    pub async fn select_playlist(&mut self){
        let playlist_id = self.playlists[self.current_playlist.selected().unwrap_or(0)].id;
        let playlist_detail = playlist_detail(&playlist_id.to_string()).await;
        self.local_tracks.clear();
        self.local_tracks.append(&mut playlist_detail.tracks.unwrap());
        self.current_track.select(Some(0));
    }

    pub async fn play_track(&mut self){
        let current_track = self.local_tracks[self.current_track.selected().unwrap_or(0)].to_owned();
        //println!("{}", current_track.name);
        let player_info = player_info(&current_track.id.to_string()).await;
        //println!("song url {:?}", player_info.url);
        //fetch_data(&player_info.url).await;
        let d = Task{
            key: [player_info.md5.clone(), ".".to_string(), player_info.typ.clone()].concat(),
            url: player_info.url.clone()
        };
        self.music_player.spawn_task(d).await
    }
}

#[tokio::main]
pub async fn main(){
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    //let tick_rate = Duration::from_millis(250);

    let mut app = App::default().await;
    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();

    if let Err(err) = res {
        println!("{:?}", err)
    }
    //a().await;
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| main_ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('w')=> {
                    match app.menuType {
                        MenuType::Playlists => {
                            app.current_playlist.select(Some(
                                (app.current_playlist.selected().unwrap_or(0) + app.playlists.len() - 1) % app.playlists.len()
                            ))
                        }
                        MenuType::Tracks => {
                            app.current_track.select(Some(
                                (app.current_track.selected().unwrap_or(0) + app.local_tracks.len() - 1) % app.local_tracks.len()
                            ))
                        }
                    }
                }
                KeyCode::Char('s')=> {
                    match app.menuType {
                        MenuType::Playlists => {
                            app.current_playlist.select(Some(
                                (app.current_playlist.selected().unwrap_or(0) + app.playlists.len() + 1) % app.playlists.len()
                            ))
                        }
                        MenuType::Tracks => {
                            app.current_track.select(Some(
                                (app.current_track.selected().unwrap_or(0) + app.local_tracks.len() + 1) % app.local_tracks.len()
                            ))
                        }
                    }
                }
                KeyCode::Char('f')=> {
                    match app.menuType {
                        MenuType::Playlists => {
                            app.select_playlist().await;
                            app.menuType = MenuType::Tracks;
                        }
                        MenuType::Tracks => {
                            app.play_track().await;
                        }
                    }
                }
                KeyCode::Char('a')=> {
                   match app.menuType {
                       MenuType::Playlists => {app.menuType = MenuType::Tracks}
                       MenuType::Tracks => {app.menuType = MenuType::Playlists}
                   }
                }
                KeyCode::Char('d')=> {
                    match app.menuType {
                        MenuType::Playlists => {app.menuType = MenuType::Tracks}
                        MenuType::Tracks => {app.menuType = MenuType::Playlists}
                    }
                }
                KeyCode::Char('x') => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

