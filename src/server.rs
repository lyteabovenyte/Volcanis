use crate::{Command, Connection, Kv, Shutdown};

use tokio::io;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::broadcast;

struct Server {
    // Database state
    kv: Kv,
    // TCP Listener for incoming connections
    listener: TcpListener,
    // Multi-cast channel for notifying connected clients for shutdown
    notify_shutdown: broadcast::Sender<()>,
}

struct Handler {
    // database state
    kv: Kv,
    // the TCP connection decorated with the redis protocol encoder/decoder
    connections: Connection,
    // shutdown notification
    shutdown: Shutdown,
}

pub async fn run() -> io::Result<()> {
    let (notify_shutdown, _) = broadcast::channel(1);
    let mut server = Server {
        notify_shutdown: notify_shutdown,
        kv: Kv::new(),
        listener: TcpListener::bind("127.0.0.1:6969").await?,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                eprintln!("ERROR: Failed to accept connections. err: {err}");
            }
        }
        _ = signal::ctrl_c() => {
            println!("Shutting down");
        }
    }

    Ok(())
}

impl Server {
   async fn run(&mut self) -> io::Result<()> {
    loop {
        let (socket, _) = self.listener.accept().await?;

        let mut handler= Handler{
            kv: self.kv.clone(),
            connections: Connection::new(socket),
            shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
        };

        tokio::spawn(async move {
            if let Err(err) = handler.run().await {
                eprintln!("ERROR: Failed to handle connection. err: {err}");
            }
        });
    }
    }
}

impl Handler {
    pub async fn run(&mut self) -> io::Result<()> {
        while !self.shutdown.is_shutdown {
            let maybe_frame = tokio::select! {
                res = self.connections.read_frame() => res?,
                _ = self.shutdown.recv() => {
                    break;
                }
            };
            let frame = match maybe_frame {
                Some(frame) => frame,
                None => {
                    return Ok(());
                }
            };

            let cmd = Command::from_frame(frame)?;
            cmd.apply(&self.kv, &mut self.connections, &mut self.shutdown).await?;
        }
        Ok(())
    }
}