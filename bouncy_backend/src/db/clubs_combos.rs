use crate::{
    club::ClubId, combo::ComboId, db_err_to_status, AppState, CheckedClubId, CheckedComboId,
};
use axum::http::StatusCode;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ClubCombo {
    pub club_id: ClubId,
    pub combo_id: ComboId,
    pub is_private: bool,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub(crate) struct ClubComboRow {
    pub(super) club_id: i64,
    #[allow(dead_code)]
    pub(super) combo_id: i64,
    is_private: bool,
}

impl ClubCombo {
    pub async fn assign_to_club(
        state: &AppState,
        checked_club_id: CheckedClubId,
        checked_combo_id: CheckedComboId,
        is_private: bool,
    ) -> Result<ClubCombo, (StatusCode, &'static str)> {
        // must be admin of a club
        let club_id = checked_club_id.assert_write_access()?;
        // can be a combo by someone else, as long as it is readable
        let combo_id = checked_combo_id.assert_public_read_access()?;

        let row = sqlx::query_as!(
            ClubComboRow,
            r#"
            INSERT INTO clubs_combos (club_id, combo_id, is_private)
            VALUES ($1, $2, $3)
            RETURNING
                club_id,
                combo_id,
                is_private
            "#,
            club_id.num(),
            combo_id.num(),
            is_private
        )
        .fetch_one(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        let club_combo = ClubCombo::from(row);
        Ok(club_combo)
    }

    // TODO
    #[allow(dead_code)]
    pub async fn remove_from_club(
        state: &AppState,
        checked_club_id: CheckedClubId,
        checked_combo_id: CheckedComboId,
    ) -> Result<bool, (StatusCode, &'static str)> {
        let club_id = checked_club_id.assert_write_access()?;
        let combo_id = checked_combo_id.assert_public_read_access()?;

        let res = sqlx::query!(
            r#"
            DELETE FROM clubs_combos
            WHERE club_id = $1 AND combo_id = $2
            "#,
            club_id.num(),
            combo_id.num()
        )
        .execute(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        Ok(res.rows_affected() > 0)
    }

    // TODO
    #[allow(dead_code)]
    pub async fn update_privacy(
        state: &AppState,
        checked_club_id: CheckedClubId,
        checked_combo_id: CheckedComboId,
        is_private: bool,
    ) -> Result<bool, (StatusCode, &'static str)> {
        let club_id = checked_club_id.assert_write_access()?;
        let combo_id = checked_combo_id.assert_public_read_access()?;

        let res = sqlx::query!(
            r#"
            UPDATE clubs_combos
            SET is_private = $3
            WHERE club_id = $1 AND combo_id = $2
            "#,
            club_id.num(),
            combo_id.num(),
            is_private
        )
        .execute(&state.pg_db_pool)
        .await
        .map_err(db_err_to_status)?;

        Ok(res.rows_affected() > 0)
    }
}

impl From<ClubComboRow> for ClubCombo {
    fn from(other: ClubComboRow) -> ClubCombo {
        ClubCombo {
            club_id: other.club_id(),
            combo_id: other.combo_id(),
            is_private: other.is_private,
        }
    }
}
