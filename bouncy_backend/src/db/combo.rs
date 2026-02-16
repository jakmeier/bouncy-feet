use crate::{
    api_endoints::combo::ComboInfo,
    user::{User, UserId},
    AppState,
};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ComboId(i64);

#[derive(Clone, Debug)]
pub struct Combo {
    pub id: ComboId,
    #[allow(unused)]
    pub user_id: UserId,
    pub is_private: bool,
    pub sort_order: Option<i32>,
    pub free_form_category: Option<String>,
    pub title: Option<String>,
    pub video_short_uuid: Option<String>,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub(crate) struct ComboRow {
    id: i64,
    pub user_id: i64,
    is_private: bool,
    sort_order: Option<i32>,
    free_form_category: Option<String>,
    title: Option<String>,
    video_short_uuid: Option<String>,
}

impl ComboId {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Combo {
    pub async fn create(
        state: &AppState,
        user_id: UserId,
        is_private: bool,
        sort_order: Option<i32>,
        free_form_category: Option<&str>,
        title: Option<&str>,
        video_short_uuid: Option<&str>,
    ) -> Result<Combo, sqlx::Error> {
        let row = sqlx::query_as!(
            ComboRow,
            r#"
            INSERT INTO combos (
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            "#,
            user_id.num(),
            is_private,
            sort_order,
            free_form_category,
            title,
            video_short_uuid
        )
        .fetch_one(&state.pg_db_pool)
        .await?;

        Ok(Combo::from(row))
    }

    #[allow(unused)]
    pub async fn lookup(state: &AppState, id: ComboId) -> Result<Option<Combo>, sqlx::Error> {
        let row = sqlx::query_as!(
            ComboRow,
            r#"
            SELECT
                id,
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            FROM combos
            WHERE id = $1
            "#,
            id.num()
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        Ok(row.map(Combo::from))
    }

    pub async fn is_owned_by(
        state: &AppState,
        id: ComboId,
        user_id: UserId,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM combos
                WHERE id = $1 AND user_id = $2
            )
            "#,
            id.num(),
            user_id.num()
        )
        .fetch_one(&state.pg_db_pool)
        .await
        .map(|maybe| maybe.unwrap_or(false))
    }

    pub async fn list_by_user(
        state: &AppState,
        user_id: UserId,
    ) -> Result<Vec<Combo>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ComboRow,
            r#"
            SELECT
                id,
                user_id,
                is_private,
                sort_order,
                free_form_category,
                title,
                video_short_uuid
            FROM combos
            WHERE user_id = $1
            "#,
            user_id.num()
        )
        .fetch_all(&state.pg_db_pool)
        .await?;

        let combos = rows.into_iter().map(Combo::from).collect();
        Ok(combos)
    }

    pub async fn update_combo(
        state: &AppState,
        current_user: &User,
        payload: ComboInfo,
    ) -> Result<Option<Combo>, sqlx::Error> {
        let row = sqlx::query_as!(
            ComboRow,
            r#"
        UPDATE combos
        SET
            is_private = $3,
            sort_order = $4,
            free_form_category = $5,
            title = $6,
            video_short_uuid = $7
        WHERE id = $1
            AND user_id = $2
        RETURNING
            id,
            user_id,
            is_private,
            sort_order,
            free_form_category,
            title,
            video_short_uuid
        "#,
            payload.id.num(),
            current_user.id.num(),
            payload.is_private,
            payload.sort_order,
            payload.free_form_category.as_deref(),
            payload.title.as_deref(),
            payload.video_short_uuid.as_deref(),
        )
        .fetch_optional(&state.pg_db_pool)
        .await?;

        Ok(row.map(Combo::from))
    }
}

impl From<ComboRow> for Combo {
    fn from(row: ComboRow) -> Self {
        Combo {
            id: ComboId(row.id),
            user_id: row.user_id(),
            is_private: row.is_private,
            sort_order: row.sort_order,
            free_form_category: row.free_form_category,
            title: row.title,
            video_short_uuid: row.video_short_uuid,
        }
    }
}
