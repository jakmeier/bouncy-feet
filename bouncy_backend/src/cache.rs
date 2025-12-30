use crate::{api_endoints::club::ClubInfo, club::ClubId};
use dashmap::DashMap;

#[derive(Clone, Default)]
pub struct ClubsCache {
    infos: std::sync::Arc<DashMap<i64, ClubInfo>>,
}

impl ClubsCache {
    pub fn invalidate_club(&self, id: ClubId) {
        self.infos.remove(&id.num());
    }

    pub(crate) fn insert_club_info(&self, id: ClubId, info: ClubInfo) {
        self.infos.insert(id.num(), info);
    }

    pub(crate) fn club_info(
        &self,
        id: ClubId,
    ) -> Option<dashmap::mapref::one::Ref<'_, i64, ClubInfo>> {
        self.infos.get(&id.num())
    }
}
