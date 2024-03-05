use fltk::{prelude::*, *,app, button::Button, frame::Frame, group::Flex,window::Window };
use std::{error::Error, process};
use chrono::{DateTime, Utc};

struct Person {
    town: String,
    district: String,
    lon: String,
    lat: String,
}


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

fn choose_file(){

    let mut chooser = dialog::FileChooser::new(
        ".",                    // directory
        "*.csv",                    // filter or pattern
        dialog::FileChooserType::Single, // chooser type
        "Select CSV",     // title
    );
    chooser.show();
    chooser.window().set_pos(300, 300);
    // Block until user picks something.
    //     (The other way to do this is to use a callback())
    //
    while chooser.shown() {
        app::wait();
    }
    // User hit cancel?
    if chooser.value(1).is_none() {
        println!("(User hit 'Cancel')");
        return;
    }
    let file_path = chooser.value(1).unwrap();

    if let Err(err) = parse_csv(file_path) {
        println!("error running example: {}", err);
        process::exit(1);

    }

  
}

fn parse_csv(file_path: String) -> Result<(), Box<dyn Error>> {

    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);

    let mut rdr = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_path(file_path)?;

    let persons: Vec<Person> = rdr.records()
    .map(|result| result.expect("error reading CSV record"))
    .map(|record| Person {

        town: record[0].to_string(),
        district: record[1].to_string(),
        lon: record[2].to_string(),
        lat: record[3].to_string()
    })
    .collect();

    
    let count = persons.len();

  
    println!("The vector has {} elements.", count); // Prints "The vector has 5 elements."
  
    let complete: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", complete);
    Ok(())
}
