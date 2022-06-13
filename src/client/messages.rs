use crate::{Client, Message};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender, Receiver};

impl Client {

    pub fn messages(&self) -> Result<Receiver<(Message, SocketAddr)>, Box<dyn Error>> {

        let (sender, receiver): (Sender<(Message, SocketAddr)>, Receiver<(Message, SocketAddr)>) = channel();

        self.incoming(sender);

        self.listen();

        Ok(receiver)

    }
}
