use anyhow::Result;
use async_channel::Receiver;
use async_trait::async_trait;
use aws_sdk_s3::Client;
use tokio::{runtime::Handle, task::JoinHandle};

use log::{info, trace};

use crate::db::DbPool;
use crate::models::current_collection_datas::{
    query_info_by_collection_address_name, CurrentCollectionDataQuery,
};
use crate::models::current_token_datas::CurrentTokenData;
use crate::models::market_collections::{insert_collection, CollectionInsert};
use crate::models::tokens::{
    insert_token, query_token_by_hash_id, TokenInsert,
};

#[async_trait]
pub trait WorkerTrait {
    async fn run(&mut self, runtime_handle: &Handle) -> JoinHandle<Result<()>>;
}

#[derive(Debug)]
pub enum Worker {
    Collction(CurrentCollectionDataQuery),
    NewNftsOrOwnerCanged(CurrentTokenData), /* TODO all type here are
                                             * holder DB origin
                                             * type.like(DB) */
}

impl From<CurrentCollectionDataQuery> for Worker {
    fn from(value: CurrentCollectionDataQuery) -> Self {
        Self::Collction(value)
    }
}

impl From<CurrentTokenData> for Worker {
    fn from(v: CurrentTokenData) -> Self {
        Self::NewNftsOrOwnerCanged(v)
    }
}

#[derive(Clone, Debug)]
pub struct WorkerService {
    rx: Receiver<Worker>,
    db: DbPool,
    indexer_db: DbPool,
    _aws_s3: Client,
}

impl WorkerService {
    pub fn new(
        rx: Receiver<Worker>,
        db: DbPool,
        indexer_db: DbPool,
        client: Client,
    ) -> Self {
        //	let client =
        Self {
            rx,
            db,
            indexer_db,
            _aws_s3: client,
        }
    }
}

#[async_trait]
impl WorkerTrait for WorkerService {
    async fn run(&mut self, runtime_handle: &Handle) -> JoinHandle<Result<()>> {
        let rx = self.rx.clone();
        let mkdb = self.db.clone();
        let indexer_db = self.indexer_db.clone();
        //let s3 = self.aws_s3.clone();

        runtime_handle.spawn(async move {
	    let mut db = mkdb
                    .get()
                .expect("couldn't get market_db connection from pool");
	    let mut apt_db = indexer_db.get().expect("couldn't get indexer_db connection from pool");

            loop {
                tokio::select! {
                    new_worker = rx.recv() => {
			match new_worker {
			    Ok(Worker::Collction(c)) => {
				let id = c.collection_data_id_hash.clone();
				trace!("Got new collection_id {}",id);
				// Insert to
				insert_collection(&mut db,CollectionInsert::from(c)).expect("Fail to insert db");
				trace!("finesh the collection_id {}",id);
			    }
			    Ok(Worker::NewNftsOrOwnerCanged(nft)) => {
				let id = nft.token_data_id_hash.clone();
				trace!("Got new nft id:{}",&id);
				if query_token_by_hash_id(&mut db,&id).is_err() {
				    // New nft
				    let collection = query_info_by_collection_address_name(&mut apt_db,&nft.creator_address,&nft.collection_name).expect("fail to query collection. pls checkt this");
				    let mut token = TokenInsert::from(nft);
				    token.collection_id = collection.collection_data_id_hash;
				    // get resoures type.
				    let metadata_uri = token.metadata_uri.trim();
				    //let name = format!("{}.png",id.clone());
				    if metadata_uri.ends_with(".json") {
					//erc721 metadata_uri
					token.metadata_json = Some(String::from("123"));
					//token.image_uri =
					info!("Got erc721 metadata {}",metadata_uri);
				    } else {
					// let mut file = tokio::fs::File::create(&name).await?;
					// let bytes = reqwest::get(metadata_uri.clone()).await?
					//    .bytes().await?;
					// file.write_all(&bytes).await?;
					// upload to aws
					// upload_object(&s3,"cargosnft",&name,&name).await.expect("fail to update");
					// token.image = Some(name.clone());
				    }
				    // download images
				    // save image to db
				    // Insert to db
				    insert_token(&mut db,token).expect("Fail to insert db");
				}
				trace!("finesh the nft id {}",id);
			    }
			    _=> {
				unreachable!();
			    }
			}
                    },
                }
            }
        })
    }
}
