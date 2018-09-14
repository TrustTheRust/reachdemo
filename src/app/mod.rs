use network_agent;
use yew::prelude::*;


#[derive(Debug)]
pub enum Message {
    NetworkAgentResponse(network_agent::Response),
}


pub struct Model {
    _network_agent: Box<dyn Bridge<network_agent::NetworkAgent>>,
    properties: Properties,
}


impl Component for Model {
    type Message = Message;
    type Properties = Properties;


    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.properties = properties;

        true
    }


    fn create(properties: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|response| Message::NetworkAgentResponse(response));

        let network_agent = network_agent::NetworkAgent
                                         ::bridge(callback);

        let app_model = Model {
            _network_agent: network_agent,
            properties: properties,
        };

        app_model
    }


    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::NetworkAgentResponse(response) => {
                match response {
                    network_agent::Response::ConnectionStatusChanged(connection_status) => {
                        self.properties.network_agent_connection_status = connection_status;

                        self.change(self.properties);
                    },
                }
            },
        }

        true
    }
}


impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{ network_agent::string_for_connection_status(self.properties.network_agent_connection_status) }</p>
        }
    }
}


#[derive(Copy, Debug, Eq, PartialEq)]
pub struct Properties {
    pub network_agent_connection_status: network_agent::ConnectionStatus,
}


impl Clone for Properties {
    fn clone(&self) -> Self {
        Properties {
            network_agent_connection_status: self.network_agent_connection_status.clone(),
        }
    }
}


impl Default for Properties {
    fn default() -> Self {
        Properties {
            network_agent_connection_status: network_agent::ConnectionStatus::Disconnected,
        }
    }
}
