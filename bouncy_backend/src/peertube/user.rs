use crate::{
    peertube::{check_peertube_response, token::OAuthToken, PeerTubeError},
    user::User,
    AppState,
};

/// Local user.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(transparent)]
pub(crate) struct PeerTubeUserId(pub i64);

/// Accounts are users or channels, local or federated.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(transparent)]
pub(crate) struct PeerTubeAccountId(pub i64);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeerTubeAccount {
    pub id: PeerTubeAccountId,
    pub name: String,
    pub display_name: String,
    // other fields omitted
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MyPeerTubeUser {
    pub id: PeerTubeUserId,
    pub username: String,
    pub account: PeerTubeAccount,
    // other fields omitted
}

/// Lazily set the PeerTube user id.
///
/// Here we rely on the "my user" endpoint to find the user id.
/// It would be nice to look it up from the OIDC subject but, alas,
/// there is no such endpoint in PeerTube at the time of writing.
pub(crate) async fn ensure_peertube_id(
    state: &AppState,
    user: &mut User,
    token: &OAuthToken,
) -> anyhow::Result<()> {
    if user.peertube_account_id.is_none() {
        let peertube_user = fetch_my_user(state, token).await?;
        user.add_peertube_id(state, peertube_user.account.id)
            .await?;
    }
    Ok(())
}

async fn fetch_my_user(
    state: &AppState,
    token: &OAuthToken,
) -> Result<MyPeerTubeUser, PeerTubeError> {
    let url = state
        .peertube_url
        .join("api/v1/users/me")
        .expect("must be valid url");

    let response = state
        .http_client
        .get(url.as_str())
        .bearer_auth(token.token())
        .send()
        .await;

    let ok_response = check_peertube_response(response).await?;
    let status = ok_response.status();

    let msg: Result<MyPeerTubeUser, _> = ok_response.json().await;
    let user = msg.map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    Ok(user)

    // let msg: Result<Vec<PeerTubeAccount>, _> = ok_response.json().await;
    // let users = msg.map_err(|err| PeerTubeError::JsonParsingFailed(status, err))?;
    // if users.len() != 1 {
    //     return Err(PeerTubeError::BadResponse(format!(
    //         "got {} me users instead of one",
    //         users.len()
    //     )));
    // }
    // let me = users.into_iter().next().unwrap();
    // Ok(me)
}
