use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub struct Sound {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    source: String,
    volume: f32,
    loaded: bool,
    is_playing: bool,
    loop_sound:bool,
}

impl Sound {
    pub fn new(source: &String, volume: f32) -> Sound {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((stream, handle)) => (stream, handle),
            Err(_) => panic!("Failed to create stream"),
        };
        let sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(_) => panic!("Failed to create sink"),
        };
        Sound {
            stream,
            stream_handle,
            sink,
            source: source.clone(),
            volume,
            loaded: false,
            is_playing:false,
            loop_sound:true
        }
    }

    pub fn set_source(&mut self, source: &String){
        self.clear_if_playing();
        self.source=source.clone();
        self.loaded=false;
        self.load();
    }

    pub fn load(&mut self) {
        if self.loaded {
            return;
        }
        self.add_to_queue(&self.source.clone());
        //self.add_to_queue(&self.source.clone());
        self.set_volume(self.volume);
        //self.sink.append(SineWave::new(440.).take_duration(Duration::from_secs_f32(5.)).amplify(0.20));
        println!("Playing sound: {} with volume {}", self.source, self.volume);
        self.loaded = true;
    }

    pub fn add_to_queue(&mut self, source :&String){
        let file: BufReader<File> = BufReader::new(File::open(source).unwrap());
        let buffer = Decoder::new(file).unwrap();
        self.sink.append(buffer);
    }

    pub fn update(&mut self){
        let queue_size= self.sink.len();
        self.is_playing=queue_size!=0;
        if queue_size==1 && self.loop_sound{
            println!("Adding source to queue");
            self.add_to_queue(&self.source.clone());
        }
    }

    pub fn play(&mut self) {
        self.load();
        self.sink.play();
    }

    pub fn reload(&mut self){
        println!("Reloading sound");
        self.clear_if_playing();
        self.loaded=false;
        self.load();
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn clear_if_playing(&mut self){
        if self.is_playing(){
            self.stop();
            self.sink.clear();
        }
    }

    /*pub fn is_playing(&self) -> bool {
        let duration = self.stream_handle.duration();
        let position = self.sink.stream_position();
        if position >= duration {
            return true;
        }
        false
    }*/
}