# kwi-kraken-rest
This crate contains a collection of structs to deserialize json responses from the kraken trading platform. It also contains convenient functions to send HTTP requests. HTTP requests use the reqwest crate. Deserialisation is handled by the serde crate. The libraries are organised in a similar way as the official API documentation at https://docs.kraken.com/api/.

## Disclaimer
This library is an unofficial implementation of the Kraken trading platform REST API. It is not affiliated with, endorsed by, or maintained by Kraken in any way.

Use this library at your own risk. The author assumes no responsibility for any financial losses, incorrect data, or other issues that may arise from using this software. Always double-check your API requests and responses before executing trades.

This library is as of yet incomplete. Some functions have never been tested and some don't work at all. Please test the and take a look at the source code.

## Features
- Public endpoints
- Private endpoints

## Usage
Add the crate to your Cargo.toml: `cargo add kwi-kraken-rest`

### Public endpoints
Example getting a public endpoint data.
```rust
    use kwi_kraken_rest::market_data::Ohlc;
    let ohlc_data = Ohlc::get("SOLEUR", Some(1), None).await.unwrap();
    println!("ohlc data: {:#?}", ohlc);
```

### Private endpoints
For personal account related data you will need API keys. So get your api keys with the required permissions. Then set them as environment variables

e.g Linux
```bash
    export API_KEY="sjdfoisdnconsdiocnsodjcosjdcjsdj"
    export API_SEC="Xihdfoeinfcicwpdoqwhdicoidjsajbdsaknlkwpdjw" 
```

```rust
    use kwi_kraken_rest::account_data::AccountBalance;
    let balance = AccountBalance::get().unwrap();
    println!("Account balance: {:#?}", balance);
```

### Using examples
The dotenvy crate is used to conveniently load environment variables during testing. Create a .env file in your root directory with your keys.

```
API_KEY="sjdfoisdnconsdiocnsodjcosjdcjsdj"
API_SEC="Xihdfoeinfcicwpdoqwhdicoidjsajbdsaknlkwpdjw"
```
