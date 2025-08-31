use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use url::Url;

pub fn in_memory_cookie_session_layer(url: &Url) -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();
    let domain = url.domain().expect("invalid url").to_string();
    SessionManagerLayer::new(session_store)
        // absolutely don't leak the session id, authentication relies on it!
        .with_secure(true)
        // .with_secure(false )// dev?
        // js access is not needed
        .with_http_only(true)
        // Same-site works within the same registerable domain, e.g. bouncy-feet.ch
        // https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-cookie-same-site-00#section-2
        .with_same_site(SameSite::Strict)
        .with_domain(domain)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
        .with_always_save(true)
}
