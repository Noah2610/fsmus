use std::fmt;
use std::io::Write;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestMessage {
    Play,
    Pause,
    Next,
    Seek(u64),
}

pub fn request(message: RequestMessage) -> Result<(), String> {
    let config = crate::config::Config::load()?;

    let response = ron::ser::to_string(&message)
        .map_err(|e| format!("serialize error: {}", e))?;
    let mut stream = TcpStream::connect((config.host, config.port))
        .map_err(|e| format!("connection error: {}", e))?;
    stream
        .write(response.as_bytes())
        .map_err(|e| format!("write error: {}", e))?;
    stream.flush().map_err(|_| String::from("flush error"))?;

    Ok(())
}

impl fmt::Display for RequestMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
