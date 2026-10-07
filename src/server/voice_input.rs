use base::config::keys;
use hbb_common::config::Config;

pub(super) fn should_auto_accept(
    authorized: bool,
    remote_control: bool,
    audio_allowed: bool,
    peer_id: &str,
) -> bool {
    if !authorized || !remote_control || !audio_allowed || peer_id.is_empty() {
        return false;
    }
    allows_peer(
        &Config::get_option(keys::OPTION_AUTO_ACCEPT_VOICE_FROM_IDS),
        peer_id,
    )
}

fn allows_peer(allowed_ids: &str, peer_id: &str) -> bool {
    allowed_ids
        .split(',')
        .map(str::trim)
        .any(|allowed| !allowed.is_empty() && allowed == peer_id)
}

#[cfg(test)]
mod tests {
    use super::allows_peer;

    #[test]
    fn voice_auto_accept_requires_exact_id() {
        assert!(allows_peer("1001, 2002", "2002"));
        assert!(!allows_peer("", "2002"));
        assert!(!allows_peer("*", "2002"));
        assert!(!allows_peer("2002", "20"));
    }
}
