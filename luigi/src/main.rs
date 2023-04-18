use std::env;
use std::io::{BufWriter, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let message1 = &args[1];
    let message2 = &args[2];

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!(
        "Click here to open the BRO stack: \u{001B}[38;5;51;4mhttp://localhost:8080/\u{001B}[0m"
    );
    for stream in listener.incoming() {
        let mut stream = stream?;
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><head><style>body {{color:white; background-image: url('https://pbs.twimg.com/media/Ensf3-JXYAMNHpx?format=jpg&name=4096x4096'); background-repeat: no-repeat; background-position: center center; }}</style></head><div><h1 style='text-align:center'>{}</h1><h1 style='text-align:center'>It's a me, Luigi!</h1><h1 style='text-align:center'>{}</h1></div></body></html>", message1, message2);

        let mut writer = BufWriter::new(&mut stream);
        writer.write_all(response.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}
