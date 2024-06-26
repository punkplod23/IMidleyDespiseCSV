use fltk::{prelude::*, *,app, button::Button, frame::Frame, group::Flex,window::Window };
use std::{error::Error,process};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use csv_index::RandomAccessSimple;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

struct CSVData{
   header_count:i32,
   headers:HashMap<usize,String>,
   csv_data:Vec<HashMap<String,String>>,
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


//TODO:: Stick with the ABC.
//Write some error checking on the csv.
fn parse_csv(file_path: String) -> Result<(), Box<dyn Error>>{

   
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    let file_path_clone = file_path.clone();

    let file = File::open(file_path_clone)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);

    let mut rdr = csv::ReaderBuilder::new()
    .has_headers(true)
    .from_path(file_path)?;
    let mut rdr2 = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_reader(transcoded);

    let mut wtr = io::BufWriter::new(File::create("data.csv.idx")?);
    RandomAccessSimple::create(&mut rdr, &mut wtr)?;
    wtr.flush()?;

    let mut idx = RandomAccessSimple::open(File::open("data.csv.idx")?)?;
    if idx.is_empty() {
        return Err(From::from("expected a non-empty CSV index"));
    }
    let last = idx.len() - 1;
    let pos = idx.get(last)?;
    rdr.seek(pos)?;

    // Read the next record.
    if let Some(result) = rdr.records().next() {
        let record = result?;
        let record_length = record.len();  
        println!("{:?}", record_length);
        println!("{:?}", record);
   
        let complete: DateTime<Utc> = Utc::now();
        let mut csv_data = Vec::new();
        csv_data.clear();
        let mut ittr = 0;
        
        let mut header = HashMap::<usize,String>::new();
        for result in rdr2.records() {
    
            let recordcsv = result?;
       
            let mut map = HashMap::<String,String>::new();
            for n in 0..record_length {
                let value = recordcsv.get(n).as_ref().map(|x| &**x).unwrap_or("default string");
                let key = n;
                if ittr == 0{
                    header.insert(
                        key,
                        value.to_string(),
                    );
        
                }
                map.insert(
                    header[&key].to_string(),
                    value.to_string(),
                );
                //println!("{:?}", value);
            }
            csv_data.push(map);
            ittr = ittr+1;
        }
    
        let csv_data = CSVData{
            header_count:record_length as i32,
            headers:header,
            csv_data:csv_data,
        };
    
    
        println!("UTC now is: {}", complete);
        create_table(csv_data);

        Ok(())
    
    } else {
        Err(From::from("expected at least one record but got none"))
    }
    

}

fn create_table(csvdata:CSVData){

   
    let count = csvdata.csv_data.len() as i32;
    println!("The vector has {} elements.", count); // Prints "The vector has 5 elements."
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);
    let mut table = table::Table::default()
        .with_size(800 - 10, 600 - 10)
        .center_of(&wind);
    table.set_rows(count);
    table.set_row_header(true);
    table.set_row_resize(true);
    //Count the header get the correct amount
    table.set_cols(csvdata.header_count);
    table.set_col_header(true);
    table.set_col_width_all(80);
    table.set_col_resize(true);
    table.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

     // Called when the table is drawn then when it's redrawn due to events
     table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
        table::TableContext::ColHeader => {
            draw_header(&format!("{}", (col + 64) as u8 as char), x, y, w, h)
        } // Column titles
        //table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h), // Row titles
        table::TableContext::Cell => draw_data(
            &csvdata,
            row,
            col,
            x,
            y,
            w,
            h,
            t.is_selected(row, col),
        ), // Data in cells
        _ => (),
    });
    app.run().unwrap();

}

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(
        enums::FrameType::ThinUpBox,
        x,
        y,
        w,
        h,
        enums::Color::FrameDefault,
    );
    draw::set_draw_color(enums::Color::Black);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::pop_clip();
}

fn get_value_by_column(txt: &CSVData,col:i32,row:i32) -> String {
    let key = col as usize;
    let keyr = row as usize;
    println!("{}",txt.headers[&key]);
    let string_key = txt.headers[&key].to_string();
    return txt.csv_data[keyr][&string_key].to_string();
}



// The selected flag sets the color of the cell to a grayish color, otherwise white
fn draw_data(txt: &CSVData,row:i32,col:i32, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
    } else {
        draw::set_draw_color(enums::Color::White);
    }


    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Gray0);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(&get_value_by_column(txt,col,row), x, y, w, h, enums::Align::Center);
    draw::draw_rect(x, y, w, h);

    
    draw::pop_clip();

}
