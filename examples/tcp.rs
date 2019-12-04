use async_std::io;
use async_std::net;
use async_std::prelude::*;
use solitario::task;

fn main() -> Result<(), Exception> {
    task::block_on(async {
        let listener = net::TcpListener::bind(("127.0.0.1", 8080)).await?;
        let addr = format!("http://{}", listener.local_addr()?);
        println!("listening on {}", addr);
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            task::spawn_on_worker(async move {
                let stream = stream.unwrap();
                let (reader, writer) = &mut (&stream, &stream);
                io::copy(reader, writer).await.unwrap();
            });
        }
        Ok(())
    })
}

type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;
