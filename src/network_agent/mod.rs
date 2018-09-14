use failure::Error;
use yew::format::{Json};
use yew::prelude::worker::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
}


#[derive(Debug)]
pub enum Message {
    WebsocketResponse(Result<String, Error>),
    WebsocketStatusChanged(WebSocketStatus),
}


pub struct NetworkAgent {
    connection: Option<WebSocketTask>,
    connection_status: ConnectionStatus,
    handler_id: Option<HandlerId>,
    link: AgentLink<NetworkAgent>,
    service: WebSocketService,
}


impl NetworkAgent {
    fn transition_to_connection_status(&mut self, connection_status: ConnectionStatus) {
        if connection_status == ConnectionStatus::Disconnected {
            self.connection = None;
        }

        self.connection_status = connection_status;

        if let Some(handler_id) = self.handler_id {
            self.link.response(handler_id, Response::ConnectionStatusChanged(connection_status.clone()));
        }
    }
}


impl Agent for NetworkAgent {
    type Reach = Context; // Public doesn't initiate a WebSocket connection and Global causes a panic.
    type Message = Message;
    type Input = Request;
    type Output = Response;


    fn connected(&mut self, id: HandlerId) {
        self.handler_id = Some(id);
    }


    fn create(link: AgentLink<Self>) -> Self {
        let mut network_agent = NetworkAgent {
            connection: None,
            connection_status: ConnectionStatus::Disconnected,
            handler_id: None,
            link: link,
            service: WebSocketService::new(),
        };

        let websocket_message = network_agent.link.send_back(|Json(data)| Message::WebsocketResponse(data));

        let websocket_notification = network_agent.link.send_back(|websocket_status| {
            Message::WebsocketStatusChanged(websocket_status)
        });

        let connection = network_agent.service.connect("wss://example.com", websocket_message, websocket_notification);

        network_agent.connection = Some(connection);

        network_agent
    }


    fn handle(&mut self, _message: Self::Input, _id: HandlerId) {
    }


    fn update(&mut self, message: Self::Message) {
        match message {
            Message::WebsocketStatusChanged(websocket_status) => {
                let connection_status;

                match websocket_status {
                    WebSocketStatus::Closed => {
                        connection_status = ConnectionStatus::Disconnected;
                    },
                    WebSocketStatus::Error => {
                        connection_status = ConnectionStatus::Disconnected;
                    },
                    WebSocketStatus::Opened => {
                        connection_status = ConnectionStatus::Connected;
                    },
                }

                self.transition_to_connection_status(connection_status);
            },
            Message::WebsocketResponse(_response) => {
            },
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub enum Notification {
    ConnectionStatus,
}


#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
}


impl Transferable for Request {}


#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    ConnectionStatusChanged(ConnectionStatus),
}


impl Transferable for Response {}


pub fn string_for_connection_status(connection_status: ConnectionStatus) -> String {
    match connection_status {
        ConnectionStatus::Connected => {
            String::from("connected")
        },
        ConnectionStatus::Connecting => {
            String::from("connecting")
        },
        ConnectionStatus::Disconnected => {
            String::from("disconnected")
        },
    }
}
