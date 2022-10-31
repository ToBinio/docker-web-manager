use actix::Message;
use crate::container::types::NewContainer;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Uuid, std::io::Error>")]
pub struct Add {
    pub container: NewContainer,
}

#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct GetALl {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    pub(crate) uuid: Uuid,
}

