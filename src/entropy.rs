use getrandom::{register_custom_getrandom, Error};

use crate::time::datetime::utc_timestamp;
use crate::time::instant::Instant;

fn mre_custom_entropy(dest: &mut [u8]) -> Result<(), Error> {
    let mut i = 0;
    while i < dest.len() {
        let ticks = Instant::now().ticks as u32;
        let utc = utc_timestamp().unwrap_or(0) as u32;
        
        let entropy = ticks.wrapping_mul(utc ^ 0xDEADBEEF);
        
        let bytes = entropy.to_le_bytes();
        let take = core::cmp::min(dest.len() - i, 4);
        
        dest[i..i + take].copy_from_slice(&bytes[..take]);
        i += take;
    }

    Ok(())
}

register_custom_getrandom!(mre_custom_entropy);