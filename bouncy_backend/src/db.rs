pub(crate) mod beat;
pub(crate) mod client_session;
pub(crate) mod club;
pub(crate) mod clubs_combos;
pub(crate) mod combo;
pub(crate) mod dance_activity;
pub(crate) mod playlist;
pub(crate) mod timestamp;
pub(crate) mod user;

#[cfg(test)]
pub(crate) mod test_helpers;

use axum::http::StatusCode;
use axum::response::IntoResponse;

#[track_caller]
pub fn db_err_to_response(err: sqlx::Error) -> axum::response::Response {
    db_err_to_status(err).into_response()
}

#[track_caller]
pub fn db_err_to_status(err: sqlx::Error) -> (axum::http::StatusCode, &'static str) {
    let location = std::panic::Location::caller();
    tracing::error!(
        ?err,
        file = location.file(),
        line = location.line(),
        "DB error"
    );
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB ERROR")
}

/// Crate a wrapper around IDs for validated access checks. ("don't validate, parse")
///
/// All DB APIs should only accept wrapper IDs and perform the check internally.
macro_rules! checked_id {
    (
        $checked:ident,
        $id:ty,
        $not_found_msg:expr
    ) => {
        #[derive(Debug, Clone, Copy)]
        /// Wrapper around ID after it has gone through access checks.
        pub enum $checked {
            #[allow(dead_code)]
            /// Full read + write access granted to the current user.
            Owned($id),
            #[allow(dead_code)]
            /// Entity exists and the current user can view private and public details.
            FullReadAccess($id),
            /// Entity exists and the current user can view public info.
            PublicReadAccess($id),
            #[allow(dead_code)]
            /// May also mean it exists but is private.
            NotFound,
        }

        impl $checked {
            #[allow(dead_code)]
            pub fn assert_public_read_access(self) -> Result<$id, (StatusCode, &'static str)> {
                match self {
                    Self::Owned(id) => Ok(id),
                    Self::FullReadAccess(id) => Ok(id),
                    Self::PublicReadAccess(id) => Ok(id),
                    Self::NotFound => Err((StatusCode::NOT_FOUND, $not_found_msg)),
                }
            }
            #[allow(dead_code)]
            pub fn assert_private_read_access(self) -> Result<$id, (StatusCode, &'static str)> {
                match self {
                    Self::Owned(id) | Self::FullReadAccess(id) => Ok(id),
                    Self::PublicReadAccess(_id) => Err((StatusCode::FORBIDDEN, "no read access")),
                    Self::NotFound => Err((StatusCode::NOT_FOUND, $not_found_msg)),
                }
            }

            #[allow(dead_code)]
            pub fn assert_write_access(self) -> Result<$id, (StatusCode, &'static str)> {
                match self {
                    Self::Owned(id) => Ok(id),
                    Self::PublicReadAccess(_) | Self::FullReadAccess(_) => {
                        Err((StatusCode::FORBIDDEN, "no write access"))
                    }
                    Self::NotFound => Err((StatusCode::NOT_FOUND, $not_found_msg)),
                }
            }
        }
    };
}

checked_id!(CheckedUserId, crate::db::user::UserId, "user not found");

checked_id!(CheckedClubId, crate::db::club::ClubId, "club not found");

checked_id!(
    CheckedComboId,
    crate::db::combo::ComboId,
    "combo not found for user"
);

checked_id!(
    CheckedBeatId,
    crate::db::beat::BeatId,
    "beat not found for user"
);

checked_id!(
    CheckedTimestampId,
    crate::db::timestamp::TimestampId,
    "timestamp not found for user"
);
