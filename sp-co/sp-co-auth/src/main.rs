use std::collections::HashMap;
use base64::encode;
use serde_json::{json, Value, to_vec};
use sp_auth::create_auth_token;
use streaming_platform::{client, MagicBall, sp_dto::{MsgMeta, Message, Response, resp}};

#[derive(Debug)]
enum SideKick {
    IncorrectKeyInRequest,
    //SerdeJson(serde_json::Error)
}

impl std::fmt::Display for SideKick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl std::error::Error for SideKick {}

pub async fn process_event(config: HashMap<String, String>, mut mb: MagicBall, msg: Message<Value>, _: ()) -> Result<(), Box<dyn std::error::Error>>  {
    //info!("{:#?}", msg);
    
    Ok(())
}

pub async fn process_rpc(config: HashMap<String, String>, mut mb: MagicBall, msg: Message<Value>, _: ()) -> Result<Response<Value>, Box<dyn std::error::Error>> {   
    //info!("{:#?}", msg);

    let res = match msg.meta.key.action.as_ref() {
        "Auth" => {
            let auth_token_key = b"This is key omg";

            let cookie_payload = json!({

            });
            
            let cookie_hash = create_auth_token(auth_token_key, &cookie_payload)?;

            let part1 = encode(&cookie_hash);
            let part2 = encode(&to_vec(&cookie_payload)?);

            json!({
                "auth_token": part1 + "." + &part2
            })
        }
        _ => return Err(Box::new(SideKick::IncorrectKeyInRequest))
    };

    resp(res)
}

pub async fn startup(config: HashMap<String, String>, mut mb: MagicBall, startup_data: Option<Value>, _: ()) {
}

pub fn main() {
    env_logger::init();

    let mut config = HashMap::new();

    config.insert("addr".to_owned(), "Auth".to_owned());
    config.insert("host".to_owned(), "127.0.0.1:11001".to_owned());
    config.insert("access_key".to_owned(), "".to_owned());
 
    client::start(config, process_event, process_rpc, startup, None, ());
 }