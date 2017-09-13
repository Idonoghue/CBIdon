use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write, ErrorKind};
use super::inbox::Inbox;
use super::type_registry::ShortTypeId;
use super::messaging::{Message, Packet};
use compact::Compact;

pub struct Networking {
    pub machine_id: u8,
    network: Vec<SocketAddr>,
    network_connections: Vec<Option<Connection>>,
}

impl Networking {
    pub fn new(machine_id: u8, network: Vec<SocketAddr>) -> Networking {
        Networking {
            machine_id,
            network_connections: (0..network.len()).into_iter().map(|_| None).collect(),
            network,
        }
    }

    pub fn connect(&mut self) {
        let mut unmapped_streams = Vec::<TcpStream>::new();

        for (machine_id, address) in self.network.iter().enumerate() {
            if machine_id > self.machine_id as usize {
                unmapped_streams.push(TcpStream::connect(address).unwrap());
            }
        }

        let listener = TcpListener::bind(self.network[self.machine_id as usize]).unwrap();

        while unmapped_streams.len() < self.network.len() - 1 {
            let (stream, connected_addr) = listener.accept().unwrap();

            println!("{} connected!", connected_addr);

            unmapped_streams.push(stream)
        }

        println!("All connected");

        for stream in &mut unmapped_streams {
            stream.write_all(&[self.machine_id]).unwrap();
            stream.flush().unwrap();
        }

        for mut stream in unmapped_streams {
            let mut buf = [0];
            stream.read_exact(&mut buf).unwrap();

            let remote_machine_id = buf[0];
            stream.set_nonblocking(true).unwrap();
            self.network_connections[remote_machine_id as usize] = Some(Connection::new(stream))
        }

        println!("All mapped");
    }

    pub fn receive(&mut self, inboxes: &mut [Option<Inbox>]) {
        for maybe_connection in &mut self.network_connections {
            if let Some(ref mut connection) = *maybe_connection {
                connection.try_receive(inboxes)
            }
        }
    }

    pub fn send<M: Message>(&mut self, message_type_id: ShortTypeId, mut packet: Packet<M>) {
        let total_size = ::std::mem::size_of::<ShortTypeId>() + Compact::total_size_bytes(&packet);
        let machine_id = packet.recipient_id.machine;

        let connection = self.network_connections[machine_id as usize]
            .as_mut()
            .expect("expected machine to exist");

        // write total size (message type + packet)
        connection
            .stream
            .write_all(unsafe {
                ::std::slice::from_raw_parts(
                    &total_size as *const usize as *const u8,
                    ::std::mem::size_of::<usize>(),
                )
            })
            .unwrap();

        // write packet type
        connection
            .stream
            .write_all(unsafe {
                ::std::slice::from_raw_parts(
                    &message_type_id as *const ShortTypeId as *const u8,
                    ::std::mem::size_of::<ShortTypeId>(),
                )
            })
            .unwrap();

        // write packet
        // TODO: extra buffer avoidable?
        let mut buf = vec![0; Compact::total_size_bytes(&packet)];

        unsafe {
            Compact::compact_behind(&mut packet, &mut buf[0] as *mut u8 as *mut Packet<M>);
        }

        connection.stream.write_all(buf.as_slice()).unwrap()
    }
}

pub struct Connection {
    stream: TcpStream,
    reading_state: ReadingState,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            reading_state: ReadingState::AwaitingLength,
        }
    }
}

pub enum ReadingState {
    AwaitingLength,
    AwaitingPacketOfLength(usize),
}

impl Connection {
    pub fn try_receive(&mut self, inboxes: &mut [Option<Inbox>]) {
        self.reading_state = match self.reading_state {
            ReadingState::AwaitingLength => {
                let mut length_buf = [0; 8];
                match self.stream.read_exact(&mut length_buf) {
                    Ok(()) => ReadingState::AwaitingPacketOfLength(unsafe { ::std::mem::transmute(length_buf) }),
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => ReadingState::AwaitingLength,
                    Err(e) => panic!("{}", e),
                }
            }
            ReadingState::AwaitingPacketOfLength(length) => {
                let mut buf = vec![0u8; length];
                match self.stream.read_exact(&mut buf) {
                    Ok(()) => {
                        println!("Receiving packet of size {}", length);
                        let type_id = (&buf[0] as *const u8) as *const ShortTypeId;

                        unsafe {
                            if let Some(ref mut inbox) = inboxes[(*type_id).as_usize()] {
                                inbox.put_raw(&buf);
                            } else {
                                panic!("No inbox for {:?} (coming from network)", (*type_id).as_usize())
                            }
                        }

                        ReadingState::AwaitingLength
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        ReadingState::AwaitingPacketOfLength(length)
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
}