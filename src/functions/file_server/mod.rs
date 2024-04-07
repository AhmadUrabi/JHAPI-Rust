use rocket::tokio::io::AsyncReadExt;
use ssh2::Session;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use tokio::fs::File;

use magick_rust::{magick_wand_genesis, MagickWand, PixelWand};
use std::sync::Once;

use crate::utils::structs::APIErrors;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

// Testing image resize function
fn resize(filename: &str, name: &str) -> Result<bool, String> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    let mut wand = MagickWand::new();

    match wand.read_image(&filename) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading image: {:?}", e);
            return Err(format!("Error reading image: {:?}", e));
        }
    };

    // Set background color
    let mut pixelwand = PixelWand::new();
    pixelwand.set_color("white").unwrap();
    wand.set_background_color(&pixelwand).unwrap();
    wand.set_format("jpg").unwrap();

    let temp = wand.write_image_blob("jpg");
    if temp.is_err() {
        println!("Error writing image blob");
        return Err(format!("Error writing image blob: {}", temp.err().unwrap()));
    }
    let temp = temp.unwrap();

    // Too many unwraps here, but magickwand is not very good at error handling
    let mut magickwand = MagickWand::new();
    magickwand.read_image_blob(&temp).unwrap();
    magickwand.set_image_gravity(5).unwrap();
    magickwand.set_gravity(5).unwrap();
    magickwand.fit(640, 640);

    let width = magickwand.get_image_width() as isize;
    let x_offset: isize = (640 - width) / 2 * -1;

    let height = magickwand.get_image_height() as isize;
    let y_offset: isize = (640 - height) / 2 * -1;

    magickwand
        .extend_image(640, 640, x_offset, y_offset)
        .unwrap();

    let res_file = "tmp/".to_string() + name + ".jpg";

    magickwand.write_image(&res_file).unwrap();

    Ok(true)
}

pub async fn download_file(file_name: &String) -> Result<(), APIErrors> {
    // No caching, will always download file

    // Delete temporary file
    // first check if file exists
    let file_path = Path::new("tmp/tmpdownload.jpg");
    if file_path.exists() {
        std::fs::remove_file("tmp/tmpdownload.jpg").unwrap();
    }

    // Connect to the local SSH server
    let tcp_stream =
        TcpStream::connect(std::env::var("SFTP_HOST").expect("SFTP_HOST must be set.")).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp_stream);
    match sess.handshake() {
        Ok(_) => println!("SFTP Handshake successful"),
        Err(e) => {
            error!("SFTP Handshake failed: {:?}", e);
            return Err(APIErrors::SFTPError);
        }
    };

    // Error if username or password is not set, but should not panic
    let username = std::env::var("SFTP_USERNAME");
    let password = std::env::var("SFTP_PASSWORD");
    if username.is_err() || password.is_err() {
        println!("SFTP_USERNAME or SFTP_PASSWORD not set");
        return Err(APIErrors::InvalidCredentials);
    }
    let username = username.unwrap();
    let password = password.unwrap();

    match sess.userauth_password(&username, &password) {
        Ok(_) => println!("Authentication successful"),
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return Err(APIErrors::InvalidCredentials);
        }
    };

    let filetarget = "/u02/forms/erp/images/".to_string() + file_name + ".jpg";

    if let Ok((mut remote_file, stat)) = sess.scp_recv(Path::new(&filetarget)) {
        println!("File exists");
        println!("remote file size: {}", stat.size());
        let mut contents = Vec::new();
        match remote_file.read_to_end(&mut contents) {
            Ok(_) => (),
            Err(e) => {
                error!("File read failed: {:?}", e);
                return Err(APIErrors::SFTPError);
            }
        };
        // Close the channel and wait for the whole content to be tranferred
        // Too many unwraps here, should be handled better
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();

        //Save file

        let local_file = std::fs::File::create("tmp/tmpdownload.jpg");
        if local_file.is_err() {
            error!("File create failed: {:?}", local_file.err().unwrap());
            return Err(APIErrors::InternalServerError);
        }
        let mut local_file = local_file.unwrap();
        local_file.write_all(&contents).unwrap();
        Ok(())
    } else {
        println!("File does not exist");
        return Err(APIErrors::FileNotFound);
    }
}

pub async fn upload_file(item_code: &String, filepath: &String) -> Result<(), APIErrors> {
    match resize(&filepath, &item_code) {
        Ok(_) => (),
        Err(e) => {
            error!("Error resizing image: {:?}", e);
            return Err(APIErrors::InternalServerError);
        }
    }
    let temp_file = "tmp/".to_string() + item_code + ".jpg";

    let mut f = File::open(&temp_file).await.unwrap();
    let mut buffer = Vec::new();
    let file_size = f.metadata().await.unwrap().len();
    f.read_to_end(&mut buffer).await.ok();

    let tcp =
        TcpStream::connect(std::env::var("SFTP_HOST").expect("SFTP_HOST must be set.")).unwrap();
    if let Ok(mut sess) = Session::new() {
        println!("Session created");

        sess.set_tcp_stream(tcp);
        match sess.handshake() {
            Ok(_) => println!("SFTP Handshake successful"),
            Err(e) => {
                error!("SFTP Handshake failed: {:?}", e);
                return Err(APIErrors::SFTPError);
            }
        };

        // Error if username or password is not set, but should not panic
        let username = std::env::var("SFTP_USERNAME");
        let password = std::env::var("SFTP_PASSWORD");
        if username.is_err() || password.is_err() {
            println!("SFTP_USERNAME or SFTP_PASSWORD not set");
            return Err(APIErrors::InvalidCredentials);
        }
        let username = username.unwrap();
        let password = password.unwrap();

        match sess.userauth_password(&username, &password) {
            Ok(_) => println!("SFTP Authentication successful"),
            Err(e) => {
                error!("SFTP Authentication failed: {:?}", e);
                return Err(APIErrors::InvalidCredentials);
            }
        };

        // Default static path for image files
        let file_name = "/u02/forms/erp/images/".to_string() + item_code + ".jpg";

        if let Ok(mut remote_file) = sess.scp_send(Path::new(&file_name), 0o644, file_size, None) {
            println!("File Send successful");
            println!("remote file size: {}", file_size);
            remote_file.write_all(&buffer).unwrap();
            // Close the channel and wait for the whole content to be tranferred
            remote_file.send_eof().unwrap();
            remote_file.wait_eof().unwrap();
            remote_file.close().unwrap();
            remote_file.wait_close().unwrap();
            std::fs::remove_file(temp_file).unwrap();
            return Ok(());
        } else {
            println!("File Send Unsuccessful");
            std::fs::remove_file(temp_file).unwrap();
            return Err(APIErrors::SFTPError);
        }
    } else {
        println!("Session not created");
        std::fs::remove_file(temp_file).unwrap();
        return Err(APIErrors::SFTPError);
    }
}
