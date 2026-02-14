use std::time::{Duration, Instant};

use crate::{
    api_endoints::club::ClubInfo,
    club::ClubId,
    peertube::user::{PeerTubeAccount, PeerTubeHandle},
};
use dashmap::DashMap;

const ACCOUNT_TTL: Duration = Duration::from_hours(1);

#[derive(Clone, Default)]
/// Caching for common requests to PeerTube.
pub struct DataCache {
    clubs: std::sync::Arc<DashMap<i64, ClubInfo>>,
    user_accounts: std::sync::Arc<DashMap<String, ExpiringEntry<PeerTubeAccount>>>,
}

pub(crate) struct ExpiringEntry<T> {
    value: T,
    inserted_at: Instant,
}

impl DataCache {
    pub fn invalidate_club(&self, id: ClubId) {
        self.clubs.remove(&id.num());
    }

    pub(crate) fn insert_club_info(&self, id: ClubId, info: ClubInfo) {
        self.clubs.insert(id.num(), info);
    }

    pub(crate) fn club_info(
        &self,
        id: ClubId,
    ) -> Option<dashmap::mapref::one::Ref<'_, i64, ClubInfo>> {
        self.clubs.get(&id.num())
    }

    pub fn invalidate_user_account(&self, id: &PeerTubeHandle) {
        self.user_accounts.remove(&id.0);
    }

    pub(crate) fn insert_user_account(&self, id: PeerTubeHandle, info: PeerTubeAccount) {
        self.user_accounts.insert(id.0, ExpiringEntry::new(info));
    }

    pub(crate) fn user_account(
        &self,
        id: &PeerTubeHandle,
    ) -> Option<
        dashmap::mapref::one::MappedRef<
            '_,
            String,
            ExpiringEntry<PeerTubeAccount>,
            PeerTubeAccount,
        >,
    > {
        let entry = self.user_accounts.get(&id.0)?;

        if entry.inserted_at.elapsed() > ACCOUNT_TTL {
            self.invalidate_user_account(id);
            return None;
        }

        Some(entry.map(|e| &e.value))
    }
}

impl<T> ExpiringEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            inserted_at: Instant::now(),
        }
    }
}
