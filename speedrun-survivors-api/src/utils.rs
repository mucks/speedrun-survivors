use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::SystemTime;

struct Base64;
impl Distribution<char> for Base64 {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .choose(rng)
            .unwrap() as char
    }
}

pub fn weak_random_base64_string(len: usize) -> String {
    rand::thread_rng().sample_iter(&Base64).take(len).collect()
}

pub fn unixtime() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}
