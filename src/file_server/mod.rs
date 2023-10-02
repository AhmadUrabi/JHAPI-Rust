use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use rocket::tokio::io::AsyncReadExt;
use std::io::Read;
use tokio::fs::File;
use ssh2::Session;

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



pub async fn upload_file(item_code: &String) -> bool {

    let mut f = File::open("tmp/test.jpg").await.unwrap();
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
        return true
    } else {
        println!("File Send Unsuccessful");
        return false;
    }
    } else {
        println!("Session not created");
        return false;
    }


}