use crate::crypto::{keccak256, recover, Address};
use crate::db::{
    is_namespace_owner, namespace_create, pub_table_create, pub_table_insert,
    schema_to_table_create_sql, tx_to_table_inserts_sql,
};
use basin_protocol::{publications, tx};
use capnp::{capability::Promise, data, message, private::units::BYTES_PER_WORD};
use capnp_rpc::pry;
use log::debug;
use sqlx::postgres::PgPool;

/// RPC service wrapper for publications.
pub struct Publications {
    pub(crate) pool: PgPool,
}

impl publications::Server for Publications {
    /// Receives new namespace requests.
    /// fixme: validate owner byte length is address (42)
    fn create(
        &mut self,
        params: publications::CreateParams,
        _: publications::CreateResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = pry!(args.get_ns()).to_string();
        let rel = pry!(args.get_rel()).to_string();
        let schema = pry!(args.get_schema());
        let owner = pry!(args.get_owner());
        let owner_addr = Address::from_slice(owner);
        let table_stmt = pry!(schema_to_table_create_sql(ns.clone(), rel.clone(), schema));

        debug!(
            "publication create {}.{} for {}: {}",
            ns.clone(),
            rel,
            owner_addr.to_string(),
            table_stmt
        );

        let p = self.pool.clone();
        Promise::from_future(async move {
            namespace_create(&p, ns, owner_addr).await?;
            pub_table_create(&p, &table_stmt).await?;
            Ok(())
        })
    }

    /// Receives publication data.
    fn push(
        &mut self,
        params: publications::PushParams,
        _: publications::PushResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = pry!(args.get_ns()).to_string();
        let rel = pry!(args.get_rel()).to_string();
        let tx = pry!(args.get_tx());
        let sig = pry!(args.get_sig());
        let owner_addr = pry!(recover_addr(tx, sig));
        let insert_stmt = pry!(tx_to_table_inserts_sql(ns.clone(), rel.clone(), tx));

        debug!(
            "publication push {}.{} for {}: {:?}",
            ns.clone(),
            rel,
            owner_addr.to_string(),
            insert_stmt
        );

        let p = self.pool.clone();
        Promise::from_future(async move {
            let is_owner = is_namespace_owner(&p, ns, owner_addr).await?;
            if is_owner {
                pub_table_insert(&p, insert_stmt).await?;
                Ok(())
            } else {
                Err(capnp::Error::failed("Unauthorized".into()))
            }
        })
    }
}

/// Recovers address from tx:Reader
fn recover_addr(tx: tx::Reader, sig: data::Reader) -> capnp::Result<Address> {
    let payload = canonicalize_tx(tx)?;
    let hash = keccak256(payload.as_slice());
    let addr = recover(hash.as_slice(), &sig[..64], sig[64] as i32)?;
    Ok(addr)
}

/// Returns the canonical bytes of tx::Reader
fn canonicalize_tx(reader: tx::Reader) -> capnp::Result<Vec<u8>> {
    let size = reader.total_size()?.word_count + 1;
    let mut message =
        message::Builder::new(message::HeapAllocator::new().first_segment_words(size as u32));
    message.set_root_canonical(reader)?;
    let output_segments = message.get_segments_for_output();
    if output_segments.len() != 1 {
        return Err(capnp::Error::failed(format!(
            "canonical tx has {} segments; expected 1",
            output_segments.len()
        )));
    }
    let output = output_segments[0];
    if (output.len() / BYTES_PER_WORD) as u64 > size {
        return Err(capnp::Error::failed(format!(
            "canonical tx size must be less than {}",
            size
        )));
    }
    Ok(output.to_vec())
}
