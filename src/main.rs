use fltk::{prelude::*,app, button::Button, frame::Frame, group::Flex,window::Window };
use std::{error::Error,process};
use chrono::{DateTime, Utc};
use std::io::{Write};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use csv::StringRecord;
use std::fs::OpenOptions;

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut col = Flex::default_fill().column();
    col.set_margins(120, 80, 120, 80);
    let _frame = Frame::default();
    let mut but = Button::default().with_label("Select CSV");

    col.fixed(&but, 40);
    col.end();
    wind.make_resizable(true);
    wind.end();
    wind.show();

    but.set_callback(move |_| choose_file());

    app.run().unwrap();
}


fn choose_file() {

    let path = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("CSV", &["csv", "CSV"])
    .show_open_single_file()
    .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        println!("{}",path.display().to_string());
        if let Err(err) =  parse_csv(path.display().to_string()) {
            println!("error running example: {}", err);
            process::exit(1);
        }
       
    }    

}

fn parse_csv(file_path: String) -> Result<(), Box<dyn Error>>{

    println!("{}",file_path);

    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("my-file.txt")
    .unwrap(); 

    let mut rdr = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_path(file_path)?;
    let mut ittr = 0;
    let mut header = StringRecord::new();
     
    if let Err(e) = write!(file, "[") {
        eprintln!("Couldn't write to file: {}", e);
    }
    
    if let Some(result) = rdr.records().next() {
        header = result?;
    } 

    for result in rdr.records() {      

        let json = create_json_record_map(header.clone(), result?,ittr).to_string();
        if let Err(e) = write!(file,"{}", json) {
            eprintln!("Couldn't write to file: {}", e);
        }
  
        ittr = ittr+1;
    }
    if let Err(e) = write!(file, "]") {
        eprintln!("Couldn't write to file: {}", e);
    }

    let now: DateTime<Utc> = Utc::now();

    println!("End:: {}", now);
    Ok(())
}

fn create_json_record_map(mut _keys: StringRecord,mut values: StringRecord,record_no: i32) -> String {
    let record_length = values.len(); 
    let mut json = String::new();
    if record_no > 0{
        json+= ",";
    }
    json+="{";
    for n in 0..record_length {
        if n > 0{
            json+= ",";
        }
        json+= "'";
        json+= _keys.get(n).as_ref().map(|x| &**x).unwrap_or("default string");
        json+= "'";
        json+= ":";
        json+= "'";
        json+= values.get(n).as_ref().map(|x| &**x).unwrap_or("default string");
        json+= "'";
    }
    values.clear();
    json+="}";
    return json;
}