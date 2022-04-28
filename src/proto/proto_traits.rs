use crate::proto::command_id::CommandID;
use std::io::{self, Read, Write};

pub trait ReadFrom: Sized {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self>;
}

pub trait WriteTo {
    fn write_to<W: Write>(&self, wtr: W) -> io::Result<()>;
}

pub trait Command {
    fn command_id(&self) -> CommandID;
}

pub trait SendRecieve {
    // TODO add router as param that implements read to write
    fn send_receive(&self) -> io::Result<()>;
}
