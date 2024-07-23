use std::{collections::HashMap, io::Error, net::SocketAddr, process::exit, time::Duration};

use connections::Token;
use db::DB;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::RwLock, task::JoinHandle, time::timeout};

use lazy_static::lazy_static;
use user::User;

use super::command::Command;

mod user;
mod connections;
mod db;
mod file;


lazy_static!{
    static ref connection_map: RwLock<HashMap<Token, User>> = RwLock::new(HashMap::new());
    static ref sql_pool: RwLock<Option<DB>> = RwLock::new(None);
}

pub async fn run(){
    //Setup database

    //Save database to lazy
    let mut pool_lock = sql_pool.write().await;
    *pool_lock = Some(DB::new("data").await);

    drop(pool_lock);

    //Initializate port for listening
    let address: SocketAddr;
    let listener: TcpListener;


    //Controling the amounts of tries befor giving up
    let mut counter: u8 = 0;
    let iteration_limit:u8 = 5;

    //Try to open tcp
    loop{
        let result: Result<TcpListener, Error> = TcpListener::bind("25.58.24.153:0").await;

        //Match initiation result
        match result {
            Ok(x) => {
                //Save and print address if initiated
                address = x.local_addr().unwrap(); //TODO! Handle error
                listener = x;
                println!(">> Listening at: {address}", );
                break;
            },
            Err(e) => {
                //Handle initiation error
                eprint!(">> Error creating server: {}", e);
                counter += 1;
                if counter > iteration_limit{
                    exit(1)
                } 
                continue;
            }
        }
    }

    //Connection handdler thread
    let listener_handle: JoinHandle<()> = tokio::spawn(async move {
        handle_socket_inputs(listener).await;
    });

    //Manager terminal thread
    let operator_handle: JoinHandle<()> = tokio::spawn(async move {
        handle_operator_terminal().await;
    });


    //Sync threads to make sure everything stops when it should
    tokio::select! {
        _ = listener_handle => {},
        _ = operator_handle => {},
    }


}

async fn handle_operator_terminal(){
    loop{}
}

async fn handle_socket_inputs(listener: TcpListener){
    loop{
        match  listener.accept().await {
            Ok((stream, _)) =>{
                handle_stream(stream).await;
            },
            Err(_) => {

            }
            
        }
    }
}


async fn handle_stream(mut stream: TcpStream){
    let timeout_duration = 5;
    //Tries to listen during the duration of the timeout
    //Closes the connection after timer finishes
    loop {
        //Buffer for reading input
        let mut buf: Vec<u8> = vec![0; 1024];
        //Tokio timeout allows to listen for a certain period of time
        match timeout(Duration::from_secs(timeout_duration), stream.read(&mut buf)).await {
            //Connection has been closed externally
            Ok(Ok(n)) if n == 0 => {
                println!("Connection closed");
                return;
            }
            //Data recieved
            Ok(Ok(n)) => {
                println!("Read {} bytes: {:?}", n, &buf[..n]);

                // Echo the data back to the client
                handle_stream_input(&mut stream, buf).await;
            }
            //Error reading stream
            Ok(Err(e)) => {
                println!("Failed to read from stream: {}", e);
                return;
            }
            //Timed out
            Err(_) => {
                println!("Read timed out");
                stream.shutdown().await.unwrap_or_else(|e| {eprint!("Failed to shutdown connection: {e}")});
                break;
            }
        }
    }
}

async fn handle_stream_input(stream: &mut TcpStream, data: Vec<u8>){
    let cmd_type = data[0];
    match cmd_type {
        0 => handle_check_status(stream, data).await,
        1 => handle_login(stream, data).await,
        2 => handle_signup(stream, data).await,
        3 => handle_logout(stream, data).await,
        4 => handle_get_files_addr(stream, data).await,
        5 => handle_get_file(stream, data).await,
        6 => handle_add_file(stream, data).await,
        7 => handle_update_file(stream, data).await,
        8 => handle_delete_file(stream, data).await,
        9 => handle_get_file_relations(stream, data).await,
        10 => handle_add_relation(stream, data).await,
        11 => handle_delete_relation(stream, data).await,
        _ => println!("Other")
    }
}

async fn handle_check_status(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_check_status");
}

async fn handle_login(stream: &mut TcpStream, data: Vec<u8>) {
    let cmd = Command::new_from_b_array(data);

    // Get username and password
    let user = match cmd.get_arg(1) {
        Some(u) => u,
        None => {
            //TODO! Handle return error
            return;
        }
    };

    let password = match cmd.get_arg(2) {
        Some(p) => p,
        None => {
            //TODO! Handle return error
            return;
        }
    };

    // Access database reference
    let db_lock = sql_pool.read().await;

    // Unwrap option db
    if let Some(db) = db_lock.as_ref() {
        // Verify user credentials
        if db.verify_user_credentials(user, password).await {
            let cmd = Command::new(0);
            stream.write_all(&cmd.get_cmd()).await.unwrap(); //TODO! Handle error
        } else {
            let mut cmd = Command::new(2);
            cmd.add_arg("User does not exist");
            stream.write_all(&cmd.get_cmd()).await.unwrap(); //TODO! Handle error
        }
    } else {
        //TODO! Manage database not existing (Shouldnt happen)
    }

    println!("handle_login");
}

async fn handle_signup(stream: &mut TcpStream, data: Vec<u8>) {
    let cmd = Command::new_from_b_array(data);

    // Get username and password
    let user = match cmd.get_arg(1) {
        Some(u) => u,
        None => {
            //TODO! Handle return error
            return;
        }
    };

    let password = match cmd.get_arg(2) {
        Some(p) => p,
        None => {
            //TODO! Handle return error
            return;
        }
    };

    let db_lock = sql_pool.read().await;

    // Unwrap option db
    if let Some(db) = db_lock.as_ref() {
        // Verify user credentials
        let res = db.add_user(user, password).await;
        match res {
            Ok(_) => {
                let cmd = Command::new(0);
                stream.write_all(&cmd.get_cmd()).await.unwrap()
            },
            Err(e) => {
                let mut cmd = Command::new(2);
                cmd.add_arg(&e.to_string());
                stream.write_all(&cmd.get_cmd()).await.unwrap();
            }
        }
    } else {
        //TODO! Manage database not existing (Shouldnt happen)
    }
    println!("handle_signup");
}

async fn handle_logout(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_logout");
}

async fn handle_get_files_addr(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_get_files_addr");
}

async fn handle_get_file(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_get_file");
}

async fn handle_add_file(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_add_file");
}

async fn handle_update_file(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_update_file");
}

async fn handle_delete_file(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_delete_file");
}

async fn handle_get_file_relations(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_get_file_relations");
}

async fn handle_add_relation(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_add_relation");
}

async fn handle_delete_relation(stream: &mut TcpStream, data: Vec<u8>) {
    // TODO!
    println!("handle_delete_relation");
}

