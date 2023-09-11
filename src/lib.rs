#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::ops::Deref;

use napi::Result;
use lair_keystore_api::{ipc_keystore::ipc_keystore_connect, dependencies::url::Url};
use holochain_types::prelude::ZomeCallUnsigned;
use holochain_zome_types::Signature;
use sodoken::BufRead;

mod types;
use types::*;
pub mod utils;

#[napi]
pub async fn sign_zome_call_with_client(
  zome_call_unsigned_js: ZomeCallUnsignedNapi,
  connection_url: String,
  passphrase: String
) -> Result<ZomeCallNapi> {
  let connection_url_parsed = Url::parse(connection_url.deref()).unwrap();
  let passphrase_bufread: BufRead = passphrase.as_bytes().into();

  let client = ipc_keystore_connect(connection_url_parsed, passphrase_bufread)
    .await
    .unwrap(); 

  // sign the zome call
  let zome_call_unsigned: ZomeCallUnsigned = zome_call_unsigned_js.clone().into();
  let pub_key = zome_call_unsigned.provenance.clone();
  let mut pub_key_2 = [0; 32];
  pub_key_2.copy_from_slice(pub_key.get_raw_32());

  let data_to_sign = zome_call_unsigned.data_to_sign().unwrap();
  
  let sig = client.sign_by_pub_key(
    pub_key_2.into(),
     None,
    data_to_sign)
    .await
    .unwrap();

  let signature = Signature(*sig.0);

  let signed_zome_call = ZomeCallNapi {
    cell_id: zome_call_unsigned_js.cell_id,
    zome_name: zome_call_unsigned.zome_name.to_string(),
    fn_name: zome_call_unsigned.fn_name.0,
    payload: zome_call_unsigned_js.payload,
    cap_secret: zome_call_unsigned_js.cap_secret,
    provenance: zome_call_unsigned_js.provenance,
    nonce: zome_call_unsigned_js.nonce,
    expires_at: zome_call_unsigned_js.expires_at,
    signature: signature.0.to_vec()
  };

  Ok(signed_zome_call)
}