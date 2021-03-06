use crate::handler::Handler;
use crate::eosnode::EosNodeInfo;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
use std::str::FromStr;
use std::fs;
use serde::{ Deserialize, Serialize };
use rmps::Serializer;
use uuid::Uuid;

/**
 * The following struct are used for MultiParty Threshold
 */
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct TupleKey {
    pub first: String,
    pub second: String,
    pub third: String,
    pub fourth: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PartySignup {
    pub number: u32,
    pub uuid: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Index {
    pub key: TupleKey,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub key: TupleKey,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub parties: String,
    pub threshold: String,
}

/**
 * MPT /set. Send a key on the DHT
 */
#[post("/set", format = "json", data = "<request>")]
fn set(
    handler: State<Arc<Mutex<Handler>>>,
    request: Json<Entry>,
) -> Json<Result<(), ()>> {
    let entry: Entry = request.0;
    let mut handler = handler.lock().unwrap();
    let mut buf = Vec::new();
    entry.key.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // The following value is used to get unique hashes for nodes
    // Because if all nodes are using the same hashes, we can have
    // multiple MPT transactions
    let salt: Vec<u8> = handler.info().id.as_bytes().to_vec();
    buf.extend(salt);
    handler.insert(buf, entry.value);
    Json(Ok(()))
}

#[post("/get", format = "json", data = "<request>")]
fn get(
    handler: State<Arc<Mutex<Handler>>>,
    request: Json<Index>,
) -> Json<Result<Entry, ()>> {
    let index: Index = request.0;
    let mut handler = handler.lock().unwrap();
    let mut buf = Vec::new();
    index.key.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // Same here
    let salt: Vec<u8> = handler.info().id.as_bytes().to_vec();
    buf.extend(salt);
    match handler.get(buf) {
        Some(v) => {
            let entry = Entry {
                key: index.key,
                value: format!("{}", v.clone()),
            };
            Json(Ok(entry))
        }
        None => Json(Err(())),
    }
}

#[post("/signupkeygen", format = "json")]
fn signup_keygen(
    handler: State<Arc<Mutex<Handler>>>
) -> Json<Result<PartySignup, ()>> {
    let data = fs::read_to_string("config/server.conf")
        .expect("Unable to read params, make sure config file is present in the same folder ");
    let params: Params = serde_json::from_str(&data).unwrap();
    let parties: u32 = params.parties.parse::<u32>().unwrap();
    // The initial hash will be hash(TupleKey + salt)
    let key = TupleKey {
        first: "signup".to_string(),
        second: "keygen".to_string(),
        third: "".to_string(),
        fourth: "".to_string(),
    };
    let party_signup: PartySignup;
    let mut handler = handler.lock().unwrap();
    let mut buf = Vec::new();
    key.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // same here
    let salt: Vec<u8> = handler.info().id.as_bytes().to_vec();
    buf.extend(salt);
    {
        let value = handler.get(buf.clone()).unwrap();
        let party_i_minus1_signup: PartySignup = serde_json::from_str(&value).unwrap();
        if party_i_minus1_signup.number < parties {
            let party_num = party_i_minus1_signup.number + 1;
            party_signup = PartySignup {
                number: party_num.clone(),
                uuid: party_i_minus1_signup.uuid,
            };
        } else {
            let uuid = Uuid::new_v4().to_string();
            let party1 = 1;
            party_signup = PartySignup {
                number: party1,
                uuid,
            };
        }
    }
    handler.insert(buf, serde_json::to_string(&party_signup).unwrap());
    return Json(Ok(party_signup));
}

#[post("/signupsign", format = "json")]
fn signup_sign(handler: State<Arc<Mutex<Handler>>>) -> Json<Result<PartySignup, ()>> {
    //read parameters:
    let data = fs::read_to_string("config/server.conf")
        .expect("Unable to read params, make sure config file is present in the same folder ");
    let params: Params = serde_json::from_str(&data).unwrap();
    let threshold: u32 = params.threshold.parse::<u32>().unwrap();
    let key = TupleKey {
        first: "signup".to_string(),
        second: "sign".to_string(),
        third: "".to_string(),
        fourth: "".to_string(),
    };
    let party_signup: PartySignup;
    let mut handler = handler.lock().unwrap();
    let mut buf = Vec::new();
    key.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // same here
    let salt: Vec<u8> = handler.info().id.as_bytes().to_vec();
    buf.extend(salt);
    {
        let value = handler.get(buf.clone()).unwrap();
        let party_i_minus1_signup: PartySignup = serde_json::from_str(&value).unwrap();
        if party_i_minus1_signup.number < threshold + 1 {
            let party_num = party_i_minus1_signup.number + 1;
            party_signup = PartySignup {
                number: party_num.clone(),
                uuid: party_i_minus1_signup.uuid,
            };
        } else {
            let uuid = Uuid::new_v4().to_string();
            let party1 = 1;
            party_signup = PartySignup {
                number: party1,
                uuid,
            };
        }
    }
    handler.insert(buf, serde_json::to_string(&party_signup).unwrap());
    return Json(Ok(party_signup));
}

/**
 * Return currently detected nodes on the DHT
 */
#[get("/getConnectedNodes")]
fn connected_nodes(handler: State<Arc<Mutex<Handler>>>) -> Json<HashMap<String, String>> {
    let mut handler = handler.lock().unwrap();
    let mut connected_nodes = handler.get_connected_nodes();
    connected_nodes.insert("Result".to_string(), "Success".to_string());
    Json(connected_nodes)
}

/**
 * Return nodes location on the DHT
 */
#[get("/getNodesLocation")]
fn nodes_location(handler: State<Arc<Mutex<Handler>>>) -> Json<HashMap<String, Vec<String>>> {
    let mut handler = handler.lock().unwrap();
    let nodes_location = handler.get_nodes_location();
    Json(nodes_location)
}

/**
 * Return node's infos
 */
#[get("/")]
fn node_infos(handler: State<Arc<Mutex<Handler>>>) -> Json<EosNodeInfo> {
    let mut handler = handler.lock().unwrap();
    let info = handler.info();
    Json(info)
}

pub struct Server;

impl Server {
    pub fn run(handler: Arc<Mutex<Handler>>) {
        // CORS!
        let allowed_origins = rocket_cors::AllowedOrigins::all();
        let allowed_methods: rocket_cors::AllowedMethods = ["Get", "Post", "Delete", "Head", "Options", "Put", "Patch"]
            .iter().map(|s| FromStr::from_str(s).unwrap()).collect();

        let cors = rocket_cors::CorsOptions {
            allowed_origins,
            allowed_methods,
            allowed_headers: rocket_cors::AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors().ok().expect("Incorrect CORS specified");

        // Announce uuid for MPT.
        // TODO: move?
        let keygen_key = TupleKey {
            first: "signup".to_string(),
            second: "keygen".to_string(),
            third: "".to_string(),
            fourth: "".to_string(),
        };
        let sign_key = TupleKey {
            first: "signup".to_string(),
            second: "sign".to_string(),
            third: "".to_string(),
            fourth: "".to_string(),
        };
        let uuid_keygen = Uuid::new_v4().to_string();
        let uuid_sign = Uuid::new_v4().to_string();

        let party1 = 0;
        let party_signup_keygen = PartySignup {
            number: party1.clone(),
            uuid: uuid_keygen,
        };
        let party_signup_sign = PartySignup {
            number: party1.clone(),
            uuid: uuid_sign,
        };
        {
            let mut handler = handler.lock().unwrap();
            let mut buf = Vec::new();
            keygen_key.serialize(&mut Serializer::new(&mut buf)).unwrap();
            let salt: Vec<u8> = handler.info().id.as_bytes().to_vec();
            buf.extend(salt.clone());
            handler.insert(
                buf,
                serde_json::to_string(&party_signup_keygen).unwrap(),
            );
            let mut buf = Vec::new();
            sign_key.serialize(&mut Serializer::new(&mut buf)).unwrap();
            buf.extend(salt);
            handler.insert(buf, serde_json::to_string(&party_signup_sign).unwrap());
        }

        // Launch server
        rocket::ignite()
            .manage(handler)
            .mount("/", routes![nodes_location, connected_nodes, get, set, signup_keygen, signup_sign, node_infos])
            .attach(cors)
            .launch();
    }
}