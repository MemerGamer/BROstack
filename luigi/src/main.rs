use std::env;
use std::io::{BufWriter, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mario = &args[1];
    let giuseppe = &args[2];
    let luigi = "It's a me, Luigi!";
    let image = "https://raw.githubusercontent.com/MemerGamer/BROstack/main/img/BROstack.jpg";

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!(
        "Click here to open the BRO stack: \u{001B}[38;5;51;4mhttp://localhost:8080/\u{001B}[0m"
    );
    for stream in listener.incoming() {
        let mut stream = stream?;
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><head><style>*{{ box-sizing: border-box; }} h1{{margin: 0; position: relative; top: 50%; left: 50%; transform: translate(-50%, -50%);}} body,html {{ color:white; height: 99%; background-color:black; }}</style></head><div style='background-image: url(\"{}\"); height:100%; background-repeat: no-repeat; background-size:cover; filter: blur(8px); -webkit-filter: blur(8px); '></div> <div style=\"background-color: rgb(0,0,0); /* Fallback color */background-color: rgba(0,0,0, 0.4); /* Black w/opacity/see-through */ color: white; font-weight: bold; border: 3px solid #000; position: absolute; top: 50%; left: 50%;transform: translate(-50%, -50%); z-index: 2;width: 80%;padding: 20px; padding-top: 50px; margin-bottom: 10px; text-align: center;\"><h1 style='text-align:center'>{}</h1><h1 style='text-align:center'>{}</h1><h1 style='text-align:center'>{}</h1></div></body></html>", image , mario, luigi, giuseppe);
        let mut writer = BufWriter::new(&mut stream);
        writer.write_all(response.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}
