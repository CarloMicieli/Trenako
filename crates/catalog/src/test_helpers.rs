use fake::{Fake, StringFaker};

pub fn random_str(len: usize) -> String {
    const ASCII: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let f = StringFaker::with(Vec::from(ASCII), len);
    f.fake()
}
