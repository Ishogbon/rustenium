mod connection;
pub mod process;
pub mod session;
pub mod transport;

mod listeners;

pub use session::Session;
pub use connection::find_free_port;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
