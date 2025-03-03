//! Webhook model and implementations.

#[cfg(feature = "model")]
use secrecy::ExposeSecret;
use secrecy::SecretString;

#[cfg(feature = "model")]
use super::channel::Message;
use super::id::{ChannelId, GuildId, WebhookId};
use super::user::User;
use super::utils::secret;
#[cfg(feature = "model")]
use crate::builder::{Builder, EditWebhook, EditWebhookMessage, ExecuteWebhook};
#[cfg(feature = "cache")]
use crate::cache::{Cache, GuildRef};
#[cfg(feature = "model")]
use crate::http::{CacheHttp, Http};
#[cfg(feature = "model")]
use crate::internal::prelude::*;
use crate::model::prelude::*;
#[cfg(feature = "model")]
use crate::model::ModelError;

enum_number! {
    /// A representation of a type of webhook.
    ///
    /// [Discord docs](https://discord.com/developers/docs/resources/webhook#webhook-object-webhook-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    #[non_exhaustive]
    pub enum WebhookType {
        /// An indicator that the webhook can post messages to channels with a token.
        Incoming = 1,
        /// An indicator that the webhook is managed by Discord for posting new messages to
        /// channels without a token.
        ChannelFollower = 2,
        /// Application webhooks are webhooks used with Interactions.
        Application = 3,
        _ => Unknown(u8),
    }
}

impl WebhookType {
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Incoming => "incoming",
            Self::ChannelFollower => "channel follower",
            Self::Application => "application",
            Self::Unknown(_) => "unknown",
        }
    }
}

/// A representation of a webhook, which is a low-effort way to post messages to channels. They do
/// not necessarily require a bot user or authentication to use.
///
/// [Discord docs](https://discord.com/developers/docs/resources/webhook#webhook-object).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Webhook {
    /// The unique Id.
    ///
    /// Can be used to calculate the creation date of the webhook.
    pub id: WebhookId,
    /// The type of the webhook.
    #[serde(rename = "type")]
    pub kind: WebhookType,
    /// The Id of the guild that owns the webhook.
    pub guild_id: Option<GuildId>,
    /// The Id of the channel that owns the webhook.
    pub channel_id: Option<ChannelId>,
    /// The user that created the webhook.
    ///
    /// **Note**: This is not received when getting a webhook by its token.
    pub user: Option<User>,
    /// The default name of the webhook.
    ///
    /// This can be temporarily overridden via [`ExecuteWebhook::username`].
    pub name: Option<String>,
    /// The default avatar.
    ///
    /// This can be temporarily overridden via [`ExecuteWebhook::avatar_url`].
    pub avatar: Option<ImageHash>,
    /// The webhook's secure token.
    #[serde(with = "secret", default)]
    pub token: Option<SecretString>,
    /// The bot/OAuth2 application that created this webhook.
    pub application_id: Option<ApplicationId>,
    /// The guild of the channel that this webhook is following (returned for
    /// [`WebhookType::ChannelFollower`])
    pub source_guild: Option<WebhookGuild>,
    /// The channel that this webhook is following (returned for
    /// [`WebhookType::ChannelFollower`]).
    pub source_channel: Option<WebhookChannel>,
    /// The url used for executing the webhook (returned by the webhooks OAuth2 flow).
    #[serde(with = "secret", default)]
    pub url: Option<SecretString>,
}

/// The guild object returned by a [`Webhook`], of type [`WebhookType::ChannelFollower`].
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WebhookGuild {
    /// The unique Id identifying the guild.
    pub id: GuildId,
    /// The name of the guild.
    pub name: String,
    /// The hash of the icon used by the guild.
    ///
    /// In the client, this appears on the guild list on the left-hand side.
    pub icon: Option<ImageHash>,
}

#[cfg(feature = "model")]
impl WebhookGuild {
    /// Tries to find the [`Guild`] by its Id in the cache.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn to_guild_cached(self, cache: &impl AsRef<Cache>) -> Option<GuildRef<'_>> {
        cache.as_ref().guild(self.id)
    }

    /// Requests [`PartialGuild`] over REST API.
    ///
    /// **Note**: This will not be a [`Guild`], as the REST API does not send
    /// all data with a guild retrieval.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user is not in the guild.
    #[inline]
    pub async fn to_partial_guild(self, cache_http: impl CacheHttp) -> Result<PartialGuild> {
        #[cfg(feature = "cache")]
        {
            if let Some(cache) = cache_http.cache() {
                if let Some(guild) = cache.guild(self.id) {
                    return Ok(guild.clone().into());
                }
            }
        }

        cache_http.http().get_guild(self.id).await
    }

    /// Requests [`PartialGuild`] over REST API with counts.
    ///
    /// **Note**: This will not be a [`Guild`], as the REST API does not send all data with a guild
    /// retrieval.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user is not in the guild.
    #[inline]
    pub async fn to_partial_guild_with_counts(
        self,
        http: impl AsRef<Http>,
    ) -> Result<PartialGuild> {
        http.as_ref().get_guild_with_counts(self.id).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WebhookChannel {
    /// The unique Id of the channel.
    pub id: ChannelId,
    /// The name of the channel.
    pub name: String,
}

#[cfg(feature = "model")]
impl WebhookChannel {
    /// Attempts to find a [`Channel`] by its Id in the cache.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn to_channel_cached(self, cache: impl AsRef<Cache>) -> Option<Channel> {
        cache.as_ref().channel(self.id)
    }

    /// First attempts to find a [`Channel`] by its Id in the cache, upon failure requests it via
    /// the REST API.
    ///
    /// **Note**: If the `cache`-feature is enabled permissions will be checked and upon owning the
    /// required permissions the HTTP-request will be issued. Additionally, you might want to
    /// enable the `temp_cache` feature to cache channel data retrieved by this function for a
    /// short duration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the channel retrieval request failed.
    #[inline]
    pub async fn to_channel(self, cache_http: impl CacheHttp) -> Result<Channel> {
        #[cfg(feature = "cache")]
        {
            if let Some(cache) = cache_http.cache() {
                if let Some(channel) = cache.channel(self.id) {
                    return Ok(channel);
                }
            }
        }

        let channel = cache_http.http().get_channel(self.id).await?;

        #[cfg(all(feature = "cache", feature = "temp_cache"))]
        {
            if let Some(cache) = cache_http.cache() {
                if let Channel::Guild(guild_channel) = &channel {
                    cache.temp_channels.insert(guild_channel.id, guild_channel.clone());
                }
            }
        }

        Ok(channel)
    }
}

#[cfg(feature = "model")]
impl Webhook {
    /// Retrieves a webhook given its Id.
    ///
    /// This method requires authentication, whereas [`Webhook::from_id_with_token`] and
    /// [`Webhook::from_url`] do not.
    ///
    /// # Examples
    ///
    /// Retrieve a webhook by Id:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// # use serenity::model::{webhook::Webhook, id::WebhookId};
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Http = unimplemented!();
    /// let id = WebhookId::new(245037420704169985);
    /// let webhook = Webhook::from_id(&http, id).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user is not authenticated, or if the webhook does
    /// not exist.
    ///
    /// May also return an [`Error::Json`] if there is an error in deserialising Discord's
    /// response.
    pub async fn from_id(http: impl AsRef<Http>, webhook_id: impl Into<WebhookId>) -> Result<Self> {
        http.as_ref().get_webhook(webhook_id.into()).await
    }

    /// Retrieves a webhook given its Id and unique token.
    ///
    /// This method does _not_ require authentication.
    ///
    /// # Examples
    ///
    /// Retrieve a webhook by Id and its unique token:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// # use serenity::model::{webhook::Webhook, id::WebhookId};
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Http = unimplemented!();
    /// let id = WebhookId::new(245037420704169985);
    /// let token = "ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
    ///
    /// let webhook = Webhook::from_id_with_token(&http, id, token).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the webhook does not exist, or if the token is invalid.
    ///
    /// May also return an [`Error::Json`] if there is an error in deserialising Discord's
    /// response.
    pub async fn from_id_with_token(
        http: impl AsRef<Http>,
        webhook_id: impl Into<WebhookId>,
        token: &str,
    ) -> Result<Self> {
        http.as_ref().get_webhook_with_token(webhook_id.into(), token).await
    }

    /// Retrieves a webhook given its url.
    ///
    /// This method does _not_ require authentication.
    ///
    /// # Examples
    ///
    /// Retrieve a webhook by url:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// # use serenity::model::webhook::Webhook;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Http = unimplemented!();
    /// let url = "https://discord.com/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
    /// let webhook = Webhook::from_url(&http, url).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the url is malformed, or otherwise if the webhook does not
    /// exist, or if the token is invalid.
    ///
    /// May also return an [`Error::Json`] if there is an error in deserialising Discord's
    /// response.
    pub async fn from_url(http: impl AsRef<Http>, url: &str) -> Result<Self> {
        http.as_ref().get_webhook_from_url(url).await
    }

    /// Deletes the webhook.
    ///
    /// If [`Self::token`] is set, then authentication is _not_ required. Otherwise, if it is
    /// [`None`], then authentication _is_ required.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the webhook does not exist, the token is invalid, or if the
    /// webhook could not otherwise be deleted.
    #[inline]
    pub async fn delete(&self, http: impl AsRef<Http>) -> Result<()> {
        let http = http.as_ref();
        match &self.token {
            Some(token) => {
                http.delete_webhook_with_token(self.id, token.expose_secret(), None).await
            },
            None => http.delete_webhook(self.id, None).await,
        }
    }

    /// Edits the webhook.
    ///
    /// If [`Self::token`] is set, then authentication is _not_ required. Otherwise, if it is
    /// [`None`], then authentication _is_ required.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// # use serenity::builder::EditWebhook;
    /// # use serenity::model::webhook::Webhook;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Http = unimplemented!();
    /// let url = "https://discord.com/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
    /// let mut webhook = Webhook::from_url(&http, url).await?;
    ///
    /// let builder = EditWebhook::new().name("new name");
    /// webhook.edit(&http, builder).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if [`Self::token`] is [`None`].
    ///
    /// May also return an [`Error::Http`] if the content is malformed, or if the token is invalid.
    ///
    /// Or may return an [`Error::Json`] if there is an error in deserialising Discord's response.
    pub async fn edit(
        &mut self,
        cache_http: impl CacheHttp,
        builder: EditWebhook<'_>,
    ) -> Result<()> {
        let token = self.token.as_ref().map(ExposeSecret::expose_secret).map(String::as_str);
        *self = builder.execute(cache_http, (self.id, token)).await?;
        Ok(())
    }

    /// Executes a webhook with the fields set via the given builder.
    ///
    /// # Examples
    ///
    /// Execute a webhook with message content of `test`:
    ///
    /// ```rust,no_run
    /// # use serenity::builder::ExecuteWebhook;
    /// # use serenity::http::Http;
    /// # use serenity::model::webhook::Webhook;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Http = unimplemented!();
    /// let url = "https://discord.com/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
    /// let mut webhook = Webhook::from_url(&http, url).await?;
    ///
    /// let builder = ExecuteWebhook::new().content("test");
    /// webhook.execute(&http, false, builder).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Execute a webhook with message content of `test`, overriding the username to `serenity`,
    /// and sending an embed:
    ///
    /// ```rust,no_run
    /// # use serenity::http::Http;
    /// # use serenity::model::webhook::Webhook;
    /// #
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let http: Http = unimplemented!();
    /// use serenity::builder::{CreateEmbed, ExecuteWebhook};
    ///
    /// let url = "https://discord.com/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
    /// let mut webhook = Webhook::from_url(&http, url).await?;
    ///
    /// let embed = CreateEmbed::new()
    ///     .title("Rust's website")
    ///     .description(
    ///         "Rust is a systems programming language that runs blazingly fast, prevents \
    ///         segfaults, and guarantees thread safety.",
    ///     )
    ///     .url("https://rust-lang.org");
    ///
    /// let builder = ExecuteWebhook::new().content("test").username("serenity").embed(embed);
    /// webhook.execute(&http, false, builder).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if [`Self::token`] is [`None`].
    ///
    /// May also return an [`Error::Http`] if the content is malformed, or if the webhook's token
    /// is invalid.
    ///
    /// Or may return an [`Error::Json`] if there is an error deserialising Discord's response.
    #[inline]
    pub async fn execute(
        &self,
        cache_http: impl CacheHttp,
        wait: bool,
        builder: ExecuteWebhook,
    ) -> Result<Option<Message>> {
        let token = self.token.as_ref().ok_or(ModelError::NoTokenSet)?.expose_secret();
        builder.execute(cache_http, (self.id, token, wait)).await
    }

    /// Gets a previously sent message from the webhook.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the [`Self::token`] is [`None`].
    ///
    /// May also return [`Error::Http`] if the webhook's token is invalid, or the given message Id
    /// does not belong to the current webhook.
    ///
    /// Or may return an [`Error::Json`] if there is an error deserialising Discord's response.
    pub async fn get_message(
        &self,
        http: impl AsRef<Http>,
        thread_id: Option<ChannelId>,
        message_id: MessageId,
    ) -> Result<Message> {
        let token = self.token.as_ref().ok_or(ModelError::NoTokenSet)?.expose_secret();
        http.as_ref().get_webhook_message(self.id, thread_id, token, message_id).await
    }

    /// Edits a webhook message with the fields set via the given builder.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if [`Self::token`] is [`None`], or if the message content is
    /// too long.
    ///
    /// May also return an [`Error::Http`] if the content is malformed, the webhook's token is
    /// invalid, or the given message Id does not belong to the current webhook.
    ///
    /// Or may return an [`Error::Json`] if there is an error deserialising Discord's response.
    pub async fn edit_message(
        &self,
        cache_http: impl CacheHttp,
        message_id: MessageId,
        builder: EditWebhookMessage,
    ) -> Result<Message> {
        let token = self.token.as_ref().ok_or(ModelError::NoTokenSet)?.expose_secret();
        builder.execute(cache_http, (self.id, token, message_id)).await
    }

    /// Deletes a webhook message.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the [`Self::token`] is [`None`].
    ///
    /// May also return an [`Error::Http`] if the webhook's token is invalid or the given message
    /// Id does not belong to the current webhook.
    pub async fn delete_message(
        &self,
        http: impl AsRef<Http>,
        thread_id: Option<ChannelId>,
        message_id: MessageId,
    ) -> Result<()> {
        let token = self.token.as_ref().ok_or(ModelError::NoTokenSet)?.expose_secret();
        http.as_ref().delete_webhook_message(self.id, thread_id, token, message_id).await
    }

    /// Retrieves the latest information about the webhook, editing the webhook in-place.
    ///
    /// As this calls the [`Http::get_webhook_with_token`] function, authentication is not
    /// required.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the [`Self::token`] is [`None`].
    ///
    /// May also return an [`Error::Http`] if the http client errors or if Discord returns an
    /// error. Such as if the [`Webhook`] was deleted.
    ///
    /// Or may return an [`Error::Json`] if there is an error deserialising Discord's response.
    pub async fn refresh(&mut self, http: impl AsRef<Http>) -> Result<()> {
        let token = self.token.as_ref().ok_or(ModelError::NoTokenSet)?.expose_secret();
        http.as_ref().get_webhook_with_token(self.id, token).await.map(|replacement| {
            *self = replacement;
        })
    }

    /// Returns the url of the webhook.
    ///
    /// ```rust,ignore
    /// assert_eq!(hook.url(), "https://discord.com/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV")
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the [`Self::token`] is [`None`].
    pub fn url(&self) -> Result<String> {
        let token = self.token.as_ref().ok_or(ModelError::NoTokenSet)?.expose_secret();
        Ok(format!("https://discord.com/api/webhooks/{}/{token}", self.id))
    }
}

#[cfg(feature = "model")]
impl WebhookId {
    /// Requests [`Webhook`] over REST API.
    ///
    /// **Note**: Requires the [Manage Webhooks] permission.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the http client errors or if Discord returns an error. Such
    /// as if the [`WebhookId`] does not exist.
    ///
    /// May also return an [`Error::Json`] if there is an error in deserialising the response.
    ///
    /// [Manage Webhooks]: super::permissions::Permissions::MANAGE_WEBHOOKS
    #[inline]
    pub async fn to_webhook(self, http: impl AsRef<Http>) -> Result<Webhook> {
        http.as_ref().get_webhook(self).await
    }
}
