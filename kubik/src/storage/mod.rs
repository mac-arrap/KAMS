use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use ffi_support::FfiStr;
use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;
use serde::{Serialize, Deserialize};

use lox::keyring::get_os_keyring;



struct ErrorCode {
    code : i32,
    message : *const c_char
}

#[derive(Serialize, Deserialize, Debug)]
struct StorageConfig {
    name : String,
    storage_type : String,
    credentials : Vec<String>

}

fn open(config : FfiStr) ->  ErrorCode {

    let config_string = config.into_string();
    let storage_config : StorageConfig = match serde_json::from_str(config_string.as_str()) {
        Ok(v) => v,
        Err(e) =>  return ErrorCode{ code : 1 , message: r#"random error"#.as_ptr() as *const _ },
    };


    if storage_config.storage_type == "postgres" {

        let database_url = &storage_config.credentials[0];

        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
    }



    //    else if storage_config.storage_type == "couchdb " {
    //
    //    }


    ErrorCode{ code : 1 , message: r#"random error"#.as_ptr() as *const _ }

}







#[cfg(test)]
mod open_tests {

    use crate::storage::StorageConfig;
    use crate::storage::open;
    use ffi_support::FfiStr;
    use std::ffi::CStr;

    #[test]
    fn serialize_storage_config() {

        let storage_config = StorageConfig{ name : "randomstorage".to_string(), storage_type: "postgres".to_string(), credentials: vec!["randomcreds".to_string()]};

        let serialized = serde_json::to_string(&storage_config).unwrap();
        let data = r#"{"name":"randomstorage","storage_type":"postgres","credentials":["randomcreds"]}"#;

        assert_eq!(serialized, data.to_string())
    }


    #[test]
    fn open_database_succes(){


        let config =  r#"{"name":"randomstorage","storage_type":"postgres","credentials":["postgres://postgres:default123@localhost"]}"#.as_ptr();
        unsafe {

            let config_cstr = CStr::from_ptr(config as *const _);
            let config_ffi_str =  FfiStr::from_cstr(config_cstr);
            open(config_ffi_str);
        }

    }

}