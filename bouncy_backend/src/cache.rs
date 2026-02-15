use crate::{
    api_endoints::club::ClubInfo,
    club::ClubId,
    peertube::user::{PeerTubeAccount, PeerTubeHandle},
};
use moka::future::Cache;
use std::time::Duration;

#[cfg(not(test))]
mod conf {
    use super::*;

    pub(super) const ACCOUNT_TTL: Duration = Duration::from_hours(1);
    pub(super) const ACCOUNT_CACHE_SIZE: u64 = 10_000;
    pub(super) const CLUB_CACHE_SIZE: u64 = 1000;
}

#[derive(Clone)]
/// Caching for common requests to PeerTube.
pub struct DataCache {
    clubs: Cache<i64, ClubInfo>,
    user_accounts: Cache<String, PeerTubeAccount>,
}

impl Default for DataCache {
    fn default() -> Self {
        Self {
            clubs: Cache::new(conf::CLUB_CACHE_SIZE),
            user_accounts: Cache::builder()
                .max_capacity(conf::ACCOUNT_CACHE_SIZE)
                .time_to_live(conf::ACCOUNT_TTL)
                .build(),
        }
    }
}

impl DataCache {
    pub async fn invalidate_club(&self, id: ClubId) {
        self.clubs.invalidate(&id.num()).await;
    }

    pub(crate) fn insert_club_info(
        &self,
        id: ClubId,
        info: ClubInfo,
    ) -> tokio::task::JoinHandle<()> {
        let cache = self.clubs.clone();
        tokio::spawn(async move { cache.insert(id.num(), info).await })
    }

    pub(crate) async fn club_info(&self, id: ClubId) -> Option<ClubInfo> {
        self.clubs.get(&id.num()).await
    }

    #[allow(dead_code)]
    pub async fn invalidate_user_account(&self, id: &PeerTubeHandle) {
        self.user_accounts.invalidate(&id.0).await;
    }

    pub(crate) fn insert_user_account(
        &self,
        id: PeerTubeHandle,
        info: PeerTubeAccount,
    ) -> tokio::task::JoinHandle<()> {
        let cache = self.user_accounts.clone();
        tokio::spawn(async move { cache.insert(id.0, info).await })
    }

    pub(crate) async fn user_account(&self, id: &PeerTubeHandle) -> Option<PeerTubeAccount> {
        self.user_accounts.get(&id.0).await
    }
}

#[cfg(test)]
mod conf {
    use super::*;

    pub(super) const ACCOUNT_TTL: Duration = Duration::from_secs(1);
    pub(super) const ACCOUNT_CACHE_SIZE: u64 = 10;
    pub(super) const CLUB_CACHE_SIZE: u64 = 5;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::peertube::user::PeerTubeAccountId;

    #[tokio::test]
    async fn test_expiring_account() {
        let cache = DataCache::default();

        let handle = PeerTubeHandle("test_user".to_owned());
        let account = PeerTubeAccount {
            id: PeerTubeAccountId(0),
            name: handle.clone(),
            display_name: "Test User".to_owned(),
            avatars: vec![],
            description: None,
        };

        cache
            .insert_user_account(handle.clone(), account.clone())
            .await
            .expect("insertion must not fail");

        let cache = std::sync::Arc::new(cache);

        // Read async with a timeout ti check the result is read without deadlock.
        check_read_async(cache.clone(), handle.clone(), Some(account.clone())).await;

        // Now move time forward, invalidating the expiring entry, then check reading again.
        tokio::time::sleep(conf::ACCOUNT_TTL + Duration::from_millis(10)).await;
        check_read_async(cache.clone(), handle.clone(), None).await;
    }

    async fn check_read_async(
        cache: std::sync::Arc<DataCache>,
        handle: PeerTubeHandle,
        expected_account: Option<PeerTubeAccount>,
    ) {
        tokio::time::timeout(Duration::from_millis(200), async move {
            let actual_account: Option<PeerTubeAccount> = cache.user_account(&handle).await;
            assert_eq!(
                format!("{expected_account:?}"),
                format!("{actual_account:?}")
            );
        })
        .await
        .expect("suspecting deadlock in DataCache::user_account()");
    }
}
