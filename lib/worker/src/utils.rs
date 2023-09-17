use basin_protocol::tx;
use capnp::{data, message, private::units::BYTES_PER_WORD, Result};
use ethers::{types::Address, utils::keccak256};

/// Recovers address from tx:Reader
pub fn recover_addr(tx: tx::Reader, sig: data::Reader) -> Result<Address> {
    if sig.len() != 65 {
        return Err(capnp::Error::failed("signature must be 65 bytes".into()));
    }
    let payload = canonicalize_tx(tx)?;
    let hash = keccak256(payload.as_slice());
    let addr = crate::crypto::recover(hash.as_slice(), &sig[..64], sig[64] as i32)?;
    Ok(addr)
}

/// Returns the canonical bytes of tx::Reader
pub fn canonicalize_tx(reader: tx::Reader) -> Result<Vec<u8>> {
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
            "canonical tx size must be less than {size}"
        )));
    }
    Ok(output.to_vec())
}
