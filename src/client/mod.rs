use crate::console::{ask_for_input, show_info, show_menu};
use super::command::Command;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream, sync::{mpsc, oneshot::{self, Sender}}, task::JoinHandle};

pub async fn run(){
    //Get server address
    let ip: String = ask_for_input(">> Insert the server ip: ").trim_end().to_string();
    let port: String = ask_for_input(">> Inset the server port: ").trim_end().to_string();

    //Print result
    let address: String = format!("{ip}:{port}");
    show_info("Server Address", &address);

    //Initiate channel
    let (cmd_tx, cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<Command>)>(10000);

    //Msg handdling thread
    let msg_handler: JoinHandle<()> = tokio::spawn(async move{
        handle_tcp_stream(address, cmd_rx).await;
    });

    //Console handdling thread
    let console_handler: JoinHandle<()> = tokio::spawn(async move{
        handle_console(cmd_tx).await;
    });

    tokio::select! {
        _ = msg_handler => {}
        _ = console_handler => {}
    }
}

async fn handle_tcp_stream(address: String, mut cmd_rx: mpsc::Receiver<(Command, Sender<Command>)>){

    loop{
        if let Some((cmd, tx)) = cmd_rx.recv().await {
            let mut stream = TcpStream::connect(&address).await.unwrap();
            let mut buf: Vec<u8> = vec![0; 1024];

            stream.writable().await.unwrap_or_else(|e| {eprintln!("{e}")});
            stream.write_all(&cmd.get_cmd()).await.unwrap();
            loop {

                //If not readeable wait to send info and inititate new connection
                match stream.readable().await{
                    Ok(_) => {},
                    Err(_) => break,
                }

                // Try to read data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                match stream.try_read(&mut buf) {
                    Ok(n) => {
                        buf.truncate(n);
                        let cmd = Command::new_from_b_array(buf);
                        tx.send(cmd).unwrap();
                        break;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        eprintln!("{e}")
                    }
                }
            }
            
        }
    }
}

async fn handle_console(cmd_tx: mpsc::Sender<(Command, Sender<Command>)>){
    loop{
        let option = show_menu(vec!["Login", "Sign Up", "Exit"]);
        match option {
            1 => {
                let mut cmd = Command::new(1);
                cmd.add_arg(&ask_for_input("Insert your username: "));
                cmd.add_arg(&ask_for_input("Insert your password: "));
                let (tx, rx) = oneshot::channel::<Command>();
                cmd_tx.send((cmd, tx)).await.unwrap();

                let response = rx.await.unwrap();
                println!("{:?}", response);
            },
            2 => {
                let mut cmd = Command::new(2);
                cmd.add_arg(&ask_for_input("Insert your username: "));
                cmd.add_arg(&ask_for_input("Insert your password: "));
                let (tx, rx) = oneshot::channel::<Command>();
                cmd_tx.send((cmd, tx)).await.unwrap();

                let response = rx.await.unwrap();
                println!("{:?}", response);
            },
            3 => break,
            _ => continue,
        }
    }
    
}