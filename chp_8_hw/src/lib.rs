pub mod error;
pub mod types;

#[cfg(test)]
mod test {
    use super::*;

    use types::LeftChannel;

    #[test]
    fn before_zeroize_test() {
        let mut chan_left = LeftChannel::new_channel_left();
        chan_left.get_left();
        assert_eq!(chan_left.get_left(), 0);
        chan_left.increment_left();
        chan_left.get_left();
    }
}

mod scary {
    macro_rules! scary {
        ($var:ident) => {
            if cfg!(feature = "mal") {
                "Scary identity!".to_string()
            } else {
                $var
            }
        };
    }
}
