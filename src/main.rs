use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use postgres::{Client, NoTls, Error};

const DB_URL: Option<&'static str> = option_env!("DATABASE_URL");

const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main()
{
    if let Err(error) = setup_database()
    {
        println!("Error: {}", error);

        return;
    }

    // start the server
    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();

    println!("Server started at port 8080.");

    // handle the client
    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) => { handle_client(stream); }

            Err(error) => { println!("Error: {}", error); }
        }
    }
}

fn handle_client(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];
    let mut request: String = String::new();

    match stream.read(&mut buffer)
    {
        Ok(size) =>
        {
            request.push_str(String::from_utf8_lossy(&buffer[0..size]).as_ref());

            let (status_line, content) = match &*request
            {
                // TODO: POST, GET, PUT, DELETE

                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            // take the HTTP response and send it over the connection
            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }

        Err(error) => { println!("Error: {}", error); }
    }
}

fn setup_database() -> Result<(), Error>
{
    // connect to the database
    Client::connect(DB_URL.unwrap_or(""), NoTls)?;

    Ok(())
}