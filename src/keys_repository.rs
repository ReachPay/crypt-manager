use std::{collections::HashMap};

use tokio::sync::RwLock;

pub struct KeysRepository{
    pub keys: RwLock<HashMap<String, Vec<u8>>>,
}

impl KeysRepository{

    pub fn new() ->Self{
        Self{
            keys: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_merchant_private_key(&self, merchant_id: &String) -> Option<Vec<u8>>{
        let lock = self.keys.read().await;

        match lock.get(merchant_id) {
            Some(key) => Some(key.clone()),
            None => None
        }
    }

    pub async fn update_key(&self, merchant_id: &String, key_pem: Vec<u8>   ) -> Option<Vec<u8>>{
        let mut lock = self.keys.write().await;
        return lock.insert(merchant_id.clone(), key_pem);
    }
}