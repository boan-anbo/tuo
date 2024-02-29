use async_openai::types::CreateChatCompletionRequest;
use tuo_core::core::messaging::message::Message;
use tuo_core::error::TuoError;

pub trait ConvertMessageTo<T> {
    fn convert_to(message: &Message, model_name: &str) ->  Result<T,TuoError>;
}
// This trait defines how an instance of any type T can be converted into a Message.
pub trait ConverttoMessage<T> {
    fn convert_to_message(item: &T) ->  Result<Message,TuoError>;
}

// Extension trait for Message to use the conversion
pub trait MessageExt {
    // Define a generic method to_model_request
    fn to_model_request<T>(&self, model_name: &str) ->  Result<T,TuoError>
        where
            Self: ConvertMessageTo<T>; 
}

impl MessageExt for Message {
    fn to_model_request<T>(&self, model_name: &str) -> Result<T,TuoError>
        where
            Self: ConvertMessageTo<T>,
    {
        Self::convert_to(self, model_name)
    }
}

// Extension trait for any type T to use the conversion into a Message.
pub trait IntoMessageExt {
    // Define a generic method into_message
    fn to_message(&self) ->  Result<Message,TuoError>
        where
            Self: Sized,
            Message: ConverttoMessage<Self>; // Ensure Message implements conversion from Self
}

impl<T> IntoMessageExt for T
    where
        Message: ConverttoMessage<T>,
{
    fn to_message(&self) ->  Result<Message,TuoError> {
        Message::convert_to_message(self)
    }
}

