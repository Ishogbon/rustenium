mod browser;
mod session;
mod browsing_context;
mod script;
mod emulation;
mod input;
mod log;
mod network;
mod storage;
mod web_extension;

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
