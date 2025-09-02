use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use url::Url;

pub fn in_memory_cookie_session_layer(url: &Url) -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();
    let domain = url.domain().expect("invalid url").to_string();
    let is_dev = domain.contains("localhost");

    let session = SessionManagerLayer::new(session_store)
        // js access is not needed
        .with_http_only(true)
        .with_domain(domain)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
        .with_always_save(true);

    if is_dev {
        session
            .with_secure(false) // dev?
            // For dev, using a mix of, localnet with two ports and and
            // auth.bouncy-feet.ch, somehow `Strict` it doesn't work
            .with_same_site(SameSite::Lax)
    } else {
        session
            // absolutely don't leak the session id, authentication relies on it!
            .with_secure(true) // dev?
            // Same-site should work within the same registerable domain, e.g. bouncy-feet.ch
            // https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-cookie-same-site-00#section-2
            .with_same_site(SameSite::Strict)
    }
}
