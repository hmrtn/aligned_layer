use risc0_zkvm::{compute_image_id, Receipt};

const ELF: &[u8] = include_bytes!("../elf/fibonacci");
pub const MAX_PROOF_SIZE: usize = 1024 * 1024;

#[no_mangle]
pub extern "C" fn verify_risc0_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
) -> bool {
    // if let Ok(proof) = bincode::deserialize(&proof_bytes[..proof_len]) {
    //     return verify(compute_image_id(ELF), &proof).is_ok();
    // }

    // false
    let receipt: Receipt = bincode::deserialize(&proof_bytes[..proof_len]).unwrap();

    println!("{:?}", receipt);
    // let id = compute_image_id(ELF);

    // let result = receipt.verify(id);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8; 215514] =
        include_bytes!("../../../../tests/testing_data/risc0_fibonacci.proof");

    #[test]
    fn verify_risc0_proof_works() {
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);
        let result = verify_risc0_proof_ffi(&proof_buffer, proof_size);
        assert!(result)
    }
}
