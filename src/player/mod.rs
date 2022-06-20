use std::io::{BufReader, Error};
use std::thread::spawn;
use tokio::sync::mpsc;
use std::fs::File;
use crate::downloader::fetch_data;
use rodio::Source;
use std::borrow::Borrow;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::runtime::Builder;

pub struct MusicPlayer {
    cmd_spawn: mpsc::Sender<Task>,
}

#[derive(Debug)]
pub struct Task {
    pub key: String,
    pub url: String,
}
enum MusicPlayerEvent{
    SongEnd,
}

const CACHE_DIR: &str = "cache/";

impl MusicPlayer {
    pub fn default() -> MusicPlayer {
        let (send, mut recv) = mpsc::channel::<Task>(16);
        //let (event_send, mut event_recv) = mpsc::channel::<MusicPlayerEvent>(16);
        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        std::thread::spawn(move || {
            rt.block_on(async move {
                while let Some(task) = recv.recv().await {
                    println!("{:?}", task);
                    let file_path = [CACHE_DIR, &task.key].concat();
                    match File::open(&file_path) {
                        Ok(f) => (),
                        Err(_) => {
                            let mut file = File::create(&file_path).unwrap();
                            fetch_data(&task.url, &mut file).await;
                        }
                    };
                    let file = File::open(&file_path).unwrap();

                    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
                    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                    //sink.sleep_until_end()

                }
            });
        });
        MusicPlayer{
            cmd_spawn: send,
            //player_event_rec: event_recv
        }
    }

    pub async fn spawn_task(&self, task: Task){
        match self.cmd_spawn.send(task).await {
            Ok(()) => {},
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}