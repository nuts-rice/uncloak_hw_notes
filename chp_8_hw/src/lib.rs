pub mod error;
pub mod types;

#[cfg(test)]
mod test {
    use super::*;
    use crate::scary::*;
    use types::LeftChannel;

    #[test]
    fn increment_passing_test() {
        let mut chan_left = LeftChannel::new_channel_left();
        chan_left.get_left();
        assert_eq!(chan_left.get_left(), 0);
        chan_left.increment_left();
        chan_left.get_left();
    }
    #[test]
    fn increment_failing_test() {
        let chan_left = LeftChannel::new_channel_left();
        chan_left.get_left();
        assert_eq!(
            chan_left.increment_right(),
            error::ChannelError::ChannelOpError
        );
    }

    #[test]
    fn scary_test() {
        let executed = possible_malice();
        if cfg!(feature = "mal") {
            assert_eq!(executed, Err(error::ChannelError::ChannelScaryError));
        } else {
            assert_eq!(executed, Ok(0));
        }
    }
}
/*
    #[test]
    fn zeroize_test() {
        let mut chan_left = Zeroizing::LeftChannel::new_channel_left();
        chan_left.increment_left();
    }
}
*/

//this is a sort of distinguisher (see Session 3 notes) for evaluating a secret
//if we evaluate on a specific length for SECRET, and it takes longer (probably by using cargo
//bench) to evaluate, then
//SECRET must be longer!

pub mod secret {
    use super::*;
    pub const SECRET: u64 = 123456789012345678;

    pub fn branch_secret(guess: u64) {
        let mut left_chan = types::LeftChannel::new_channel_left();
        let mut right_chan = types::RightChannel::new_channel_right();
        for _i in 0..=999 {
            if guess == SECRET {
                left_chan.increment_left();
            } else {
                right_chan.increment_right();
            }
        }
    }
}

mod scary {
    use super::*;

    use crate::types::LeftChannel;

    macro_rules! scary {
        ($var:ident) => {
            if cfg!(feature = "mal") {
                "Scary identity!".to_string();
                return Err(error::ChannelError::ChannelScaryError);
            } else {
                Ok($var)
            }
        };
    }

    pub fn possible_malice() -> Result<u64, error::ChannelError> {
        let chan_left = LeftChannel::new_channel_left();
        let executed = chan_left.get_left();
        scary!(executed)
    }
}
