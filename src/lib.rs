pub mod buffer;
pub mod ciphers;

pub use self::ciphers::Cipher;
pub use self::buffer::CharStream;

pub fn test() {
    let mut data = CharStream::from("Hello world!");

    let affine = ciphers::Affine::with_params(3, 5);
    affine.encrypt(&mut data).unwrap();
    affine.decrypt(&mut data).unwrap();

    println!("{}", data);
}
