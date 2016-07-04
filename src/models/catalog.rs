#[macro_use(bson, doc)]
extern crate bson;

use std::str;
use std::string::ToString;
use std::str::FromStr;

use ::{json, bson, mongodb, BTreeMap, env};
use mongodb::coll::options::FindOptions;
use bson::Document;
use bson::oid::ObjectId;

// pub struct Cid(String);

#[deriving(debug,Clone,Eq)]
pub struct Catalog {
    id    : String,
    name  : String,
    descp : String,
    hookid: Option<String>,
    uid   : String,
    gid   : String,
    ctime : f64,
    utime : f64
}

impl json::ToJson for Catalog {
    fn to_json(&self) -> json::Json {
        let mut d: BTreeMap<String, json::Json> = BTreeMap::new();
        d.insert("id".to_string(),       self.id.to_json());
        d.insert("name".to_string(),     self.name.to_json());
        d.insert("descp".to_string(),    self.descp.to_json());
        d.insert("hookid".to_string(),   self.hookid.to_json());
        d.insert("descp".to_string(),    self.descp.to_json());
        d.insert("uid".to_string(),      self.uid.to_json());
        d.insert("gid".to_string(),      self.gid.to_json());
        d.insert("ctime".to_string(),    self.ctime.to_json());
        d.insert("utime".to_string(),    self.utime.to_json());
        Json::Object(d)
    }
}
impl ToString for Catalog {
    fn to_string(&self) -> String {
        self.to_json().to_string()
    }
}
impl FromStr for Catalog {
    type Err = ();
    fn from_str(s: &str) -> Result<Catalog, ()> {
        let j = match json::Json::from_str(s) {
            Ok(j)  => j,
            Err(_) => return Err(())
        };
        Catalog::from_json(j)
    }
}
impl Catalog {
    pub fn from_json(j: json::Json) -> Result<Catalog, ()> {
        if !j.is_object() {
            return Err(());
        }
        let obj = j.as_object().unwrap();
        let id  = match obj.get("id") {
            Some(ref id) => match id {
                json::Json::String(ref id) => id,
                _                          => return Err(())
            },
            None         => return Err(())
        };
        let name  = match obj.get("name") {
            Some(ref name) => match name {
                json::Json::String(ref name) => name,
                _                             => return Err(())
            },
            None         => return Err(())
        };
        let descp  = match obj.get("descp") {
            Some(ref descp) => match descp {
                json::Json::String(ref descp) => descp,
                _                             => return Err(())
            },
            None         => return Err(())
        };
        let hookid  = match obj.get("hookid") {
            Some(ref hookid) => match hookid {
                json::Json::String(ref hookid) => Some(hookid),
                json::Json::Null               => None,
                _                              => return Err(())
            },
            None         => return Err(())
        };

        let uid  = match obj.get("uid") {
            Some(ref uid) => match uid {
                json::Json::String(ref uid) => uid,
                _                           => return Err(())
            },
            None         => return Err(())
        };
        let gid  = match obj.get("gid") {
            Some(ref gid) => match gid {
                json::Json::String(ref gid) => gid,
                _                           => return Err(())
            },
            None         => return Err(())
        };
        let ctime  = match obj.get("ctime") {
            Some(ref ctime) => match ctime {
                json::Json::String(ref ctime) => ctime,
                _                             => return Err(())
            },
            None         => return Err(())
        };
        let utime  = match obj.get("utime") {
            Some(ref utime) => match utime {
                json::Json::String(ref utime) => utime,
                _                             => return Err(())
            },
            None         => return Err(())
        };

        Ok(Catalog {
            id    : id,
            name  : name,
            descp : descp,
            hookid: hookid,
            uid   : uid,
            gid   : gid,
            ctime : ctime,
            utime : utime
        })
    }
    pub fn to_bson(&self) -> Document {
        doc!{ _id    => self.id,
              name   => self.name,
              descp  => self.descp,
              hookid => self.hookid,
              uid    => self.uid,
              gid    => self.gid,
              ctime  => self.ctime,
              utime  => self.utime
        }
    }
}


impl Catalog {
    pub fn new() -> Catalog {
        let id = str::from_utf8(&ObjectId::new().to_bytes()).is_ok().unwrap().to_string();

    }
    pub fn keys(&self) -> Vec<&str> {
        vec!["id", "name", "descp", "hookid", "uid", "gid", "ctime", "utime"]
    }
    pub fn parent(&self) -> Option<Catalog> {
        match self.hookid {
            Some(hookid) => {
                let oid = match ObjectId::with_string(&hookid) {
                    Ok(oid) => oid,
                    Err(_)  => return None
                };
                let document = doc!{_id => oid};
                Catalog::find(document, None)
            },
            None         => None
        }
    }
    pub fn children(&self) -> Option<Vec<Catalog>> {
        let oid = match ObjectId::with_string(&self.id) {
            Ok(oid) => oid,
            Err(_)  => return None
        };
        let document = doc!{_id => oid};
        Catalog::find(document, None)
    }
    pub fn find(filter: Option<Document>, options: Option<FindOptions>) -> Option<Vec<Catalog>> {
        let db_uri: String = env::var("mongo_db_uri").unwrap();
        let client = Client::with_uri(&db_uri).ok().expect("Failed to initialize client.");
        let coll   = client.db("openansible").collection("catalogs");
        let cursor = coll.find(filter, options).unwrap();
        
    }
    pub fn save(&self) -> bool {
        // Sync catalog data to mongodb.
        true
    }
}

