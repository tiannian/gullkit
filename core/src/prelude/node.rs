use std::error::Error;

use async_trait::async_trait;

use crate::UniversalMessage;

#[async_trait]
pub trait Node {
    type Error: Error;

    type ReturnedUniversalMessage: UniversalMessage;

    async fn process(
        self,
        message: impl UniversalMessage,
    ) -> Result<Self::ReturnedUniversalMessage, Self::Error>;
}
