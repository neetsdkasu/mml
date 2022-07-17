// mml-core
// author: Leonardone @ NEETSDKASU

mod mml;
mod tone_control;

pub use mml::MMLError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
