use trytes::*;
use tmath::*;
use curl::*;
use alloc::Vec;

pub fn mask<C>(payload: &[Trit], keys: &[Vec<Trit>]) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut out: Vec<Trit> = Vec::with_capacity(payload.len());
    let mut curl = C::default();
    for key in keys {
        curl.absorb(&key);
    }
    let key_chunk = curl.rate();
    for chunk in payload.chunks(HASH_LENGTH) {
        let mut c: Vec<Trit> = chunk
            .iter()
            .cloned()
            .zip(key_chunk.iter().cloned())
            .map(bct_sum)
            .collect();
        out.append(&mut c);
    }
    out
}

pub fn unmask<C>(payload: &[Trit], keys: &[Vec<Trit>]) -> Vec<Trit>
where
    C: Curl<Trit>,
{
    let mut out: Vec<Trit> = Vec::with_capacity(payload.len());
    let mut curl = C::default();
    for key in keys {
        curl.absorb(&key);
    }

    let key_chunk: Vec<Trit> = curl.rate().iter().map(|t| -t).collect();
    for chunk in payload.chunks(HASH_LENGTH) {
        let mut c: Vec<Trit> = chunk
            .iter()
            .cloned()
            .zip(key_chunk.iter().cloned())
            .map(bct_sum)
            .collect();
        out.append(&mut c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use curl_cpu::*;
    use alloc::Vec;
    use alloc::*;
    #[test]
    fn it_can_unmask() {
        let payload: Vec<Trit> = "AMESSAGEFORYOU9"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let auth_id: Vec<Trit> = "MYMERKLEROOTHASH"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let index: Vec<Trit> = "AEOWJID999999"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();
        let keys: Vec<Vec<Trit>> = vec![auth_id, index];
        let cipher = mask::<CpuCurl<Trit>>(&payload, &keys);
        let plain: Vec<Trit> = unmask::<CpuCurl<Trit>>(&cipher.clone(), &keys);
        assert_eq!(trits_to_string(&plain), trits_to_string(&payload));
    }
}
