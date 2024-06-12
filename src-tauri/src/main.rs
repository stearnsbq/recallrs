// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



use std::sync::Arc;
use std::path;
use std::thread::spawn;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use oasysdb::metadata::Metadata;
use ocr::ocr::OCRService;
use text_embedding::text_embedding::TextEmbeddingService;
use vector_database::vector_database::VectorDatabaseService;
use xcap::Monitor;
mod ocr;
mod text_embedding;
mod vector_database;
use std::fs;
use std::path::PathBuf;

const RUN_CAPTURE : AtomicBool = AtomicBool::new(false);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}


fn take_screenshot(){

    let monitors = Monitor::all().unwrap();

    let ocr_service = OCRService::new("resources/model/text-detection.rten", "resources/model/text-recognition.rten");
    let embed_service = TextEmbeddingService::new(None);
    let mut db_service = VectorDatabaseService::new("data");


    for monitor in monitors {
       // println!("ocr on monitor");
        let image = monitor.capture_image().unwrap();

        let texts = ocr_service.get_lines_from_image(&image);

        let embeddings = embed_service.embed(texts.iter().map(|s| s.as_str()).collect()).unwrap();


        for (i, vector) in embeddings.iter().enumerate(){
            //println!("Saving vector");

            let text = texts.get(i)
            .unwrap_or(&String::from(""))
            .to_string();

            let metadata = Metadata::Text(text);

        

            db_service.add_vector("farts", vector.to_vec(), Some(&metadata)).unwrap();
        }

    
    

    }

    let query = "ONNX";

    let query_embedded = embed_service.embed(vec![query]).unwrap();

    let results = db_service.query("farts", query_embedded.into_iter().nth(0).unwrap(), 5).unwrap();

    for result in results{
        let metadata = db_service.get("farts", result.id).unwrap().data;

        match metadata {
            Metadata::Text(txt) =>  println!("{}", txt),
            Metadata::Integer(_) => todo!(),
            Metadata::Float(_) => todo!(),
            Metadata::Array(_) => todo!(),
            Metadata::Object(_) => todo!(),
        }

       ;
    }


}

fn capture_thread()  {
    let interval = Duration::from_secs(5);


    loop {
        if RUN_CAPTURE.load(Ordering::Relaxed) {
            break;
        }
        take_screenshot();
        sleep(interval);
    }

}

fn main() {
   // spawn(capture_thread);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    

 
}
