use rss::Channel;
use snafu::{ResultExt, Snafu};
use url::Url;

pub async fn channel_from_url(url: Url) -> Result<Channel, ChannelFromUrlError> {
    let raw_resp = reqwest::get(url)
        .await
        .context(HttpSnafu)?
        .bytes()
        .await
        .context(TextSnafu)?;
    let channel = Channel::read_from(raw_resp.as_ref())
        .context(RssSnafu)?;

    Ok(channel)
}

#[derive(Debug, Snafu)]
pub enum ChannelFromUrlError {
    HttpError { source: reqwest::Error },
    TextError { source: reqwest::Error },
    RssError { source: rss::Error },
}
