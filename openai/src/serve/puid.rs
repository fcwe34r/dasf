use std::str::FromStr;

use super::error::ResponseError;
use crate::arkose::GPTModel;
use crate::{context, URL_CHATGPT_API};
use moka::sync::Cache;
use tokio::sync::OnceCell;

static PUID_CACHE: OnceCell<Cache<String, String>> = OnceCell::const_new();

pub(super) fn reduce_cache_key(token: &str) -> Result<String, ResponseError> {
    let token_profile = crate::token::check(token)
        .map_err(ResponseError::Unauthorized)?
        .ok_or(ResponseError::BadRequest(anyhow::anyhow!(
            "invalid access token"
        )))?;
    Ok(token_profile.email().to_owned())
}

async fn puid_cache() -> &'static Cache<String, String> {
    PUID_CACHE
        .get_or_init(|| async {
            Cache::builder()
                .time_to_live(std::time::Duration::from_secs(3600 * 24))
                .build()
        })
        .await
}

pub(super) async fn get_or_init_puid(
    token: &str,
    model: &str,
    cache_id: String,
) -> Result<Option<String>, ResponseError> {
    let token = token.trim_start_matches("Bearer ");
    let mut m_puid = None;
    if GPTModel::from_str(model)?.is_gpt4() {
        let puid_cache = puid_cache().await;
        if let Some(puid) = puid_cache.get(&cache_id) {
            m_puid = Some(puid);
        } else {
            let resp = context::get_instance()
                .client()
                .get(format!("{URL_CHATGPT_API}/backend-api/models"))
                .bearer_auth(token)
                .send()
                .await
                .map_err(ResponseError::InternalServerError)?
                .error_for_status()
                .map_err(ResponseError::BadRequest)?;

            if let Some(c) = resp.cookies().into_iter().find(|c| c.name().eq("_puid")) {
                m_puid = Some(c.value().to_owned());
                puid_cache.insert(cache_id, m_puid.clone().expect("puid is none"));
            };
        }
    }
    Ok(m_puid)
}
