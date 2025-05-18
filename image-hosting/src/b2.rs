use crate::CFG;
use lazy_static::lazy_static;
use s3::{creds::Credentials, Bucket};
use std::sync::Arc;

lazy_static! {
    pub static ref B2: Arc<Box<Bucket>> = {
        let cred = Credentials {
            access_key: Some(CFG.b2_key_id.clone()),
            secret_key: Some(CFG.b2_application_key.clone()),
            security_token: None,
            session_token: None,
            expiration: None,
        };
        Arc::new(
            Bucket::new(
                &CFG.b2_bucket_name,
                CFG.b2_endpoint.as_str().parse().unwrap(),
                cred,
            )
            .unwrap(),
        )
    };
}
