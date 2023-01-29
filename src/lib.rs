use async_net::{
    AsyncToSocketAddrs, TcpListener, TcpStream};
    use async_tungstenite::tungstenite::Message;
    use async_tungstenite::WebSocketStream;
    use bevy::prelude::Component;
    use bevy::prelude::{App, Commands, Plugin, Res, ResMut};
    use bevy::tasks::{IoTaskPool, Task};
    use crossbeam_channel::{Receiver, Sender};
    use futures::{select, FutureExt, SinkExt, StreamExt};
    
    pub struct WsPlugin;

    impl Plugin for WsPlugin {
        fn build(&self, app: &mut App) {
            let task_pool = IoTaskPool(app.world.resource::<IoTaskPool>().0.clone());
            let (ws_tx, es_rx) = crossbeam_channel::unbounded();
            app.insert_resource(WsListener::new(task_pool, ws_tx));
            app.insert_resource(WsAcceptQueue { ws_rs })
                .add_system(accept_ws_from_queue);
        }

    }

    pub struct WsListener {
        task_pool: IoTaskPool,
        ws_tx: Sender<WebSocketStream<TcpStream>>   
    }

    pub struct WsAcceptQueue {
        wx_rx : Receiver<WebSocketStream<TcpStream>>
    }

    impl WsListener {
        pub fn new(task_pool: IoTaskPool, ws_tx: Sender<WebSocketStream<TcpStream>>) -> Self {
            Self { task_pool, ws_tx }
        }
        
        pub fn listen(&self, bind_to: impl AsyncToSocketAddrs) {
            let listener = futures::executor::block_on(TcpListener::bind(bind_to))
                .expect("Connot bind to the address");
        
            let task_pool = self.task_pool.clone();
            let ws_tx = self.ws_tx_clone();
            let task = self.task_pool.spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            log::debug!("new connection from {}", addr);
                            let ws_tx = ws_tx.clone();
                            let accept = async move {
                                match async_tungstenite::accept_async(stream).await {
                                    Ok(websocket) => {
                                        let _ = ws_tx.send(websocket);

                                    }
                                    Err(e) => {
                                        log::error!("error handshaking a new websocket ; {}", e);
                                    }
                                }
                            };
                            task_pool.spawn(accept).detach();
                        }
                        Err(e) => {
                            log::error!("Error accepting a new connection : {}", e);
                        }
                    }
                }
            });
        
            task.detach();

            }
    }

#[derive(Component)]
pub struct WsConnection {
    _io : Task<()>,
    sender: async_channel::Sender<Message>,
    receiver: async_channel::Receiver<Message>
}


pub use async_channel::TryRecvError as ReceiveError;

impl WsConnection {
    pub fn send(&self, message: Message) -> bool {
        self.sender.try_send(message).is_ok()
    }


    pub fn receiver(&self) -> Result<Message, ReceiveError> {
        self.receiver.try_recv()
    }
}


pub fn accept_ws_from_queue(
    mut commands: Commands,
    pool : Res<IoTaskPool>,
    queue: ResMut<WsAcceptQueue>
) {
    for mut websocket in queue.ws_rx.try_iter()
     {
        let (message_tx, io_message_tx) = async_channel::unbounded::<Message>();
        let (io_message_tx, message_rx) = async_channel::unbounded::<Message>();

        let io = pool.spawn(async move {
            loop {
                let mut from_channel = io_message_rx.recv().fuse();
                let mut from_ws = websocket.next().fuse();
                select! {
                    message = from_channel => if let Ok(message) = message {
                        let _ = websocket.send(message).await;
                    } else {
                        break;
                    }, 
                    message = from_ws => if let Some(Ok(message)) = message {
                        let _ = io_message_tx_send(message).await;
                    } else {
                        break;
                    },
                    complete => break,
                }
            }
        });

        commands.spawn().insert(WsConnection {
            _io : io,
            sender: message_tx,
            receiver: message_rx,
        })
     }
}
