mod base62;

use rand::*;

pub struct TokenValue([u8; LEN]);

const LEN: usize = 20 * 2;
const BASE62_LEN: usize = 27 * 2;

impl TokenValue {
    pub fn generate() -> Self {
        let mut rng = OsRng::new().expect("Error opening random number generator");
        let mut value: TokenValue = TokenValue([0; LEN]);
        rng.fill_bytes(&mut value.0);
        value
    }

    pub fn to_base62(&self) -> String {
        let mut scratch = self.0;
        let mut out = vec![0; BASE62_LEN];
        base62::encode_raw(scratch.as_mut(), out.as_mut());

        // This is valid because base 62 encoded data contains only ASCII alphanumeric characters.
        unsafe { String::from_utf8_unchecked(out) }
    }
}
