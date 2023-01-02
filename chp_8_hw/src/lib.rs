pub mod error;
pub mod types;

#[cfg(test)]
mod test {
    use super::*;
    use tracing::info;
    use types::LeftChannel;

    #[test]
    fn before_zeroize_test() {
        let chan_left = LeftChannel::new_channel_left();
        assert_eq!(chan_left.get_left(), 0);
    }
}
