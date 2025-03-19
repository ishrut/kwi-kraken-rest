use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub expires: u32,
}

impl Token {
    /// Returns a Token struct with the token and its expiration time.
    ///
    /// It is to be used for private websockets calls.
    ///
    /// Note: It needs private keys set as env variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use kraken_rest::trading::Token;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = Token::get().await;
    ///     println!("token: {:?}", token);
    /// }
    /// ```
    pub async fn get() -> Result<Token, Error> {
        let response =
            crate::request::get_private_json::<Token>(WEBSOCKETS_TOKEN_URI, None).await?;
        Ok(response.result)
    }
}
