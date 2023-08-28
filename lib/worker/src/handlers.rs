use crate::crypto::{keccak256, recover, Address};
use crate::db::{
    is_namespace_owner, namespace_create, pub_table_create, pub_table_insert,
    schema_to_table_create_sql, tx_to_table_inserts_sql,
};
use basin_protocol::{publications, tx};
use capnp::{capability::Promise, data, message, private::units::BYTES_PER_WORD};
use capnp_rpc::pry;
use sqlx::postgres::PgPool;

/// RPC service wrapper for publications.
pub struct Publications {
    pub(crate) pool: PgPool,
}

impl publications::Server for Publications {
    /// Receives new namespace requests.
    /// fixme: error handling
    /// fixme: validate owner byte length is address (42)
    fn create(
        &mut self,
        params: publications::CreateParams,
        _: publications::CreateResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = args.get_ns().unwrap().to_string();
        let rel = args.get_rel().unwrap().to_string();
        let schema = args.get_schema().unwrap();
        let owner = args.get_owner().unwrap();
        let owner_addr = Address::from_slice(owner);
        let table_stmt = schema_to_table_create_sql(ns.clone(), rel, schema).unwrap();

        let p = self.pool.clone();
        Promise::from_future(async move {
            namespace_create(&p, ns, owner_addr).await.unwrap();
            pub_table_create(&p, &table_stmt).await.unwrap();
            Ok(())
        })
    }

    /// Receives publication data.
    /// fixme: error handling
    fn push(
        &mut self,
        params: publications::PushParams,
        _: publications::PushResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = args.get_ns().unwrap().to_string();
        let rel = args.get_rel().unwrap().to_string();
        let tx = args.get_tx().unwrap();
        let sig = args.get_sig().unwrap();
        let owner_addr = recover_addr(tx, sig);
        let insert_stmt = tx_to_table_inserts_sql(ns.clone(), rel, tx).unwrap();

        let p = self.pool.clone();
        Promise::from_future(async move {
            let is_owner = is_namespace_owner(&p, ns, owner_addr).await.unwrap();
            if is_owner {
                pub_table_insert(&p, insert_stmt).await.unwrap();
                Ok(())
            } else {
                Err(capnp::Error::failed("Unauthorized".into()))
            }
        })
    }
}

/// Returns the canonical bytes of tx::Reader
fn canonicalize_tx(reader: tx::Reader) -> Vec<u8> {
    let size = reader.total_size().unwrap().word_count + 1;
    let mut message =
        message::Builder::new(message::HeapAllocator::new().first_segment_words(size as u32));
    message.set_root_canonical(reader).unwrap();
    let output_segments = message.get_segments_for_output();
    debug_assert_eq!(1, output_segments.len());
    let output = output_segments[0];
    debug_assert!((output.len() / BYTES_PER_WORD) as u64 <= size);
    output.to_vec()
}

/// Recovers address from tx:Reader
fn recover_addr(tx: tx::Reader, sig: data::Reader) -> Address {
    let payload = canonicalize_tx(tx);
    let hash = keccak256(payload.as_slice());
    recover(hash.as_slice(), &sig[..64], sig[64] as i32).unwrap()
}
