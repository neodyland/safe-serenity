#[cfg(feature = "http")]
use super::{check_overflow, Builder};
use super::{
    CreateActionRow,
    CreateAllowedMentions,
    CreateAttachment,
    CreateEmbed,
    EditAttachments,
};
#[cfg(feature = "http")]
use crate::constants;
#[cfg(feature = "http")]
use crate::http::CacheHttp;
use crate::internal::prelude::*;
use crate::json::{self, json};
use crate::model::prelude::*;

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object).
#[derive(Clone, Debug)]
pub enum CreateInteractionResponse {
    /// Acknowledges a Ping (only required when your bot uses an HTTP endpoint URL).
    ///
    /// Corresponds to Discord's `PONG`.
    Pong,
    /// Responds to an interaction with a message.
    ///
    /// Corresponds to Discord's `CHANNEL_MESSAGE_WITH_SOURCE`.
    Message(CreateInteractionResponseMessage),
    /// Acknowledges the interaction in order to edit a response later. The user sees a loading
    /// state.
    ///
    /// Corresponds to Discord's `DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE`.
    Defer(CreateInteractionResponseMessage),
    /// Only valid for component-based interactions (seems to work for modal submit interactions
    /// too even though it's not documented).
    ///
    /// Acknowledges the interaction. You can optionally edit the original message later. The user
    /// does not see a loading state.
    ///
    /// Corresponds to Discord's `DEFERRED_UPDATE_MESSAGE`.
    Acknowledge,
    /// Only valid for component-based interactions.
    ///
    /// Edits the message the component was attached to.
    ///
    /// Corresponds to Discord's `UPDATE_MESSAGE`.
    UpdateMessage(CreateInteractionResponseMessage),
    /// Only valid for autocomplete interactions.
    ///
    /// Responds to the autocomplete interaction with suggested choices.
    ///
    /// Corresponds to Discord's `APPLICATION_COMMAND_AUTOCOMPLETE_RESULT`.
    Autocomplete(CreateAutocompleteResponse),
    /// Not valid for Modal and Ping interactions
    ///
    /// Responds to the interaction with a popup modal.
    ///
    /// Corresponds to Discord's `MODAL`.
    Modal(CreateModal),
}

impl serde::Serialize for CreateInteractionResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        use serde::ser::Error as _;

        #[allow(clippy::match_same_arms)] // hurts readability
        json!({
            "type": match self {
                Self::Pong { .. } => 1,
                Self::Message { .. } => 4,
                Self::Defer { .. } => 5,
                Self::Acknowledge { .. } => 6,
                Self::UpdateMessage { .. } => 7,
                Self::Autocomplete { .. } => 8,
                Self::Modal { .. } => 9,
            },
            "data": match self {
                Self::Pong => json::NULL,
                Self::Message(x) => json::to_value(x).map_err(S::Error::custom)?,
                Self::Defer(x) => json::to_value(x).map_err(S::Error::custom)?,
                Self::Acknowledge => json::NULL,
                Self::UpdateMessage(x) => json::to_value(x).map_err(S::Error::custom)?,
                Self::Autocomplete(x) => json::to_value(x).map_err(S::Error::custom)?,
                Self::Modal(x) => json::to_value(x).map_err(S::Error::custom)?,
            }
        })
        .serialize(serializer)
    }
}

impl CreateInteractionResponse {
    #[cfg(feature = "http")]
    fn check_length(&self) -> Result<()> {
        if let CreateInteractionResponse::Message(data)
        | CreateInteractionResponse::Defer(data)
        | CreateInteractionResponse::UpdateMessage(data) = self
        {
            if let Some(content) = &data.content {
                check_overflow(content.chars().count(), constants::MESSAGE_CODE_LIMIT)
                    .map_err(|overflow| Error::Model(ModelError::MessageTooLong(overflow)))?;
            }

            if let Some(embeds) = &data.embeds {
                check_overflow(embeds.len(), constants::EMBED_MAX_COUNT)
                    .map_err(|_| Error::Model(ModelError::EmbedAmount))?;

                for embed in embeds {
                    embed.check_length()?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "http")]
#[async_trait::async_trait]
impl Builder for CreateInteractionResponse {
    type Context<'ctx> = (InteractionId, &'ctx str);
    type Built = ();

    /// Creates a response to the interaction received.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points, and embeds must be under
    /// 6000 code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the message content is too long. May also return an
    /// [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is an error in
    /// deserializing the API response.
    async fn execute(
        mut self,
        cache_http: impl CacheHttp,
        ctx: Self::Context<'_>,
    ) -> Result<Self::Built> {
        self.check_length()?;
        let files = match &mut self {
            CreateInteractionResponse::Message(msg)
            | CreateInteractionResponse::Defer(msg)
            | CreateInteractionResponse::UpdateMessage(msg) => msg.attachments.take_files(),
            _ => Vec::new(),
        };

        cache_http.http().create_interaction_response(ctx.0, ctx.1, &self, files).await
    }
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-messages).
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct CreateInteractionResponseMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<InteractionResponseFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<CreateActionRow>>,
    attachments: EditAttachments,
}

impl CreateInteractionResponseMessage {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether the message is text-to-speech.
    ///
    /// Think carefully before setting this to `true`.
    ///
    /// Defaults to `false`.
    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    /// Appends a file to the message.
    pub fn add_file(mut self, file: CreateAttachment) -> Self {
        self.attachments = self.attachments.add(file);
        self
    }

    /// Appends a list of files to the message.
    pub fn add_files(mut self, files: impl IntoIterator<Item = CreateAttachment>) -> Self {
        for file in files {
            self.attachments = self.attachments.add(file);
        }
        self
    }

    /// Sets a list of files to include in the message.
    ///
    /// Calling this multiple times will overwrite the file list. To append files, call
    /// [`Self::add_file`] or [`Self::add_files`] instead.
    pub fn files(mut self, files: impl IntoIterator<Item = CreateAttachment>) -> Self {
        self.attachments = EditAttachments::new();
        self.add_files(files)
    }

    /// Set the content of the message.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    #[inline]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Adds an embed to the message.
    ///
    /// Calling this while editing a message will overwrite existing embeds.
    pub fn add_embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.get_or_insert_with(Vec::new).push(embed);
        self
    }

    /// Adds multiple embeds for the message.
    ///
    /// Calling this while editing a message will overwrite existing embeds.
    pub fn add_embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds.get_or_insert_with(Vec::new).extend(embeds);
        self
    }

    /// Sets a single embed to include in the message
    ///
    /// Calling this will overwrite the embed list. To append embeds, call [`Self::add_embed`]
    /// instead.
    pub fn embed(self, embed: CreateEmbed) -> Self {
        self.embeds(vec![embed])
    }

    /// Sets a list of embeds to include in the message.
    ///
    /// Calling this will overwrite the embed list. To append embeds, call [`Self::add_embeds`]
    /// instead.
    pub fn embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    /// Set the allowed mentions for the message.
    pub fn allowed_mentions(mut self, allowed_mentions: CreateAllowedMentions) -> Self {
        self.allowed_mentions = Some(allowed_mentions);
        self
    }

    /// Sets the flags for the message.
    pub fn flags(mut self, flags: InteractionResponseFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Adds or removes the ephemeral flag.
    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        let mut flags = self.flags.unwrap_or_else(InteractionResponseFlags::empty);

        if ephemeral {
            flags |= InteractionResponseFlags::EPHEMERAL;
        } else {
            flags &= !InteractionResponseFlags::EPHEMERAL;
        };

        self.flags = Some(flags);
        self
    }

    /// Sets the components of this message.
    pub fn components(mut self, components: Vec<CreateActionRow>) -> Self {
        self.components = Some(components);
        self
    }
    super::button_and_select_menu_convenience_methods!(self.components);
}

// Same as CommandOptionChoice according to Discord, see
// [Autocomplete docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete).
#[must_use]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AutocompleteChoice(CommandOptionChoice);
impl AutocompleteChoice {
    pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self {
        Self(CommandOptionChoice {
            name: name.into(),
            name_localizations: None,
            value: value.into(),
        })
    }

    pub fn add_localized_name(
        mut self,
        locale: impl Into<String>,
        localized_name: impl Into<String>,
    ) -> Self {
        self.0
            .name_localizations
            .get_or_insert_with(Default::default)
            .insert(locale.into(), localized_name.into());
        self
    }
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-autocomplete)
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct CreateAutocompleteResponse {
    choices: Vec<AutocompleteChoice>,
}

impl CreateAutocompleteResponse {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// For autocomplete responses this sets their autocomplete suggestions.
    ///
    /// See the official docs on [`Application Command Option Choices`] for more information.
    ///
    /// [`Application Command Option Choices`]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
    pub fn set_choices(mut self, choices: Vec<AutocompleteChoice>) -> Self {
        self.choices = choices;
        self
    }

    /// Add an int autocomplete choice.
    ///
    /// **Note**: There can be no more than 25 choices set. Name must be between 1 and 100
    /// characters. Value must be between -2^53 and 2^53.
    pub fn add_int_choice(self, name: impl Into<String>, value: i64) -> Self {
        self.add_choice(AutocompleteChoice::new(name, value))
    }

    /// Adds a string autocomplete choice.
    ///
    /// **Note**: There can be no more than 25 choices set. Name must be between 1 and 100
    /// characters. Value must be up to 100 characters.
    pub fn add_string_choice(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_choice(AutocompleteChoice::new(name, value.into()))
    }

    /// Adds a number autocomplete choice.
    ///
    /// **Note**: There can be no more than 25 choices set. Name must be between 1 and 100
    /// characters. Value must be between -2^53 and 2^53.
    pub fn add_number_choice(self, name: impl Into<String>, value: f64) -> Self {
        self.add_choice(AutocompleteChoice::new(name, value))
    }

    fn add_choice(mut self, value: AutocompleteChoice) -> Self {
        self.choices.push(value);
        self
    }
}

#[cfg(feature = "http")]
#[async_trait::async_trait]
impl Builder for CreateAutocompleteResponse {
    type Context<'ctx> = (InteractionId, &'ctx str);
    type Built = ();

    /// Creates a response to an autocomplete interaction.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the API returns an error.
    async fn execute(
        self,
        cache_http: impl CacheHttp,
        ctx: Self::Context<'_>,
    ) -> Result<Self::Built> {
        cache_http.http().create_interaction_response(ctx.0, ctx.1, &self, Vec::new()).await
    }
}

/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-modal).
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct CreateModal {
    components: Vec<CreateActionRow>,
    custom_id: String,
    title: String,
}

impl CreateModal {
    /// Creates a new modal.
    pub fn new(custom_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            components: Vec::new(),
            custom_id: custom_id.into(),
            title: title.into(),
        }
    }

    /// Sets the components of this message.
    ///
    /// Overwrites existing components.
    pub fn components(mut self, components: Vec<CreateActionRow>) -> Self {
        self.components = components;
        self
    }
}
