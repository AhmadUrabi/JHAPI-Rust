use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use rocket::tokio::io::AsyncReadExt;
use std::io::Read;
use tokio::fs::File;
use ssh2::Session;

use magick_rust::{MagickWand, magick_wand_genesis, PixelWand};
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();


// Testing image resize function
fn resize(filename: &str,name: &str)  {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();


    wand.read_image(&filename).unwrap();
    
    let mut pixelwand = PixelWand::new();
    pixelwand.set_color("white").unwrap();
    wand.set_background_color(&pixelwand).unwrap();
    wand.set_format("jpg").unwrap();
    let temp: Vec<u8> = wand.write_image_blob("jpg").unwrap();
    let mut newWand = MagickWand::new();
    
    newWand.read_image_blob(&temp).unwrap();
    newWand.set_image_gravity(5).unwrap();
    newWand.set_gravity(5).unwrap();
    newWand.fit(640,640);
    
    let width = newWand.get_image_width() as isize;
    let x_offset: isize = (640 - width) / 2 * -1;

    let height = newWand.get_image_height() as isize;
    let y_offset: isize = (640 - height) / 2 * -1;

    newWand.extend_image(640,640, x_offset, y_offset).unwrap();
    
    let resFile = "tmp/".to_string() + name + ".jpg";

    newWand.write_image(&resFile).unwrap();
}


pub async fn download_file(file_name: &String) -> bool { 

    // Delete temporary file
    // first check if file exists
    let file_path = Path::new("tmp/tmpdownload.jpg");
    if file_path.exists() {
        std::fs::remove_file("tmp/tmpdownload.jpg").unwrap();
    }

    // Connect to the local SSH server
    let tcp = TcpStream::connect(std::env::var("SFTP_HOST").expect("SFTP_HOST must be set.")).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    match sess.handshake() {
        Ok(_) => println!("Handshake successful"),
        Err(e) => {
            error!("Handshake failed: {:?}", e);
            return false;},
    };

    let username = std::env::var("SFTP_USERNAME").expect("SFTP_USERNAME must be set.");
    let password = std::env::var("SFTP_PASSWORD").expect("SFTP_PASSWORD must be set.");

    match sess.userauth_password(&username, &password){
        Ok(_) => println!("Authentication successful"),
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return false;
        },
    };

    let filetarget = "/u02/forms/erp/images/".to_string() + file_name + ".jpg";

    if let Ok((mut remote_file, stat)) = sess.scp_recv(Path::new(&filetarget)) {
        println!("File exists");
        println!("remote file size: {}", stat.size());
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();

        // Close the channel and wait for the whole content to be tranferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        //Save file

        let mut local_file = std::fs::File::create("tmp/tmpdownload.jpg").unwrap();
        local_file.write_all(&contents).unwrap();
        true
    } else {
        println!("File does not exist");
        return false;
    }


}



pub async fn upload_file(item_code: &String, filepath: &String) -> bool {  
    resize(&filepath, &item_code);
    let tempFile = "tmp/".to_string() + item_code + ".jpg";

    let mut f = File::open(&tempFile).await.unwrap();
    let mut buffer = Vec::new();
    let fileSize = f.metadata().await.unwrap().len();
    f.read_to_end(&mut buffer).await.ok();

    let tcp = TcpStream::connect(std::env::var("SFTP_HOST").expect("SFTP_HOST must be set.")).unwrap();
    if let Ok(mut sess) = Session::new() {
        println!("Session created");
        
    sess.set_tcp_stream(tcp);
    match sess.handshake() {
        Ok(_) => println!("Handshake successful"),
        Err(e) => {
            error!("Handshake failed: {:?}", e);
            return false;},
    };

    let username = std::env::var("SFTP_USERNAME").expect("SFTP_USERNAME must be set.");
    let password = std::env::var("SFTP_PASSWORD").expect("SFTP_PASSWORD must be set.");

    match sess.userauth_password(&username, &password){
        Ok(_) => println!("Authentication successful"),
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return false;
        },
    };

    let fileName = "/u02/forms/erp/images/".to_string() + item_code + ".jpg";

    if let Ok(mut remote_file) = sess.scp_send(Path::new(&fileName), 0o644, fileSize, None) {
        println!("File Send successful");
        println!("remote file size: {}", fileSize);
        remote_file.write_all(&buffer).unwrap();
        // Close the channel and wait for the whole content to be tranferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();
        std::fs::remove_file(tempFile).unwrap();
        return true
    } else {
        println!("File Send Unsuccessful");
        std::fs::remove_file(tempFile).unwrap();
        return false;
    }
    } else {
        println!("Session not created");
        std::fs::remove_file(tempFile).unwrap();
        return false;
    }


}