# Change Log

All notable changes to this project will be documented in this file.
This project mostly adheres to [Semantic Versioning][semver].

## [0.11.7] - 2023-10-24

Thanks to the following for their contributions:

- [@arqunis]
- [@B-2U]
- [@Collin-Swish]
- [@caoculus]
- [@Dean-Coakley]
- [@HexPandaa]
- [@mattfbacon]
- [@mkrasnitski]
- [@OnlyCS]

### Notable changes

* ([#2493](https://github.com/serenity-rs/serenity/pull/2493)) Add `MessageComponentInteraction::edit_original_message()` for editing the original message of the message component.
* ([#2504](https://github.com/serenity-rs/serenity/pull/2504)) Implement `std::str::FromStr` for the ID types to allow converting from a string. This does not affect the pre-existing implementation on `ChannelId`, `RoleId`, and `UserId`.
* ([#2506](https://github.com/serenity-rs/serenity/pull/2506)) Add missing `set_components()` methods on `EditInteractionResponse` and `EditWebhookMessage`
* ([#2533](https://github.com/serenity-rs/serenity/pull/2533)) Add additional `delete_reaction` and `delete_reactions` methods for deleting reactions to `ChannelId`, `GuildChannel`, and `Message`.
* ([#2562](https://github.com/serenity-rs/serenity/pull/2562)) Bump `base64` dependency to `0.21`

## [0.11.6] - 2023-06-30

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Chronophylos]
- [@cyril-marpaud]
- [@dclamage]
- [@dpytaylo]
- [@float3]
- [@ghost]
- [@GnomedDev]
- [@ivancernja]
- [@jontze]
- [@kangalio]
- [@MarkusTheOrt]
- [@mkrasnitski]
- [@NinekoTheCat]
- [@Polyhistorian]
- [@rand0m-cloud]
- [@sandlotie]
- [@ShashankKumarSaxena]
- [@squili]
- [@Web-44]
- [@zackradisic]
- [@zzzzDev4]

### Notable changes

* ([#2076](https://github.com/serenity-rs/serenity/pull/2076)) Make `Timestamp` usable regardless of the `chrono` or `time` features.
* ([#2077](https://github.com/serenity-rs/serenity/pull/2077)) Deprecate modifying command application permissions in bulk. The corresponding endpoint is already deprecated by Discord.
* ([#2130](https://github.com/serenity-rs/serenity/pull/2130)) Standard Framework: Implicitly set `BucketBuilder::await_ratelimits` to `1` upon `BucketBuilder::delay_action` being set.
* ([#2133](https://github.com/serenity-rs/serenity/pull/2133)) Add Voice channels to the list of text-based channel types.
* ([#2136](https://github.com/serenity-rs/serenity/pull/2136)) Add an event for HTTP ratelimits, and a corresponding `EventHandler::ratelimit` method.
* ([#2154](https://github.com/serenity-rs/serenity/pull/2154)) Add the following fields to `MessageUpdateEvent`:
     - `mention_channels`
     - `reactions`
     - `components`
     - `sticker_items`
* ([#2155](https://github.com/serenity-rs/serenity/pull/2155)) Expose `Shard::handle_event` publicly.
* ([#2214](https://github.com/serenity-rs/serenity/pull/2214)) Add `User::member`, as well as `Message:{thread, application_id}` fields.
* ([#2223](https://github.com/serenity-rs/serenity/pull/2223), [#2290](https://github.com/serenity-rs/serenity/pull/2290)) Add a `defer_ephemeral` helper method to many interaction types.
* ([#2265](https://github.com/serenity-rs/serenity/pull/2265)) Add `as_*` and `into_*` helper methods to the `Interaction` type for converting to each of its respective variants.
* ([#2281](https://github.com/serenity-rs/serenity/pull/2281)) Add the `UserPublicFlags::ACTIVE_DEVELOPER` flag.
* ([#2298](https://github.com/serenity-rs/serenity/pull/2298)) Add the following fields to guild channel, relevant for forums:
     - `flags`
     - `total_messages_sent`
     - `available_tags`
     - `applied_tags`
     - `default_reaction_emoji`
     - `default_thread_rate_limit_per_user`
     - `default_sort_order`
* ([#2330](https://github.com/serenity-rs/serenity/pull/2330)) Add support for `owner_id` in thread and forum channels.
* ([#2346](https://github.com/serenity-rs/serenity/pull/2346)) Add the `SUPPRESS_NOTIFICATIONS` message flag.
* ([#2431](https://github.com/serenity-rs/serenity/pull/2431)) Add support for getting application commands with localizations via the following methods:
    - `Http::get_{guild,global}_application_commands_with_localizations`
    - `Command::get_global_application_commands_with_localizations`
    - `{GuildId,Guild,PartialGuild}::get_application_commands_with_localizations`
* ([#2444](https://github.com/serenity-rs/serenity/pull/2444)) Add a `remove_all_attachments` method to `EditMessage`.

## [0.11.5] - 2022-07-29

Thanks to the following for their contributions:

- [@bumblepie]

### Notable changes

- Make select menu values optional to fix deserialization of message component interactions ([@bumblepie])

## [0.11.4] - 2022-07-19

Thanks to the following for their contributions:

- [@anden3]

### Notable changes

- Fix deserialization error of `GuildChannel::message_count` ([@anden3])

## [0.11.3] - 2022-07-19

Thanks to the following for their contributions:

- [@AlexisTM]
- [@aawilson]
- [@acdenisSK]
- [@bumblepie]
- [@DRuppFv]
- [@FallenWarrior2k]
- [@GnomedDev]
- [@kangalio]
- [@Milo123459]
- [@mkrasnitski]
- [@NotNorom]
- [@nickelc]
- [@SimonZehetner]

### Notable changes

- Temporarily fix `GuildChannel::message_count` in a non-breaking way ([@GnomedDev])
- Add support for `Auto Moderation` feature ([@nickelc])
- Add optional min/max length fields for application command string options ([@nickelc])
- Allow select menu response for modals ([@AlexisTM])
- Add `app_permissions` field on interactions ([@nickelc])
- Enable `Invite::expires_at` field ([@mkrasnitski])
- Fix "missing field `discriminator`" serde error for presence updates ([@nickelc])
- Add webhook example ([@NotNorom])
- Add `MessageType::AutoModerationAction` enum variant ([@nickelc])
- Attempt to fix lifetime bug around `CustomisedHelpData` ([@mkrasnitski])
- Auto-impl `CacheHttp` for `Arc<T>` if `T` also implements it ([@mkrasnitski])
- Add audit log action types for scheduled events ([@nickelc])
- Fill gaps in all `model::application` types ([@kangalio])
- Add support for slash command localization ([@kangalio])
- Implement announcement channel following ([@GnomedDev])
- Add event handler methods for scheduled events ([@nickelc])
- Add methods to `Webhook` to enable direct creation ([@mkrasnitski])
- Consolidate `interactions` & `oauth2` model types into the `application` module ([@nickelc])
- Fix compile errors in builders when the `model` feature is disabled ([@FallenWarrior2k])

## [0.11.2] - 2022-05-08

Thanks to the following for their contributions:

- [@acdenisSK]
- [@elkowar]
- [@GnomedDev]
- [@kangalio]
- [@MelonShooter]
- [@mkrasnitski]
- [@nickelc]
- [@tedtramonte]

### Added

- [model] Implement Guild Scheduled Events support ([@mkrasnitski]) [c:e690ccd]
- [model] Add initial application command permissions v2 support ([@nickelc]) [c:bca105a]
- [model] Implement `Ord` and `PartialOrd` for `Timestamp` ([@tedtramonte]) [c:b1c3a62]
- [model] Add new enum variant to `audit_log::Action` to represent unknown values ([@nickelc]) [c:b2ae872]
- [model] Add `SUPPRESS_EMBEDS` flag for interaction response messages ([@nickelc]) [c:888c37f]
- [builder] Add function for interaction followup messages to set or unset the ephemeral flag ([@nickelc]) [c:10d9297]
- [model] Implement conversion from `Guild` to `PartialGuild` ([@kangalio]) [c:475fc1f]

### Changed

- [model] Split up a few functions to minimize monomorphization-bloat ([@elkowar]) [c:39fb310]
- [misc] Adapt the library to pass the majority of `clippy::pedantic` lints ([@GnomedDev]) [c:dc22cf2]
- [utils] Strip leading "Bot " in parse_token ([@kangalio]) [c:0a544c6]
- [model] Replace feature gated `simd-json` trait imports with `json::prelude::*` ([@nickelc]) [c:3e454cc]
- [gateway] Improve binary message deserialization time by optimizing `convert_ws_message` ([@MelonShooter]) [c:8b2326c]
- [gateway] Remove duplicate WS client creation for native/rustls backends ([@nickelc]) [c:be17752]
- [utils] Use `Cow` in `utils/content_safe.rs` to save on allocations ([@mkrasnitski]) [c:79a3cc8]
- [misc] Use `dashmap` 5.2.0 for the MSRV build job ([@nickelc]) [c:2898af2]
- [misc] Update `simd-json` to 0.4.14 ([@nickelc]) [c:0ba01a7]
- [model] Optimize guild create & delete event deserialization ([@nickelc]) [c:47433de]
- [model] Use `AttachmentType::data` in `EditRole::icon` ([@mkrasnitski]) [c:c3ad0b2]
- [misc] Deny broken and private intra doc links ([@nickelc]) [c:56293bb]
- [misc] Update `.git-blame-ignore-revs` ([@nickelc]) [c:75df53e]

### Fixed

- [misc] Fix wrong ID type in doc comment ([@nickelc]) [c:e1eaeeb]
- [model] Fix lifetimes on interaction response methods ([@kangalio]) [c:dd9d255]
- [framework] Fix argument parsing when no delimiters are specified ([@acdenisSK]) [c:628c1ea] [c:889540a]
- [misc] Fix unused import and deny unused items ([@nickelc]) [c:b1cc702]
- [misc] Fix broken links in the readme ([@nickelc]) [c:fe53b16]

### Removed

- [misc] Remove unnecessary braces in use statement ([@nickelc]) [c:2f8e2b0]
- [voice] Remove unused code related to the former voice functionality ([@nickelc]) [c:97ef906]
- [examples] Remove unused `chrono` dependency in example 09 ([@acdenisSK]) [c:0450dde]

## [0.11.1] - 2022-04-18

This is a light release to fix compilation on https://docs.rs.

Thanks to the following for their contributions:

- [@acdenisSK]

### Changed

- [misc] Manually specify list of features for docs.rs to build ([@acdenisSK]) [c:9f11a3f]

## [0.11.0] - 2022-04-18

Thanks to the following for their contributions:

- [@AldanTanneo]
- [@AngelOnFira]
- [@AnnikaCodes]
- [@acdenisSK]
- [@almeidx]
- [@Bloectasy]
- [@ben-brook]
- [@Chronophylos]
- [@devtomio]
- [@drklee3]
- [@FallenWarrior2k]
- [@Gentoli]
- [@GnomedDev]
- [@gradiuscypher]
- [@HarmoGlace]
- [@hybras]
- [@JellyWX]
- [@kangalio]
- [@Lakelezz]
- [@Licenser]
- [@MathyouMB]
- [@Milo123459]
- [@mkrasnitski]
- [@natto1784]
- [@nickelc]
- [@pascalharp]
- [@rasm47]
- [@Rstar284]
- [@Some-Dood]
- [@squili]
- [@TheBlackfurGuy]
- [@tylerd008]
- [@vaporox]
- [@vicky5124]
- [@xMAC94x]
- [@xfix]

### Added

- [ci] Add build job to test the `native-tls` backend ([@nickelc]) [c:440f0fa]
- [model] Add new command data field `guild_id` ([@nickelc]) [c:4dbf3d0]
- [model] Add new "Directory" channel type ([@nickelc]) [c:3b99eeb]
- [utils] Add `users` argument to `utils::content_safe` ([@GnomedDev]) [c:c18e889]
- [model] Add attachment support to interaction responses ([@GnomedDev]) [c:dc400de]
- [model] Add new audit change variant for `image_hash` key ([@nickelc]) [c:985212a]
- [misc] Add `.git-blame-ignore-revs` file ([@nickelc]) [c:3a3e399]
- [builder] Add function to `CreateInteractionResponseData` to set or unset the ephemeral flag ([@pascalharp]) [c:283af98]
- [model] Add method to create `model::Timestamp` from a UNIX timestamp ([@nickelc]) [c:66f8be7]
- [model] Add new message flag `FAILED_TO_MENTION_SOME_ROLES_IN_THREAD` ([@nickelc]) [c:a75ac69]
- [model] Add missing stuff for `GatewayIntents` ([@vaporox]) [c:14fc9c6]
- [model] Add shorthand for checking the `GUILD_MESSAGES` intent bitflag ([@Bloectasy]) [c:6f28486]
- [model] Add `Webhook::get_message` for retrieving messages sent by a webhook ([@GnomedDev]) [c:a2b388f]
- [builder/model] Enable compilation of builder/interaction code without a backend ([@FallenWarrior2k]) [c:4cd6caa]
- [http] Support more options for querying invites ([@AnnikaCodes]) [c:c1ef6d9]
- [builder] Add components support to ExecuteWebhook and EditWebhookMessage ([@mkrasnitski]) [c:cb7f211]
- [http/gateway] Add support for v10 of Discord's HTTP API and Gateway ([@squili]) [c:5c6c72a]
- [model] Add `GenericId` snowflake type for unspecific entities in audit logs ([@nickelc]) [c:146b1ac]
- [model] Add support for the `Attachment` option type ([@JellyWX]) [c:9866a85]
- [model] Derive `Copy` and `Clone` traits for the audit action enums ([@nickelc]) [c:dfac5d7]
- [model] Provide parsing of RFC 3339 strings for `model::Timestamp` ([@nickelc]) [c:3aca5ca]
- [builder] Add convenient methods for creating message components ([@nickelc]) [c:6476ed1]
- [model] Derive `Default` trait for all bitflags types ([@nickelc]) [c:92de3da]
- [builder] Add support for `allowed_mentions` in the `ExecuteWebhook` builder  ([@mkrasnitski]) [c:71583c3]
- [model] Add support for `locale` and `guild_locale` fields in interactions ([@Rstar284]) [c:b4d0765]
- [model] Add `data` and `filename` methods to `AttachmentType` ([@mkrasnitski]) [c:ad9e987]
- [builder] Support more integer types for min_value / max_value ([@kangalio]) [c:0da4ab5]
- [model] Add a common constructor for the different `ActivityType`s ([@nickelc]) [c:b4e4add]
- [json] Expose more types and functions from `serde_json` and `simd-json` ([@vicky5124]) [c:f61fae4]
- [client] Add cache update calls to dispatched thread events ([@AngelOnFira]) [c:7e701dc]
- [model] Support application webhooks ([@vicky5124]) [c:2c73618]
- [model] Add `USE_EXTERNAL_STICKERS` permission and audit log support for stickers ([@drklee3]) [c:1fbf1e9]
- [http] Allow sending files when creating and editing followup messages ([@acdenisSK]) [c:147ed03]
- [model/http] Add support for sticker packs, guild stickers, and sticker routes ([@drklee3]) [c:a88673f]
- [http] Add audit log reason support to most HTTP endpoints ([@kangalio]) [c:bec660d]
- [http] Integrate attachments into the `Request` system ([@drklee3]) [c:57582d2]
- [http] Implement optional pagination for `Http::get_guilds` ([@tylerd008]) [c:06d101b]
- [model] Add support for adding attachments in message edits ([@kangalio]) [c:27bf301]
- [framework] Add the name of the command to the `Dispatch` error hook ([@kangalio]) [c:c2130a8]
- [misc] Add support for simd-json via a feature flag ([@Licenser]) [c:afb3c37]
- [model] Add basic support for forum channels ([@kangalio] [@TheBlackfurGuy]) [c:35ee68b]
- [model] Add missing method for converting `Interaction` to `ModalSubmitInteraction` ([@gradiuscypher]) [c:e187f73]
- [builder] Add support to set a single child builder for several parent builders ([@natto1784]) [c:91ee596]
- [collector] Add modal submit collector ([@pascalharp]) [c:e5073ae]
- [client] Add a method to pass an `EventHandler` in an `Arc` directly to the `Client` ([@squili]) [c:16c2089]
- [model] Add Modal Interactions and Input Text ([@pascalharp]) [c:92fe5bb]
- [builder] Add `set_components` to `EditMessage` ([@Chronophylos]) [c:57c0826]
- [model] Add `message_id` field to audit log options for pinned/unpinned messages ([@nickelc]) [c:3a64da1]
- [model] Add `add_member` to `Guild` and `GuildId` ([@vaporox]) [c:82c2415]
- [builder] Add support for `allowed_mentions` in `EditMessage` ([@acdenisSK]) [c:86dbaee]

### Changed

- [utils] Use `str::split_once` for parsing webhooks and user tags ([@mkrasnitski]) [c:fbfe0cf]
- [utils] Refactor and fix bugs in `utils::content_safe` ([@mkrasnitski]) [c:32cb31c]
- [model] Move `misc::Mention{able}` into their own `model/mention` module ([@mkrasnitski]) [c:f5458f1]
- [client] Require setting gateway intents explicitly ([@kangalio]) [c:d245e67]
- [model] Update methods and properties of `Permissions` ([@devtomio]) [c:10410a2]
- [http] Pass parsed token to `Ratelimiter::new()` ([@vaporox]) [c:f4f310d]
- [http] Simplify constructor functions for `Http` ([@vaporox]) [c:f8bc937]
- [model] Remove unnecessary `CacheHttp` requirements ([@vaporox]) [c:a36353b]
- [model] Update documentation around webhook components ([@mkrasnitski]) [c:6028072]
- [misc] Format imports with module granularity ([@nickelc]) [c:4c97810]
- [gateway] Set `Http::application_id` in ready event handler ([@vaporox]) [c:c5f9cbe]
- [misc] Switch to using URL over URI ([@vaporox]) [c:81a9bc2]
- [model] Deserialize the interaction data's `target_id` unconditionally ([@nickelc]) [c:c5dc80b]
- [model] Reduce size of the `ResolvedTarget` enum variants ([@nickelc]) [c:404ab03]
- [model] Change matches where the match expression is a bool to be more idiomatic ([@nickelc]) [c:977946f]
- [cache] Place temporary caching functionality under a separate feature flag ([@acdenisSK]) [c:d03809f]
- [misc] Make chrono a default optional feature ([@xfix]) [c:e69bcd3]
- [model] Clean up macro usage in `model::misc` ([@mkrasnitski]) [c:5a0e8f4]
- [model] Alter `Guild::member_named`'s implementation to use `utils::parse_user_tag` ([@mkrasnitski]) [c:8c9670f]
- [utils] Make `ArgumentConvert` compatible without the `cache` feature ([@kangalio]) [c:cdaa70c]
- [model] Document that the `MESSAGE_CONTENT` intent is now privileged ([@kangalio]) [c:63a1000]
- [client] Use `Option` in more places in the `ClientBuilder` ([@vaporox]) [c:8bca94a]
- [misc] Simplify the hidden preparation code in doc examples ([@nickelc]) [c:d67a21d]
- [model] Remove kind check for sending webhook components ([@GnomedDev]) [c:a292c2f]
- [model] Remove the `unstable_discord_api` feature guard from the application commands ([@nickelc]) [c:1ce8060]
- [model] Turn the resolved target of the command interaction data into a method ([@nickelc]) [c:0036620]
- [model] Clean up API around Mention/Mentionable ([@mkrasnitski]) [c:28e6732]
- [examples] Use the new builder method for ephemeral messages in example ([@nickelc]) [c:aada1b9]
- [utils] Replace custom trait with `Into<Content>` ([@mkrasnitski]) [c:c9e4f27]
- [model] Make the non-cache version of the `shard_id` methods synchronous ([@nickelc]) [c:787e7d9]
- [model] Remove manual deserialization of `ApplicationCommandInteractionDataResolved` ([@nickelc]) [c:56cab22]
- [ci] Pin the Rust version for the build jobs with the nightly toolchain ([@nickelc]) [c:36eac42]
- [model] Use `ApplicationFlags` bitflags in the application info structs ([@nickelc]) [c:33d402f]
- [misc] Unify `std::fmt` imports and `Display` & `Debug` trait implementations ([@nickelc]) [c:7a8f2cf]
- [misc] Update `parking_lot` to 0.12 ([@xfix]) [c:d5b5970]
- [misc] Update `async-tungstenite` to 0.17 ([@nickelc]) [c:4ff348e]
- [misc] Increase MSRV to 1.53 ([@nickelc]) [c:87bb9a8]
- [misc] Put `tokio::task::Builder` usage behind a feature flag ([@Milo123459]) [c:9624af0]
- [cache] Temporarily cache users and channels in the `to_user` and `to_channel` functions ([@Milo123459]) [c:1be625b]
- [model] Rename the enum sub types of the audit log actions ([@nickelc]) [c:56c40fc]
- [model] Make the `model::guild::audit_log` module public ([@nickelc]) [c:7d99bcb]
- [collector] Make collector builders synchronous ([@vaporox]) [c:d640238]
- [model] Change the data mapping of `AuditLogs` ([@nickelc]) [c:7823b6e]
- [model] Move the audit log serde modules into a separate `utils` module ([@nickelc]) [c:5a4fd2a]
- [model] Change the audit log `Change` struct into an enum with the actual types ([@nickelc]) [c:594a00d]
- [model] Rework the audit log serde module for optional integer strings ([@nickelc]) [c:74bffd0]
- [misc] Update `dashmap` to v5.1.0 ([@acdenisSK] [@vicky5124] [@Some-Dood]) [c:6298f67] [c:342fdbb] [c:28e0311]
- [model] Replace deserialization helper functions with the required types ([@nickelc]) [c:938936e]
- [model] Implement `serde` traits for the audit `Action` enum directly ([@nickelc]) [c:0f77d31]
- [http] Refactor `HttpBuilder` to avoid `unwrap` calls ([@vaporox]) [c:89499b2]
- [utils] Move token parse and validate functions into the `utils::token` module ([@nickelc]) [c:97ea22f]
- [model] Make certain embed fields optional  ([@Chronophylos]) [c:daa9434]
- [utils] Move `utils::content_safe` and dependants into its own module ([@nickelc]) [c:ab4e7fe]
- [model] Replace manual deserialization of `VoiceState` with helper structs ([@nickelc]) [c:14e747d]
- [misc] Move macOS job out of the build job matrix ([@nickelc]) [c:23ed9ea]
- [builder] Avoid `Vec` allocation by using the iterators directly ([@nickelc]) [c:4556594]
- [builder] Change `emoij` parameters in component builders to `Into<ReactionType>` ([@nickelc]) [c:8fe7127]
- [model] Deserialize Discord's status page responses with an interim struct ([@nickelc]) [c:a29e0f7]
- [framework] Replace levenshtein implementation with the `levenshtein` crate ([@nickelc]) [c:5f6ad2a]
- [framework] Use subslice patterns to improve code readablility ([@nickelc]) [c:08b7163]
- [model] Use the `is_false` function from the `model::utils` module ([@nickelc]) [c:5b43bdb]
- [model] Use the internal `bitflags!` macro for `ApplicationFlags` ([@nickelc]) [c:dd5661c]
- [model] Replace the use of `DateTime<Utc>` with a newtype struct `Timestamp` ([@nickelc]) [c:1577efc]
- [model] Introduce internal `bitflags!` macro with serde implementation ([@nickelc]) [c:4d5f4f1]
- [misc] Decouple the minimal versions check from the `nightly` build job ([@nickelc]) [c:fbba88c]
- [http] Change the return type of `Route::stage_instances` to `&'static str` ([@nickelc]) [c:a3dd38e]
- [misc] Improve usage of the `api!` macro ([@nickelc]) [c:a95ac81]
- [cache] Move the implementations of `CacheUpdate` into the `cache::event` module ([@nickelc]) [c:afba7f5]
- [model] Use the `bitflags!` macro instead of the undocumented macro ([@nickelc]) [c:701d759]
- [model] Replace `GuildStatus` with `UnavailableGuild` in the ready event ([@nickelc]) [c:5063de1]
- [model] Deserialize `Permissions` without an extra `String` allocation ([@nickelc]) [c:f26fb7f]
- [model] Separate `Webhook::edit` into three separate methods ([@mkrasnitski]) [c:1715476]
- [model] Replace manual deserialization of `RoleTags` ([@nickelc]) [c:363a311]
- [model] Replace (de)serialization implementation of bitflags with a macro ([@nickelc]) [c:bdf99b9]
- [model] Replace manual (de)serialization for single field gateway events ([@nickelc]) [c:599efc1]
- [model] Move `GuildWelcomeScreen` and dependant types into a separate module ([@nickelc]) [c:bc46711]
- [misc] Improve usage of the `cdn!` macro ([@nickelc]) [c:9a2cce8]
- [http] Remove manual deserialization of `DiscordJsonError` ([@nickelc]) [c:080f5b0]
- [model] Move `GatewayIntents` to `model` ([@Gentoli]) [c:f46bbc8]
- [http] Change methods on `Http` to take `&JsonMap` instead of `JsonMap` ([@mkrasnitski]) [c:b1389aa]
- [misc] Replace the `actions-rs` GitHub actions with equivalent commands ([@nickelc]) [c:48de0e5]
- [misc] Allow `clippy::unwrap_used` for tests globally ([@nickelc]) [c:f9f6079]
- [misc] Speed up the `Examples` build job by just checking instead of building ([@nickelc]) [c:34881b9]
- [model] Refactor `GuildWelcomeChannel` and dependant types ([@nickelc]) [c:dfe38c2]
- [utils] Move the `hashmap_to_json_map` function into the `json` module ([@nickelc]) [c:34c484c]
- [model] Move the event deserialization of `Role` creation and update into a helper function ([@nickelc]) [c:25cdfeb]
- [client] Streamline `EventHandler` by removing redundant `guild_id` parameters ([@nickelc]) [c:a5f9a4f]
- [model] Deserialize the `GuildMembersChunkEvent` struct manually ([@nickelc]) [c:ae93570]
- [model] Move `User`'s discriminator de-/serialization into the `model::user` module ([@nickelc]) [c:178f62c]
- [model] Move Discord's snowflake de-/serialization into the `model::id` module ([@nickelc]) [c:68d5049]
- [model] Use serde's primitive integer deserializers for Discord's integer bitflags ([@nickelc]) [c:7a38ad1]
- [model] Use type inference in serde functions ([@nickelc]) [c:d504bf2]
- [misc] Change `match`es where the match expression is a bool to be more idiomatic ([@nickelc]) [c:cb5d090]
- [model] Reuse the `HashMap` values serialization in the other serde modules ([@nickelc]) [c:5ce8177]
- [model] Add serde visitor to deserialize sequences directly as hashmaps. ([@nickelc]) [c:e57433f]
- [model] Move serde functions into modules to be used with `#[serde(with = "...")]` ([@nickelc]) [c:173c1a6]
- [misc] Increase MSRV to 1.51 ([@nickelc]) [c:590437b]
- [http] Convert `HttpBuilder` to be synchronous ([@Some-Dood]) [c:fa68bb1]
- [examples] Update dependencies in examples and the library ([@vicky5124]) [c:228ee8e]
- [model] Rename the `category_id` field of channel types to `parent_id` ([@AldanTanneo]) [c:cf1a897]
- [model] Rename `ApplicationCommand` message type to `ChatInputCommand` ([@HarmoGlace]) [c:bc9315c]
- [cache] Make the cache synchronous and switch to `DashMap` in its internals ([@vicky5124]) [c:9e2b9df]
- [model] Add fallback to HTTP for `Message::channel` ([@kangalio]) [c:c889c7e]
- [framework] Change return signatures of help commands to return `Result`s ([@acdenisSK]) [c:a8cd62d]
- [model] Make `Embed::colour` an `Option` ([@drklee3]) [c:5223ea0]
- [model] Make `RichInvite::inviter` optional and replace `InviteUser` with `User` ([@acdenisSK]) [c:82e5095]
- [misc] Update to `reqwest` v0.11.7 and `async-tungstenite` v0.16 ([@nickelc]) [c:c80084c] [c:d1a3ded] [c:a168913]

### Fixed

- [misc] Fix minor issues with doc comments & examples ([@nickelc]) [c:d3ae156]
- [misc] Fix compilation error ([@nickelc]) [c:1dbe9f1]
- [misc] Fix clippy warnings suggested by `clippy::pedantic` ([@nickelc]) [c:f6a3700]
- [model] Fix parsing interactions with DM channel argument ([@kangalio]) [c:f6c4db0]
- [misc] Fix `unused_imports` warning ([@nickelc]) [c:a9d1919]
- [ci] Fix typo in environment variable in `Publish docs` build job ([@nickelc]) [c:11fff2f]
- [http] Fix docstring of `Route::ChannelsIdMessagesId` ([@mkrasnitski]) [c:1a9bb54]
- [misc] Fix usage of builder pattern across code, docs, and examples ([@mkrasnitski]) [c:36e0909]
- [builder] Clarify documentation for using components with webhooks ([@mkrasnitski]) [c:87f5bae]
- [model] Fix the mapping of the `Change::Type` variant ([@nickelc]) [c:3f93af8]
- [model] Fix serialization of the role/user ID in `PermissionOverwrite` ([@nickelc]) [c:1f13d70]
- [model] Fix broken resolving logic for `AttachmentType::Image` ([@mkrasnitski]) [c:0632019]
- [model] Fix missing `communication_disabled_until` data in member update & chunk events ([@nickelc]) [c:bb638c0]
- [misc] Fix & deny intra-doc links to private items ([@nickelc]) [c:86a5861]
- [misc] Fix unused import & function warnings for certain feature combinations ([@nickelc]) [c:6139a63]
- [misc] Fix clippy warnings ([@nickelc] [@acdenisSK]) [c:f724aec] [c:56a9fcb] [c:508864d] [c:2060cfb] [c:111f437]
- [model] Fix lifetime definitions for builders with attachments ([@acdenisSK]) [c:65c0122]
- [misc] Update `JOIN_MESSAGES` constant to be up to date ([@acdenisSK]) [c:6219fec]
- [examples] Fix the `e09_create_message_builder` example when the `time` feature is enabled ([@nickelc]) [c:74f4396]
- [model] Fix ID type deserialization from signed integers ([@nickelc]) [c:ba84c77]
- [misc] Allow feature combination `builder,unstable_discord_api` ([@Gentoli]) [c:eda64a6]
- [model] Fix faulty (de)serialization of `ActivityFlags` ([@nickelc]) [c:1725627]
- [model] Fix faulty (de)serialization of `MessageFlags` and missing constants ([@nickelc]) [c:1a0cc66]
- [model] Fix the `EmojiId` serialization in the `ReactionType` serialization ([@nickelc]) [c:5114d80]
- [model] Replace some incorrect usage of `UserId` with `ApplicationId` ([@kangalio]) [c:4b2c4d9]
- [client] Update the cache when a thread is created, updated, or removed ([@xMAC94x]) [c:c95bcd3]
- [model] Fix serialisation and deserialisation of `Integration::synced_at` ([@vicky5124]) [c:983d0e4]
- [model] Fix deserialisation of `PresenceUser`'s `discriminator` field ([@acdenisSK]) [c:584d3a9]
- [model] Return a proper error type for `EmojiIdentifier`'s `FromStr` impl ([@kangalio]) [c:faebd0c]
- [model] Fix `Message::edit`'s lifetimes ([@acdenisSK]) [c:22a3f64]
- [model] Correctly cache guild categories on add/delete/update ([@drklee3]) [c:3b2b18c]
- [model] Ensure `Activity::url` is a valid URL at a type-level ([@hybras]) [c:3f985b5]
- [model] Alter deserialisation of `Presence::user` to properly handle optional fields ([@kangalio]) [c:a3a861c]
- [misc] Fix common spelling mistakes using `codespell` ([@acdenisSK]) [c:4eee51c]
- [misc] Fix typo (`titel` -> `title`) ([@acdenisSK]) [c:519ea83]
- [model] Update `communication_disabled_until` field if cached member exists ([@drklee3]) [c:7a8adfb]
- [model] Fix mention syntax of guild emojis ([@pascalharp]) [c:48ca506]

### Removed

- [misc] Remove the `Error::Num` enum variant ([@nickelc]) [c:2596927]
- [any] Remove redundant `ClientBuilder::application_id` field ([@nickelc]) [c:d7ef273]
- [misc] Remove the `Error::Rustls` enum variant ([@nickelc]) [c:8d3a079]
- [framework] Remove unused `found_group_prefix` parameter ([@vaporox]) [c:e283b51]
- [misc] Remove unused `update_cache_timeout` from `CacheAndHttp` ([@vaporox]) [c:c2b9445]
- [misc] Remove `*tls_backend_marker` features ([@vaporox]) [c:16661a3]
- [client] Remove unused `client::Extras` struct ([@nickelc]) [c:1ed7ac5]
- [model] Remove sunsetted store channel type ([@nickelc]) [c:1976428] [c:8b21193]
- [misc] Remove superfluous error heading ([@nickelc]) [c:c92acfd]
- [model] Remove unused `ApplicationInfo` & `BotApplication` structs ([@nickelc]) [c:1e427c6]
- [misc] Remove `tokio` 0.2 compatibility ([@xfix]) [c:04a57c3]
- [misc] Remove usage of `#[doc(cfg(...))]` ([@acdenisSK]) [c:602bcb6]
- [model] Remove unused `model::guild::Target` enum ([@nickelc]) [c:b767215]
- [model] Remove manual deserialization of `AuditLogs` ([@nickelc]) [c:16dcc69]
- [misc] Remove deprecated `Error::description` implementation ([@nickelc]) [c:63cdeec]
- [http] Remove usage of `Arc` for `reqwest::Client` ([@vaporox]) [c:a1cdd7f]
- [model] Remove the unused `Component` enum ([@nickelc]) [c:1e11fb4]
- [misc] Remove deprecated fields, gateway events, endpoints and traits ([@nickelc]) [c:75062ae]
- [model] Remove workaround for serde_json's `arbitrary_precision` feature ([@nickelc]) [c:bc77218]
- [misc] Remove superfluous link reference definitions ([@nickelc]) [c:0bb6bb1] [c:d51b0b1]
- [model] Remove the manual deserialization of `Activity` ([@nickelc]) [c:4e7c60a]
- [model] Remove obsolete `#[cfg(feature = "utils")]` attributes on model methods ([@nickelc]) [c:0675fd1]
- [model] Remove unused `NeverFails` error type ([@nickelc]) [c:5774572]
- [client] Remove useless lifetime from `ClientBuilder` ([@kangalio]) [c:e6039c4]
- [model] Remove the `VoiceRegion::vip` field ([@almeidx]) [c:e748deb]
- [gateway] Remove `Shard::is_shutdown` ([@rasm47]) [c:0ccb793]
- [framework] Remove `Box` indirection on `framework`. ([@Lakelezz]) [c:2f48379]
- [model] Remove invalid `GuildChannel::send_message` note ([@MathyouMB]) [c:73ff5ba]
- [examples] Remove outdated reference to `kankyo::load()` in example 06 ([@ben-brook]) [c:db101fd]

## [0.10.10] - 2022-01-22

Thanks to the following for their contributions:

- [@acdenisSK]
- [@AldanTanneo]
- [@almeidx]
- [@Atakku]
- [@drklee3]
- [@Friz64]
- [@HarmoGlace]
- [@JellyWX]
- [@JohnTheCoolingFan]
- [@KaDiWa4]
- [@kafinsalim]
- [@kangalio]
- [@kristopherbullinger]
- [@lhjt]
- [@lo48576]
- [@mbenoukaiss]
- [@mdonoughe]
- [@Milo123459]
- [@mkrasnitski]
- [@nickelc]
- [@OverHash]
- [@pascalharp]
- [@Splingush]
- [@squili]
- [@Th3-M4jor]
- [@Vaimer9]
- [@vicky5124]
- [@woongzeyi]

### Added

- [misc] Implement missing fields, methods, endpoints, and other values that are documented ([@vicky5124]) [c:be5cb54]
- [builder] Add a `timeout` doc alias to various methods ([@Milo123459]) [c:1d05a1e]
- [builder] Add support for enabling slowmode on thread creation ([@nickelc]) [c:69896e0]
- [ci] Add GitHub Actions workflow for labeling pull requests ([@nickelc]) [c:93b66d0]
- [model] Add `banner` and `accent_color` fields to `User` ([@drklee3]) [c:9e10d54]
- [client] Add `get_*` methods for many `ClientBuilder` fields ([@kangalio]) [c:90dae53]
- [model] Add support for member timeout ([@kristopherbullinger]) [c:944cd54]
- [utils] Add `get_webhook_from_url` ([@mkrasnitski]) [c:b026f00] [c:6bd2f74] [c:7b89775]
- [examples] Add an example for message components ([@pascalharp]) [c:ee00e92]
- [examples] Add step for installing `cargo make` to the Running Examples section ([@kafinsalim]) [c:47a297b]
- [model] Add support for `min_value` and `max_value` to slash command options ([@kangalio]) [c:cfd518e]
- [model] Add `SUPPRESS_JOIN_NOTIFICATION_REPLIES` system channel flag ([@almeidx]) [c:6121fd0]
- [model] Add support for Autocomplete interactions ([@Th3-M4jor]) [c:c322657]
- [misc] Format code in documentation comments ([@acdenisSK]) [c:5b84896]
- [client] Add a method to `ClientBuilder` for retrieving the current token ([@kangalio]) [c:6f87d25]
- [utils] Add Discord's new branding, CSS & role colours ([@nickelc]) [c:c0463c0]
- [misc] Add `poise` to the "Projects extending Serenity" section ([@vicky5124]) [c:bfb5f13]
- [model] Add channel types to application command options ([@vicky5124]) [c:5a700f7]
- [model] Add new audit log models for `STAGE_INSTANCE`, `STICKER`, and `THREAD` ([@woongzeyi]) [c:0bc3ce0]
- [model] Add `defer` helper methods for deferring interactions ([@Milo123459]) [c:d50628e]
- [examples] Add a database example using SQLite ([@kangalio]) [c:02a5e15]
- [model] Add support for editing threads ([@lhjt]) [c:afeb76e]
- [ci] Use a better cache designed for Rust software in CI ([@Milo123459]) [c:c74162c]
- [examples] Add a simple web dashboard example ([@vicky5124]) [c:6c5aa4d]
- [model] Add new invite fields ([@vicky5124]) [c:cf4e216]
- [model] Add `ApplicationCommand::version` field  ([@HarmoGlace]) [c:c85d44a]
- [http] Add the guild active threads endpoint ([@HarmoGlace]) [c:93d6ab8]
- [builder] Implement `From<Datetime>` for `Timestamp` ([@drklee3]) [c:b905554]
- [command_attr] Add undocumented `#[required_permissions(perms)]` requirements ([@woongzeyi]) [c:f9de6c5]
- [http] Add paths to HTTP errors ([@Splingush]) [c:2da46a8]
- [builder] Add a `set_component` method to respective builders ([@JellyWX]) [c:99eba2b]
- [model] Support context menus ([@HarmoGlace]) [c:e87293e]
- [model] Add `ContextMenuCommand` message type ([@HarmoGlace]) [c:2a571d3]

### Changed

- [model] Mark the `model::guild::GuildStatus` enum as deprecated ([@nickelc]) [c:bd1342e]
- [model] Suppress deprecation warning for the use of `Region` ([@nickelc]) [c:2dcb9b3]
- [builder] Deprecate `slow_mode_rate`/`rate_limit` methods and field ([@nickelc]) [c:5c1e31e]
- [builder] Update the notice about the max value for the "per user" ratelimit ([@nickelc]) [c:32f4430]
- [model] Mark the `model::guild::Region` enum as deprecated ([@nickelc]) [c:6d476cf]
- [model] Improve the download of `Attachment`s by calling `Bytes::to_vec` ([@nickelc]) [c:f630f32]
- [examples] Refactor examples 03 and 13 to better use the builder pattern ([@mkrasnitski]) [c:34c2f22]
- [examples] Refactor example 09 to more idiomatically use the builder pattern ([@mkrasnitski]) [c:4f7dad7]
- [misc] Increase MSRV to 1.51 ([@acdenisSK]) [c:04fc541]
- [model] Use the `write_all` variant in the `Attachment::download` example ([@acdenisSK]) [c:a8b4e2f]
- [misc] Use `Formatter::write_str` directly instead of `writeln` for string literals ([@nickelc]) [c:6a0106c]
- [model] Use the permission flags for the `permissions::PRESET_*` constants ([@nickelc]) [c:fc7c76a]
- [model] Use bitshift notation for the values of Discord's bit flags ([@nickelc]) [c:57a2e4d]
- [model] Make the `u64` value of `AttachmentId`s public ([@nickelc]) [c:5379d3e]
- [model] Simplify the `*Id::created_at` method by using `Utc.timestamp_millis` ([@nickelc]) [c:1977e9f]
- [misc] Set `rustfmt` edition to match library edition ([@mkrasnitski]) [c:4f2a73b]
- [misc] Remove old version of `time` from dependency tree ([@KaDiWa4]) [c:d9c8980]
- [framework] Change `Configuration::prefix`  to accept types that implement `ToString` ([@mkrasnitski]) [c:738465f]
- [gateway] Remove guilds from cache if they were removed between a disconnect and a ready ([@squili]) [c:b21f053]
- [internal] Put the internal `tokio` module behind `tokio*` features ([@nickelc]) [c:921ff6f]
- [examples] Use `ClientBuilder::type_map_insert` in examples where it makes sense ([@nickelc]) [c:c3358cf]
- [examples] Simplify `tracing_subscriber` initialization in the examples ([@nickelc]) [c:46f7608]
- [ci] Specify MSRV for clippy and use stable toolchain for the lint job ([@nickelc]) [c:bade6e5]
- [misc] Replace instances of `tokio::spawn` with `spawn_named` ([@Milo123459] [@acdenisSK]) [c:5f81d4a] [c:247c073] [c:b2197db]
- [voice] Replace some instances of `match` with the `matches!` macro  ([@Milo123459]) [c:6987175]
- [utils] Fall back to `Member` parsing if `User` is not in the cache in `ArgumentConvert` ([@kangalio]) [c:6085aad]
- [model] Define `Sticker::pack_id` as optional ([@Atakku]) [c:cf040cb]
- [model] Guarantee the `MessageComponentInteraction::message` to not be partial ([@HarmoGlace]) [c:f915fee]

### Fixed

- [examples] Fix a typo in example 12 ([@Vaimer9]) [c:38b11da]
- [model] Fix documentation typo in `Message::crosspost` ([@OverHash]) [c:b180f21]
- [model] Fix non-idiomatic comment ([@nickelc]) [c:f70112d]
- [model] Fix panic in `ReactionType` deserialization ([@nickelc]) [c:fe946df]
- [model] Fix faulty (de)serialization of `InteractionApplicationCommandDataFlags` ([@nickelc]) [c:62b9c84]
- [model] Fix the example for `permissions::PRESET_GENERAL` ([@nickelc]) [c:707697e]
- [command_attr] Fix `#[help]` to generate its variables without an extra underscore ([@JohnTheCoolingFan]) [c:df47df1]
- [model] Fix `Permissions` bitflags ([@AldanTanneo]) [c:0acabdf]
- [misc] Make the library compilable with minimal dependency versions ([@lo48576]) [c:37e33bd]
- [model] Perform owner check when calculating permissions for a member ([@acdenisSK]) [c:ccf0d5b]
- [misc] Clean up Clippy errors ([@mdonoughe]) [c:2eeb336]
- [framework] Handle Unicode double-quote characters when parsing arguments ([@acdenisSK]) [c:9bfc1e7]
- [builder] Fix permission overwrites by sending Discord the correct overwrite types ([@woongzeyi]) [c:f36b4c1]
- [misc] Fix compilation without default features ([@nickelc]) [c:0611e2c]
- [framework] Check for discrepancies in default commands of groups ([@acdenisSK]) [c:cfbb7b3]
- [misc] Apply fixes by clippy ([@Milo123459]) [c:7e1a64e]
- [command_attr] Fix support for raw identifiers in `command_attr`'s macros ([@Friz64]) [c:5b7a23c]
- [utils] Use the correct method for formatting an emoji in `MessageBuilder` documentation ([@mbenoukaiss]) [c:eb29760]
- [model] Make `CreateApplicationCommandPermissions` public and fix documentation links ([@squili]) [c:7392f3f]

### Removed

- [model] Remove unused `#[serde(deserialize_with = "..")]` attribute ([@nickelc]) [c:e848af8]
- [misc] Remove `#[tokio::test]` from synchronous tests ([@nickelc]) [c:0d1c88e]
- [utils] Remove unused function names from the `colour!` macro ([@nickelc]) [c:096d1da]

## [0.10.9] - 2021-08-15

Thanks to the following for their contributions:

- [@acdenisSK]
- [@adumbidiot]
- [@Dinnerbone]
- [@drklee3]
- [@HarmoGlace]
- [@kangalio]
- [@KangarooCoder]
- [@kotx]
- [@MelonShooter]
- [@rasm47]
- [@sbrocket]
- [@Splingush]
- [@squili]
- [@vicky5124]

### Added

- [model] Add model methods for creating public and private threads ([@acdenisSK]) [c:dbf4d68]
- [client/cache] Add support for defining cache settings in the `Client` builder. ([@Splingush]) [c:ea8ec29]
- [builder] Add 'number' type to `ApplicationCommandOption` ([@Splingush]) [c:312ae16]
- [collector] Add a generic collector for events ([@sbrocket]) [c:3117f1d]
- [model] Mention the type of event that failed to deserialise ([@acdenisSK]) [c:1d446fe]
- [model] Add methods to `Event` to get related user, guild, channel, and message IDs ([@sbrocket]) [c:0aa1a6c]
- [model] Derive `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` for `UserPublicFlags` ([@KangarooCoder]) [c:e7ba5bf]
- [cache] Add method `Cache::channel_messages_field` ([@kangalio]) [c:ca02662]
- [http/builder/model] Add support for generation of invite links with custom OAuth2 scopes ([@vicky5124]) [c:50cd285]
- [model] Switch to API v9 and add support for threads ([@HarmoGlace]) [c:4c53b48]
- [model] Add field `Guild::stage_instances` ([@HarmoGlace]) [c:13cf056]
- [model] Support stage instances ([@HarmoGlace]) [c:3e6d131]
- [model] Add support for guild search members endpoint ([@drklee3]) [c:09b523d]
- [model] Add avatar field to member update event ([@drklee3]) [c:4f488f8]
- [http] Add support for fetching the current user's connections ([@Dinnerbone]) [c:6a400f9]
- [http] Allow passing bearer tokens to `Http::new_with_token` ([@squili]) [c:537753b]
- [model] Add support for per guild avatars ([@drklee3]) [c:eb09d13]
- [builder] Support sending multiple embeds in a message ([@drklee3]) [c:e215c9d]
- [model] Add support to (Partial)Guild::nsfw_level ([@HarmoGlace]) [c:59ab451]

### Changed

- [utils] Redesign the `Parse` trait and add support for most applicable model types ([@kangalio]) [c:eb14984]
- [http] Include Discord's error explanations in `http::Error`'s `Display` implementation  ([@kangalio]) [c:ce97f9e]
- [model] Separate interaction stuctures per kind ([@HarmoGlace]) [c:2b2006c]
- [misc] Move workspace up to repo root, instead of under examples ([@sbrocket]) [c:46ace1f]
- [model] Clarify slash commands implementation ([@HarmoGlace]) [c:ae09e57]
- [builder] Improve implementations of embed methods on `Message` builders ([@kangalio]) [c:0be7d1a]
- [model] Guarantee the `Interaction::user` field ([@HarmoGlace]) [c:9d18334]

### Fixed

- [model] Make `GuildChannel::send_message` work in threads ([@kangalio]) [c:fb203ae]
- [model] Fix broken `GuildChannel` collector functionality ([@kotx]) [c:413e3ef]
- [model] Handle flags in `MessageUpdateEvent` ([@sbrocket]) [c:6192107]
- [builder] Clear the flag when passing `false` to `EditMessage::suppress_embeds` ([@sbrocket]) [c:47b9afb]
- [builder] Fix `cliend_id` typo ([@vicky5124]) [c:5cb9cb9]
- [misc] Apply automatic clippy fixes ([@sbrocket]) [c:988bfcf]
- [model] Default certain select menu fields that might not be always present ([@HarmoGlace]) [c:87564a4]
- [collector] Lazily clone structs only when collector filters are actually matched ([@sbrocket]) [c:695bbef]
- [misc] Replace unnecessary guild cloning with cheaper alternatives ([@MelonShooter]) [c:fb83066]
- [misc] Fix and/or remove broken links in documentation comments ([@rasm47]) [c:504f5a4]
- [model] Skip serializating `Interaction`'s `Option` fields if they are `None` ([@sbrocket]) [c:c21f052]
- [collector] Remove collectors that have closed receivers ([@drklee3]) [c:d3cf53a]
- [model] Update all cached guild fields on guild update ([@drklee3]) [c:7a7e8cc]
- [client/http] Fix validation error in the last step of `parse_token` ([@adumbidiot]) [c:3fba372]
- [builder] Fix `CreateSelectMenuOption`'s `description` method ([@HarmoGlace]) [c:cb30408]
- [builder] Re-export forgotten builder types for select menus ([@acdenisSK]) [c:5d50c56]

## [0.10.8] - 2021-06-12

Thanks to the following for their contributions:

- [@acdenisSK]
- [@drklee3]
- [@Gabriel-Paulucci]
- [@HarmoGlace]
- [@JellyWX]
- [@kangalio]
- [@Lakelezz]
- [@Max2408]
- [@NotNorom]
- [@TehPers]
- [@Unoqwy]
- [@vicky5124]

### Added

- [http] Implement `CacheHttp` for `Http` ([@acdenisSK]) [c:5c9c194]
- [model/builder/collector] Support components (buttons and dropdowns) ([@HarmoGlace]) [c:47a0a14]
- [model] Add support for `Activity` buttons ([@HarmoGlace]) [c:2635f7b]
- [builder] Add `set_embed` to `EditMessage` ([@Gabriel-Paulucci]) [c:073c22f]
- [model] Add a `roles` method on GuildId for retrieving the guild's roles ([@JellyWX]) [c:38a39f5]
- [model] Add new user flag for Discord Certified Moderator ([@Max2408]) [c:c60d504]
- [model] Add a few missing methods for (Partial)Guild ([@NotNorom]) [c:7c50e70]

### Changed

- [client/gateway] Improve documentation of gateway intents ([@Lakelezz]) [c:b03d14b]
- [misc] Update Github/Gitlab username in README and CHANGELOG for vicky5124 ([@vicky5124]) [c:7b95b41]
- [client] Implement stricter `validate_token` checks ([@kangalio]) [c:132fd68]

### Fixed

- [builder] Fix application command permission builder closures ([@HarmoGlace]) [c:1c7e98b]
- [misc] Prevent CI clippy errors ([@Unoqwy]) [c:a788fe2]
- [client] Remove early return when handling collector filters ([@drklee3]) [c:cbfd92b]
- [model] Fix embeds in interaction follow-up messages ([@TehPers]) [c:f7907ed]
- [client] Check existing `application_id` of `Http` when creating a `Client` ([@drklee3]) [c:1744883]
- [model] Fix being unable to mention additional users with `Message::reply(_ping)` ([@kangalio]) [c:22d2276]
- [http] Fix paths in `DiscordJsonError` messages ([@HarmoGlace]) [c:97c01ee]
- [model] Apply the same fix from #1330 for `create_webhook_with_avatar` ([@JellyWX]) [c:d390ca1]

## [0.10.7] - 2021-05-14

Thanks to the following for their contributions:

- [@HarmoGlace]

### Fixed

- [model/http] Fix GuildEmbed removal to avoid breaking change ([@HarmoGlace]) [c:822406e]

## [0.10.6] - 2021-05-13

Thanks to the following for their contributions:

- [@acdenisSK]
- [@drklee3]
- [@HarmoGlace]
- [@kangalio]
- [@Lakelezz]
- [@legendofmiracles]
- [@mTvare6]
- [@nickelc]
- [@Sei4or]
- [@sudomann]
- [@vicky5124]
- [@zacck]

### Added

- [model/gateway/client] Support command create, update, and delete events ([@HarmoGlace]) [c:aa9748e]
- [model] Support the `Team::name` field ([@HarmoGlace]) [c:07f15ad]
- [model] Support getting interaction response, editing/deleting followup and flags ([@HarmoGlace]) [c:59bc14c]
- [model] Support mentionable application command option type ([@HarmoGlace]) [c:a710113]
- [builder] Support editing voice channel region and video quality mode ([@HarmoGlace]) [c:7983a8e]
- [model] Add support for all guild fields (welcome screen, rules channel, nsfw, etc.) ([@HarmoGlace]) [c:9b8a31a]
- [http] Support more detailed HTTP errors from Discord ([@HarmoGlace]) [c:f88355b]
- [model] Support member field on message reaction add event ([@HarmoGlace]) [c:879f193]
- [model] Support all interactions features ([@HarmoGlace]) [c:d6e86df]
- [model] Add role tags support ([@HarmoGlace]) [c:7b25308]
- [http] Add Discord API proxy support ([@drklee3]) [c:badb448]
- [model] Handle voice status (deaf, mute) in the `MemberUpdate` event ([@vicky5124]) [c:3a6eb58]
- [model] Add the `content_type` field to `Attachment` ([@vicky5124]) [c:1b17732]
- [model] Add the invite reminder system message and flag ([@vicky5124]) [c:f238a46]
- [model] Add watching activity ([@vicky5124]) [c:47d9d1c]
- [model] Adds support to message crossposting ([@HarmoGlace]) [c:25ecaa6]
- [builder] Add a space in the example for `CreateEmbed::timestamp` ([@legendofmiracles]) [c:d0b404d]

### Changed

- [model] Use `Permissions` instead of `String` for the `PartialMember::permissions` field ([@HarmoGlace]) [c:3b662e0]
- [misc] Update link to Lavalink to account for the author's name change ([@mTvare6]) [c:7315d78]
- [model] Default to the `Unknown` variant for new model types ([@acdenisSK]) [c:cdbd14b]
- [misc] Introduce intradoc-links in a lot of places ([@kangalio]) [c:b2565da]
- [misc] Update Discord guild badge ([@Lakelezz]) [c:5e5feea]
- [framework] Type check the return type of the body of async functions ([@acdenisSK]) [c:fa0bdd8]
- [examples] Rephrase documentation of `before` for accuracy in the framework example ([@sudomann]) [c:392a534]
- [model] Update the documentation for `Message::mention_channels` with constraints ([@zacck]) [c:8ab0800]

### Fixed

- [model] Fix message link when Discord does not provide `guild_id` ([@HarmoGlace]) [c:01f70db]
- [model] Fix webhook creation on news channel ([@HarmoGlace]) [c:f34c7bb]
- [misc] Fix clippy warnings ([@nickelc]) [c:6f30b0e]
- [http] Only deserialise the `DiscordJsonError::errors` field if present ([@HarmoGlace]) [c:bab4d78]
- [framework] Consider group restrictions when displaying command availability ([@acdenisSK]) [c:d508c35]
- [model] Silence warnings related to the deprecation of `(Partial)Guild::region` ([@acdenisSK]) [c:b995482]
- [framework/client] Avoid cloning messages if the framework feature is disabled ([@acdenisSK]) [c:c7caedd]
- [client] Silence `dead_code` lint for `ClientBuilder::token` ([@acdenisSK]) [c:8e13e74]
- [misc] Remove debug tool ([@HarmoGlace]) [c:cfe5f01]
- [model] Fix misspelling of custom in `ActivityType`'s documentation ([@Sei4or]) [c:dca5c45]

### Removed

- [builder] Remove useless `mut` modifiers in builder code ([@acdenisSK]) [c:9334af5]

## [0.10.5] - 2021-04-04

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Daggy1234]
- [@drklee3]
- [@FelixMcFelix]
- [@kangalio]
- [@Lakelezz]
- [@miqbalrr]

### Added

- [model] Add support for stage channels ([@drklee3]) [c:4018632]
- [http/model/builder] Allow editing and deleting messages from webhooks ([@acdenisSK]) [c:8d0b307]
- [model] Add support for `GuildChannel::{rtc_region, video_quality_mode}` fields  ([@drklee3]) [c:95c2f78]
- [command_attr] Add a way for joining lines in `#[doc]` comments using `\$` ([@acdenisSK]) [c:dbc40cb] [c:cc184a4]
- [model] Add `User::public_flags` field ([@miqbalrr] [@acdenisSK]) [c:b999f46] [c:8ab0305]
- [model] Add a `Parse` trait for some model types ([@kangalio]) [c:3088652]
- [command_attr] Add documentation for the `#[hook]` macro ([@acdenisSK]) [c:412f5a9]
- [model] Add new Emoji fields and adjust existing ones ([@acdenisSK]) [c:8dfd97d]
- [model] Apply changes to interaction response types and add a new interaction type ([@acdenisSK]) [c:eba755c]

### Changed

- [builder] Update `EditGuild` to match current Discord API ([@Lakelezz]) [c:feda47c]
- [misc] Signify code that needs the `collector` feature ([@acdenisSK]) [c:a04291f]
- [framework] Make the `Suggestions::{as_vec, join}` methods public ([@Daggy1234]) [c:4186996]
- [gateway] Alter `Activity` constructor methods to use `ToString` ([@acdenisSK]) [c:859355c]
- [model] Improve conformity and fix updating certain fields for `Member` ([@acdenisSK]) [c:909bf8e]

### Fixed

- [command_attr] Fix a case of iterator invalidation when moving `#[doc]` attributes ([@acdenisSK]) [c:166c248]
- [http] Escape hashes in HTTP reaction methods ([@acdenisSK]) [c:17233a2]
- [gateway] Reconnect in event of failed ShardActions ([@FelixMcFelix]) [c:03dd250]
- [framework] Do not invoke the help command in DMs if `allow_dm` is false ([@acdenisSK]) [c:16e7556]
- [examples] Fix tracing compilation error in example 7 ([@acdenisSK]) [c:3aa6aae]

## [0.10.4] - 2021-03-03

Thanks to the following for their contributions:

- [@acdenisSK]

### Fixed

- [misc] Fix invalid usage of `#[doc(cfg)]` ([@acdenisSK]) [c:b192609]

## [0.10.3] - 2021-03-03

Thanks to the following for their contributions:

- [@AriusX7]
- [@acdenisSK]
- [@aria-7553]
- [@baeuric]
- [@cab404]
- [@Daggy1234]
- [@DrBluefall]
- [@dapper-gh]
- [@drklee3]
- [@drp19]
- [@Lakelezz]
- [@lapin-b]
- [@merlleu]
- [@nickelc]
- [@Prof-Bloodstone]
- [@SinsofSloth]
- [@s0lst1ce]
- [@Th3-M4jor]

### Added

- [framework/command_attr] Rename `#[doc]` attributes to `#[description]` for groups ([@acdenisSK]) [c:1cbc935]
- [model] Add missing variants to `MessageType`  ([@merlleu]) [c:8f38985] [c:e5ae947]
- [model] Add Requiring Guild Presences Intent Note ([@drklee3]) [c:136adef]
- [misc] Add clippy lint to deny let underscore binding of values which must be used ([@Th3-M4jor]) [c:53d5007]
- [misc] Add clippy lint for denying missing error documentation ([@Th3-M4jor]) [c:dcc1ac4]
- [framework] Document help-macro attributes `#[lacking_conditions]` and `#[wrong_channel]` ([@aria-7553]) [c:8931c4c]
- [model] Add new fields for the `Member` model ([@acdenisSK]) [c:810b033]
- [misc] Add a clippy lint for denying the usage of `unwrap()` ([@Th3-M4jor]) [c:3f93c87]
- [model] Add interaction create methods to `Guild`, `GuildId`, `PartialGuild`, and `Interaction` ([@Th3-M4jor]) [c:abfbb9f]
- [builder] Allow adding permissions when editing a channel ([@baeuric]) [c:abe955f]
- [builder] Add builders for creating interactions ([@Th3-M4jor]) [c:0d743fd]
- [client] Allow users to reuse `Http` when building the `Client` ([@acdenisSK]) [c:0e2b648]
- [builder] Add builders for interaction responses ([@SinsofSloth]) [c:835e826]

### Changed

- [model] Ignore Missing Cache Items on Permission Checks ([@Lakelezz]) [c:2812236] [c:75b404a]
- [framework] Capture sub-commands and aliases in `similar_commands` ([@Lakelezz]) [c:5892db2]
- [framework] Handle nested sub-commands in help ([@Lakelezz]) [c:3db32fb]
- [model] Don't require the `cache` feature for `MembersIter` and `GuildId::members_iter` ([@AriusX7]) [c:4ed30c3]
- [misc] Bump dependencies to make them build with minimal versions ([@nickelc]) [c:53b2dc5]
- [misc] Update to `tracing` 0.1.23 ([@nickelc]) [c:2a21347]
- [builder] Use vectors for passing permissions in the builders' documentation ([@acdenisSK]) [c:2e50a12]
- [misc] Update the contributing guide to reflect the state of the project ([@acdenisSK]) [c:7e8620f]
- [model] Change the type of the `embeds` field in `MessageUpdateEvent` back to `Embed` ([@cab404]) [c:8133a43]
- [misc] Enable the `log` feature for `tracing` ([@DrBluefall]) [c:73e6d8a]
- [examples] Make `ShardManagerContainer` in the `e06_sample_bot_structure` example public ([@Daggy1234]) [c:dfd0b32]
- [misc] Format the repository and add a workflow for formatting and linting ([@acdenisSK]) [c:9bbb25a]
- [client] Ignore the `msg` parameter when creating a tracing span ([@acdenisSK]) [c:cb6b528]

### Fixed

- [model] Fix the type of `token` and add field `kind` to `Webhook` ([@acdenisSK]) [c:de72951]
- [framework] Skip the `Available` segment in help if empty. ([@Lakelezz]) [c:6b6e5fe]
- [model] Check if Private Channel Before Returning `NotAuthor` ([@Lakelezz]) [c:1ab52a0]
- [framework/command_attr] Preserve documentation comments for the help command ([@acdenisSK]) [c:83360f8]
- [model] Fix typo on `MessageCollector` ([@s0lst1ce]) [c:291e4f2]
- [model] Fix outdated link to example 5 in `Client::data`'s documentation ([@aria-7553]) [c:2be89e2]
- [framework] Add missing verb 'be' in `Configuration::no_dm_prefix`'s documentation  ([@aria-7553]) [c:4477690]
- [examples] Update comments in the 7th example to mention `tracing` ([@acdenisSK]) [c:10de463]
- [misc] Fix commit links for the v0.9.3 release ([@acdenisSK]) [c:5457cda]
- [gateway] Fix `WebSocketGatewayClientExt::send_chunk_guild` to not get disconnected by Discord's gateway ([@lapin-b]) [c:2f835a3]
- [gateway] Fix token logging from `WebSocketGatewayClientExt::send_resume` ([@Prof-Bloodstone]) [c:2854e6b]
- [builder] Fix a typo in the create interaction builder ([@drp19]) [c:5254e2b]
- [examples] Fix the owner check in the `e05_command_framework` example ([@dapper-gh]) [c:e59314e]


## [0.10.2] - 2021-01-09

Thanks to the following for their contributions:

- [@Lakelezz]
- [@nickelc]

### Added

- [misc] Add compatibility features for tokio `0.2` and `1.0` ([@Lakelezz]) [c:580d6de]

### Fixed

- [misc] Fix doc_cfg attributes for the `unstable_discord_api` feature ([@nickelc]) [c:9a97f9b]

## [0.10.1] - 2021-01-07

Thanks to the following for their contributions:

- [@nickelc]

### Fixed

- [misc] Bump the required version of the `command_attr` crate ([@nickelc]) [c:ab8c82b]

## [0.10.0] - 2021-01-06

Thanks to the following for their contributions:

- [@AgathaSorceress]
- [@acdenisSK]
- [@Baev1]
- [@casey]
- [@chocological00]
- [@Daggy1234]
- [@drklee3]
- [@FelixMcFelix]
- [@ikkerens]
- [@JellyWX]
- [@james7132]
- [@KamranMackey]
- [@Lakelezz]
- [@LavaToaster]
- [@nickelc]
- [@peppizza]
- [@SadiinsoSnowfall]
- [@SinsofSloth]
- [@sam-kirby]
- [@Th3-M4jor]
- [@ThatsNoMoon]
- [@thebongy]
- [@Wolvereness]

### Added

- [client/voice] Document VoiceGatewayManager interface ([@FelixMcFelix]) [c:7647e1e]
- [misc] Describe the `unstable_discord_api` feature ([@acdenisSK]) [c:0b1fc27]
- [gateway] Log unhandled errors when handling events in the gateway ([@LavaToaster]) [c:2e8f968]
- [model/http] Add Slash Command Gateway and REST API Support ([@Baev1]) [c:649f8f2]
- [framework] Add Max Delay Limits, Delay Hook, and `RateLimitInfo` ([@Lakelezz]) [c:aed3886]
- [model] Add partial application information to the Ready event. ([@KamranMackey]) [c:d4caf12]
- [model] Add Default Limit And Validate When Fetching Members ([@SadiinsoSnowfall]) [c:f1204a8]
- [model] Add Guild::banner_url() and support animated guild icons via Guild::icon_url() ([@drklee3]) [c:2ef6da8]
- [model] Add sync_id and session_id fields to Activity. ([@KamranMackey]) [c:6240625]
- [framework] Add Reverting and Awaiting Bucket Rate Limits ([@Lakelezz]) [c:1589475]
- [model] Add support for stickers ([@drklee3]) [c:1b4d408]
- [framework] Support custom colours in the help command attributes ([@acdenisSK]) [c:fb1f0bf]
- [model] Add a helper method for partial comparing unicode reaction emojis ([@Th3-M4jor]) [c:46c74e9]
- [framework] Add summary as a brief description for groups ([@Lakelezz]) [c:ce66f8e]
- [framework] Add support for displaying sub-commands in plaintext help ([@KamranMackey]) [c:977fa19]
- [framework] Implement the `Display` and `Error` traits for the check `Reason` ([@peppizza]) [c:2c9c64f]
- [model] Add a helper method for comparing unicode reaction emojis ([@Th3-M4jor]) [c:0de77e7]
- [framework] Add sub-command support to help ([@Lakelezz]) [c:707752c]
- [model] Add functions to use inlined replies on `Message` ([@Lakelezz]) [c:35c8f7b]
- [model] Implement inline replies ([@ikkerens]) [c:e521849]
- [voice] Implement Songbird driver configuration ([@FelixMcFelix]) [c:4fdbe98]
- [model] Add `guild_id`, `member`, and `self_video` fields to VoiceState model ([@sam-kirby]) [c:7034564]
- [http] Allow sending files with webhooks ([@JellyWX]) [c:99c27fe]
- [model] Add associated constants to `EventType` and conversion methods between `Event` ([@casey]) [c:f3e4a6b]

### Changed

- [model] Change `Mentionable` to return a struct that implements `Display` ([@Wolvereness]) [c:2624170]
- [misc] Update to tokio 1.0 and reqwest 0.11 ([@nickelc]) [c:a27d7bb]
- [model] Point to `User::nick_in` in `Message::author_nick`'s documentation ([@Wolvereness]) [c:51dc943]
- [model] Improve permission checking ([@ThatsNoMoon]) [c:dbb3669]
- [model] Change signatures of `Emoji` methods to require `Cache` and `Http`. ([@Lakelezz]) [c:6aa2841]
- [model] Check @ everyone and user permission overwrites for uncached users ([@ThatsNoMoon]) [c:12d1b22]
- [framework] Rework buckets to offer fine-grained control ([@Lakelezz]) [c:309bd5d]
- [framework] Clarify framework configuration dynamic_prefix docs ([@drklee3]) [c:0ca5813]
- [framework] Remove only one leading space in command descriptions ([@acdenisSK]) [c:216d579]
- [misc] Increase MSRV to 1.48 ([@acdenisSK]) [c:e74fd6f]
- [docs] Replace links with intra-doc links ([@nickelc]) [c:69cae50]
- [framework] Replace `CheckResult` with a simple `Result<(), Reason>` ([@Lakelezz]) [c:6e39727]
- [model] Switch from `u64` to `usize` to represent overflow in a message ([@SadiinsoSnowfall]) [c:b7b0318]
- [example] Make the `ShardManagerContainer` type in example 6 public ([@Daggy1234]) [c:59e09d0]
- [model] Expose guild id in the message delete event handlers ([@SadiinsoSnowfall]) [c:08bd87f]
- [voice] Move songbird and examples relying on it to the new repository ([@acdenisSK]) [c:be6eefe]
- [voice] Update versions for twilight and serenity-voice-model in songbird ([@FelixMcFelix]) [c:b3a71db]
- [model/framework] Retrieve the member from HTTP if missing in cache when calculating permissions ([@JellyWX]) [c:6dd66e1]
- [docs] Update the version in lib.rs to rely on any patch version in 0.9 ([@acdenisSK]) [c:1606e09]
- [misc] Credit the real author of the Serenity.await commit ([@acdenisSK]) [c:2587e6a]
- [voice] Document intents for Songbird ([@FelixMcFelix]) [c:49be2ab]
- [framework] Apply cooked attributes on generated functions ([@acdenisSK]) [c:879ae4b]
- [misc] Bump MSRV to 1.43.0 ([@nickelc]) [c:477925f]
- [model] Change the use of the `#[doc]` attribute on `VerificationLevel` to doc comments ([@nickelc]) [c:2b1e2fc]
- [gateway] Use the info level when notifying the user that a shard is running ([@acdenisSK]) [c:49ebbd7]
- [client/gateway/http/model] Introduce initial changes for v8 of the gateway ([@james7132]) [c:d74a0a6]
- [voice] Voice Rework -- Events, Track Queues ([@FelixMcFelix]) [c:fd6f08b]
- [misc] Reduce size of the crate by only including files that are necessary ([@nickelc]) [c:a836180]

### Fixed

- [http] Fix local ratelimit bug ([@chocological00] [@Lakelezz]) [c:3d56a57] [c:4f07798]
- [http] Fix path to the HTTP endpoint of interaction responses ([@SinsofSloth]) [c:fcb1daa]
- [model] Fix deserialization of interactions ([@acdenisSK]) [c:b42591c]
- [model] Fix doc comments for `ReactionCollector` ([@Th3-M4jor]) [c:7684323]
- [model] Fix the stated order of `ChannelId::messages_iter` ([@acdenisSK]) [c:03a1f97]
- [model] Use  `ApplicationId` instead of `u64` in `MessageApplication` (#1158) ([@sam-kirby]) [c:66e7add]
- [model] Use guild splash hash for splash_url() instead of icon hash ([@drklee3]) [c:d3ab6a7]
- [model/http] Handle 200 Response when modifying Members on API v8 ([@drklee3]) [c:8471f5b]
- [framework] Check for discrepancies in prefixless groups ([@acdenisSK]) [c:692e984]
- [framework] Check a sub-command's parent first. ([@Lakelezz]) [c:20043d4]
- [model] Percent encode audit log reasons to prevent missing characters or truncation ([@drklee3]) [c:02916dc]
- [model] Reverse the message buffer before returning messages from it ([@acdenisSK]) [c:e49fae4]
- [misc] Fix clippy warnings ([@nickelc]) [c:a7ab03e] [c:7ad3abe] [c:a42bba2] [c:3282d77]
- [model] Fix voice region deserialization ([@KamranMackey]) [c:ba85299]
- [framework] Avoid cloning the entire guild object upon a command invocation ([@acdenisSK]) [c:ac23e19]
- [framework] Fix typo in check-docs. ([@Lakelezz]) [c:5fea8e4]
- [example] Fix typo of the strikethrough option in the framework example ([@thebongy]) [c:177adbf]
- [model] Fix url for animated emojis in `EmojiIdentifier` ([@AgathaSorceress]) [c:e8ae00f]
- [model] Use the correct bits for privileged intents ([@FelixMcFelix]) [c:2225c8d]
- [voice] Handle Voice close codes, prevent Songbird spinning WS threads ([@FelixMcFelix]) [c:79c506e]
- [model] Fix serialization of `PermissionOverwrite`s ([@acdenisSK]) [c:8fb18e6]
- [model] Correctly set gateway to v8 and fix `Permission` deserialisation ([@FelixMcFelix]) [c:b1bcc67]
- [framework] Allow missing documentation for generated statics and functions ([@acdenisSK]) [c:9cc30c3]

### Removed

- [misc] Remove `#[deprecated]` items. ([@Lakelezz]) [c:aca54ac]
- [client] Remove unused event handlers for groups ([@acdenisSK]) [c:52f9dca]
- [framework/command_attr] Remove unnecessary `extern crate` in `command_attr` ([@acdenisSK]) [c:d6984a8]
- [framework/command_attr] Remove recursion limit in `command_attr` ([@acdenisSK]) [c:4729f26]
- [misc] Remove remaining instances of private `_nonexhausive` fields ([@KamranMackey]) [c:ac66a54]

## [0.9.4] - 2021-01-07

Thanks to the following for their contributions:

- [@acdenisSK]

### Fixed

- [misc] Fixate `command_attr` version to 0.3.2 ([@acdenisSK]) [c:b03df91]

## [0.9.3] - 2020-12-18

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Lakelezz]
- [@SadiinsoSnowfall]

### Added

- [command_attr/framework] Support custom colours in the help command attributes ([@acdenisSK]) [c:1cc66b3]

### Fixed

- [framework] Fix invocation by defaulting ticket limit to 1. ([@Lakelezz]) [c:1047eac]
- [framework] Fix handling sub-commands in the help-system. Adds an example command with sub-command to the command framework example as well. ([@Lakelezz]) [c:fac94f2]
- [framework] Fix not setting `set_time` by default. ([@Lakelezz]) [c:5399d44]
- [framework] Check for discrepancies in prefixless groups ([@acdenisSK]) [c:75feff2]
- [command_attr/framework] Remove only one leading space in command descriptions ([@acdenisSK]) [c:8f5a2e9]
- [model] Reverse the message buffer before returning messages from it ([@acdenisSK]) [c:1dba16a]
- [model] Fix incorrect documentation for the `MessageTooLong` error ([@SadiinsoSnowfall]) [c:8b42790]

## [0.9.2] - 2020-11-28

Thanks to the following for their contributions:

- [@acdenisSK]

### Changed

- [misc] Update the version in lib.rs to rely on any patch version in 0.9 ([@acdenisSK]) [c:85e5722]
- [misc] Credit the real author of the Serenity.await commit ([@acdenisSK]) [c:757e0d6]

### Fixed

- [framework] Avoid cloning the entire guild object upon a command invocation ([@acdenisSK]) [c:bc78991]
- [model] Only add roles that the member does not possess when calling `Member::add_roles` ([@acdenisSK]) [c:e6fd09b]

## [0.9.1] - 2020-11-08

Thanks to the following for their contributions:

- [@acdenisSK]
- [@nickelc]

### Changed

- [gateway] Use the info level when notifying the user that a shard is running ([@acdenisSK]) [c:7ea8098]
- [command_attr] Allow missing documentation for generated statics and functions ([@acdenisSK]) [c:182ee78]
- [model] Change the use of the `#[doc]` attribute on `VerificationLevel` to doc comments ([@nickelc]) [c:58e3d49]
- [misc] Reduce size of the crate by only including files that are necessary ([@nickelc]) [c:a836180]

### Fixed

- [command_attr] Apply cooked attributes on generated functions ([@acdenisSK]) [c:61597d6]

## [0.9.0] - 2020-10-30

Thanks to the following for their contributions:

- [@james7132]
- [@JellyWX]
- [@Qeenon]
- [@vicky5124]

### Added

- [client] Mention the required intents for certain events ([@vicky5124]) [c:c9971b6]
- [gateway] Add support for filtering by user ids when requesting guild chunks ([@james7132]) [c:0d9b821]

### Changed

- [gateway] Change `Shard::chunk_guilds` to `chunk_guild` ([@james7132]) [c:50d9643]
- [framework] Use `~` as a default command prefix ([@Qeenon]) [c:d9e9bf7]
- [utils] Make `utils::shard_id` accept the guild id with `impl Into<u64>` ([@JellyWX]) [c:85b5489]

## [0.9.0-rc.4] - 2020-10-24

Thanks to the following for their contributions:

- [@acdenisSK]

### Changed

- [misc] Increase the type length limit ([@acdenisSK]) [c:7e55a0e]

## [0.9.0-rc.3] - 2020-10-23

Thanks to the following for their contributions:

- [@AriusX7]
- [@acdenisSK]
- [@Baev1]
- [@Flat]
- [@FelixMcFelix]
- [@ikkerens]
- [@leumasme]
- [@MonliH]
- [@nickelc]
- [@Prof-Bloodstone]
- [@peppizza]
- [@Qeenon]
- [@TheElec]
- [@tmcarr]
- [@u5surf]
- [@vicky5124]

### Added

- [misc] Switch to Github Actions ([@acdenisSK]) [c:5bb8342] [c:a1b3c8d]
- [model] Add missing fields to `GuildMembersChunkEvent` and add nonce to `ShardMessenger::chunk_guilds` ([@TheElec]) [c:8c30b40]
- [model] Implement `create_invite` on `ChannelId` ([@acdenisSK]) [c:c0c2137]
- [client] Derive `Clone` for `CacheAndHttp` ([@acdenisSK]) [c:2b66828]
- [examples] Update example 07 to use tracing and dotenv ([@vicky5124]) [c:2386690]
- [examples] Add Parallel Loops example. ([@vicky5124]) [c:b38cdf5]
- [examples] Add an example for client data. ([@vicky5124]) [c:3fe8056]
- [http] Add methods to the http error type to query the presence of variants and to return the status code  ([@Baev1]) [c:7ef12ee]
- [http] Add methods to start and stop typing ([@AriusX7]) [c:bcf8249]

### Changed

- [examples] Gracefully shutdown all shards upon receiving Ctrl-C in example 7 ([@peppizza]) [c:860a2c5]
- [misc] Update and reduce dependencies ([@nickelc]) [c:bba7d55] [c:35e2a40] [c:6577838]
- [examples] Replace several instances of `if let Err` with `?` in example 5 ([@peppizza]) [c:b1187ce]
- [examples] Use the `writeln!` macro instead of `write!` in example 5 ([@peppizza]) [c:c6150ae]
- [http] Prevent discord token from being logged ([@Prof-Bloodstone]) [c:5cc67c3]
- [misc] Replace instances of `super::super` with absolute imports ([@nickelc]) [c:47e2f34]
- [examples] Directly link `TypeMap` to its crate ([@nickelc]) [c:519b67e]
- [builder] Allow setting the embed builders directly ([@Prof-Bloodstone]) [c:b522e83]
- [model] Change order rules of MessagesIter from new to old ([@Qeenon]) [c:aa589da]
- [voice] Limit `tokio/process` and `tokio/udp` to the voice feature ([@nickelc]) [c:d514806]
- [misc] Update `base64` and `typemap_rev` dependencies to their latest versions ([@Qeenon]) [c:ebd1241]
- [examples] Update example 05 to specifically get the current bot id ([@Flat]) [c:de085ff]
- [voice] Update voice cipher to the latest version ([@FelixMcFelix]) [c:87df95f]
- [framework] Change the default help text to use the word 'server' ([@acdenisSK]) [c:f0cfc0e]

### Fixed

- [examples] Fix a typo in the examples' README  ([@leumasme]) [c:186fd6c]
- [examples] Fix a small typo in example 14 ([@MonliH]) [c:3467ddc]
- [model] Deserialize the correct data from the `GUILD_DELETE` event ([@acdenisSK]) [c:b044b6d]
- [model] Correctly update the guild instance in the members chunk event ([@acdenisSK]) [c:42da202]
- [framework] Ignore empty command arguments  ([@acdenisSK]) [c:b9252a5]
- [misc] Fix broken links & related doc comments ([@nickelc]) [c:83e1d12]
- [misc] Fix all clippy warnings and deprecate `Client::new` ([@ikkerens]) [c:ffc2997]
- [examples] Send the shutdown message earlier in the sample bot example ([@tmcarr]) [c:2d365d0]
- [model] Fix `ReactionType`'s `Display` impl to support animated emojis ([@u5surf]) [c:f0a3947]

### Removed

- [model] Remove unnecessary `Option` from `Message::channel_mentions` ([@vicky5124]) [c:626bdfd]

## [0.9.0-rc.2] - 2020-09-27

Thanks to the following for their contributions:

- [@acdenisSK]
- [@bdashore3]
- [@Deebster]
- [@FelixMcFelix]
- [@ikkerens]
- [@LikeLakers2]
- [@vicky5124]

### Added

- [model] Add the new 'competing in' activity type ([@vicky5124]) [c:ebc2cc7]
- [model] Add `create_webhook` methods to `GuildChannel`. ([@vicky5124]) [c:fb44a15]
- [meta] Add cargo-make support for building the examples. ([@vicky5124]) [c:b137f51]
- [model] Add new guild features to the documentation ([@vicky5124]) [c:f2191d5]
- [model/client] Add support for invite events ([@acdenisSK]) [c:7146fd7]

### Changed

- [model] Retrieve the member from HTTP if it is missing in the cache ([@acdenisSK]) [c:a3f9186]
- [framework] Call the `should_fail_common` check earlier ([@acdenisSK]) [c:bab4b24]
- [meta] Update the MSRV to 1.40 ([@acdenisSK]) [c:3f5882c]
- [model] Use `#[non_exhaustive]` where applicable ([@LikeLakers2]) [c:9ee42f1]
- [meta] Switch from log to tracing. ([@vicky5124]) [c:78dd862]
- [utils] Make `parse_emoji` accept animated emojis ([@bdashore3]) [c:66e05ea]
- [meta] Insert the shard manager in example 7 ([@acdenisSK]) [c:7c2d4f7]

### Fixed

- [client] Fix Tungstenite IO error being discarded in ws_impl ([@ikkerens]) [c:49c1603]
- [meta] Fix feature dependencies in examples 2, 3, 4, 9, 11 ([@Deebster]) [c:9e77154]
- [framework] Temporarily fix the permissions check in the framework ([@acdenisSK]) [c:88af7cf]
- [model] Fix ReactionType not remembering animated status. ([@ikkerens]) [c:2eac499]
- [framework] Fix parsing of commands that have requirements ([@acdenisSK]) [c:323875f]
- [meta] Fix example 5's retrieval of owners when the bot belongs to a team ([@vicky5124]) [c:5c0c045]
- [framework] Fix bucket duration order ([@acdenisSK]) [c:5805539]
- [voice] Update Voice WS Sender on reconnect. ([@FelixMcFelix]) [c:659448b]
- [gateway] Fix shutdown regression ([@ikkerens]) [c:fa03b75]

### Removed

- [framework] Do not emit dispatch errors when ignoring bots or webhooks ([@acdenisSK]) [c:41698b6]

## [0.9.0-rc.1] - 2020-09-03

Thanks to the following for their contributions:

- [@acdenisSK]
- [@AriusX7]
- [@ThatsNoMoon]
- [@bdashore3]
- [@qm3ster]
- [@nickelc]
- [@Qeenon]
- [@vicky5124]

### Added

- [misc] Add latency to the general group in the 5th example ([@acdenisSK]) [c:c8dacf2]
- [model] Add `MessagesIter` and `messages_iter`  (#942) ([@AriusX7]) [c:a7e2f74]
- [model] Implement `Display` for `Permissions` (#939) ([@AriusX7]) [c:1eb2714]
- [model/http] Implement endpoints for getting emojis of a guild (#937) ([@acdenisSK]) [c:2a1c385]
- [client] Implement AsRef<ShardMessenger> for ShardMessenger (#936) ([@ThatsNoMoon]) [c:a9c9caa]
- [builder] Implement a builder for allowed mentions (#931) ([@vicky5124]) [c:59fb7b9]

### Changed

- [framework/command_attr] Fix if condition that inspects the name of a check ([@acdenisSK]) [c:7d8c806]
- [framework] Consider `Group` checks when displaying commands in help (#946) ([@AriusX7]) [c:f00f1c0]
- [model] Remove the colon from the message generated by `Message::reply` (#943) ([@Qeenon]) [c:13bf356]
- [model] Remove usage of chrono in a few places and switch from FixedOffset to Utc (#935) ([@qm3ster]) [c:ed7b18e]
- [framework] Abstract CommandResult type (#934) ([@bdashore3]) [c:cfbd0be]
- [misc] Update async-tungstenite to v0.8 (#930) ([@nickelc]) [c:3d44d66]

### Removed

- [misc] Remove unused dependencies ([@acdenisSK]) [c:b414f29]

## [0.9.0-rc.0] - 2020-08-11

# Asynchronous Serenity

It has long been `.await`ed for, but it's finally here! Serenity now leverages `async` functions, thanks to the `tokio` runtime.

A big thanks goes to [@Lakelezz]. Without the dedication to her efforts, it wouldn't have been possible to make the switch easily.

# Release candidate

As that is a huge change, among other changes, this is actually a *testing release*. It is encouraged to consumers of this release to provide feedback for any warts you might encounter, be it missing features, or bugs. One important bug is the OOM bug. After a period of time has passed since the bot connected, there might be a big random spike in memory usage, prompting the OOM killer to shutdown the bot's process. It is unknown where the spike comes from, or when exactly does it occur. It doesn't happen to every bot either. If you find your bot suddenly using a lot of memory, or being killed, please let us know by opening an Issue.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@asherkin]
- [@bdashore3]
- [@DimiDimit]
- [@jmgao]
- [@Lakelezz]
- [@MaxOhn]
- [@mendess]
- [@mjsir911]
- [@mysteriouspants]
- [@NieDzejkob]
- [@NilsIrl]
- [@vicky5124]
- [@Th3-M4jor]
- [@tverghis]

### Added

- [model] Add `nick` field to `PartialMember` ([@asherkin]) [c:28f9f8a]
- [model] Add teams support ([@vicky5124]) [c:5a6979a]
- [model] Add `mentions_current_user` helper ([@mysteriouspants]) [c:e049bc1]
- [model/client] Add the `nonce` field to `GuildMembersChunk` and the event handler ([@mjsir911]) [c:614f99a]
- [voice] Add `self_stream` field to `VoiceState` ([@DimiDimit]) [c:c60f397]
- [model] Add a method to retrieve a webhook's URL ([@vicky5124]) [c:5931d1c]
- [model] Add a method to retrieve a link to a message ([@acdenisSK]) [c:a92ca08]
- [command_attr] Add the delimiters option to the documentation ([@acdenisSK]) [c:1594a3e]
- [model] Add pagination for CurrentUser::guilds ([@NilsIrl]) [c:9eadffa]
- [model] Add methods to disconnect a member from a voice channel ([@mendess]) [c:4e8e40b]
- [model] Add `GuildId::name()` ([@tverghis]) [c:d99bee9]

### Changed

- [misc] Update rustls dependency to 0.18 ([@acdenisSK]) [c:3dda20d]
- [all] Serenity.await ([@Lakelezz]) [c:5df4c6d]
- [misc] Address the renaming of the lavalink project ([@acdenisSK]) [c:802df92]
- [utils] Replace the in-house typemap implementation with `typemap_rev` ([@bdashore3]) [c:c1c7300]
- [misc] Apply copyright to all people who have contributed to Serenity ([@acdenisSK]) [c:287245a]
- [model] Make `Message::reply` take a content implementing `std::fmt::Display` ([@vicky5124]) [c:00683ef]
- [framework] Expose CustomisedHelpData fields, improve help strikethrough message attributes. ([@jmgao]) [c:0d72b6e]
- [framework] Return `Option<Message>` in help command functions ([@MaxOhn]) [c:4472dec]
- [model] Make `Message::react` return its `Reaction` ([@NieDzejkob]) [c:e7eb52d]
- [model] Check the cache for an existing private channel when creating a new one ([@NieDzejkob]) [c:94d22c3]
- [framework] Replace `CommandError` with an alias to `Box<dyn Error>` ([@acdenisSK]) [c:a406d48]
- [misc] Mention projects that extend the functionality of the library instead ([@acdenisSK]) [c:9f75c09]
- [model] Adjust the ban methods to be consistent with the kick methods ([@acdenisSK]) [c:8628b2a]
- [http] Inject the guild id into the roles returned by the http client ([@acdenisSK]) [c:5edd11d]
- [client] Replace `&mut Context` with `&Context` ([@Th3-M4jor]) [c:3e4294b]

### Fixed

- [model] Restrict mention parsing to be specific to the type of id ([@acdenisSK]) [c:dd580fb]
- [framework] Fix `unrecognised_command()` not working if the last added group has prefixes ([@vicky5124]) [c:8040fa5] [c:760ed71] [c:21a5d8e]
- [misc] Remove unnecessary feature gates and fix incorrect ones ([@acdenisSK]) [c:ed61459]

### Removed

- [model] Remove `U: Into<UserId>` type parameter for `partial_guild::edit_member()` ([@vicky5124]) [c:4227a52]
- [misc] Remove Zeyla Hellyer as an author of Serenity ([@acdenisSK]) [c:9cfc410]
- [misc] Remove the matches developmental dependency ([@acdenisSK]) [c:2418edd]
- [misc] Remove usages of mem::replace where its return value is unused ([@acdenisSK]) [c:3135e0f]

## [0.8.9] - 2020-12-18

Thanks to the following for their contributions:

- [@Lakelezz]
- [@SadiinsoSnowfall]

### Fixed

- [framework] Fix invocation by defaulting ticket limit to 1. ([@Lakelezz]) [c:eaead53]
- [framework] Fix handling sub-commands in the help-system. Adds an example command with sub-command to the command framework example as well. ([@Lakelezz]) [c:b96529e]
- [model] Fix incorrect documentation for the `MessageTooLong` error ([@SadiinsoSnowfall]) [c:bb04fad]

## [0.8.8] - 2020-11-28

Thanks to the following for their contributions:

- [@acdenisSK]

### Changed

- [framework] Backport changes in the framework discrepancy check from 0.9.x ([@acdenisSK]) [c:72f287c]

## [0.8.7] - 2020-08-11

This is a small release to publish a bunch of fixes the `current` branch has accumulated.

Thanks to the following for their contributions:

- [@7596ff]
- [@acdenisSK]
- [@dmarcoux]
- [@FelixMcFelix]
- [@fenhl]
- [@jmgao]
- [@johnchildren]
- [@ks129]
- [@s0lst1ce]
- [@SOF3]
- [@tarcieri]
- [@tweirtx]
- [@zack37]

### Changed

- [misc] Use a cargo workspace to compile the examples efficiently ([@acdenisSK]) [c:6b1021f]
- [misc] Prepend 'e' to all example names ([@acdenisSK]) [c:9f848af]
- [misc] Update dependencies rustls, tungstenite, webpki-roots, parking_lot, and base64 ([@johnchildren]) [c:912d759]
- [misc] Update the Discord invite link ([@acdenisSK]) [c:1b195d7]
- [model] Support animated emojis in Emoji's Display impl ([@zack37]) [c:08d0608]
- [voice] Use the `xsalsa20poly1305` crate for encryption in voice ([@tarcieri]) [c:3965e00]
- [model] Consider nickname mentions in `Message::content_safe` ([@ks129]) [c:46db84a]
- [misc] Update almost all instances of discordapp.com to discord.com ([@s0lst1ce]) [c:f98e61e]

### Fixed

- [model] Fix `User::nick_in` unconditionally returning if the cache is passed ([@acdenisSK]) [c:9c198ed]
- [CI] Fix the Linux CI job by adding the missing system dependency ([@tweirtx]) [c:550a63c]
- [misc] Address some clippy warnings ([@dmarcoux]) [c:2a54410]
- [utils] Preserve hostname when escaping invite links ([@fenhl]) [c:7dd01eb]
- [voice] Fix `.mute(...)` not preventing the bot from sending frames. ([@FelixMcFelix]) [c:638780e]
- [nidek] Fix doc-link to the `Message::guild_id` field in the `Message::guild` method ([@SOF3]) [c:84b04ba]
- [framework] Fix check behaviour in the help command ([@jmgao]) [c:e000833]
- [model] Fix the documentation order of `permissions_for_role` and `permissions_for_user` ([@7596ff]) [c:9356a5c]

## [0.8.6] - 2020-04-26

This release prevents a loop when requesting a restart of a Shard, resulting in the Shard hanging forever.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [gateway] Return instantly when requesting restart. ([@Lakelezz]) [c:b8e2a7d]

## [0.8.5] - 2020-04-22

The last bugfix release was ironically broken because of a slight oversight. This release fixes that.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [gateway] Resume on Reconnect events. ([@Lakelezz]) [c:13d5481]

## [0.8.4] - 2020-04-18

This release fixes Serenity's handling of Discord's new rebalances. You should no longer encounter many Ready events.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [gateway] Fix handling rebalances. ([@Lakelezz]) [c:003c8e1]

## [0.8.3] - 2020-04-13

Another bugfix release. This time it fixes a panic that would occur when using a string delimiter and messages contained a unicode character after the delimiter.

Thanks to the following for their contributions:

- [@acdenisSK]

### Fixed

- [framework] Fix a panic when parsing unicode characters after a string delimiter ([@acdenisSK]) [c:7f04ecf] [c:9ebd779]

## [0.8.2] - 2020-04-11

This is a release for a critical bugfix.

In an attempt to fix one thing, other things broke. Any time Serenity made a request to a POST or PUT endpoint (such as creating reactions or banning users), Discord would return HTTP 400. This release corrects that.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [http] Always send the `Content-Length` header ([@Lakelezz]) [c:f5dd8bf]

## [0.8.1] - 2020-04-02

This is the last release for the 0.8.x series. 0.9.x will contain async/await support!

It is mostly comprised of bugfixes and quality of life changes.

### Note

While this is a minor release, a change that is technically breaking has been included. Some `MessageBuilder` methods were forgotten about when changing builders to mutably borrow in 0.8.0. This release fixes that.


Thanks to the following for their contributions:

- [@acdenisSK]
- [@bikeshedder]
- [@Elinvynia]
- [@KamranMackey]
- [@Lakelezz]
- [@MaxOhn]
- [@natsukagami]
- [@vicky5124]
- [@Noituri]
- [@NovusTheory]
- [@TitusEntertainment]
- [@vivianhellyer]

### Added

- [model] Add `guild_id` into the `Reaction` model struct ([@Elinvynia]) [c:820d50e]

- [model] Add missing `guild_id` to various events ([@NovusTheory]) [c:3ca41fd]

- [model] Add support for client statuses ([@KamranMackey]) [c:5f9a27a]

- [http] make error module public ([@vivianhellyer]) [c:d2b19a2]

- [builder] Reexport `Timestamp` from the builder's module. ([@acdenisSK]) [c:3a313c8]

- [model] Implement various `kick_with_reason()` methods ([@vicky5124]) [c:5b0e9f3]

### Changed

- [framework] If finding the argument fails, return to the original position ([@acdenisSK]) [c:e005ef1]

- [framework] Display groups without commands in help. ([@Lakelezz]) [c:d6b0038]

- [model] Make `Member::distinct()` show the discriminator as a 4-digit number ([@natsukagami]) [c:a23acc7]

- [http] Deserialize from slices in `fire` ([@Noituri]) [c:a44f16d]

- [utils] &mut self instead of mut self for MessageBuilder methods ([@MaxOhn]) [c:91f10dd]

- [utils] Implement `Color` type alias to `Colour` ([@Elinvynia]) [c:c3d5264]

- [http] Only set the content type header if there's a body ([@acdenisSK]) [c:d851fea]

- [framework] Store command names in lowercase when case-insensitivity is on ([@acdenisSK]) [c:8bba7b0]

### Fixed

- [misc] Fix release dates ([@bikeshedder]) [c:f27c7c1]

- [framework/docs] Fill in the missing attribute options ([@acdenisSK]) [c:683ff27]

- [http/docs] Fix link to the `fire` method ([@acdenisSK]) [c:1361b33]

- [framework] Fix strikethrough options refusing to accept `name = value` syntax ([@acdenisSK]) [c:581eb2f]

- [framework/docs] Fix a broken link in docs ([@Elinvynia]) [c:48c4b59]

- [misc] Fix a typo in the message builder example ([@TitusEntertainment]) [c:f2d0ad5]

- [framework] Fix `check_in_help` being unaccounted for ([@acdenisSK]) [c:a692bcd]

- [http] Support image URLs in `add_file` ([@Elinvynia]) [c:61bcfbc]

- [http] Add impls for borrowed `Arc`s to `CacheHttp` ([@acdenisSK]) [c:4b67d8e]

### Removed

- [all] Remove usage of deprecated `std::error::Error` functions ([@acdenisSK]) [c:ec306ee]

## [0.8.0] - 2020-01-12

The next major release of Serenity, coupled with improvements, bugfixes and some breaking changes.

### `group!` is now `#[group]`

To stay consistent with the `#[command]`, `#[help]` and `#[check]` macros,
the function-like `group!` procedural macro has also joined in to
the squad of the attribute procedural macros.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@CarlGroth]
- [@Erk-]
- [@ikkerens]
- [@Lakelezz]
- [@LeSeulArtichaut]
- [@Mendess2526]
- [@nickelc]
- [@TheUnitedStatesOfAmerica]
- [@zeyla]

### Added

- [framework] Allow multiple examples in commands ([@Mendess2526]) [c:08d894e]
- [gateway] Add support for receiving custom statuses ([@Erk-]) [c:f897a8d]

### Changed

- [http] Rename 'raw' module to 'client' ([@zeyla]) [c:8326dc7]
- [http] Rework ratelimit structure ([@zeyla]) [c:5dbe078]
- [cache/http] Use `CacheRwLock` in `CacheAndHttp` ([@Lakelezz]) [c:28a91c6]
- [framework] Apply case-insensitivity on prefixes ([@acdenisSK]) [c:b2c951d]
- [framework] Turn off the `owner_privilege` option ([@acdenisSK]) [c:d4b45f4]
- [model] Ensure nullable Discord API values are optional in audit log fields ([@ikkerens]) [c:3a3f9b3]
- [framework] Turn the function-like group macro to an attribute macro ([@acdenisSK]) [c:5b01290]
- [http] Utilise Discord's new precision and reset-after headers ([@acdenisSK]) [c:6916bfc]
- [http] Change `AttachmentType` to use `Cow`s instead ([@ikkerens]) [c:b5deb39]
- [framework] Format the list of attribute options with tables ([@acdenisSK]) [c:3c2f9ad]
- [framework] Impose thread-safety requirements on the event handlers ([@acdenisSK]) [c:3a449ee]
- [framework] Deduplicate Client initialisation code ([@acdenisSK]) [c:e807288] [c:ab7f631]
- [client/gateway] Add an option to turn off guild subscriptions ([@acdenisSK]) [c:db5a09e]
- [framework] Interpret doc-comments as the description attribute ([@acdenisSK]) [c:cc2f918]
- [framework] Account for checks in the help command ([@acdenisSK]) [c:240d3e6]
- [framework] Add documentation to the `check` macro ([@acdenisSK]) [c:0b3ad00]
- [model] Mark the `Event` enum as untagged (serde) ([@CarlGroth]) [c:173f7fa]
- [model] Add new auditlog type enums ([@ikkerens]) [c:aed4b24]
- [framework] Abuse the compiler to do type checking for us ([@acdenisSK]) [c:d6699c2]
- [meta] Upgrade to reqwest v0.10.0 ([@nickelc]) [c:69f2dff]

### Fixed

- [http] Fix crash due to Bearer token not having the 'email' scope ([@LeSeulArtichaut]) [c:ae0acd0]
- [model] Fix `Guild::edit_role_position` example ([@LeSeulArtichaut]) [c:346a7fe]
- [utils] Fix compilation of the `utils` feature without the `model` feature ([@Erk-]) [c:487fa04]

### Removed

- [framework] Get rid of the `Arc` implementation for `Framework` ([@acdenisSK]) [c:05044b6]
- [client] Turn the function-like group macro to an attribute macro ([@acdenisSK]) [c:5b01290]
- [http] Remove april fools functions ([@TheUnitedStatesOfAmerica]) [c:caeeda1]

## [0.7.8] - 2020-04-26

This release prevents a loop when requesting a restart of a Shard, resulting in the Shard hanging forever.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [gateway] Return instantly when requesting restart. ([@Lakelezz]) [c:0959f15]

## [0.7.7] - 2020-04-22

The last bugfix release was ironically broken because of a slight oversight. This release fixes that.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [gateway] Resume on Reconnect events. ([@Lakelezz]) [c:f543b57]

## [0.7.6] - 2020-04-18

This release fixes Serenity's handling of Discord's new rebalances. You should no longer encounter many Ready events.

Thanks to the following for their contributions:

- [@Lakelezz]

### Fixed

- [gateway] Fix handling rebalances. ([@Lakelezz]) [c:f708bec]

## [0.7.5] - 2020-01-13

An emergency release to fix build breakage due to violation of SemVer for the `command_attr` crate.

## [0.7.4] - 2019-12-13

Thanks to the following for their contributions:

- [@acdenisSK]
- [@LeSeulArtichaut]

### Added

- [framework] Enable pub restrictions ([@acdenisSK]) [c:e6ed1b5]
- [framework] Implement Default for CommandOptions and GroupOptions ([@LeSeulArtichaut]) [c:918273b]

### Fixed

- [framework] Fix regression of default option initialisation ([@acdenisSK]) [c:42937e9]


## [0.7.3] - 2019-11-19

Small release including fixes for Discord API changes. Please note with this version the minimum supported version of Rust is 1.37.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@ikkerens]

### Fixed

- [framework] Properly `pub`licise the iterator ([@acdenisSK]) [c:1924946]
- [ci] Try to update repository information beforehand ([@acdenisSK]) [c:19b590a]
- [audit log] Cover all error cases for audit log deserialization ([@ikkerens]) [c:beb4d5a] [c:afc04e4]

## [0.7.2] - 2019-10-21

A tiny release for a fix to voice

Thanks to the following for their contributions:

- [@acdenisSK]
- [@MOZGIII]

### Fixed

- [voice] Fix `ClientVoiceManager::remove` to actually call `Manager::remove` ([@MOZGIII]) [c:2734e27]
- [voice] Use the correct ip for the UDP socket ([@acdenisSK]) [c:c4b1c60]

## [0.7.1] - 2019-9-29

## Departure of a lead developer

It seems Discord has a thing against library developers. [They disabled the account of a discord.js developer because they were allegedly "underage" (below 13 years old)](https://github.com/discordjs/discord.js/issues/3440). There were credit card transactions to defend their innocence, but Discord argued that they need a photo of their face to properly verify their age, a request the developer declined to comply. Consequently, they chose to no longer be on Discord.

Recently, they did the same thing to [@Lakelezz], a huge contributor to Serenity. However, this time they did not state their exact reason, simply saying "in violation of the ToS". Just like the JS developer, she decided to stop affiliating herself with the platform, if this is how it presents its "gratitude" towards her. But also, to cease development of the library.

And thus, on her behalf, I, the main lead developer [@acdenisSK], announce her retirement of the project.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Erk-]
- [@ikkerens]
- [@kyranet]
- [@Lakelezz]
- [@shnarazk]
- [@Zalaxx]

### Added

- [model] Add support for the `preferred_locale` field ([@Erk-]) [c:2d3e585]
- [meta] Add missing word `need`. ([@Lakelezz]) [c:65837f5]
- [model] Add new message fields ([@Erk-]) [c:e762ea9]
- [gateway/client] Implement WebSocket shutdown support ([@ikkerens]) [c:711882b]
- [utils] Add more formats and case insensitivity to `parse_invite` ([@ikkerens]) [c:0183714]
- [model] Add optional inviter field to Invite ([@ikkerens]) [c:21c95fd]

### Changed

- [meta] Set minimum Rust version to `1.37.0`.  ([@Lakelezz]) [c:de9e8a6]
- [meta] Update related project's hrefs ([@kyranet]) [c:445810f]

### Fixed

- [meta] Fix serenity version in the readme ([@Zalaxx]) [c:730c959]
- [framework] Fix incorrect label usage in plain help commands ([@acdenisSK]) [c:d427da4]
- [model]  Fix `has_role` returning an incorrect result if the member is not cached ([@ikkerens]) [c:96b49f9]

### Removed

- [meta] Remove the last mention of the global CACHE ([@shnarazk]) [c:ebdeb4e]

## [0.7.0] - 2019-8-29

An emergency release to fix a conflict in our [`ring`](https://github.com/briansmith/ring) dependency that prevents compilation if you pull in an older and newer version at the same time.

Thanks to the following for their contributions:
- [@Lakelezz]

### Changed

- [meta] Update all dependencies ([@Lakelezz]) [c:50d2a76]

## [0.6.4] - 2019-8-27

Thanks to the following for their contributions:
- [@Alch-Emi]
- [@AregevDev]
- [@acdenisSK]
- [@Erk-]
- [@Jerald]
- [@Lakelezz]
- [@leo-lb]
- [@Sreyas-Sreelal]

### Added

- [model] Add a method to create and iterable of `Member`s in a `Guild` ([@Alch-Emi]) [c:aa1070d]
- [utils] Add quoting functionality to `MessageBuilder`  ([@AregevDev]) [c:720d9ad]
- [model] Add support for new message types ([@Erk-]) [c:c45c1d4]
- [model] Add support for store channel ([@Erk-]) [c:8594c29]
- [model] Link to `ShardMessenger::chunk_guilds` in `Guild`'s `member` field ([@Alch-Emi]) [c:8e926f9]
- [framework]  Add group-related removal and non-consuming adding functions to `StandardFramework` ([@Jerald]) [c:3a4e2ed]
- [framework] Allow delimiters to be set on a per command basis ([@acdenisSK]) [c:6f7797e]
- [voice] Play a YouTube Search's first video ([@Sreyas-Sreelal]) [c:ccbba0a]
- [model] Add methods to get permissions of `Role`s in `GuildChannel`s ([@Lakelezz]) [c:09c1e01]
- [utils] Allow users to create their own messages ([@acdenisSK]) [c:e8da420]

### Changed

- [model] Fetch the guild id only if necessary ([@acdenisSK]) [c:85dd1a0]
- [meta] Move `webpki` and `webpki-roots` to the `rustls_backend`-feature ([@leo-lb]) [c:2439275]

### Fixed

- [model] Fix content of a message if there's an attachment ([@Erk-]) [c:6d06632]
- [meta/examples] Fix a typo in the examples ([@Sreyas-Sreelal]) [c:22f3d2a]
- [framework] Fix plain help suggestions ([@Lakelezz]) [c:ec687ad]

## [0.6.3] - 2019-7-24

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Erk-]
- [@hyarsan]
- [@Lakelezz]
- [@Mendess2526]
- [@NieDzejkob]
- [@xacrimon]

### Added

- [cache] Implement `CacheHttp` for `Arc<Http>` ([@Lakelezz]) [c:1a209e8]

### Changed

- [framework] Update module-level example ([@acdenisSK]) [c:26192fa]
- [client/model/utils] Changed some `&str`-parametres to `impl AsRef<str>` ([@xacrimon]) [c:abd84c2]
- [framework] Try to always ignore bots and webhooks when configured by the user ([@acdenisSK]) [c:4cf4b21] [c:b7b3a85]
- [framework] Apply case-insensitivity to help if needed ([@acdenisSK]) [c:cd4ca1b]

### Fixed

- [voice] Fix mistake in `voice::Audio`'s documentation with `play_only` ([@Mendess2526]) [c:e6c5d41]
- [utils]  Make `MessageBuilder::push_spoiler_*` consistent with the other `push_` functions ([@hyarsan]) [c:d2df2b9]
- [misc.] Update to the actual minimum Rust version Serenity supports ([@acdenisSK]) [c:d280ed1]
- [misc.] Fix comment in the group prefixes example ([@NieDzejkob]) [c:81d5af1]:
- [framework] Fix `Reason`'s doc-link by using `enum` ([@Lakelezz]) [c:a8f0387]
- [client] Fix private channel deletions making serenity panic ([@Erk-]) [c:67f5e3d]
- [model] Fix `create_invite`'s doc-example ([@Lakelezz]) [c:45d44cb]
- [framework] Update `help_commands`'s module example ([@acdenisSK]) [c:8cdfd7c]
- [framework] Remove unnecessary ticks ([@Lakelezz]) [c:eddef7b]

## [0.6.2] - 2019-6-30

A small release to address a severe deserialization bug.

Thanks to the following for their contributions:
- [@benjaminrsherman]
- [@Lakelezz]
- [@zeyla]

### Changed

- [meta] Reduce versioning in examples to major.minor ([@Lakelezz]) [c:13595ff]
- [framework/command_attr] Escape tags and add newline in documentation ([@Lakelezz]) [c:b28716c]

### Fixed

- [framework/command_attr] Fix invalid documentation for the group macro ([@benjaminrsherman]) [c:17f1dc2]
- [model] Fix guild deserialisation regression ([@zeyla]) [c:e628614]

## [0.6.1] - 2019-6-29

Thanks to the following for their contributions:

- [@acdenisSK]
- [@hyarsan]
- [@Lakelezz]
- [@rsaihe]
- [@xacrimon]
- [@zeyla]

## Added

- [framework/command_attr] Add the option to override the display name of a group ([@acdenisSK]) [c:759a278]
- [framework] Add `remains`, an optional alternative to `rest` ([@hyarsan]) [c:3e15bb8]

## Changed

- [meta] Update the versions to be latest in the README. ([@xacrimon]) [c:335701e]
- [model] Change the generic of `members` to encompass the `Option` ([@zeyla]) [c:3a72058]
- [framework] Remove `set_remove` with hint to the `#[check]` ([@Lakelezz]) [c:1527838]

## Fixed

- [model] Revert `say` taking `self` to `&self` ([@zeyla]) [c:e5081db]
- [framework] Give the owner privilege if only both the group and its command give consent. ([@acdenisSK]) [c:030bb4d]
- [command_attr] Fix `command_attr` documentation using `#[sub]` instead of `#[sub_commands]` ([@rsaihe]) [c:7a0d169]

## [0.6.0] - 2019-6-21

🎉 It has finally come for the biggest release of Serenity yet! 🎉

Thanks to the following for their contributions:

- [@AregevDev]
- [@acdenisSK]
- [@andreasots]
- [@Celti]
- [@DarkKirb]
- [@Erk-]
- [@eatsfoobars]
- [@Flat]
- [@FelixMcFelix]
- [@hyarsan]
- [@icewind1991]
- [@Kroisse]
- [@Lakelezz]
- [@Mishio595]
- [@molenzwiebel]
- [@mattico]
- [@nycex]
- [@PvdBerg1998]
- [@Roughsketch]
- [@xacrimon]
- [@xSke]
- [@zeyla]


## Since rc-2.1:

### Added
- [framework/command_attr#docs] Add `#[bucket]` to the available attributes list. ([@acdenisSK]) [c:0daaac1]

### Changed

- [examples] Fix a typo of Serenity ([@Lakelezz]) [c:90b7829]
- [gateway] Remove tungstenite buffer limits ([@molenzwiebel]) [c:638b642]
- [framework/command_attr] Be more helpful when reporting errors on return types ([@acdenisSK]) [c:c8a8d4f]
- [model] Make all structs non-exhaustive ([@zeyla]) [c:dddd417]
- [http] No longer delay ratelimit sleeps by 500ms ([@acdenisSK]) [c:638bb1a]

### Fixed

- [framework/command_attr] Assign the new value to the correct colour ([@acdenisSK]) [c:d1addff]
- [model] Fix panic on deserialising `PartialGuild` with no Nitro boosters. ([@xSke]) [c:5e77718]

## rc-2

### Added

- [command_attr] Add some utility structs. ([@acdenisSK]) [c:9162929]
- [command_attr] Add docs for `lacking_ownership`. ([@acdenisSK]) [c:15e7fde]
- [example] Add new Example about Eventing and Timing. ([@Lakelezz]) [c:10b9cc2]
- [example] Add example of embedding a local image in an embed. ([@Erk-]) [c:709c9e4]
- [framework] Add back blocking guilds, channels, and users. ([@acdenisSK]) [c:33f8383]
- [framework] Output the `#[example]` text in help ([@Flat]) [c:7aea26c]
- [model] Add and use `AttachmentId`. ([@Lakelezz]) [c:c8a5f69]
- [model] Add a `channel_id_from_name`-method on `Guild`. ([@xacrimon]) [c:aae22a2]
- [model] Add `GuildChannel::members`. ([@Lakelezz]) [c:ddf7a3]
- [model] Add more guild fields from guild boosting. ([@AregevDev]) [c:4541935]
- [utils] Add `EmbedMessageBuilding`-trait. ([@zeyla]) [c:7c61f95]

### Fixed

- [builder] Return `&mut self` on `voice_channel`. ([@Lakelezz]) [c:0e55b73]
- [ci] Fix Azure Windows build. ([@Lakelezz]) [c:fc3a1f6]
- [client] Fix updates giving only new data. ([@zeyla]) [c:5f7231d]
- [clippy] Fix Clippy-lints. ([@Lakelezz]) [c:cd7d07e]
- [clippy] Implement suggestions from Clippy & remove Clippy arg max config. ([@Flat]) [c:6586830]
- [command_attr] Add missing `s`, `owner_only` became `owners_only`. ([@acdenisSK]) [c:3cf673e]
- [command_attr] Report errors from parsing group options, if any. ([@acdenisSK]) [c:8e01ff6]
- [doc] Small doc fixes for the command macro. ([@acdenisSK]) [c:186e914]
- [doc] Revise Guild's `voice_states` doc. ([@nycex]) [c:0a640a4]
- [example] Update the mentioned feature `methods` to `utils`. ([@Lakelezz]) [c:c970f44]
- [framework] Fix Help displaying Groups and their Commands. ([@Lakelezz]) [c:eca204a]
- [framework] Ensure to properly hide groups. ([@Lakelezz]) [c:5e66cd1]
- [framework] Add help for nested groups and their commands. ([@Lakelezz]) [c:6a37535]
- [framework] Get rid of a redundant feature gate. ([@acdenisSK]) [c:2ae3a48]
- [framework] Treat the actual name and aliases equally. ([@Lakelezz]) [c:82d97c2]
- [framework] Check if message author is in owners HashSet. ([@Flat]) [c:d91594b]
- [framework] Change `owners_privilege` to bypass all permission-checks. ([@Flat]) [c:98532da]
- [http] Fix setting role positions ([@icewind1991]) [c:c14ca32]
- [model] Fix lifetime issue with `send_message`. ([@acdenisSK]) [c:3902caf]
- [model] Fix no-default-features compilation. ([@zeyla]) [c:3de5378]
- [model] Fix `contains_case_insensitive` and `starts_with_case_insensitive`. ([@Flat]) [c:d27d391]
- [voice] Pipe youtube-dl to ffmpeg directly. ([@Flat]) [c:4793a84]

### Changed

- [builder] Use `ToString` on builder-arguments instead of `Display`. ([@acdenisSK]) [c:13fae29]
- [builder] Increase the capabilities for creating a channel. ([@acdenisSK]) [c:f2ff97a]
- [client] Improve `cached`'s name and documentation. ([@Lakelezz]) [c:7706475]
- [command_attr] Rectify command parsing. ([@acdenisSK]) [c:b1eff27]
- [command_attr] Use the function-name. ([@acdenisSK]) [c:05254c8]
- [command_attr] Change `only` to `only_in`. ([@acdenisSK]) [c:26b072f]
- [command_attr] Reinvent `group!` parsing. ([@acdenisSK]) [c:7f9c4e1]
- [command_attr] Stop appending `_HELP_COMMAND` to the generated instance from `#[help]` ([@acdenisSK]) [c:9783b35]
- [example] Update the framework example. ([@acdenisSK]) [c:0fcb43c]
- [example] Update to use shard manager. ([@zeyla]) [c:5375827]
- [framework] Take into regard prefixless groups ([@acdenisSK]) [c:ef15739]
- [framework] Ensure prefixes to be mandatory on help ([@Lakelezz]) [c:ab34f75]
- [model] Update `*Id::created_at()` to return a `DateTime<FixedOffset>` instead. ([@AregevDev]) [c:8d50840]
- [model] Replace `Context` as argument ([@Lakelezz]) [c:62e19a7] [c:58fa50c]
- [rustc] Set minimum Rust version to `1.35.0`. ([@Lakelezz]) [c:8c83fec]
- [voice] Add infinite retry arg to ytdl for rst packets. ([@Flat]) [c:86ec810]

### Removed

- [builder] Remove unused `build`-method. ([@Lakelezz]) [c:c6ae140]
- [client] Remove `quit` method. ([@zeyla]) [c:f7109ee]
- [utils] Remove `VecMap`. ([@acdenisSK]) [c:9450d4b]

## rc-1

### Added

- [model] Add missing fields of `current_application_info` ([@mattico]) [c:23bed41]
- [builder] Allow for channels to be (or not be) set as nsfw ([@acdenisSK]) [c:1bd5bbc]
- [framework] Bring back old parsing behaviour ([@acdenisSK]) [c:64e97c5]
- [http] `AsRef<Http>` Implementation for `Http` ([@Lakelezz]) [c:b425ceb]

### Fixed

- [misc.] Fix Doc-Links and update Changelog ([@Lakelezz]) [c:c63eaea]

### Changed

- [framework] Take into equation ignoring bots and webhooks for help ([@acdenisSK]) [c:b1559bc]
- [general] Increase minimum support Rust version ([@acdenisSK]) [c:61ac765]
- [general/framework] Shackle the minimum version of uwl to 0.3.2 ([@acdenisSK]) [c:decbc04]

## rc-0

## Added

- [builder/model] Permit sending files through the `CreateMessage` builder. ([@Roughsketch]) [c:5405ac2]
- [client] Add Rich Presence parsing support ([@zeyla]) [c:f7360e6]
- [model] Add Slow Mode Rate ([@Lakelezz]) [c:7512c19]
- [voice] Voice reconnection ([@FelixMcFelix]) [c:25cb595] [c:4026d77] [c:2f613c0] [c:0a58e85]
- [model] Add a position propagation method to Channel ([@Erk-]) [c:59b4c60]
- [misc.] Re-export `typemap::sharemap` ([@zeyla]) [c:d2233e2]
- [framework] Add new Check System ([@Lakelezz]) [c:2969561]
- [http/gateway] Rustls support ([@Erk-]) [c:faa773a]
- [model] Add news channel ([@Lakelezz]) [c:1074b28]
- [client] Add EventHandler for raw Events ([@DarkKirb]) [c:2b453c3]
- [model] Add millisecond accuracy to `ID.created_at()` ([@DarkKirb]) [c:965fa7b]
- [http/gateway] Add Rustls and Native-TLS Backends ([@Lakelezz]) [c:15e2c41]

## Changed

- [model] Make MessageUpdateEvent::embeds a Vec<Embed> ([@zeyla]) [c:00f465c]
- [voice] Voice fixes, better API adherence, bitrate control, documentation ([@FelixMcFelix]) [c:393a5ae]
- [builder] Make builders mutably borrowed ([@zeyla], [@Flat], [@Lakelezz], [@Celti]) [c:1546171] [c:6d87d71] [c:b7a6fee] [c:b012ab7]
- [utils] Make Message Builder use &mut self instead of self ([@PvdBerg1998]) [c:1546171]
- [misc.] Update `parking_lot` and `multipart` dependencies ([@Kroisse]) [c:1e50d30]
- [framework] Make sure `delimiter` clears current and default delimiters. ([@Lakelezz]) [c:3f81cf3]
- [framework] Underline command name and "Commands" in plain help ([@hyarsan]) [c:87bc6ca]
- [http]  Replace `hyper` with `reqwest` ([@Lakelezz]) [c:86a8b60]
- [client/gateway] Switch to tungstenite from rust-websocket ([@zeyla]) [c:a5aa2a9]
- [misc.] Update to Rust 2018 ([@Lakelezz]) [c:21518c8]
- [http/model/all] Remove global Cache and HTTP ([@Lakelezz]) [c:712cfa5] [c:3f0ea69]
- [client] Change the `Context::data` field to use an `RwLock` ([@Erk-]) [c:661d778]
- [cache] Pass old Message to `message_update` ([@Mishio595]) [c:40bf272]
- [framework] Check for Ownership in Help System ([@Lakelezz]) [c:fa0376c]
- [framework] Improve Help Consistency ([@Lakelezz]) [c:51b48f4]
- [misc.] Adhere to Rust 2018's idioms ([@Lakelezz]) [c:5d6dc37]
- [client] Add different `Context::new`s based on feature-set. ([@Lakelezz]) [c:625b764]
- [framework] Remodel `Args`'s API ([@acdenisSK]) [c:c472ddd]
- [framework] Rewrite the framework to attributes ([@acdenisSK]) [c:cc81e47]
- [framework] Handle Sub-Groups in the Help-System ([@Lakelezz]) [c:9b591ec]
- [voice] Fewer ffprobe calls when playing audio through ffmpeg ([@FelixMcFelix]) [c:5dff7eb]
- [voice] Optional impls and additional events for AudioReceiver ([@FelixMcFelix]) [c:d955df4]
- [voice] ClientConnect message handling ([@FelixMcFelix]) [c:fa11a30]
- [client] Pass the old voice state if the cache is enabled ([@andreasots]) [c:bd45e42]
- [http] Specify Header's Content Length as `0` ([@eatsfoobars]) [c:a713b40]
- [voice] Switch to `audiopus` ([@Lakelezz]) [c:4af7a98]
- [model] Make `enum`s non-exhaustive ([@Lakelezz]) [c:9cc8816]
- [http] Make the HttpError Send+Sync ([@Erk-]) [c:6cfc0e1]
- [framework] Update `on_mention` to take a `UserId` ([@Celti]) [c:d995fa0]
- [utils] Simplify `from_rgb`, turn some of Colour's functions to `const`. ([@hyarsan]) [c:c149e36]

## Fixed

- Fix ActivityFlags/ActivityTimestamps/ActivityParty deserialization ([@zeyla]) [c:0a77330] [c:d01eeae]
- Fix `MessageBuilder`'s doctests ([@Flat]) [c:a3477a2]

## Removed

- [client] Remove deprecated `Context::edit_profile` ([@zeyla]) [c:bc0d82e]
- [misc.] Remove everything marked `deprecated` since `v0.5.x` or older ([@Lakelezz]) [c:70720ae]

## [0.6.0-rc.2] - 2019-6-14

Thanks to the following for their contributions:
- [@acdenisSK]
- [@AregevDev]
- [@Erk-]
- [@Flat]
- [@icewind1991]
- [@Lakelezz]
- [@nycex]
- [@xacrimon]
- [@zeyla]

A crucial release fixing a lot of misbehaviour:

### Added

- [command_attr] Add some utility structs. ([@acdenisSK]) [c:9162929]
- [command_attr] Add docs for `lacking_ownership`. ([@acdenisSK]) [c:15e7fde]
- [example] Add new Example about Eventing and Timing. ([@Lakelezz]) [c:10b9cc2]
- [example] Add example of embedding a local image in an embed. ([@Erk-]) [c:709c9e4]
- [framework] Add back blocking guilds, channels, and users. ([@acdenisSK]) [c:33f8383]
- [framework] Output the `#[example]` text in help ([@Flat]) [c:7aea26c]
- [model] Add and use `AttachmentId`. ([@Lakelezz]) [c:c8a5f69]
- [model] Add a `channel_id_from_name`-method on `Guild`. ([@xacrimon]) [c:aae22a2]
- [model] Add `GuildChannel::members`. ([@Lakelezz]) [c:ddf7a3]
- [model] Add more guild fields from guild boosting. ([@AregevDev]) [c:4541935]
- [utils] Add `EmbedMessageBuilding`-trait. ([@zeyla]) [c:7c61f95]

### Fixed

- [builder] Return `&mut self` on `voice_channel`. ([@Lakelezz]) [c:0e55b73]
- [ci] Fix Azure Windows build. ([@Lakelezz]) [c:fc3a1f6]
- [client] Fix updates giving only new data. ([@zeyla]) [c:5f7231d]
- [clippy] Fix Clippy-lints. ([@Lakelezz]) [c:cd7d07e]
- [clippy] Implement suggestions from Clippy & remove Clippy arg max config. ([@Flat]) [c:6586830]
- [command_attr] Add missing `s`, `owner_only` became `owners_only`. ([@acdenisSK]) [c:3cf673e]
- [command_attr] Report errors from parsing group options, if any. ([@acdenisSK]) [c:8e01ff6]
- [doc] Small doc fixes for the command macro. ([@acdenisSK]) [c:186e914]
- [doc] Revise Guild's `voice_states` doc. ([@nycex]) [c:0a640a4]
- [example] Update the mentioned feature `methods` to `utils`. ([@Lakelezz]) [c:c970f44]
- [framework] Fix Help displaying Groups and their Commands. ([@Lakelezz]) [c:eca204a]
- [framework] Ensure to properly hide groups. ([@Lakelezz]) [c:5e66cd1]
- [framework] Add help for nested groups and their commands. ([@Lakelezz]) [c:6a37535]
- [framework] Get rid of a redundant feature gate. ([@acdenisSK]) [c:2ae3a48]
- [framework] Treat the actual name and aliases equally. ([@Lakelezz]) [c:82d97c2]
- [framework] Check if message author is in owners HashSet. ([@Flat]) [c:d91594b]
- [framework] Change `owners_privilege` to bypass all permission-checks. ([@Flat]) [c:98532da]
- [http] Fix setting role positions ([@icewind1991]) [c:c14ca32]
- [model] Fix lifetime issue with `send_message`. ([@acdenisSK]) [c:3902caf]
- [model] Fix no-default-features compilation. ([@zeyla]) [c:3de5378]
- [model] Fix `contains_case_insensitive` and `starts_with_case_insensitive`. ([@Flat]) [c:d27d391]
- [voice] Pipe youtube-dl to ffmpeg directly. ([@Flat]) [c:4793a84]

### Changed

- [builder] Use `ToString` on builder-arguments instead of `Display`. ([@acdenisSK]) [c:13fae29]
- [builder] Increase the capabilities for creating a channel. ([@acdenisSK]) [c:f2ff97a]
- [client] Improve `cached`'s name and documentation. ([@Lakelezz]) [c:7706475]
- [command_attr] Rectify command parsing. ([@acdenisSK]) [c:b1eff27]
- [command_attr] Use the function-name. ([@acdenisSK]) [c:05254c8]
- [command_attr] Change `only` to `only_in`. ([@acdenisSK]) [c:26b072f]
- [command_attr] Reinvent `group!` parsing. ([@acdenisSK]) [c:7f9c4e1]
- [command_attr] Stop appending `_HELP_COMMAND` to the generated instance from `#[help]` ([@acdenisSK]) [c:9783b35]
- [example] Update the framework example. ([@acdenisSK]) [c:0fcb43c]
- [example] Update to use shard manager. ([@zeyla]) [c:5375827]
- [framework] Take into regard prefixless groups ([@acdenisSK]) [c:ef15739]
- [framework] Ensure prefixes to be mandatory on help ([@Lakelezz]) [c:ab34f75]
- [model] Update `*Id::created_at()` to return a `DateTime<FixedOffset>` instead. ([@AregevDev]) [c:8d50840]
- [model] Replace `Context` as argument ([@Lakelezz]) [c:62e19a7] [c:58fa50c]
- [rustc] Set minimum Rust version to `1.35.0`. ([@Lakelezz]) [c:8c83fec]
- [voice] Add infinite retry arg to ytdl for rst packets. ([@Flat]) [c:86ec810]

### Removed

- [builder] Remove unused `build`-method. ([@Lakelezz]) [c:c6ae140]
- [client] Remove `quit` method. ([@zeyla]) [c:f7109ee]
- [utils] Remove `VecMap`. ([@acdenisSK]) [c:9450d4b]


## [0.6.0-rc.1] - 2019-5-14

Thanks to the following for their contributions:
- [@acdenisSK]
- [@Lakelezz]
- [@mattico]

A short release for some things we overlooked.

## Added

- [model] Add missing fields of `current_application_info` ([@mattico]) [c:23bed41]
- [builder] Allow for channels to be (or not be) set as nsfw ([@acdenisSK]) [c:1bd5bbc]
- [framework] Bring back old parsing behaviour ([@acdenisSK]) [c:64e97c5]
- [http] `AsRef<Http>` Implementation for `Http` ([@Lakelezz]) [c:b425ceb]

## Fixed

- [misc.] Fix Doc-Links and update Changelog ([@Lakelezz]) [c:c63eaea]

## Changed

- [framework] Take into equation ignoring bots and webhooks for help ([@acdenisSK]) [c:b1559bc]
- [general] Increase minimum support Rust version ([@acdenisSK]) [c:61ac765]
- [general/framework] Shackle the minimum version of uwl to 0.3.2 ([@acdenisSK]) [c:decbc04]

## [0.6.0-rc.0] - 2019-5-6

Thanks to the following for their contributions:

- [@acdenisSK]
- [@andreasots]
- [@Celti]
- [@DarkKirb]
- [@eatsfoobars]
- [@Erk-]
- [@FelixMcFelix]
- [@Flat]
- [@hyarsan]
- [@Kroisse]
- [@Lakelezz]
- [@Mishio595]
- [@PvdBerg1998]
- [@Roughsketch]
- [@zeyla]

# Release candidate

This is a *testing release* for receiving feedback regarding the new big changes introduced, whether they’re satisfactory, or horrid and should be revised, before we officially stabilise them.

Please inform us of any suggestions, or bugs you might have!

# Major breaking changes

Serenity has migrated to the 2018 Rust edition, whose lints and idioms are enforced in its codebase.

The cache and http are no longer globally accessible.  The `Context` now carries instances to them, and as such, all functions that had used the cache and http before, now accept the context as their first parameter in order to operate. *Passing the fields present on the context is acceptable too.*

The framework had been swayed off of builders, and proselytised to procedural, macro-based attributes.
Giving options to your commands might have looked like this:

```rust
command!(foo(ctx, msg, args) {
    ...
});

framework.command("foo", |c|
   c.description("I am foobar")
       .min_args(1)
       .max_args(2)
       .usage("#foo bar baz")
       .cmd(foo));
```

But now, it will be:

```rust
#[command] // Marks this function as a command.
#[description = "I am foobar"] // These are the "parameter" attributes, for providing the options to the attribute macro.
#[min_args(1)]
#[max_args(2)]
#[usage("#foo bar baz")]
fn foo(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    ...

    Ok(())
}
```

The same happened to creating groups, but with `macro!` style flavour, which have become a compulsory step in registering your commands:

```rust
group!({
    name: "fizzbuzz",
    options: {
        prefix: "fezz",
        ...
    },
    commands: [foo],
});
```

All `.command`s and `.on`s are thus replaced with simple calls to `.group`:

```rust
framework.group(&FIZZBUZZ_GROUP); // !
```

! - procedural macros are functions that accept Rust code, return Rust code. The Rust code that the `#[command]` (and similarly, `group!`) macro generates is the function you supplied it with, and a `static` instance of options that you've configured the command with. The static is assigned a suffixed, all uppercase version of the function’s name (or in the case of `group!`,  of the `name` field). Hence this weird identifier from nowhere.

# Book

To help new (and existing) users familiarise themselves with the library better, we have decided to write a book similar to one of Rust's official learning material to the language, [The Book](https://doc.rust-lang.org/book/ch00-00-introduction.html).

It's no ready yet, but we hope that on its release that it will clear misunderstandings (if any), explain the why and how of the library and put you in the right direction of Discord bot making!

## Added

- [builder/model] Permit sending files through the `CreateMessage` builder. ([@Roughsketch]) [c:5405ac2]
- [client] Add Rich Presence parsing support ([@zeyla]) [c:f7360e6]
- [model] Add Slow Mode Rate ([@Lakelezz]) [c:7512c19]
- [voice] Voice reconnection ([@FelixMcFelix]) [c:25cb595] [c:4026d77] [c:2f613c0] [c:0a58e85]
- [model] Add a position propagation method to Channel ([@Erk-]) [c:59b4c60]
- [misc.] Re-export `typemap::sharemap` ([@zeyla]) [c:d2233e2]
- [framework] Add new Check System ([@Lakelezz]) [c:2969561]
- [http/gateway] Rustls support ([@Erk-]) [c:faa773a]
- [model] Add news channel ([@Lakelezz]) [c:1074b28]
- [client] Add EventHandler for raw Events ([@DarkKirb]) [c:2b453c3]
- [model] Add millisecond accuracy to `ID.created_at()` ([@DarkKirb]) [c:965fa7b]
- [http/gateway] Add Rustls and Native-TLS Backends ([@Lakelezz]) [c:15e2c41]

## Changed

- [model] Make MessageUpdateEvent::embeds a Vec<Embed> ([@zeyla]) [c:00f465c]
- [voice] Voice fixes, better API adherence, bitrate control, documentation ([@FelixMcFelix]) [c:393a5ae]
- [builder] Make builders mutably borrowed ([@zeyla], [@Flat], [@Lakelezz], [@Celti]) [c:1546171] [c:6d87d71] [c:b7a6fee] [c:b012ab7]
- [utils] Make Message Builder use &mut self instead of self ([@PvdBerg1998]) [c:1546171]
- [misc.] Update `parking_lot` and `multipart` dependencies ([@Kroisse]) [c:1e50d30]
- [framework] Make sure `delimiter` clears current and default delimiters. ([@Lakelezz]) [c:3f81cf3]
- [framework] Underline command name and "Commands" in plain help ([@hyarsan]) [c:87bc6ca]
- [http]  Replace `hyper` with `reqwest` ([@Lakelezz]) [c:86a8b60]
- [client/gateway] Switch to tungstenite from rust-websocket ([@zeyla]) [c:a5aa2a9]
- [misc.] Update to Rust 2018 ([@Lakelezz]) [c:21518c8]
- [http/model/all] Remove global Cache and HTTP ([@Lakelezz]) [c:712cfa5] [c:3f0ea69]
- [client] Change the `Context::data` field to use an `RwLock` ([@Erk-]) [c:661d778]
- [cache] Pass old Message to `message_update` ([@Mishio595]) [c:40bf272]
- [framework] Check for Ownership in Help System ([@Lakelezz]) [c:fa0376c]
- [framework] Improve Help Consistency ([@Lakelezz]) [c:51b48f4]
- [misc.] Adhere to Rust 2018's idioms ([@Lakelezz]) [c:5d6dc37]
- [client] Add different `Context::new`s based on feature-set. ([@Lakelezz]) [c:625b764]
- [framework] Remodel `Args`'s API ([@acdenisSK]) [c:c472ddd]
- [framework] Rewrite the framework to attributes ([@acdenisSK]) [c:cc81e47]
- [framework] Handle Sub-Groups in the Help-System ([@Lakelezz]) [c:9b591ec]
- [voice] Fewer ffprobe calls when playing audio through ffmpeg ([@FelixMcFelix]) [c:5dff7eb]
- [voice] Optional impls and additional events for AudioReceiver ([@FelixMcFelix]) [c:d955df4]
- [voice] ClientConnect message handling ([@FelixMcFelix]) [c:fa11a30]
- [client] Pass the old voice state if the cache is enabled ([@andreasots]) [c:bd45e42]
- [http] Specify Header's Content Length as `0` ([@eatsfoobars]) [c:a713b40]
- [voice] Switch to `audiopus` ([@Lakelezz]) [c:4af7a98]
- [model] Make `enum`s non-exhaustive ([@Lakelezz]) [c:9cc8816]
- [http] Make the HttpError Send+Sync ([@Erk-]) [c:6cfc0e1]
- [framework] Update `on_mention` to take a `UserId` ([@Celti]) [c:d995fa0]
- [utils] Simplify `from_rgb`, turn some of Colour's functions to `const`. ([@hyarsan]) [c:c149e36]

## Fixed

- Fix ActivityFlags/ActivityTimestamps/ActivityParty deserialization ([@zeyla]) [c:0a77330] [c:d01eeae]
- Fix `MessageBuilder`'s doctests ([@Flat]) [c:a3477a2]

## Removed

- [client] Remove deprecated `Context::edit_profile` ([@zeyla]) [c:bc0d82e]
- [misc.] Remove everything marked `deprecated` since `v0.5.x` or older ([@Lakelezz]) [c:70720ae]

## [0.5.14] - 2019-5-17

This release fixes a few bugs.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Lakelezz]

## Added

- [model] Allow turning on and off the nsfw property of guilds channels ([@acdenisSK]) [c:68c4f5c]

## Changed

- [builder] Remove `type`-field from `edit`'s request body. ([@Lakelezz]) [c:f648d90]

## Fixed

- [model] Handle serde_json's "arbitrary precision" feature. ([@acdenisSK]) [c:33f4adf]
- [framework] Do not display commands their `help_available` is set to `false`. ([@Lakelezz]) [c:1705338]
- [framework] Ignore bots when using the help-command if framework's `ignore_bots` is set to `true`. ([@acdenisSK]) [c:e40758e]
- [misc.] Rename the `methods`-feature inside the third example to `utils`. ([@Lakelezz]) [c:a7ee6a6]

## [0.5.13] - 2019-3-10

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Erk-]
- [@FelixMcFelix]
- [@ijks]
- [@JellyWX]
- [@Lakelezz]
- [@MOZGIII]

# Breaking change

As unusual as it may sound for a patch version, we had to bump our minimum supported Rust version to 1.31.1 as a consequence of certain dependencies publishing releases containing uncompilable code for 1.25 rustc.

## Added

- [misc.] Include the latest releases in CHANGELOG.md  (@acdenisSK) [c:201dab8] [c:201bc56]
- [misc.] Transition from Travis to Microsoft's Azure CI (@Erk-, @acdenisSK, @Lakelezz) [c:68263ac] [c:bca2f4b] [c:3b8ae67] [c:a0b1dd8] [c:bd48ac5]

## Changed

- [docs] Compile, but do not run tests that were previously ignored (@MOZGIII) [c:00990c0]
- [misc.] Lock `cc` and `base64` dependencies to specific versions (@Lakelezz) [c:bc3d978]
- [general] Update Discord's welcome messages as of 2018-12-19 (@Erk-) [c:e94388]
- [voice] Fewer ffprobe calls when playing audio through ffmpeg (@FelixMcFelix) [c:cfcd342] (Improperly credited under @acdenisSK due to a hiccup on Github's part.)
- [misc.] Define 1.31.1 as the new minimum Rust version (@acdenisSK) [c:07e81b0]
- [misc.] Revert commit [c:bc3d978](https://github.com/serenity-rs/serenity/commit/bc3d978b65ae6d07342bfba4618c249d0beae98e) (@acdenisSK) [c:498e41c]
- [misc.] Bump sodiumoxide to version 0.2 (@DoumanAsh, @MOZGIII) [c:23ae9d8] (Commit done by @acdenisSK, but the intention to upgrade the version were [Douman's](https://github.com/serenity-rs/serenity/pull/454) and [Mozgiii's](https://github.com/serenity-rs/serenity/pull/490))

## Fixed

- [model] Fix "no-cache with http" feature combo of `has_role` (@Erk-) [c:3899547]
- [docs] Use a normal `main` to fix Rust 1.25 compilation (@acdenisSK) [c:b469611]
- [docs]  Fix wording of `timestamp`'s documentation (@acdenisSK) [c:7c09cdd]
- [misc.] Fix typos and perform some language improvements (@ijks) [c:88d914e]
- [docs] Fix tests to work with default features without `cache` (@Lakelezz) [c:e6694f2]
- [voice] Fix connection error being thrown on leaving voice (@JellyWX) [c:62a1aa2](https://github.com/serenity-rs/serenity/commit/62a1aa2abcf0919bf38ef90590aaa363eb03aae0)

## [0.5.12] - 2019-2-14

This is a celebratory release for Valentine's day, which we present to you with utmost courtesy.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Erk-]
- [@Flat]
- [@Lakelezz]
- [@Mishio595]
- [@mshenrick]
- [@zeyla]

### Upgrade Path

`typemap` does not need to be pulled in as a dependency for serenity anymore.
You can remove `typemap` from your `#[dependencies]` table in your `Cargo.toml` and simply import its types from the prelude:

```rust
use serenity::prelude::{ShareMap, TypeMapKey};
```

### Added

- [model] Add a position propagation method to `Channel` ([@Erk-]) [c:2cb67df]
- [model] Implement `Into<u64>` and `Into<i64>` for ID types ([@Lakelezz]) [c:794393c]
- [general/prelude]  Re-export `ShareMap` and `Key` types from `typemap` ([@zeyla]) [c:b11b4e2]
- [utils/MessageBuilder]  Add support for spoilers ([@acdenisSK]) [c:a56d014]
- [general/example] Add usage of `colour` in example 11 ([@Erk-]) [c:7066ed2]

### Changed

- [http] Limit users from requesting over 100 users ([@Flat]) [c:8bf39a7]
- [client/event-handler] Elaborate causes for `guild_member_removal` ([@Lakelezz]) [c:dd75410]

### Fixed

- [model] Make `Region`’s `Japan`-variant lowercase (fixes inconsistency) ([@Lakelezz]) [c:065f55b]
- [model] Fix imports in `create_channel`’s example. ([@acdenisSK]) [c:bca1530]
- [framework] Fix aliases not being added to commands when using `cmd`. ([@Mishio595]) [c:e8d0628]
- [model] Fix no-cache compilation for `User::nick_in`. ([@zeyla], [@acdenisSK]) [c:11d5b72] [c:98bece3]
- [model] Fix `Emoji::url` to use `.gif` for animated emoji ([@mshenrick]) [c:ae0fc14]
- [http] Correct query string in `Route::guild_ban_optioned` ([@Mishio595]) [c:3c166e3]
- [model] Fix `has_role` temporarily (@Erk-) [c:204e0b9]

## [0.5.11] - 2018-11-12

Mini-release.

Thanks to the following for their contributions:

- [@DoumanAsh]
- [@Lakelezz]

### Added

- [framework] A callback handler that signifies a normal message was received ([@Lakelezz]) [c:16bc3815]
- [model] Convenience methods for getting a nickname ([@Lakelezz]) [c:ed17114c]
- [general] Add link for the `Voice on Windows` wiki entry to README.md ([@Lakelezz]) [c:99b72358]

### Changed

- [general] Update the `base64` and `sodiumoxide` dependencies ([@DoumanAsh]) [c:5f9ed749]
- [general/examples] Turn `unwrap`s to `expect`s and update to nested imports ([@Lakelezz]) [c:d6c4beea]

## [0.5.10] - 2018-11-5

This is a celebration release for the anniversary of the failed Gunpowder Plot enacted against King James of England and Scotland in 1605.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Bond-009]
- [@Erk-]
- [@Lakelezz]
- [@Proximyst]
- [@perryprog]
- [@zeyla]

# Known issues

Systems with OpenSSL 1.x.x installed will not be able to compile Serenity as it depends on an older version of OpenSSL (0.9.x). To circumvent this, you need to add a `patch` section to your `Cargo.toml` for [ishitatsuyuki's fork](https://github.com/ishitatsuyuki/rust-openssl), which is compatible with 1.x.x, like so:

```toml
[patch.crates-io]
openssl = { git = "https://github.com/ishitatsuyuki/rust-openssl", branch = "0.9.x" }
```

### Upgrade Path

Discord no longer considers channels with the prefix `nsfw-` in their name as NSFW. Per [c:75fb5c04], the `utils::is_nsfw` has been deprecated. Instead, the `is_nsfw` methods on the channel structs (as in `GuildChannel::is_nfsw`) are to be used for checking their nsfw-ness.

### Added

- [general/contributing] Add guideline about maximum characters per line ([@Lakelezz]) [c:12534348]
- [cache] Add a write-lock configuration option ([@Erk-]) [c:b2362dbb] [c:41ff44ba]
- [framework] Prefix-only command (@Lakelezz) [c:6a68f68e]
- [framework] Add an option to disable bypassing checks for owners ([@Lakelezz]) [c:c5285ae1]
- [framework] Add a method for trimming the current argument ([@acdenisSK], [@Lakelezz]) [c:3b050f49] [c:e763d80b]
- [model] Parse the id out of any mention ([@acdenisSK]) [c:d529cf79]
- [utils] Add function to neutralise mentions ([@Lakelezz]) [c:867a7447]

### Fixed

- [client] Compile the client without `cache` feature ([@Erk-]) [c:176fde29]
- [framework] Compile the framework without `cache` feature ([@Bond-009]) [c:9f834b2b]
- [framework] Fix Default Command to inherit group-options ([@Lakelezz]) [c:e32f9b57]
- [model] Fix NSFW checks ([@Lakelezz]) [c:75fb5c04]
- [http/docs] Fix dead links ([@Erk-]) [c:9d141bfc]

### Misc.

- [voice] Don't log event deserialization failures ([@zeyla]) [c:08511dae]
- [voice] Remove unused variable ([@Proximyst]) [c:69931fe3]
- [http] Remove inconsistent braces ([@Proximyst]) [c:ccfa7fdc]
- [cache/http] Change to UNIX line endings ([@Erk-]) [c:8e401f03]
- [docs] Typo fixes ([@perryprog]) [c:9865d9cc]
- [framework] Simplify code by removing negation ([@Lakelezz]) [c:093a1bab]
- [travis] Add `travis_wait` to extend build-time ([@Lakelezz]) [c:5b6574c3]

## [0.5.9] - 2018-09-14

This is a maintenance release fixing a number of bugs with a few miscellaneous
internal changes.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Lakelezz]
- [@Mishio595]
- [@perryprog]
- [@Roughsketch]
- [@zeyla]

### Added

- [model] Add session start info in `BotGateway` ([@zeyla]) [c:12bbc1a]
- [model] Add `Member::user_id` ([@zeyla]) [c:669da40]
- [framework] Suggest similar commands when using help ([@Lakelezz]) [c:ce79f01]
- [framework] add single group help ([@Lakelezz]) [c:75f6516]

### Fixed

- [http] Fix routing for `http::create_private_channel` (regression from 0.5.6)
  ([@zeyla]) [c:30a325e]
- [http] Fix `GuildChannel::_permissions_for` on no-cache builds (regression
  from 0.5.8) ([@zeyla]) [c:e59f766]
- [http] Change HTTP bulk delete from DELETE to POST (regression from 0.5.6)
  ([@Mishio595]) [c:ebbc324]
- [framework] Make `is_command_visible` work with DMs ([@Roughsketch])
  [c:7295079]
- [utils] Add newline in `MessageBuilder::push_codeblock_safe` ([@zeyla])
  [c:e66812a]
- [framework] Fix `has_correct_permissions` when no guild is available
  ([@Lakelezz]) [c:19c65bd]
- [framework] Check if bots are ignored before dispatching
  `unrecognised_command` ([@Lakelezz]) [c:966cb3e]
- [framework] Fix group prefix ambiguity in help ([@Lakelezz]) [c:f01e6e3],
  [c:c49e02c]
- [framework] Add missing usage on plain help ([@Lakelezz]) [c:823b829]
- [framework] Add usage sample back to help ([@Lakelezz]) [c:82dbff2]
- [framework] Check if group is empty and exclude if so ([@Lakelezz])
  [c:4778e69]
- [model] Message: avoid permission checks in non-guild channels ([@zeyla])
  [c:3fbab76]

### Misc.

- [docs] Fix a broken link in README ([@Mishio595]) [c:41b6e24]
- [docs] Properly link to User in Game docs ([@zeyla]) [c:dec3f13]
- [http] Move low-level http functions to `http::raw` and re-export ([@zeyla])
  [c:6157f61]
- [utils] Add more unit tests for `MessageBuilder` ([@zeyla]) [c:14c6099]
- [framework] Refactor help ([@Lakelezz]) [c:28cdc53]
- [docs] Update client docs to not say user token ([@perryprog]) [c:6ca4bea]
- [framework] Add tests for help ([@Lakelezz]) [c:79d8843]
- [model] Remove cache requirement on `Message::is_private` ([@zeyla])
  [c:fe69ef0]

## [0.5.8] - 2018-08-12

This is a hotfix release for incorrect routing and to fix a large number of
broken documentation links.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Erk-]
- [@Lakelezz]
- [@Lymia]
- [@Mishio595]
- [@zeyla]


### Upgrade Path

Per [c:71edc3a], methods on ID structs like `ChannelId::find` have been
deprecated and replace with `UserId::to_channel_cached`. Similarly, methods like
`GuildId::get` have been replaced with `GuildId::to_partial_guild`. While the
original methods have not been removed, they have been deprecated.

### Added

- [utils] Add `Colour::hex` ([@Mishio595]) [c:8bec4af]

### Fixed

- [http] Fix various incorrect routes ([@Lymia]) [c:826220f]
- [docs] Fix all the dead links in the docs ([@Erk-]) [c:40053a7]
- [voice] Stop attempting to send silent frames (reverts a commit) ([@zeyla])
  [c:0bbe5f5]

### Changed

- [model] Add `to_*`, `as_*` methods on Id types, deprecate `get` and `find`
  methods ([@Lakelezz]) [c:71edc3a]

### Misc.

- [framework] Fix doctest for `Args::iter_quoted` ([@acdenisSK]) [c:7b0cff6]
- [framework] Remove some code duplication ([@Lakelezz]) [c:516ede3]
- [framework] Don't trim command on failure in default command ([@Lakelezz])
  [c:46b4194]

## [0.5.7] - 2018-08-09

This is a hotfix release for an incorrect warning about cache deadlocking during
event dispatches in the client and fixing some routing method typos due to the
HTTP rewrite.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Lymia]
- [@zeyla]

### Fixed

- [client] Fix erroneous deadlock detection messages ([@Lymia]) [c:d1266fc]
- [http] Fix some routing issues ([@zeyla]) [c:04b410e]

### Misc.

- Slightly reword a cache update comment ([@acdenisSK]) [c:3a58090]

## [0.5.6] - 2018-08-07

This is a bugfix release that fixes a long-standing bug causing shards to
randomly die under certain rare conditions when dispatching the Ready event,
and compilation of the `cache` and `client` features without the `framework`
feature. This also contains an internal rewrite of the HTTP module.

The minimum required rustc version is now pinned at 1.25.0.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Erk-]
- [@Lakelezz]
- [@Mishio595]
- [@Roughsketch]
- [@zeyla]

### Upgrade Path

Per [c:01e3c33], `Context::edit_profile` has been deprecated. Call
`serenity::http::edit_profile` instead.

### Added

- [model] `impl AsRef<MessageId> for Message` ([@Mishio595]) [c:1de3937]
- [model] Add `From` impls for `Game`, genericify `Game` params ([@zeyla])
  [c:e1332a5], [c:a4c3fec]
- [http] Make `http::fire`, `http::request` public ([@zeyla]) [c:0d55363]
- [framework] Add no-parse getters and advancer to `Args` ([@acdenisSK])
  [c:73ab20f]
- [model] Add support for new `PRIORITY_SPEAKER` permission ([@Erk-])
  [c:2179623]

### Fixed

- [client] Don't delay Ready event with cache enabled ([@zeyla]) [c:12d5321]
- [framework] Handle no delimiters in `Args` ([@acdenisSK]) [c:e5ea6c1],
  [c:9568e3b]
- [client] Add missing `mut`, fixing no-framework compilation ([@acdenisSK])
  [c:90c7ec4]
- [framework] Check if message is empty in `Args` ([@acdenisSK]) [c:0501020]
- [client] Fix potential cache deadlocking when dispatching ([@zeyla])
  [c:f064d65]
- [framework] Pass failed sub-command to default command ([@Lakelezz])
  [c:db21036]
- [framework] Fix default command upon shortcut prefix ([@Lakelezz]) [c:8f128b2]

### Changed

- [client] Deprecate `Context::edit_profile` ([@zeyla]) [c:01e3c33]

### Misc.

- [model] Fix `ChannelId::send_message`'s dead links ([@acdenisSK]) [c:7a93557]
- [model] Add note about cache in `UserId::get` docs ([@zeyla]) [c:e2873c8]
- [general] Reduce required rustc to 1.25.0 ([@zeyla]) [c:f3f22d7], [c:b324774]
- [model] Make `GuildId::member` use cache when possible ([@Roughsketch])
  [c:21eb42f]
- [framework] Reword some `StandardFramework::complex_bucket` docs
  ([@acdenisSK]) [c:02de778]
- [framework] Internally refactor `positions` ([@acdenisSK]) [c:2a6c3b1]
- [framework] Update `Configuration` default value listings ([@zeyla])
  [c:602c5a7]
- [http] Maintain a single, re-used HTTP client ([@zeyla]) [c:8c0e5a3]
- [http] Redo the HTTP module internally ([@zeyla]) [c:a0b0dd2], [c:4648f58],
  [c:8918201], [c:8301333], [c:bbbf638], [c:9a863bd], [c:c458099], [c:aa437d4]
- [docs] Don't return Result from tests ([@acdenisSK]) [c:e290b03]
- [docs] Fix all dead links in permissions ([@Erk-]) [c:869fff5]

## [0.5.5] - 2018-07-25

This release is mostly a bugfix release. Thanks to the following for their
contributions:

- [@acdenisSK]
- [@drklee3]
- [@foxbot]
- [@Lakelezz]
- [@Mishio595]
- [@perryprog]
- [@TheUnitedStatesOfAmerica]
- [@zeyla]

### Added

- [framework] Add `Args::rest` ([@acdenisSK]) [c:9b2cd75]
- [model] Add `Message::guild_id` structfield ([@foxbot], [@zeyla]) [c:a9e8626],
  [c:3121f90]
- [framework] Improve logic for displaying help ([@Lakelezz]) [c:7937025]
- [http] Add `http::ratelimiting::offset` ([@zeyla]) [c:55555b8]
- [cache] Make the Cache Update API public ([@zeyla]) [c:9e56062]
- [utils] Add associated consts in `utils::Colour` ([@zeyla]) [c:bbfc8e2]
- [model] `impl From<&ID> for ID` for all Id types ([@zelya]) [c:9e45642],
  [c:530ea76]
- [cache] Add a Message cache API ([@zeyla]) [c:e602630]
- [voice] Add `streamer::ffmpeg_optioned` ([@zeyla]) [c:5dab87b], [c:1f3a57e]
- [model] Implement Mentionable for `GuildChannel` ([@Mishio595]) [c:ce8da79]
- [framework] Allow nil prefixes in DMs ([@acdenisSK]) [c:10bbffe]
- [model] Implement `Mentionable` for `ChannelCategory`, `Group`,
  `PrivateChannel` ([@zeyla]) [c:dd3744b], [c:8ce8234], [c:d11d916], [c:5abc7d1]
- [framework] Add checks for groups ([@Lakelezz]) [c:29480e5]
- [framework] Support multiple prefixes for command groups ([@Lakelezz])
  [c:305d200]
- [framework] Add default commands for command groups ([@Lakelezz]) [c:40c8248],
  [c:8aefde0]

### Fixed

- [framework] Handle debug impls better ([@acdenisSK]) [c:caeab28], [c:7eac4d5]
- [framework] Reorder some dispatch checks to fix an owner override bug
  ([@acdenisSK]) [c:8114a7a], [c:93f453b]
- [framework] Force `Args::find{,_n}` to be quote-aware ([@acdenisSK])
  [c:f0f06b7]
- [framework] Fix an `Args` test ([@zeyla]) [c:2ef660e]
- [framework] Fix command visibility on no help ([@Lakelezz]) [c:aeb89af]
- [framework] Add missing `Send + Sync` bounds on `Check` ([@acdenisSK])
  [c:f09b661]
- [utils] Fix `utils::is_nsfw` slicing ([@acdenisSK], [@zeyla]) [c:0067c33],
  [c:ccd2506]
- [utils] Fix `nsfw-` case in `utils::is_nsfw` ([@zeyla]) [c:bd4aa0a]
- [framework] Don't assume all characters at end are 1-length ([@acdenisSK])
  [c:4e4dcb1]
- [framework] Don't suggest command if no command is related to input
  ([@Lakelezz]) [c:614402f]

### Changed

- [model] Make `Invite::guild` and `RichInvite::guild` optional ([@zeyla])
  [c:3a647e3]

### Misc.

- [framework] Fix example typo ([@perryprog]) [c:d0d363f]
- [framework] Add more docs to `Args` ([@acdenisSK]) [c:04b0be1]
- [general] Fix extraneous spaces at the end of lines ([@zeyla]) [c:6ddfef8]
- [http] Add (late) april fool's functions ([@TheUnitedStatesOfAmerica])
  [c:5ffdcea]
- Rename https://github.com/serenity-rs/serenity/commit/6e1edde4a3fe27d0d90db7ea906ca5f115a2d5fb
- [framework] Remove some repitition repition ([@acdenisSK]) [c:10f7548],
  [c:1ec1086]
- [docs] Add more docs to `CreateEmbed::fields` ([@acdenisSK]) [c:703d135]
- [docs] Remove some dead links ([@acdenisSK], [@Lakelezz]) [c:eae624e],
  [c:4cf83d0]
- [docs] Remove old notice about `CreateEmbed::field` ([@acdenisSK]) [c:5b66ace]
- [examples] Add `CreateEmbed::field` and `CreateEmbed::fields` usage to example
  11 ([@drklee3]) [c:a9a2c27]
- [general] Monomorphize all functions ([@zeyla]) [c:7b9764c]
- [general] Update README logo URI ([@zeyla]) [c:2ff765b]
- [docs] Fix doc links with no anchor ([@zeyla]) [c:0d6e019]
- [docs] Add docs for `Args::new` ([@acdenisSK]) [c:b520ec7]
- [general] Fix some clippy lints ([@zeyla]) [c:9da7669]

## [0.5.4] - 2018-06-07

Thanks to the following for their contributions:

- [@acdenisSK]
- [@drklee3]
- [@Lakelezz]
- [@vityafx]
- [@zeyla]

### Added

- [model] Add `Message::member` structfield ([@zeyla]) [c:0e1e8fb]
- [docs] Document example binding names for EventHandler method signatures
  ([@acdenisSK]) [c:08a7110]
- [model] Implement `Mentionable` for `CurrentUser` ([@zeyla]) [c:4a24c90]
- [model] Implement `From<CurrentUser> for User` and
  `From<&CurrentUser> for User` ([@zeyla]) [c:af7f176]
- [framework] Add option for bots to work only in certain channels ([@vityafx])
  [c:457a17e]
- [framework] Differentiate in help whether a command is unavailable in DMs or
  guilds ([@Lakelezz]) [c:89a18aa]
- [framework] Improve `Args` docs ([@acdenisSK]) [c:2603063]
- [model] Add `Message::mentions_user_id`, `Message::mentions_user`
  ([@Lakelezz]) [c:1162e68]
- [docs] Update voice example 06 to make joining join the command invoker's
  voice channel ([@drklee3]) [c:a80aab2]

### Fixed

- [framework] Fix a framework example so it makes sense ([@acdenisSK])
  [c:63fe032]
- [model] Remove deadlocking in `Member::highest_role_info` ([@zeyla])
  [c:c659bbd]
- [framework] Dispatch to a threadpool only if required ([@Lakelezz])
  [c:23c5398]
- [framework] Fix strikethrough behaviour ([@Lakelezz]) [c:32c3bed]

### Misc.

- [general] Fix links to the new repo location ([@Lakelezz], [@zeyla])
  [c:152fe3d] [c:0324e01]
- [framework] Switch to `str::match_indices` for some Args ops ([@acdenisSK])
  [c:cc6b567]
- [framework] Remove `if length == 1` branch in Args functions ([@acdenisSK])
  [c:6346975]
- [framework] Optimize `Args::find`, `Args::find_n` ([@acdenisSK]) [c:5ba521b]
- [framework] Revamp `Args` from the ground up ([@acdenisSK]) [c:ff9edc0]

## [0.5.3] - 2018-05-01

Thanks to the following for their contributions:

- [@acdenisSK]
- [@FelixMcFelix]
- [@Lakelezz]
- [@zeyla]

### Added

- [http] Take `Date` header into account when ratelimiting ([@zeyla])
  [c:40db3c0]
- [general] Add new join messages ([@zeyla]) [c:36d7a54]

### Fixed

- [voice] Send silence frames upon connection ([@FelixMcFelix]) [c:83a0c85]
- [general] Remove spurious import warning ([@acdenisSK]) [c:64dcced]
- [docs] Fix dead link ([@Lakelezz]) [c:42063a2]
- [model] Fix "Guild Member Chunk" deserializations ([@zeyla]) [c:fd77a91]
- [voice] Fix voice hang ([@FelixMcFelix]) [c:e546fa2]
- [client] Fix panics on some guild member updates in certain situations
  ([@zeyla]) [c:526c366]

### Misc.

- [gateway] Clarify shard sequence-off log ([@zeyla]) [c:7f9c01e]
- [client] Log more information about failed deserializations ([@zeyla])
- [framework] Reword command macro docs ([@acdenisSK]) [c:a481df6]

## [0.5.2] - 2018-04-14

This release contains the usual bugfixes and helper methods.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@FelixMcFelix]
- [@ForsakenHarmony]
- [@jkcclemens]
- [@Lakelezz]
- [@megumisonoda]
- [@Roughsketch]
- [@Scetch]
- [@xentec]
- [@zeyla]

### Added

- [builder] Generalize `CreateEmbed` method parameters ([@acdenisSK])
  [c:f115c17]
- [http] Add 'Get Guild Vanity Url' endpoint ([@zeyla]) [c:dbfc06e]
- [framework] Add `unrecognized_command` method ([@Lakelezz]) [c:2937792]
- [client] Add documentation to `EventHandler` ([@acdenisSK]) [c:80dfcb0]
- [http] Support sending files with an embed ([@Scetch]) [c:7e0d908]

### Fixed

- [voice] Add `Drop` impl for ffmpeg container ([@FelixMcFelix]) [c:3d67a4e]
- [model] Pad user discrims in `content_safe` ([@megumisonoda]) [c:2ab714f]
- [framework] Properly check if `Args` input is empty ([@acdenisSK])
  [c:beebff5]
- [voice] Backport [c:7d162b9] (voice fixes) ([@FelixMcFelix]) [c:9baf167]
- [framework] Fix no-cache StandardFramework compilations ([@Lakelezz])
  [c:02dc506]
- [builder] Make `CreateEmbed` and `CreateMessage` consistent ([@acdenisSK])
  [c:77c399b]
- [framework] Fix `help` command precedence ([@acdenisSK]) [c:c6a5fe4]
- [gateway] Fix heartbeat checking ([@zeyla]) [c:21fe999]
- [framework] Fix `Args::is_empty` behaviour ([@acdenisSK]) [c:e5bcee7]
- [framework] Add `Args::full_quotes` ([@acdenisSK]) [c:24d2233]
- [http] Do not include Optional params if None for audit logs ([@jkcclemens])
  [c:bd195de]
- [model] Handle deserializing `AuditLogEntry::target_id` ([@acdenisSK])
  [c:0d779ba]
- [model] Fix `AuditLogOptions` to be correct types ([@acdenisSK], [@jkcclemens]) [c:217e1c6], [c:2791ed7]

### Misc.

- [builder] DRY in `CreateEmbed` builder methods
  ([@xentec], [@acdenisSK], [@zeyla]) [c:2e1eb4c] [c:d8c9d89], [c:a4cc582],
  [c:ffc5ea1]
- [builder] Inline some CreateEmbed builder methods ([@acdenisSK]) [c:e814e9a]
- [framework] Add tests for empty messages ([@Lakelezz]) [c:d0ae9bb]
- [general] Remove useless clones ([@Roughsketch]) [c:b71d99f]
- [framework] Add `no_run` to doctests that instantiate a Client
  ([@Roughsketch]) [c:003dc2e]
- [general] Don't create enums and IDs via macros ([@ForsakenHarmony])
  [c:fdcf44e]
- [framework] Short-circuit on errors ([@acdenisSK]) [c:82e21a6]
- [model, utils] Fix nsfw related docs ([@Lakelezz]) [c:7f09642]
- [framework] Improve docs for `Args` ([@acdenisSK]) [c:b9fa745]
- [general] Fix some documentatoin typos ([@Lakelezz]) [c:e506e9f]

## [0.5.1] - 2018-01-31

This release contains a number of fixes, a few more model helper methods, and
additional framework features.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@ConcurrentMarxistGC]
- [@FelixMcFelix]
- [@indiv0]
- [@Lakelezz]
- [@perryprog]
- [@zeyla]

### Added

- [framework] Add way to register middleware functions directly on
  `CreateCommand` ([@acdenisSK]) [c:d193975]
- [model] Add `Message::member` ([@zeyla]) [c:ce2952a]
- [http, model] Add functions to reorder a guild's channels ([@zeyla])
  [c:ab1f11a]
- [voice] Add multiple audio stream playback, volume control, and pausing
  ([@FelixMcfelix]) [c:324a288]

### Fixed

- [framework] Fix incorrect skipping for some prefixes ([@ConcurrentMarxistGC])
  [c:76bcf7d]
- [framework] Trim content after prefix mentions ([@Lakelezz]) [c:27c83e8]
- [voice] Strip RTP header extensions if present ([@indiv0]) [c:e4612ac]
- [voice] Fix voice websocket loop termination ([@indiv0]) [c:55fa37a]
- [model] Account for guild owners in member hierarchy check ([@zeyla])
  [c:03a7e3e]
- [model] Check message ID count in `delete_messages` ([@zeyla]) [c:92c91b8]
- [model] Correctly set newly created roles' positions on new roles ([@zeyla])
  [c:5a0b8a6]
- [voice] Fix an odd-to-use `Into<Option<Box<T>>>` bound ([@zeyla]) [c:eee3168]
- [framework] Fix case insensitivity for aliases ([@Lakelezz]) [c:d240074]
- [docs] Fix broken docs links caused by model module changes ([@zeyla])
  [c:8578d5f]

### Changed

### Misc.

- [general] Reduce number of clones in the library ([@zeyla]) [c:13b0de1]
- [example] Add voice receive example (example 10) ([@zeyla]) [c:b9a7e50]
- [examples, framework] Add docs for customised help functions ([@Lakelezz])
  [c:7912f23]
- [example] Add another message embed builder example ([@perryprog])
  [c:aba1ba6]

## [0.5.0] - 2018-01-20

This release is a rewrite of the client and gateway internals with a minimal
amount of breaking changes for userland code. These changes are mainly to
prepare for Tokio and to reduce the number of atomic operations per received
event, reducing the number of atomic operations by roughly 85%. The framework
has also seen a rewrite, and is now centered around a trait-based design.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@Caemor]
- [@ConcurrentMarxistGC]
- [@drklee3]
- [@fenhl]
- [@Flat]
- [@ftriquet]
- [@hsiW]
- [@indiv0]
- [@jhelwig]
- [@jkcclemens]
- [@Lakelezz]
- [@MOZGIII]
- [@nabijaczleweli]
- [@Roughsketch]
- [@tahahawa]
- [@thelearnerofcode]
- [@timotree3]
- [@zeyla]

### Upgrade Path

Per [c:91c8ec4], the `Guild::default_channel` and
`Guild::default_channel_guarenteed` methods now return
`Option<Arc<Mutex<GuildChannel>>>` instead of `Option<GuildChannel>`. This
avoids a clone. To access the channel, you just have to retrieve a read or write
lock by doing `guild.default_channel()?.read()` or
`guild.default_channel()?.write()`.

Per [c:14b9222], there is a new `Member::default_channel()` function that
returns the default channel for the user. This no longer returns the channel
with the same ID as the guild itself, as this behaviour was changed by Discord.
A member's "default channel" is now the top-most channel that it has permission
to view. Accordingly, `Guild::default_channel` matches this behaviour.

Per [c:93e0a42], the library now uses the `parking_lot` crate's `Mutex` and
`RwLock` implementations over the stdlib's. `parking_lot`s implementations are
more efficient, do not poison due to lock drops on unwinding, and implement
eventual fairness.

To account for this, change all `Mutex` lock retrievals and `RwLock` read and
write lock retrievals to not unwrap. `parking_lot`'s `Mutex::lock`,
`RwLock::read`, and `RwLock::write` don't return Results, unlike the `stdlib`'s.

Per [c:78c6df9], the `Guild::features` structfield is no longer a
`Vec<Feature>`. Discord adds guild features over time, which can cause guilds
with those new features to fail in deserialization. Instead, we're
future-proofing by making this a `Vec<String>`.

Per [c:65e3279], the `CreateEmbed` builder's `field` and `fields` functions no
longer take a builder as the argument, and instead take 3 arguments. For
example, code like this:

```rust
channel.send_message(|m| m
    .embed(|e| e
        .title("This is an embed")
        .field(|f| f
            .name("Test field")
            .value("Test value")
            .inline(true))));
```

Would now be this:

```rust
channel.send_message(|m| m
    .embed(|e| e
        .title("This is an embed")
        .field("Test field", "Test value", true)))
```

Per [c:ad0dcb3], shards can no longer have their `afk` property set, as this was
a leftover from user account support. This removes the `afk` parameter of the
`Context::set_presence` function, removal of the parameter from the
`Shard::set_presence` function, and the `Shard::set_afk` function.

Per [c:b328b3e], the `client::EventHandler` no longer prefixes all trait methods
with `on_`. An implementation that looks like this:

```rust
use serenity::client::{Context, EventHandler};
use serenity::model::Message;

struct Handler;

impl EventHandler for Handler {
    fn on_message(&self, _: Context, msg: Message) {
        // ...
    }
}
```

Now looks like this:

```rust
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        // ...
    }
}
```

(a note on the `serenity::model::channel::Message` import later.)

Per [c:b19b031], `Client::new` returns a `Result`, as it now creates some
essential information on instantiation instead of deferring it to when a
connection is started. You can probably just unwrap this Result.

Per [c:b8efeaf], [c:d5a9aa8], and [c:65233ad], the client and gateway internals
have been rebuilt to significantly reduce the number of atomic operations
(upwards of ~85%). This means that retrieval of shard information (like the
shard latency to the Discord gateway or the current connection status) are
retrieved via the encompassing [`ShardManager`][0.5.0:ShardManager] located on
the client. This can be inserted into the client's `data` structfield if you
need to access that information in event or framework command handlers. See
[this example][0.5.0:example-09] for more information. Additionally,
`Context::quit` to shutdown the shard no longer exists; go through the
`ShardManager` instead.

Per [c:aad4744], the framework's `Args::list` function has been renamed to
`Args::multiple` for consistency.

Per [c:f10b9d7], [c:1fd652b], [c:0aa55a2], the framework has been reworked to
be trait-based; thus as per [c:f61816c], [c:4e20277], allowed more useful functionality to commands.

Per [c:05f6ed4], the [client's close handle] has been removed, in favour of
doing so through the `ShardManager`.

Per [c:8c9baa7], the `Guild::default_message_notifications`, `Guild::mfa_level`,
`PartialGuild::default_message_notifications`, and `PartialGuild::mfa_level`
structfields are now enums to represent a stronger type, instead of `u64`s.

Per [c:bcd16dd], the `model` module has been broken up: instead of a giant root
module full of imports, types have been moved to where they fit. For example,
the `Message`, `Reaction`, and `Channel` structs are now in the `model::channel`
module. The `RichInvite`, `Member`, `Role`, and `MfaLevel` types are now in
`model::guild`. Refer to the commit message or the
[`model` module docs][0.5.0:model] for more information.

Per [c:be43836], the `http::HttpError` enum's `InvalidRequest` variant no longer
gives just the HTTP status code of the response. It now includes the full
Response instance.

Per [c:2edba81], the `builder` re-export in the `utils` module no longer exists
after being there in deprecation for a long time. Please import it like so:

```rust
// old
use serenity::utils::builder;

// new
use serenity::builder;
```

### Added

- [framework] Make the framework error's internal String public ([@acdenisSK])
[c:3b9f0f8]
- [client, gateway] Improve shard and shard runner logging ([@zeyla])
[c:f0ee805]
- [gateway] Have `ConnectionStage` derive `Copy` ([@acdenisSK]) [c:551f166]
- [builder, framework, http, model] Replace `Vec<T>` parameter with more generic
`IntoIterator<Item=T>` ([@ftriquet]) [c:b146501], [c:934eb3a]
- [builder, gateway, model, voice] Make more parameters generic with trait
bounds of `AsRef` ([@acdenisSK]) [c:e0e7617], [c:b62dfd4]
- [framework, model] Add help command filtering, member prefix searching
([@Lakelezz]) [c:ee207b3]
- [model] Add guild member filtering functions ([@Lakelezz]) [c:f26dad8]
- [model] `impl BanOptions for &str` ([@acdenisSK]) [c:7c911d5]
- [model] Derive `Default` on IDs and `CurrentUser` ([@acdenisSK]) [c:0881e18]
- [client] Add a threadpool for event dispatches ([@zeyla]) [c:1fa83f7],
[c:3e14067], [c:f2c21ef]
- [model] Fall back to `str::parse` if `parse_username` fails ([@acdenisSK])
[c:8c85664]
- [model] Add a parsing fallback for `RoleId` ([@acdenisSK]) [c:5d4301b]
- [http, model] Expand Audit Log support ([@acdenisSK]) [c:f491809]
- [framework] Make `Command::aliases` public ([@acdenisSK]) [c:8c83866]
- [model] `impl FromStr for ReactionType` ([@acdenisSK]) [c:2032a40],
[c:84706f1]
- [builder] Make trait bounds more generic, from `Into<String>` to `Display`
([@acdenisSK]) [c:05dad71]
- [framework, internal, model, utils] Derive `Debug` on more public types
([@thelearnerofcode]) [c:e5a6f3a]
- [model] Change `PrivateChannel::say` to accept a more generic argument
([@fenhl]) [c:a359f77]
- [model] `impl From<EmojiId, EmojiIdentifier> for ReactionType` ([@fenhl])
[c:68156c9]
- [http] `impl From<&Path> for AttachmentType` ([@zeyla]) [c:7a5aa3c]
- [model] Add `GameType::Listening` ([@hsiW], [@zeyla]) [c:40c5c12], [c:a17fea7]
- [framework] Add `cmd` function to `CreateCommand` and `CreateGroup`
([@acdenisSK]) [c:e748d1f]
- [model] Add `Reaction::message` function ([@Roughsketch]) [c:fd19446]
- [model] Add `Reaction::channel` function ([@zeyla]) [c:e02a842]
- [model] Add `Reaction::user` function ([@zeyla]) [c:82b87f1]
- [model] Implement `Deserialize` for `{,Gateway,Voice}Event` ([@zeyla])
[c:c3aa63f]
- [framework] Add `help()` to `CreateGroup` ([@Lakelezz]) [c:39a1435]
- [framework] Add a way to execute code when a command is registered
([@acdenisSK]) [c:f61816c]
- [framework] Add `before`/`after` middleware to `Command` ([@acdenisSK])
[c:4e20277]
- [general] Switch from `try_opt!` macro to using `?` operator ([@hsiW])
[c:2d23d8b]
- [framework] Make help commands customizable ([@Lakelezz]) [c:031fc92]
- [model] Add `VIEW_AUDIT_LOG` permission ([@Lakelezz]) [c:612e973]
- [model] Fallback to `str::parse` on `ChannelId` `FromStr` impl ([@acdenisSK])
[c:0525ede]
- [model] Add missing fields to `Guild` ([@zeyla]) [c:3d24033], [c:99d17d2],
[c:2abeea5]
- [framework] Add `Args::len` ([@acdenisSK]) [c:2c9b682], [c:b60d037],
[c:143fddd]
- [model] Add variant adapters to `Channel` ([@timotree3]) [c:f0a56f4]
- [model] Add `animated` field to `Emoji` and `ReactionType` ([@zeyla])
[c:f2fa349]
- [framework] Better support for multiple delimiters on `Args` ([@Lakelezz])
[c:62647f5]
- [model] Update `Region` to include new voice regions ([@Flat]) [c:d264cc3]
- [framework] Add `Args::iter_quoted` ([@acdenisSK]) [c:032c5a7]
- [model] Add missing `num` implementations on models ([@zeyla]) [c:0b1f684]
- [client] Add an event for shard connection changes ([@zeyla]) [c:7e46d8f]
- [model] Implement or derive `serde::Serialize` on all models ([@zeyla])
[c:25dddb6]
- [model] Further generic-ify `reaction_users`' `after` parameter ([@zeyla])
[c:85d7d5f]
- [model] Add `Member::highest_role` ([@zeyla]) [c:b7542f4]
- [model] Add `Guild::greater_member_hierarchy` ([@zeyla]) [c:84ff27b]
- [model] Allow channels to be moved in and out of a category ([@jkcclemens])
[c:6587655]
- [cache, model] Create partial member instances for users without a Member
instance ([@zeyla]) [c:d1113c0]

### Fixed

- [gateway] Improve shard reconnection logic ([@zeyla]) [c:45c1f27]
- [gateway] Reset shard heartbeat state on resume ([@zeyla]) [c:ae50886]
- [http] Make `webhook_id` a majour parameter in ratelimiting ([@zeyla])
[c:1735e57]
- [gateway] Resume on resumable session invalidations ([@zeyla]) [c:eb9e8df]
- [client] Fix setting of framework ([@zeyla]) [c:12317b9]
- [framework] Fix help commands to list all eligible commands in DMs
([@Lakelezz]) [c:114e43a]
- [framework] Fix command parsing behaviour when prefix has spaces
([@ConcurrentMarxistGC]) [c:10c56a9]
- [client] Attempt to restart failed shard boots ([@zeyla]) [c:8d68503]
- [client, gateway] Fix shards attempting to re-identify on their own ([@zeyla])
[c:e678883]
- [framework] Fix multiple char delimiters ([@zeyla]) [c:08febb0]
- [framework] Fix `multiple_quoted` ([@Lakelezz]) [c:9aad1aa]
- [model] Fix `#` finding in `Guild::member_named` ([@tahahawa]) [c:a7b67df]
- [builder] Convert embed footers for `impl Form<Embed> for CreateEmbed`
([@drklee3]) [c:9aaa555]
- [framework] Fix plain help command ([@Lakelezz]) [c:4bd223a]
- [model] Correctly iterate over channel permission overwrites in permission
building ([@zeyla]) [c:7566f32]
- [model] Compare instants in `Shard::latency`, avoiding panics ([@zeyla])
[c:08db9fa]
- [model] Add some role hierarchy position checks ([@zeyla]) [c:222382c]
- [framework] Add missing `correct roles` checks in help commands ([@Lakelezz])
[c:470f366]
- [framework] Fix multibyte character-based prefixes ([@ConcurrentMarxistGC])
[c:e611776]

### Changed

- [framework] Change the way users' command handlers are stored ([@acdenisSK])
[c:d90b90c]
- [model] `Guild::{default_channel, default_channel_guarenteed}` now return
an `Arc<Mutex<GuildChannel>>` instead of a clone of the channel ([@acdenisSK])
[c:91c8ec4]
- [framework] Don't default command argument delimiter to `" "` ([@jhelwig])
[c:3a4cb18]
- [model] Change behaviour of `default_channel` to match Discord's new
behaviour ([@hsiW]) [c:14b9222]
- [utils] Disallow Message Builder `I` from being user-implemented
([@acdenisSK]) [c:7cf1e52]
- [general] Switch to `parking_lot::{Mutex, RwLock}` ([@zeyla]) [c:93e0a42]
- [model] Make `{Guild, PartialGuild}::features` a `Vec<String>` ([@zeyla])
[c:78c6df9]
- [builder] Slightly change performance of builders by using `&'static str`s and
a `VecMap` ([@acdenisSK], [@zeyla]) [c:9908999], [c:3a0c890], [c:26fe139]
- [builder] Change `CreateEmbed::field{,s}` to not take builders ([@zeyla])
[c:65e3279]
- [client, gateway] Remove setting of a shard's `afk` field ([@zeyla])
[c:ad0dcb3]
- [client] Remove `on_` prefix to `EventHandler` tymethods ([@zeyla])
[c:b328b3e]
- [client] Make the Client return a result [c:b19b031]
- [client, gateway] Redo client+gateway internals to reduce atomic operations
([@zeyla]) [c:b8efeaf], [c:d5a9aa8], [c:65233ad]
- [framework] Rename `Args::list` -> `Args::multiple` ([@acdenisSK]) [c:aad4744]
- [framework] Make framework use trait-based commands ([@acdenisSK])
[c:f10b9d7], [c:1fd652b], [c:0aa55a2]
- [client] Remove client close handle ([@zeyla]) [c:05f6ed4]
- [model] Change types of some `Guild` and `PartialGuild` structfields
([@zeyla]) [c:8c9baa7]
- [model] Break up the model module ([@zeyla]) [c:bcd16dd]
- [http] Give the full hyper Response in HTTP errors ([@zeyla]) [c:be43836]
- [utils] Remove `builder` module re-export ([@zeyla]) [c:2edba81]
- [framework] Remove `is_bot` state boolean ([@zeyla]) [c:524b8f8]
- [client, framework, gateway, voice] Use an encompassing `InterMessage` enum to
communicate over the gateway ([@zeyla]) [c:9232b8f]

### Misc.

- [general] Simplify `Error`'s `Display` impl ([@zeyla]) [c:ee2bbca]
- [framework] Document that application owners bypass checks ([@fenhl])
[c:b215457]
- [general] Compile all features for docs.rs ([@zeyla]) [c:a96be90]
- [model] Document that `Reaction::{message, users}` methods hit the API
([@zeyla]) [c:141bbfc]
- [builder] Use `ToString` blanket impl for `Display`s ([@acdenisSK])
[c:3ca7e15]
- [framework] Avoid an unwrap in `Args::parse_quotes` ([@zeyla]) [c:60613ef]
- [client] Trim token given in `Client::new` ([@zeyla]) [c:25d79ac]
- [model] Fix a doc typo on `User` ([@Lakelezz]) [c:9da642a]
- [model] Fix docs for `User::has_role` ([@zeyla]) [c:b52eb9f]

[client's close handle]: https://docs.rs/serenity/0.4.7/serenity/client/struct.CloseHandle.html
[0.5.0:ShardManager]: https://docs.rs/serenity/0.5.0/serenity/client/bridge/gateway/struct.ShardManager.html
[0.5.0:example-09]: https://github.com/serenity-rs/serenity/blob/91cf5cd401d09a3bca7c2573b88f2e3beb9c0948/examples/09_shard_manager/src/main.rs
[0.5.0:model]: https://docs.rs/serenity/0.5.0/serenity/model/index.html

## [0.4.5] - 2017-12-09

This release contains a hotfix for the hotfix release, as well as a slight
behaviour change to the `EditRole` builder.

The last release contained a deserialization implementation fix which seemed to
work after running tests, but it turns out that not all deserialization issues
were fixed.

The `EditRole` builder's Default implementation no longer sets a value for each
field, as this causes problems with stateless editing of roles.

### Fixed

- [model] Fix remaining deserializers [c:52403a5]

### Changed

- [builder] Remove `EditRole::default` implementation [c:795eaa1]

## [0.4.4] - 2017-12-09

This release contains a hotfix for snowflake deserialization on `serde_json`
v1.0.8. Primary development is continuing on the v0.5.x branch and the
[library organization].

### Fixed

- [model] Fix snowflake deserializer [c:77f462e]

## [0.4.3] - 2017-11-01

This release contains bugfixes and marks the final release of the v0.4.x branch.
Future development will continue on the v0.5.x branch.

Thanks to the following for their contributions:

- [@acdenisSK]
- [@ThatsNoMoon]
- [@zeyla]

### Added

- [model] Add `Guild::member_permissions` ([@zeyla]) [c:2ba4d03]

### Changed

- [model] Rename `Guild::permissions_for` to `Guild::permissions_in`, keep an
  alias ([@zeyla]) [c:dcac271]

### Fixed

- [model] Make `Member::permissions` return guild-level permissions ([@zeyla])
  [c:d3eddc6]

### Misc.

- [model] Add some docs to `BanOptions` ([@acdenisSK]) [c:c99091d]
- [model] Have `Guild::has_perms` use `Guild::member_permissions` ([@zeyla])
  [c:1b7101f]
- [http] Slightly clarify ratelimiting documentation ([@zeyla]) [c:3be6e2e]
- [docs] Fix ping bot example ([@ThatsNoMoon]) [c:800e58f]
- [docs] Use consistent token names in examples ([@zeyla]) [c:e219a6a]

## [0.4.2] - 2017-10-29

This release contains the regular bugfixes, new features and slight behaviour
changes.

Thanks to the following people for their contributions:

- [@acdenisSK]
- [@efyang]
- [@Caemor]
- [@Flat]
- [@hsiW]
- [@Lakelezz]
- [@ConcurrentMarxistGC]
- [@zeyla]

### Added

- [general] Add a way to change a role's position ([@Flat]) [c:f47a0c8]
- [general] Add logging and dotenv to example 07 ([@zeyla]) [c:d50b129]
- [general] Add owner + quit function to example 07 ([@zeyla]) [c:41f26b3]
- [framework] Add `PartialEq` impls and doc-tests to `Args` ([@acdenisSK]) [c:f9e5e76]
- [framework] Add "zero-copy" parsing to `Args` ([@acdenisSK]) [c:9428787]
- [framework] Add a debug impl to `DispatchError` ([@acdenisSK]) [c:a58de97]

### Fixed

- [general] Fix clippy warnings ([@hsiW]) [c:fbd6258]
- [model] Fall back to `str::parse` if `utils::parse_username` fails ([@acdenisSK]) [c:292ceda]
- [model] Fix `User::has_role` ([@zeyla]) [c:d3015a0ff]
- [gateway] Fix shard connection ([@zeyla]) [c:585ac6e]
- [gateway] Fix shard shutdown via `Context` ([@zeyla]) [c:3616585]
- [framework] Fix `allow_whitespace` ([@ConcurrentMarxistGC]) [c:e694766]
- [framework, gateway, cache] Properly update emojis in the cache, fix shard re-tries and do some cleanup to `help_commands.rs` ([@Lakelezz]) [c:e02d5fb]

### Changed

- [model] Do equality and hashing on just the user's id ([@acdenisSK]) [c:b7cdf15]
- [model] defer to `delete_message` if there's just one message to delete ([@acdenisSK]) [c:c7aa27d]
- [model] Use the underlaying integer value of `ChannelType` ([@acdenisSK]) [c:e57b510]

### Misc.

- [general] Update dependencies ([@zeyla]) [c:2219bb3]
- [general] Re-export parking_lot's `Mutex` and `RwLock` from the prelude ([@zeyla]) [c:74ec713]
- [general] Update the version in `Cargo.toml` to actually be `v0.4.2` ([@Caemor]) [c:5829c67]
- [general] Cleanup gitignore to have comments ([@hsiW]) [c:ce4f8c2]
- [gateway] Use update syntax for `Shard` ([@efyang]) [c:fcc4e2c]
- [model] Deprecate some methods on `Channel` ([@zeyla]) [c:23ff6f]

## [0.4.1] - 2017-10-14

This release contains bugfixes and some newly added or newly exposed
functionality.

Thanks to the following for their contributions this release:

- [@acdenisSK]
- [@ftriquet]
- [@hsiW]
- [@Lakelezz]
- [@lolzballs]
- [@Roughsketch]
- [@zeyla]

### Added

- [general] Replace Vec parameters by `IntoIterator` ([@ftriquet])[c:55167c3]
- [general] Replace slice parameters by `IntoIterator` ([@ftriquet]) [c:022e35d]
- [model] Add `Guild::members_starting_with` ([@Lakelezz]) [c:b3aa441]
- [model] Add `Guild::members_containing` ([@Lakelezz]) [c:1b167b5]
- [model] `impl<'a> BanOptions for &'a str` ([@acdenisSK]) [c:cf40386]
- [model] Derive `Default` on `CurrentUser` and IDs ([@acdenisSK]) [c:09a8a44]
- [client] Add a configurable, shard-shared threadpool ([@zeyla]) [c:d7621aa],
  [c:8109619]
- [model] Add `Guild::members_username_containing, members_nick_containing`
  ([@Lakelezz]) [c:002ce3a]
- [framework] Add an iterator for `Args` ([@acdenisSK]) [c:0ed1972]
- [framework] Make `has_all_requirements` public ([@Lakelezz]) [c:08d390c]
- [framework] Make default help messages list help for aliases ([@Lakelezz])
  [c:0d1c0f1]

### Fixed

- [model] Use `request_client!` for attachment downloading ([@lolzballs])
  [c:71f709d]
- [client] Fix client no-framework compilation ([@zeyla]) [c:1d4ecb2]
- [client] Fix client shards not filling ([@zeyla]) [c:86d8bdd]
- [model] Fix `User::tag` and `CurrentUser::tag` discrim output ([@zeyla])
  [c:6b9dcf5]
- [framework] Modify `initialized` method purpose ([@acdenisSK]) [c:05f158f]
- [framework] Make command Error string public ([@acdenisSK]) [c:917dd30]
- [client, gateway] Improve shard logic ([@acdenisSK], [@zeyla]) [c:683691f],
  [c:7befcd5]
- [gateway] Reset shard heartbeat state on resume ([@zeyla]) [c:c98cae4]
- [general] Fix font-height and soften the logo ([@Lakelezz]) [c:3b2c246]

### Misc.

- [client, gateway] Improve shard and shard runner logging ([@zeyla])
  [c:21e194b]
- `to_owned` -> `to_string` ([@acdenisSK]) [c:1bf4d9c]
- [general] Fix most clippy warnings ([@Roughsketch]) [c:7945094]
- [framework] Add some docs to `Args` ([@acdenisSK]) [c:8572943]
- [examples] Add `env_logger` bot example [c:0df77b9]
- [general] Fix clippy lints ([@zeyla]) [c:483b069]
- [model] Optimize `Member::roles` ([@hsiW]) [c:8565fa2]
- [general] Internally use a `try_opt!` macro ([@hsiW]) [c:9b0c053]
- [general] Feature-flag extern crates behind their name ([@zeyla]) [c:11b85ca]

## [0.4.0] - 2017-09-25

This release contains a lot of added functionality, minor-scale rewrites,
bugfixes, documentation work, and the beginning of a rewrite to use the tokio
ecosystem.

The release was delayed due to a [fairly majour bug][rust-websocket:issue:137]
in rust-websocket that we have forked over to temporarily fix.

This release was lead in development by [@acdenisSK].

Thanks to the following for their contributions this release:

- [@acdenisSK]
- [@Arcterus]
- [@Bond-009]
- [@blaenk]
- [@hsiW]
- [@imnotbad]
- [@joek13]
- [@Lakelezz]
- [@Roughsketch]
- [@xentec]
- [@zeyla]

### Upgrade Path

Per commits [c:af1061b], [c:cdedf36], and [c:aa307b1], Direct Messaging other
bot users is now disallowed by the API. To fix this, simply don't do it.

Per commit [c:ebc4e51], deprecated functions were finally removed. The following
can simply have their usage renamed:

- `Cache::get_channel` --> `Cache::channel`
- `Cache::get_guild` --> `Cache::guild`
- `Cache::get_guild_channel` --> `Cache::guild_channel`
- `Cache::get_member` --> `Cache::member`
- `Cache::get_private_channel` --> `Cache::private_channel`
- `Cache::get_role` --> `Cache::role`
- `Cache::get_user` --> `Cache::user`
- `ChannelId::get_invites` --> `ChannelId::invites`
- `ChannelId::get_message` --> `ChannelId::message`
- `ChannelId::get_messages` --> `ChannelId::messages`
- `ChannelId::get_reaction_users` --> `ChannelId::get_reaction_users`
- `ChannelId::get_webhooks` --> `ChannelId::webhooks`
- `Channel::get_message` --> `Channel::message`
- `Channel::get_messages` --> `Channel::messages`
- `Channel::get_reaction_users` --> `Channel::reaction_users`
- `Client::login_bot` --> `Client::new`
- `Client::login` --> `Client::new`
- `Colour::get_b` --> `Colour::b`
- `Colour::get_g` --> `Colour::g`
- `Colour::get_r` --> `Colour::r`
- `Colour::get_tuple` --> `Colour::tuple`
- `CurrentUser::distinct` --> `CurrentUser::tag`
- `Group::get_message` --> `Group::message`
- `Group::get_messages` --> `Group::messages`
- `Group::get_reaction_users` --> `Group::reaction_users`
- `Guild::get_bans` --> `Guild::bans`
- `Guild::get_channels` --> `Guild::channels`
- `Guild::get_emoji` --> `Guild::emoji`
- `Guild::get_emojis` --> `Guild::emojis`
- `Guild::get_integrations` --> `Guild::integrations`
- `Guild::get_invites` --> `Guild::invites`
- `Guild::get_member` --> `Guild::member`
- `Guild::get_members` --> `Guild::members`
- `Guild::get_member_named` --> `Guild::member_named`
- `Guild::get_prune_count` --> `Guild::prune_count`
- `Guild::get_webhooks` --> `Guild::webhooks`
- `GuildId::get_bans` --> `GuildId::bans`
- `GuildId::get_channels` --> `GuildId::channels`
- `GuildId::get_emoji` --> `GuildId::emoji`
- `GuildId::get_emojis` --> `GuildId::emojis`
- `GuildId::get_integrations` --> `GuildId::integrations`
- `GuildId::get_invites` --> `GuildId::invites`
- `GuildId::get_member` --> `GuildId::member`
- `GuildId::get_members` --> `GuildId::members`
- `GuildId::get_prune_count` --> `GuildId::prune_count`
- `GuildId::get_webhooks` --> `GuildId::webhooks`
- `Message::get_reaction_users` --> `Message::reaction_users`
- `PartialGuild::get_bans` --> `PartialGuild::bans`
- `PartialGuild::get_channels` --> `PartialGuild::channels`
- `PartialGuild::get_emoji` --> `PartialGuild::emoji`
- `PartialGuild::get_emojis` --> `PartialGuild::emojis`
- `PartialGuild::get_integrations` --> `PartialGuild::integrations`
- `PartialGuild::get_invites` --> `PartialGuild::invites`
- `PartialGuild::get_member` --> `PartialGuild::member`
- `PartialGuild::get_members` --> `PartialGuild::members`
- `PartialGuild::get_prune_count` --> `PartialGuild::prune_count`
- `PartialGuild::get_webhooks` --> `PartialGuild::webhooks`
- `PrivateChannel::get_message` --> `PrivateChannel::message`
- `PrivateChannel::get_messages` --> `PrivateChannel::messages`
- `PrivateChannel::get_reaction_users` --> `PrivateChannel::reaction_users`
- `Role::edit_role` --> `Role::edit`
- `User::distinct` --> `User::tag`

`http::send_file` has been replaced by `http::send_files`. Instead of using `http::send_file` like so:

```rust
use serde_json::Map;
use serenity::http;
use serenity::model::ChannelId;
use std::fs::File;

let channel_id = ChannelId(253635665344987136);
let filename = "mr-sakamoto.png";
let file = File::open(&format!("./img/{}", filename))?;
let map = Map::<String, Value>::new();

http::send_file(channel_id, file, filename, map)?;
```

Instead send an attachment of files, such as:

```rust
use serde_json::Map;
use serenity::http;
use serenity::model::ChannelId;
use std::fs::File;

let channel_id = ChannelId(253635665344987136);
let files = vec![
    (File::open(&format!("./img/{}", filename))?, filename),
];
let map = Map::<String, Value>::new();

http::send_files(channel_id, files, map)?;
```

Similar logic can be applied to shortcut methods which have been removed,
namely:

- `Channel::send_file` (instead use `Channel::send_files`)
- `ChannelId::send_file` (instead use `ChannelId::send_files`)
- `Group::send_file` (instead use `Group::send_files`)
- `GuildChannel::send_file` (instead use `GuildChannel::send_files`)
- `PrivateChannel::send_file` (instead use `PrivateChannel::send_files`)

Instead of using the now-removed `Channel::delete_messages` and
`Channel::delete_permission`, use the inner channel's method:

```rust
use serenity::model::{Channel, ChannelId};

let channel = ChannelId(253635665344987136).get()?;
let message_ids = vec![
    MessageId(359845483356749825),
    MessageId(359854838403694592),
];

if let Channel::Guild(c) = channel {
    c.delete_messages(&message_ids)?;
}
```

Similar logic can be applied to `Channel::delete_permission`.

`Member::find_guild` ended up being only a shortcut to the `Member::guild_id`
structfield. Instead of calling the `find_guild` method like
`member.find_guild()`, instead access the structfield directly via
`member.guild_id`.

The `model::permissions::{general, text, voice}` methods have been removed, as
they ended up being shortcuts to the `model::permissions::PRESET_GENERAL`,
`model::permissions::PRESET_TEXT`, and `model::permissions::PRESET_VOICE`
constants, respectively.

Per commit [c:ea432af], event handling is now done via implementing a trait.
Instead of passing functions to the client directly like:

```rust
use serenity::Client;
use std::env;

let mut client = Client::new(env::var("DISCORD_TOKEN")?);

client.on_message(|ctx, msg| {
    // code
});
```

Instead implement the new EventHandler trait:

```rust
use serenity::client::{Client, Context, EventHandler};
use serenity::model::Message;

struct Handler;

impl EventHandler for Handler {
    fn on_message(&self, ctx: Context, msg: Message) {
        // code
    }
}

let client = Client::new(env::var("DISCORD_TOKEN")?);
```

Per commit [c:4f2e47f], the deprecated `ext` module (which has recently only
been a series of re-exports for the `cache`, `framework`, and `voice` modules)
was removed. Instead of using `serenity::ext::cache` for example, use
`serenity::cache`.

Per commit [c:878684f], due to the concept of default channels being changed,
`GuildId::as_channel_id` has been deprecated due to the fact that the ID of the
default channel of a guild will no longer necessarily be the same as the guild's
ID.

If you require this _same exact functionality_ (the `GuildId` as a `ChannelId`),
rewrite your code from:

```rust
use serenity::model::GuildId;

let channel_id = GuildId(81384788765712384).as_channel_id();
```

to:

```rust
use serenity::model::{ChannelId, GuildId};

let guild_id = GuildId(81384788765712384);
let channel_id = ChannelId(guild_id.0);
```

Per commits [c:2b053ea], [c:8cc2300], [c:8e29694], and [c:948b27c], custom
frameworks can now be implemented, meaning that a built implementation is now
passed instead of a base framework being provided and mutated. To use the old
framework, modify code from:

```rust
use serenity::Client;
use std::env;

let mut client = Client::new(&env::var("DISCORD_TOKEN")?);

client.with_framework(|f| f
    // method calls to mutate framework here
);
```

to the new style:

```rust
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;
use std::env;

struct Handler;

impl EventHandler for Handler { }

let mut client = Client::new(&env::var("DISCORD_TOKEN")?, Handler);

client.with_framework(StandardFramework::new()
    // method calls here to mutate framework here
);
```

Per commit [c:fc9eba3d], if you were pattern matching on the
`serenity::framework::DispatchError::CheckFailed` variant, instead either use or
ignore the matched data by rewriting code from:

```rust
use serenity::framework::DispatchError;

// Code to begin dispatch error handling here.

match dispatch_error {
    DispatchError::CheckFailed => {
        // Handle operation here.
    },
    // Other variants.
}
```

to:

```rust
// The standard implementation is now in a "standard" framework module, but
// that's unrelated.
use serenity::framework::standard::DispatchError;

match dispatch_error {
    DispatchError::CheckFailed(_) => {
        // Handle operation here.
    },
    // Other variants.
}
```

Per commits [c:45d72ef], [c:03b6d78], and [c:d35d719], the framework's
`command!` macro no longer parses arguments' types for you. You are now given an
`Args` struct that you can retrieve arguments from and parse from to a requested
type that implements `FromStr`.

For example, a simple sum function that looked like:

```rust
#[macro_use] extern crate serenity;

command!(sum(_ctx, msg, _args, x: i64, y: i64) {
    let _ = msg.reply(&format!("Result: {}", x + y));
});
```

Now looks like:

```rust
use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::Message;

fn sum(_: &mut Context, msg: &Message, args: Args) -> Result<(), String> {
    let x = match args.single::<i64>() {
        Ok(x) => x,
        Err(_) => return Ok(()),
    };
    let y = match args.single::<i64>() {
        Ok(y) => y,
        Err(_) => return Ok(()),
    };

    let _ = msg.reply(&format!("Result: {}", x + y));
}
```

Per commit [c:562ce49], `serenity::model::User`'s `FromStr` implementation can
now hit the REST API. No code changes required, but do note the possibility.

Per commit [c:40031d9], the following routes have been removed for being userbot
routes, which are leftovers from when serenity supported them and had them
removed:

- `http::get_application_info`
- `http::get_applications`
- `http::get_emoji`
- `http::get_emojis`
- `model::Guild::emoji`
- `model::Guild::emojis`
- `model::GuildId::emoji`
- `model::GuildId::emojis`
- `model::PartialGuild::emoji`
- `model::PartialGuild::emojis`

Per commit [c:092f288], bitflags has been upgraded, which introduces a minor
change in how to use permissions.

Update code from:

```rust
use serenity::model::permissions::{ADD_REACTIONS, MANAGE_MESSAGES};

foo(vec![ADD_REACTIONS, MANAGE_MESSAGES]);
```

to:

```rust
use serenity::model::Permissions;

foo(vec![Permissions::ADD_REACTIONS, Permissions::MANAGE_MESSAGES]);
```

### Added

- [framework] Make `CommandOrAlias` and `CommandGroup.commands`
  public ([@joek13]) [c:3db42c9]
- [builder] Add support for sending attachments in embeds ([@acdenisSK])
  [c:c68d4d5]
- [client] Add an `on_cached` event ([@acdenisSK]) [c:6d6063f]
- [framework] Add reaction actions
- [client] Add shard shutdown shortcut to the context ([@acdenisSK]) [c:561b0e3]
- [client] Add `is_new` paramenter to the `guild_create` handler ([@acdenisSK])
  [c:3017f6d]
- [http, model] Add ban reasons ([@acdenisSK]) [c:420f9bd], [c:8a33329],
  [c:710fa02], [c:421c709]
- [model] Add `Guild::members_with_status` ([@acdenisSK]) [c:a7a0945],
  [c:29ee627]
- [model] Make `Ban` and `User` impl `Eq`, `Hash`, and `PartialEq`
  ([@acdenisSK]) [c:64bfc54]
- [model] Return error if user exceeds reason limit ([@acdenisSK]) [c:60c33db],
  [c:25d4931]
- [builder] Add method to add multiple embed fields ([@acdenisSK]) [c:dbd6727]
- [model] Make `BanOptions` take and return an `&str` ([@acdenisSK]) [c:1ab8b31]
- [framework] Provide the command to checks ([@acdenisSK]) [c:eb47559]
- [model] Add `{ChannelId, GuildChannel, PrivateChannel}::name` functions
  ([@acdenisSK]) [c:ca0f113]
- [client] Switch to tokio for events ([@Arcterus]) [c:88765d0]
- [client] Add method to close all shards explicitly ([@acdenisSK]) [c:4d4e9dc],
  [c:c2cf691], [c:c7b8ab8], [c:9900b20], [c:d8027d7], [c:051d23d]
- [framework] Implement adding checks to buckets ([@acdenisSK]) [c:dc3a4df]
- [client] Handle the closing of shards ([@blaenk]) [c:5fd3509]
- [client] Make `CloseHandle` derive `Copy` ([@blaenk]) [c:b249c82]
- [model] Add `nsfw` property to channels ([@acdenisSK], [@Bond-009])
  [c:b602805], [c:fd89d09], [c:fd47b86]
- [http, model] Add audit log support ([@acdenisSK]) [c:6a101c4], [c:4532e4a],
  [c:9ccf388], [c:1fad3dd], [c:e2053dd]
- [model] Add `Message::is_own` ([@acdenisSK], [@zeyla]) [c:5a96724],
  [c:fdbfbe0], [c:6572580]
- [utils] Implement `From<(u8, u8, u8)> for Colour` ([@acdenisSK]) [c:6f147e1]
- [builder, model] Make some functions accept a `Display` bound instead of
  `&str` ([@acdenisSK]) [c:7e913b6], [c:05162aa], [c:0810ab7]
- [model] Add simulated default channel methods ([@acdenisSK]) [c:878684f]
- [framework] Add support for custom delimiters ([@acdenisSK]) [c:125c1b8],
  [c:fdfb184]
- [framework] Provide Args to checks ([@acdenisSK], [@Roughsketch]) [c:005437f],
  [c:68c5be8], [c:26919cf], [c:25e91da], [c:ab67c1d], [c:caf69d6]
- [model] Use cache when possible in `UserId::get` ([@Roughsketch]) [c:bfdb57c]
- [utils] Add `with_config{,_mut}` ([@acdenisSK]) [c:1a08904]
- [voice] Add ability to play DCA and Opus ([@Roughsketch]) [c:3e0b103],
  [c:e1a8fe3]
- [model] Add `{Guild,PartialGuild}::role_by_name ([@Lakelezz]) [c:f6fcf32]
- [framework] Add `CreateCommand::num_args` ([@Roughsketch]) [c:aace5fd]
- [framework] Add case insensitive command name support ([@acdenisSK])
  [c:deee38d]
- [framework] Allow commands to be limited to roles ([@Lakelezz]) [c:d925f92]
- [client] Add a way for users to get shards ([@zeyla]) [c:619a91d]
- [cache, client, model] Add channel category support ([@acdenisSK], [@zeyla])
  [c:4be6b9d], [c:870a2a5], [c:192ac8a], [c:485ad29], [c:52b8e29]
- [client] Add `Context::handle` ([@acdenisSK]) [c:97e84fe]
- [framework] Copy some functionality from Command to Group ([@Roughsketch])
  [c:8e1435f]

### Fixed

- [client] Return websocket pings with a pong ([@acdenisSK]) [c:824f8cb],
  [c:e218ce0], [c:e72e25c], [c:bd05bda]
- [utils] Fix `MessageBuilder::push_mono_safe`
- [framework] Fix args when `use_quotes` is active ([@acdenisSK]) [c:e7a5ba3]
- [model] Make `Reaction::name` optional ([@acdenisSK]) [c:8f37f78]
- [gateway] Fix presence updates due to API change ([@Roughsketch]) [c:16a5828]
- [model] Fix `permissions::PRESET_GENERAL` bits ([@zeyla]) [c:9f02720]
- [http] Update deprecated bulk delete endpoint ([@zeyla]) [c:dbcb351]
- [client] Fix subtraction overflow on guild cached dispatch ([@Roughsketch])
  [c:f830f31]
- [framework] Fix admin permission check ([@Lakelezz]) [c:2fb12e2]
- [general] Fix compiles of a variety of feature combinations ([@zeyla])
  [c:8e3b4d6]
- [client] Fix spawning of multiple events (non-v0.3 bug) ([@zeyla]) [c:7c4b052]
- [framework] Add Send/Sync to framework items (non-v0.3 bug) ([@zeyla])
  [c:50d7f00]

### Changed

- [model] Prevent Direct Messaging other bot users ([@zeyla]) [c:af1061b],
  [c:266411c]
- [cache, client] Apply API changes for bot DMs ([@acdenisSK]) [c:cdedf36],
  [c:aa307b1]
- [client] Switch to a trait-based event handler ([@acdenisSK]) [c:ea432af]
- [cache, client, http, model, utils] Remove deprecated functions ([@acdenisSK])
  [c:ebc4e51]
- [framework] Allow custom framework implementations ([@acdenisSK], [@zeyla])
  [c:2b053ea], [c:8cc2300], [c:8e29694], [c:948b27c]
- [general] Remove the BC-purposed `ext` module ([@acdenisSK]) [c:4f2e47f]
- [model] Deprecate `GuildId::as_channel_id` ([@acdenisSK]) [c:878684f]
- [utils] Remove `I` bound for MessageBuilder language params ([@acdenisSK])
  [c:f16af97]
- [cache] Split event handling to a trait ([@acdenisSK]) [c:eee857a],
  [c:32de2cb], [c:bc3491c]
- [framework] Provide command to `DispatchError::CheckFailed` ([@Lakelezz])
  [c:fc9eba3]
- [framework] Provide arguments as an iterable struct
  ([@acdenisSK], [@Roughsketch]) [c:106a4d5], [c:428cbb9], [c:45d72ef],
  [c:03b6d78], [c:d35d719]
- [model] Provide useful user/role/channel id `FromStr` parsing errors
  ([@acdenisSK]) [c:8bf77fa], [c:8d51ead]
- [model] Allow `User`'s `FromStr` impl to hit REST ([@Roughsketch]) [c:562ce49]
- [http] Remove remaining userbot endpoints ([@zeyla]) [c:40031d9]
- [general] Update bitflags, other dependencies ([@zeyla]) [c:092f288]

### Misc.

- [model] Fix a `ModelError` doctest ([@zeyla]) [c:bd9fcf7]
- [docs] Various docs fixes ([@hsiW]) [c:f05efce]
- [docs] Update links to docs ([@zeyla]) [c:78e7b1b]
- [general] Fix clippy warnings ([@imnotbad]) [c:e1912c2]
- [docs] Update to add `EventHandler` ([@acdenisSK]) [c:fdfd5bc]
- [examples] Update examples ([@acdenisSK], [@Roughsketch]) [c:3582691],
  [c:4e360cf]
- [docs] Fix doctests from `EventHandler` changes ([@acdenisSK]) [c:511ec87]
- [docs] Update readme to use correct docs link ([@acdenisSK]) [c:0240717]
- [client] Add a macro for reaction dispatching ([@acdenisSK]) [c:4efe1d1]
- [framework] Simplify an iterator usage ([@acdenisSK]) [c:fbc1ac7]
- [general] Fix clippy warnings ([@imnotbad]) [c:b6af867]
- [docs] Fix the doc on `PrivateChannel::name` ([@acdenisSK]) [c:14fd41b]
- [model, voice] Use stabilized loop-with-break-value ([@acdenisSK]) [c:f5a97d4]
- [model] Change a `match` to an `and_then` ([@acdenisSK]) [c:5e5f161]
- [framework] Make bucket checks less cache dependent ([@acdenisSK]) [c:ea1eba8]
- [framework] Remove unnecessary `Send + Sync` bounds ([@acdenisSK]) [c:3c2716b]
- [client, framework, http, utils] Remove some clones ([@acdenisSK]) [c:0d6965f]
- [cache] Remove an unnecessary map ([@acdenisSK]) [c:924c447]
- [general] Make Travis test on osx ([@Arcterus]) [c:fb2a1a9]
- [cache] Ignore private channels on create if already cached ([@acdenisSK],
  [@Lakelezz]) [c:7e8da0c], [c:e5889ed], [c:069df4f]
- [examples] Document example 05 more heavily ([@Lakelezz]) [c:0186754]
- [examples] Fix listed feature requirements in examples ([@zeyla]) [c:078947e]
- [http] Document and un-hide `http::set_token` ([@zeyla]) [c:cb18d42]
- [model] Refactor Display impl for Ids ([@acdenisSK]) [c:47ea8f7]
- [client] Add a sharding manager base ([@zeyla]) [c:6c43fed]

## [0.3.0] - 2017-06-24

This release contains a number of added methods, fixes, deprecations, and
documentation improvements. It brings a module restructure and an upgrade to
rust-websocket v0.20, hyper v0.10, and switching to `native-tls`, meaning
using an up-to-date rust-openssl v0.9 on Linux, schannel on Windows, and
Secure Transport on Mac. The long-standing issue [#56][issue:56] was closed.

Thanks to the following for their contributions this release:

- [@acdenisSK]
- [@barzamin]
- [@eLunate]
- [@Flat]
- [@fwrs]
- [@hsiW]
- [@Roughsketch]

### Upgrade Path

Invite retrieval functions now accept a `stats` argument. If you don't need
stats, just pass `false`.

`ChannelId::create_permission` and `GuildChannel::create_permission` now accept
a reference, as they do not need to own the overwrite.

The deprecated `GuildChannel` methods (`get_invites`, `get_message`,
`get_messages`, `get_reaction_users`, `get_webhooks`) have been removed. Use
their equivalents without the `get_` prefix.

The `send_file` functions have been deprecated. Use `send_files` instead by
passing a Vec.

`CurrentUser::distinct` and `User::distinct` have been deprecated. Instead use
`CurrentUser::tag` and `User::tag`.

`User::get` has been deprecated. Instead, use `UserId::get`.

`Role::edit_role` has been deprecated, renaming it to `Role::edit`.

`time` has been removed as a direct dependency, moving to `chrono`.
Public-facing fields that return `time::Timespec` or were a String in ISO-3339
format are now `chrono::DateTime<UTC>`s. Instead use its methods for what was
being done with the `Timespec`s or strings.

`User::direct_message` and `User::dm` now accept a builder to allow for more
complete, yet simple use out of the methods. Instead of passing a `&str`, use
the provided builder:

```rust
// old
user.dm("hello")?;

// new
user.dm(|m| m.content("hello"))?;
```

`Client::login` has been deprecated. Instead use `Client::new`:

```rust
use serenity::Client;
use std::env;

// old
let client = Client::login(&env::var("DISCORD_TOKEN")?);

// new
let client = Client::new(&env::var("DISCORD_TOKEN")?);
```

`Member::guild_id` is now no longer an `Option<GuildId>` -- just a `GuildId`.
Since this is now always present, `Member::find_guild` has been deprecated since
the cache now never searches the cache for the guild ID.

The deprecated `GuildChannel` methods `get_invites`, `get_message`,
`get_messages`, `get_reaction_users`, and `get_webhooks` have been removed. Use
their alternatives, such as `GuildChannel::invites`, instead.

### Added

- Add support for retrieving invites with counts ([@hsiW]) [c:302d771]
- Handle message type 7 ([@fwrs]) [c:8f88c6b]
- Add `GuildChannel::permissions_for` [c:6502ded]
- Add `Invite::url()`, `RichInvite::url()` [c:3062981]
- Reasonable derive Debug on all items [c:9dae9e6]
- Add more examples and improve others [c:8c0aeac]
- Support adding reactions when creating message ([@acdenisSK]) [c:77b5b48]
- Add `VerificationLevel::Higher` [c:7dbae6b]
- Add `CurrentUser::invite_url` ([@Roughsketch], [@Flat]) [c:e033ff3],
  [c:0b95db9]
- `impl From<char> for ReactionType` [c:2afab7c]
- Implement multiple attachments ([@Flat]) [c:46b79dd]
- Add `_line` + `_line_safe` methods to `MessageBuilder` ([@Roughsketch])
  [c:543b604]
- Add docs for `CurrentUser` ([@Roughsketch]) [c:921f7f4]
- Add cache docs ([@Roughsketch]) [c:d367a70]
- Add docs and tests for framework ([@Roughsketch]) [c:4267bdb]
- Add `Content` for `MessageBuilder` ([@eLunate]) [c:060b06e]
- Include more info on ratelimiting debugs [c:d37461b]
- Add `User::refresh` [c:8c04d31]
- Add some model docs ([@Roughsketch]) [c:c00f349]
- Add `Message::channel()` [c:063a52f]
- Add `CurrentUser::default_avatar_url` [c:2d09152]
- Add `CurrentUser::face()`, `User::face()` [c:d033909]
- Deserialize embed footers [c:e92b667]
- Add `Member::permissions` [c:39a28d3] ([@acdenisSK])
- Add `wait` parameter to `http::execute_webhook` [c:dc73d1a]

### Fixed

- Don't skip `@everyone` role when checking channel overwrites ([@Roughsketch])
  [c:b468cbf]
- Allow `unreachable_code` lint in `command!` macro ([@Flat]) [c:eb43b9c]
- Fix incorrect attempted `send_file` deserialization [c:0102706]
- Fix ratelimits causing 429s in certain situations [c:f695174]
- Check last heartbeat acknowledged in heartbeater [c:ec9b1c7]
- Make client join shards and return [c:175d3a3]
- Make client starts return an error [c:858bbf2]
- Ws read/write timeout after 90s to avoid infinite blocking [c:1700a4a]
- Fix negative nonces failing to deserialize [c:d0b64cd]
- Use HTTPS Connector with remaining HTTP functions [c:0d218e0] ([@Roughsketch])

### Changed

- Restructure modules [c:9969be6]
- Change `create_permission` to take a reference [c:aea9885]
- Remove deprecated `GuildChannel` methods [c:ab7f113]
- `Guild::create_channel` doesn't require mutability [c:494cc50]
- Deprecate `*User::distinct`, add `*User::tag` [c:6579b1f]
- Deprecate `User::get` [c:afc571f]
- Deprecate `Role::edit_role`, add `Role::edit` [c:c00f349]
- Switch to chrono [c:990e611]
- Make `User::direct_message`/`User::dm` accept a builder [c:11a02db]
- Deprecate `Client::login`, add `Client::new` [c:7990381]
- Make `Member::guild_id` non-optional [c:b4bd771]
- Remove `Context::channel_id` and `Context::queue` [c:8b504ad]
- Make the framework's `dynamic_prefix` accept an `&Message` [c:2845681]
- Deprecate `Channel::delete_messages`, `Channel::delete_permission` [c:7fc49d8]
- Make `Message::nonce` a `serde_json::Value` [c:c832009]

### Misc.

- Remove deprecated `login_bot` usage from docs ([@hsiW]) [c:ae395f4]
- Fix call to `VoiceManager::join` in example 06 ([@barzamin]) [c:6853daf]
- Sort default help by group/command names ([@Roughsketch]) [c:93416cd]
- Move `CreateGroup` docs to the struct [c:71f3dbb]
- Don't create group in help if no commands to show ([@Roughsketch]) [c:4f5fbb5]
- Move user avatar method logic out [c:8360f32]
- Upgrade rust-websocket and hyper, switch to native-tls [c:8f8a059]
- Fix broken links in README [c:51c15d0]
- Remove unused `cookie` dependency [c:92f4ec2]
- Switch from `#[doc(hidden)]` to `pub(crate)` [c:32e07e4] ([@acdenisSK])
- Re-export all errors from the prelude [c:db0f025]
- Rework shard logic and shard handling [c:601704a]

## [0.2.0] - 2017-05-13

This is a very large release with a number of rewritten components. The cache
has been rewritten to make use of memory more efficiently, the models directory
has been re-organized, structures are now deserialized via serde and
`serde_derive` - instead of the custom decoding build script we had - with a
number of bugfixes and other various changes and additions.

Thanks to the following for their contributions this release:

- [@acdenisSK]
- [@Flat]
- [@fwrs]
- [@hsiW]
- [@Roughsketch]
- [@sschroe]

### Upgrade Path

Replace uses of `ext::cache::ChannelRef` with `model::Channel`.

The following `ext::cache::Cache` method signatures are now encased in
`Arc<RwLock>`s and should be handled appropriately:

- `call`
- `channel`
- `guild`
- `guild_channel`
- `group`
- `member`
- `role`
- `user`

Additionally, `GuildId::find` and `UserId::find` now return
`Option<Arc<RwLock>>`s.

`Member::display_name` now returns a `Cow<String>` instead of a `&str`.

`client::Context` has had most of its methods removed. The methods were mostly
a copy of those on `ChannelId`. Upgrade by instead calling methods on
`ChannelId`:

```rust
command!(foo(ctx) {
    let _ = ctx.say("hello");
});

// is now written as:

command!(bar(_ctx, msg) {
    let _ = msg.channel_id.say("hello");
});
```

`CreateMessage::nonce` has been removed. Instead, simply do not provide a nonce.

`ChannelId::edit_message` now has an argument signature of:

```rust
&self, message_id: M, f: F
where F: FnOnce(CreateMessage) -> CreateMessage, M: Into<MessageId>
```

instead of

```rust
&self, message_id: M, text: &str, f: F
where F: FnOnce(CreateEmbed) -> CreateEmbed, M: Into<MessageId>
```

To account for this change, modify code like so:

```rust
channel_id.edit_message(message_id, "new content", |e| e);

// now:

channel_id.edit_message(message_id, |m| m.content("new content"));
```

`Message::edit` has also had an argument signature updated to:

```rust
&mut self, f: F where F: FnOnce(CreateMessage) -> CreateMessage
```

from:

```rust
&mut self, new_content: &str, embed: F where F: FnOnce(CreateEmbed) -> CreateEmbed
```

To account for this change, modify code like so:

```rust
message.edit("new content", |e| e.description("test"));

// now:

message.edit(|m| m.content("new content").embed(|e| e.description("test")));
```

`client::rest::accept_invite`, `Invite::accept`, and `RichInvite::accept` have
been removed. Instead, do not attempt this, as they were userbot functions.

Selfbot support has been completely removed. Review the
[commit message][c:d9118c0] for the long list of details.

Group calls and guild sync have [also been removed][c:c74cc15]. Read the commit
message for all the details.

Instead of defining multiple separate error messages for command framework
message dispatches, match the dispatch error in a single method:

```rust
// old code:
client.with_framework(|f| f
    .configure(|c| c
        .command_disabled_message("The command `%command%` was disabled")
        .blocked_guild_message("The owner of this guild has been blocked")
        .invalid_permission_message("You don't have permission to use this command")));

// new code:
client.with_framework(|f| f.on_dispatch_error(|_, msg, err| {
    match err {
        DispatchError::CommandDisabled(command_name) => {
            let _ = msg.channel_id.say(&format!("The command `{}` was disabled", command_name));
        },
        DispatchError::BlockedGuild => {
            // this change also allows for more intelligent error messages:
            if let Some(guild) = msg.guild() {
                let owner_id = guild.read().unwrap().owner_id;

                if let Some(user) = CACHE.read().unwrap().user(owner_id) {
                    let c = format!("The owner - {} - has been blocked", user.name);
                    let _ = msg.channel_id.say(&c);

                    return;
                }
            }

            let _ = msg.channel_id.say("The owner of this guild has been blocked");
        },
        DispatchError::LackOfPermissions(_) => {
            let _ = msg.channel_id.say("You don't have permission to use this command");
        },
    }
}));
```

All functions prefixed with `get_` have had the prefix removed. For example,
`Guild::get_webhooks()` is now `Guild::webhooks()`.

Instead of using `model::permissions::general()`, `model::permissions::text()`,
and `model::permissions::voice()`, use
`model::permissions::{PRESET_GENERAL, PRESET_TEXT, PRESET_VOICE}`.

### Added

- Add `say` method to `Group`, `GuildChannel`, `PrivateChannel` [c:a0bb306]
- Add missing `send_file`/`send_message` impls [c:bad9ac3]
- Add `Message::guild` [c:9ef5522]
- Add Shard Id helpers [c:1561f9e]
- Implement `From<&str> for ReactionType` [c:e7110ad]
- Check for embed lengths on message sends [c:e1079e9]
- Add `is_nsfw` check to channels [c:9268f9c]
- Add missing `Member::kick` helper [c:83b1d96]
- Derive `Eq`, `Hash`, `PartialEq` on `ReactionType` [c:86a4e00] ([@acdenisSK])

### Fixed

- Handle unsuccessful responses before decoding [c:7e254c5]
- Uniquely ratelimit message deletions [c:01f6872]
- Fix Member methods due to variant `joined_at` values [c:cd914f5]
- Fix deadlock on channel create event for DMs [c:6b0b9b2] ([@sschroe])
- Default to using `[0, 1]` shards [c:f0d1157]
- Fix ratelimiting for `Route::None` routes [c:5bf6c2d]
- Fix guild leaving result [c:ae352ea]
- Fix permissions when sending to DMs or groups [c:404a089] ([@acdenisSK])
- Check if message starts with `dynamic_prefix` result [c:9ec05e7] ([@Roughsketch])
- Trim content before parsing framework args [c:e6712c9] ([@Roughsketch])

### Changed

- Optimize caching [c:0c9ec37]
- Remove most `Context` methods [c:585af23]
- Remove sending message nonces [c:9c04a19]
- Standardize message editing methods [c:3c7c575]
- Remove invite accepting [c:e4b484f]
- Remove selfbot support [c:d9118c0] [c:c74cc15]
- Switch to using serde for deserialization [c:f6b27eb]
- Update the ways errors are handled in dispatch [c:31aae7d] ([@fwrs])
- Deprecate methods prefixed with `get_` [c:3f03f9a]
- Framework help commands now accept a slice of args [c:ff4437a]
- Make `User.discriminator` a `u16` [c:0f41ffc]
- Use constants for preset permissions [c:70d4e75]

### Misc.

- Make logo more better [c:6e11a10] ([@Flat])
- Fix incorrect cache example [c:b96f85c]
- Rework the models directory [c:9114963]
- Change permission values to byte literals [c:c8536c1]
- Fix example in README [c:d4fc8b6]

## [0.1.5] - 2017-02-08

This is a release to fix broken nightly builds, due to a change in how rustc
handles lifetimes, with a few performance optimizations and other fixes.

### Upgrade Path

For `Group::send_message`, `PrivateChannel::send_message`,
and `GuildChannel::send_message`, instead of passing in only a `&str` of
content, use a `CreateMessage` builder:

```rust
// assuming a `channel` is bound

// old signature:
channel.send_message("hello");

// new signature:
channel.send_message(|m| m.content("hello"));
```

Instead of calling `message_id.get_reaction_users` and passing in a `ChannelId`,
call `channel_id.get_reaction_users` and pass in the `MessageId`. Note that the
latter already existed.

```rust
// assuming `channel_id`, `message_id`, and `reaction_type` are bound

// removed method:
message_id.get_reaction_users(channel_id, reaction_type, Some(10), None);

// alternative method:
channel_id.get_reaction_users(message_id, reaction_type, Some(10), None);
```

### Added

- Register the `status` user setting for user accounts (e.g. online, invisible)
  [c:0b9bf91]
- Expose and document ratelimiting structures [c:eb09f2d]
- Add method to `EditGuild` to transfer ownership [c:f00e165]

### Fixed

- Fix potential unreachable pattern warning in `command!` macro [c:97f9bd1]
- Fix value of 'browser' in shard identify [c:4cf8338]
- Remove lifetime on Search builder [c:6f33a35]

### Changed

- Standardize methods for creating messages [c:c8c6b83]
- Remove `MessageId::get_reaction_users` [c:268f356]

### Misc.

- Avoid re-requesting the gateway URL when autosharding (optimization)
  [c:e891ebe]
- Avoid cloning on non-framework message create events (opt.) [c:b7cbf75]
- Avoid cloning the context on event dispatches (opt.) [c:5ee5fef]
- Optimize presence update for current user in cache (opt.) [c:9392f61]
- Make `GLOBAL` ratelimit mutex a unit (opt.) [c:55ccaca]
- Resume when restarting WS sender/receiver [c:04cfaa9]


## [0.1.4] - 2017-01-26

This is a general release for pretty much everything, from new features to
bugfixes to a switch to a more OOP style. The current minimum supported version
is rustc 1.13+.

The next release will be v0.2.0, which will feature serde codegen support along
with a rewrite of the framework. It will be a more modularized version of the
library. v0.2.0 will require rustc 1.15+, due to the stabilization of Macros
1.1.

Thanks to the following for contributions this release:

- [@acdenisSK]
- [@bippum]
- [@DeltaEvo]
- [@emoticon]
- [@foxbot]
- [@fwrs]
- [@hsiW]
- [@indiv0]
- [@SunDwarf]

Two of the major highlights of this release are that the broken pipe issue has
been fixed, and the library is more OOP now and therefore no longer relies on
the `Context` to get stuff done. The `methods` feature flag has been removed.

### Upgrade Path

When formatting using `Display` for `ChannelId`s, `RoleId`s, and `UserId`,
instead of formatting use their `Mentionable` equivilants:

```rust
use serenity::model::{ChannelId, RoleId, UserId};

// old
assert_eq!(format!("{}", ChannelId(1)), "<#1>");
assert_eq!(format!("{}", RoleId(2)), "<@&2>");
assert_eq!(format!("{}", UserId(3)), "<@3>");

// new
assert_eq!(format!("{}", ChannelId(1).mention()), "<#1>");
assert_eq!(format!("{}", RoleId(2)).mention()), "<@&2>");
assert_eq!(format!("{}", UserId(3).mention()), "<@3>");
```

When using `EmbedBuilder::{image, thumbnail}`, instead of calling another
builder, provide `url`s directly:

```rust
use serenity::model::Embed;

// old
Embed::fake(|e| e
    .image(|i| i.url("https://not.zey.moe/me.png"))
    .thumbnail(|t| t.url("https://not.zey.moe/me2.png")));

// new
Embed::fake(|e| e
    .image("https://not.zey.moe/me.png")
    .thumbnail("https://not.zey.moe/me2.png"));
```

When specifying a sharding method, instead of passing a `u8` for sharding info,
pass a `u64`:

```rust
use serenity::Client;

let client = Client::login_bot(&env::var("DISCORD_TOKEN").unwrap());

// old
client.start_shard(1u8, 5u8); // or
client.start_shards(5u8); // or
client.start_shard_range([1u8, 3u8], 8u8);

// new
client.start_shard(1u64, 5u64); // or
client.start_shards(5u64); // or
client.start_shard_range([1u64, 3u64], 8u64);
```

`Client.shards` is now private. Instead of accessing it, don't.

When creating a `Colour` struct yourself, instead of specifying a single `value`
field, pass a single tuple value:

```rust
use serenity::utils::Colour;

// old
Colour {
    value: 0,
}

// new
Colour(0);
```

Instead of using `Attachment::download_to_directory` to download an attachment
to a directory, do it yourself:

```rust
use std::fs::File;
use std::io::Write;

// assuming an `attachment` has already been bound

// old
attachment.download_to_directory("./attachments");

// new
let bytes = attachment.download().unwrap();
let filepath: PathBuf = path.as_ref().join(&attachment.filename);
let mut f = File::create(&filepath);
let _ = f.write(&bytes);
```

Instead of calling `Message::is_webhook()`:

```rust
// assuming a `message` has already been bound

// old
let _ = message.is_webhook();

// new
let _ = message.webhook_id.is_some();
```

Instead of `PartialGuild::find_role(role_id)`:

```rust
use serenity::model::RoleId;

// assuming a `guild` has already been bound

// old
let _ = guild.find_role(RoleId(1));

// new
let _ = guild.roles.get(RoleId(1));
```

Instead of `Guild::{get_channel, get_member}`, call:

```rust
use serenity::model::{ChannelId, UserId};

// assuming a `guild` has already been bound

// old
let _ = guild.get_channel(ChannelId(1));
let _ = guild.get_member(UserId(2));

// new
let _ = guild.channels.get(ChannelId(1));
let _ = guild.members.get(UserId(2));
```

Instead of using `Context` methods, use their `Id` or other struct equivalents.

### Added

- the `voice` feature no longer requires the `cache` feature to be enabled
  [c:7b45f16]
- the `framework` feature no longer requires the `cache` feature to be enabled
  [c:86cd00f]
- `Guild`, `InviteGuild`, and `PartialGuild` now have `splash_url` methods
  [c:d58c544]
- Expose `Message::webhook_id` for messages sent via webhooks ([@fwrs])
  [c:a2cbeb6]
- Framework: add option to ignore webhooks or DMs ([@fwrs]) [c:8e2c052]
- Added documentation for creating embed timestamps ([@foxbot]) [c:66546d3]
- Allow `time::Tm` to be passed into the embed timestamp field, in addition to
  a direct string [c:b001234]
- Add `Client::on_message()` example ([@indiv0]) [c:bcb70e8]
- Support webp/gif avatars/icons in URL methods [c:ab778f8]
- Update current user presence in cache on set [c:5b275fc]
- Add `CurrentUser`/`User::static_avatar_url()` methods to generate webp URLs
  [c:c36841d]
- Command (batch) alias support ([@fwrs]) [c:f96b6cc]
- Command example field for help command ([@fwrs]) [c:f96b6cc]
- Added "Meibi Pink" to the `Colour` struct ([@hsiW]) [c:2cb607d]
- Register support for `4011` code (too many shards) ([@SunDwarf]) [c:93f3c60]
- Added "Rohrkatze Blue" to the `Colour` struct ([@bippum]) [c:345e140]
- Add `User::default_avatar_url()` [c:e85e901]
- Add `Message::content_safe()` to avoid `@everyone`/`@here`s ([@fwrs])
  [c:e5a83dd]
- Add `Member::distinct()`, `User::distinct()` ([@fwrs]) [c:e5a83dd]
- Document that messages can't be older than 14 days when bulk deleting
  ([@fwrs]) [c:0a2f5ab]
- Add shard latency tracking (~~stolen~~ borrowed from brayzure/Eris)
  [c:096b0f5]
- Add guild chunking [c:3ca7ad9]

### Fixed

- `User::avatar_url` no longer mentions the user in the generated URL
  [c:0708ccf]
- Framework: `owners_only` check now functions only if the author of a message
  is an owner ([@fwrs]) [c:6355288]
- Framework: fix command cooldown timer (would always say to wait `i64::MAX`
  seconds) [c:fafa363]
- Framework: the `before` closure is now properly run when a message is sent by
  the owner [c:760a47a]
- `CurrentApplicationInfo` now properly decodes due to `flags` no longer being
  sent [c:2a743ce]
- Fix `Message::delete()` permission check [c:4229034]
- Framework: properly split messages on character boundary limits; aka thanks
  Unicode [c:c01f238]
- Remove need to import Context/Message in command macros ([@acdenisSK])
  [c:abd22d2]
- Fix a ton of gateway stuff [c:94fc85b], [c:f894cfd], [c:f894cfd]
- Specify `command!` macro signature as returning `std::result::Result`
  [c:e9aae9c]
- Fix dependency description in example 06 ([@DeltaEvo]) [c:92309b2]
- Return a `User` from `rest::get_user` -- not a `CurrentUser` [c:f57a187]
- Fix shards always booting at index 0 [c:83b29d5]
- Wait 5 seconds between shard boots to avoid session invalidations [c:fb4d411]
- Use CDN for default avatars [c:69ec62a]
- Fix `Resumed` event payload decoding [c:c2e8b69]
- Fix `CurrentApplicationInfo` decoding without `rpc_origins` [c:38db32e]
- Reboot shard on broken pipe; fixes a lot of gateway problems [c:76f9095]
- Make `rest::execute_webhook` be a POST [c:c050c59]

### Changed

- Framework: argument number is now displayed on parsing error ([@fwrs])
  [c:fb07751]
- Id display formatters use the direct u64 instead of mentioning;
  `format!("{}", UserId(7))` will format into `"7"` instead of `"<@7>"`
  [c:933ee89]
- Default the framework's `use_quotes` for quote parsing to `false` (was `true`)
  [c:38a484d]
- The `CreateEmbed` builder now has direct `image` and `thumbnail` methods
  instead of one-method builders [c:68c473d]
- Accept `u64` shard counts to allow using more than 255 shards (instead of
  `u8`s) [c:ada07fa]
- Remove user logout endpoint [c:70bf22a]
- Don't abuse unicode for message content sanitization ([@fwrs]) [c:2b237e7]
- Change `Colour` struct to be a tuplestruct [c:a8acd61]
- Make a single POST on guild role create [c:147cf01]
- Switch to a mostly-fully OOP approach [c:651c618]
- Rename `webhooks` methods to `get_webhooks`
  (eg: `GuildChannel::webhooks()` --> `GuildChannel::get_webhooks()`)
  [c:e8a9086]
- Make `Guild::create_channel` and related functions return a `GuildChannel`
  [c:5918d01]

### Misc.

- Cleaned up YAML definition layouts [c:00fb61b]
- Gateway identify compression code is now simplified [c:2416813]
- Gateway Event decoders are now abstracted to individual struct implementations
  [c:5fe6a39]
- Simplify `Role`'s' `Ord` impl ([@emoticon]) [c:6a887b2]
- Slightly simplify `Shard::set_presence` [c:5c40e85]
- Rename repo references from `serenity.rs` to `serenity` ([@fwrs]) [c:3348178]
- Simplify `Reaction::delete()` [c:1594961]
- Abstract large threshold number to a constant [c:f3f74ce]
- Avoid a needless string clone on login [c:d3389be]
- Avoid a lot of `Arc`/`Message`/`RwLock` clones [c:8c5ee70]


## [0.1.3] - 2016-12-14

This is a hotfix for applying a PR and fixing a major bug in the plain help
command.

Thanks to the following for contributions this release:

- [@fwrs]

### Upgrade Path

None.

### Added

- Blocking individual users and guilds in commands, add blocking commands, and
  configuring owners of bots ([@fwrs]) [c:a39647d]

### Fixed

- The plain help command now properly sends a message when requesting
  information about a command [c:7b4b154]

### Misc.

- Groups are now on their own lines in the plain help command [c:16bd765]

## [0.1.2] - 2016-12-14

This release focuses on revamping the framework, adding a large amount of
configuration and overall features. v0.1.3 will be focused on performance
optimizations and code cleanup.

Thanks to the following for contributions this release:

- [@acdenisSK]
- [@fwrs]

v0.1.2 can now be retrieved from the [crates.io listing].

### Upgrade Path

When using `EmbedBuilder::{image, thumbnail}`, instead of calling another
builder, provide `url`s directly:

```rust
use serenity::model::Embed;

// old
Embed::fake(|e| e
    .image(|i| i.url("https://not.zey.moe/me.png"))
    .thumbnail(|t| t.url("https://not.zey.moe/me2.png")));

// new
Embed::fake(|e| e
    .image("https://not.zey.moe/me.png")
    .thumbnail("https://not.zey.moe/me2.png"));
```

### Added

- Allow mentionable structs to be used as command arguments ([@fwrs])
  [c:626ffb2]
- Implemented `From<Embed> for CreateEmbed` [c:7914274]
- Framework command builder, quoted arguments, multi-prefixes ([@fwrs])
  [c:8f24aa3]
- `{Emoji,EmojiIdentifier}::url` [c:ef6eba3]
- Command groups and buckets [c:daf92ed]

### Fixed

- Fix mentioning in the `MessageBuilder` ([@fwrs]) [c:13de5c2]
- Don't mutate token for bots on profile change [c:8effc91]

### Changed

- Deprecate `CreateEmbedImage::{height, width}` and
  `CreateEmbedThumbnail::{height, width}`

### Misc.

- All internal `try!`s have been converted to use `?` syntax ([@acdenisSK])
  [c:f69512b]

## [0.1.1] - 2016-12-05

v0.1.1 is a "features that v0.1.0 should have had" and "miscellaneous work"
release. v0.1.2 will be focused on the framework, while v0.1.3 will be focused
on performance optimizations.

Thanks to the following for contributions this release:

- [@abalabahaha]
- [@Flat]
- [@fwrs]
- [@GetRektByMe]
- [@iCrawl]
- [@indiv0]
- [@khazhyk]
- [@SunDwarf]

v0.1.1 can now be retrieved from the [crates.io listing].

[v0.1.1:example 06]: https://github.com/serenity-rs/serenity/tree/ccb9d16e5dbe965e5a604e1cb402cd3bc7de0df5/examples/06_command_framework

### Upgrade Path

When calling `rest::get_guilds`, instead of passing no parameters, pass a
`GuildPagination` variant and a `limit`:

```rust
use serenity::client::rest::{self, GuildPagination};
use serenity::model::GuildId;

// before
rest::get_guilds();

// after
rest::get_guilds(GuildPagination::After(GuildId(777)), 50);
```

### Added

- The following colours to the Colour struct:
  - "Kerbal" ([@indiv0]) [c:c032fbe]
  - "Blurple" ([@GetRektByMe]) [c:e9282d3]
  - "Blitz Blue" ([@iCrawl]) [c:f53124e]
  - "Fabled Pink" ([@Flat]) [c:9aa357f]
  - "Fooyoo" ([@SunDwarf]) [c:49a6841]
  - "Rosewater" ([@fwrs]) [c:2eaa415]
- `Message::guild_id` as a quick method for retrieving the Id of a message's
  guild [c:bceb049]
- `CurrentUser::guilds()` to get the current user's guilds. Meant for use with
  selfbots [c:57c060f]
- `CurrentUser::edit()` to edit the current user's profile settings [c:16d1b3c]
- `User::distinct` to format a string with the `username#discriminator`
  combination ([@fwrs]) [c:31becb1]
- `Member::colour` to retrieve the member's colour ([@fwrs]) [c:43a5c5d]
- Roles can now be directly compared (`role1 < role2`) for hierarchy [c:143337a]
- Documentation:
  - `EditMember` and `EditProfile` ([@Kiseii]) [c:e2557ac]
  - Documentation for 19 model definitions ([@fwrs]) [c:2844ae1]
  - Context + permission requirements [c:d144136]
- A custom shared state (not the Cache) can now be accessed and mutated across
  commands/contexts, through the use of `Context.data`'s ShareMap. See
  [example 06][v0.1.1:example 06] for an example

### Fixed

- `rest::start_integration_sync`/`Context::start_integration_sync` now properly
  work ([@abalabahaha]) [c:7f04179]
- Role positions can now be negative; fixes issues where a guild's @everyone
  role (and other roles) are negative [c:f847638]
- `Context::move_member`'s signature is now correct [c:4de39da]
- The `command!` macro now publicly exports functions. This allows commands
  created via this macro to be separated into different modules or crates
  [c:62ed564]

### Changed

- `rest::get_guilds` now supports pagination of guilds, as the output is now
  limited to 100 [c:57c060f]

### Misc.

- `Colour::dark_green` is now sorted alphabetically ([@khazhyk]) [c:4a14b92]
- Simplify the colour macro [c:bb97211]
- Capitalize the hex value for `Colour::blitz_blue` ([@Kiseii]) [c:daa24ec]

## [0.1.0] - 2016-11-30

Initial commit.

<!-- COMPARISONS -->

[0.11.7]: https://github.com/serenity-rs/serenity/compare/v0.11.6...v0.11.7
[0.11.6]: https://github.com/serenity-rs/serenity/compare/v0.11.5...v0.11.6
[0.11.5]: https://github.com/serenity-rs/serenity/compare/v0.11.4...v0.11.5
[0.11.4]: https://github.com/serenity-rs/serenity/compare/v0.11.3...v0.11.4
[0.11.3]: https://github.com/serenity-rs/serenity/compare/v0.11.2...v0.11.3
[0.11.2]: https://github.com/serenity-rs/serenity/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/serenity-rs/serenity/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/serenity-rs/serenity/compare/v0.10.10...v0.11.0
[0.10.10]: https://github.com/serenity-rs/serenity/compare/v0.10.9...v0.10.10
[0.10.9]: https://github.com/serenity-rs/serenity/compare/v0.10.8...v0.10.9
[0.10.8]: https://github.com/serenity-rs/serenity/compare/v0.10.7...v0.10.8
[0.10.7]: https://github.com/serenity-rs/serenity/compare/v0.10.6...v0.10.7
[0.10.6]: https://github.com/serenity-rs/serenity/compare/v0.10.5...v0.10.6
[0.10.5]: https://github.com/serenity-rs/serenity/compare/v0.10.4...v0.10.5
[0.10.4]: https://github.com/serenity-rs/serenity/compare/v0.10.3...v0.10.4
[0.10.3]: https://github.com/serenity-rs/serenity/compare/v0.10.2...v0.10.3
[0.10.2]: https://github.com/serenity-rs/serenity/compare/v0.10.1...v0.10.2
[0.10.1]: https://github.com/serenity-rs/serenity/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/serenity-rs/serenity/compare/v0.9.4...v0.10.0
[0.9.4]: https://github.com/serenity-rs/serenity/compare/v0.9.3...v0.9.4
[0.9.3]: https://github.com/serenity-rs/serenity/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/serenity-rs/serenity/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/serenity-rs/serenity/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/serenity-rs/serenity/compare/v0.9.0-rc.4...v0.9.0
[0.9.0-rc.4]: https://github.com/serenity-rs/serenity/compare/v0.9.0-rc.3...v0.9.0-rc.4
[0.9.0-rc.3]: https://github.com/serenity-rs/serenity/compare/v0.9.0-rc.2...v0.9.0-rc.3
[0.9.0-rc.2]: https://github.com/serenity-rs/serenity/compare/v0.9.0-rc.1...v0.9.0-rc.2
[0.9.0-rc.1]: https://github.com/serenity-rs/serenity/compare/v0.9.0-rc.0...v0.9.0-rc.1
[0.9.0-rc.0]: https://github.com/serenity-rs/serenity/compare/v0.8.8...v0.9.0-rc.0
[0.8.9]: https://github.com/serenity-rs/serenity/compare/v0.8.8...v0.8.9
[0.8.8]: https://github.com/serenity-rs/serenity/compare/v0.8.7...v0.8.8
[0.8.7]: https://github.com/serenity-rs/serenity/compare/v0.8.6...v0.8.7
[0.8.6]: https://github.com/serenity-rs/serenity/compare/v0.8.5...v0.8.6
[0.8.5]: https://github.com/serenity-rs/serenity/compare/v0.8.4...v0.8.5
[0.8.4]: https://github.com/serenity-rs/serenity/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/serenity-rs/serenity/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/serenity-rs/serenity/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/serenity-rs/serenity/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/serenity-rs/serenity/compare/v0.7.8...v0.8.0
[0.7.8]: https://github.com/serenity-rs/serenity/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/serenity-rs/serenity/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/serenity-rs/serenity/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/serenity-rs/serenity/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/serenity-rs/serenity/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/serenity-rs/serenity/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/serenity-rs/serenity/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/serenity-rs/serenity/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/serenity-rs/serenity/compare/v0.6.4...v0.7.0
[0.6.4]: https://github.com/serenity-rs/serenity/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/serenity-rs/serenity/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/serenity-rs/serenity/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/serenity-rs/serenity/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/serenity-rs/serenity/compare/v0.6.0-rc.2...v0.6.0
[0.6.0-rc.2]: https://github.com/serenity-rs/serenity/compare/v0.6.0-rc.0...v0.6.0-rc.2
[0.6.0-rc.1]: https://github.com/serenity-rs/serenity/compare/v0.6.0-rc.0...v0.6.0-rc.1
[0.6.0-rc.0]: https://github.com/serenity-rs/serenity/compare/v0.5.14...v0.6.0-rc.0
[0.5.14]: https://github.com/serenity-rs/serenity/compare/v0.5.13...v0.5.14
[0.5.13]: https://github.com/serenity-rs/serenity/compare/v0.5.12...v0.5.13
[0.5.12]: https://github.com/serenity-rs/serenity/compare/v0.5.11...v0.5.12
[0.5.11]: https://github.com/serenity-rs/serenity/compare/v0.5.10...v0.5.11
[0.5.10]: https://github.com/serenity-rs/serenity/compare/v0.5.9...v0.5.10
[0.5.9]: https://github.com/serenity-rs/serenity/compare/v0.5.8...v0.5.9
[0.5.8]: https://github.com/serenity-rs/serenity/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/serenity-rs/serenity/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/serenity-rs/serenity/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/serenity-rs/serenity/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/serenity-rs/serenity/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/serenity-rs/serenity/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/serenity-rs/serenity/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/serenity-rs/serenity/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/serenity-rs/serenity/compare/v0.4.7...v0.5.0
[0.4.5]: https://github.com/serenity-rs/serenity/compare/v0.4.4...v0.4.5
[0.4.4]: https://github.com/serenity-rs/serenity/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/serenity-rs/serenity/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/serenity-rs/serenity/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/serenity-rs/serenity/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/serenity-rs/serenity/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/serenity-rs/serenity/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/serenity-rs/serenity/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/serenity-rs/serenity/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/serenity-rs/serenity/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/serenity-rs/serenity/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/serenity-rs/serenity/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/serenity-rs/serenity/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/serenity-rs/serenity/tree/403d65d5e98bdfa9f0c018610000c4a0b0c7d8d5
[crates.io listing]: https://crates.io/crates/serenity
[library organization]: https://github.com/serenity-rs
[semver]: http://semver.org

[issue:56]: https://github.com/serenity-rs/serenity/issues/56
[rust-websocket:issue:137]: https://github.com/cyderize/rust-websocket/issues/137

<!-- AUTHORS -->

[@7596ff]: https://github.com/7596ff
[@AgathaSorceress]: https://github.com/AgathaSorceress
[@Alch-Emi]: https://github.com/Alch-Emi
[@AldanTanneo]: https://github.com/AldanTanneo
[@AlexisTM]: https://github.com/AlexisTM
[@AngelOnFira]: https://github.com/AngelOnFira
[@AnnikaCodes]: https://github.com/AnnikaCodes
[@Arcterus]: https://github.com/Arcterus
[@AregevDev]: https://github.com/AregevDev
[@AriusX7]: https://github.com/AriusX7
[@Atakku]: https://github.com/Atakku
[@aawilson]: https://github.com/aawilson
[@abalabahaha]: https://github.com/abalabahaha
[@acdenisSK]: https://github.com/arqunis
[@arqunis]: https://github.com/arqunis
[@adumbidiot]: https://github.com/adumbidiot
[@almeidx]: https://github.com/almeidx
[@anden3]: https://github.com/anden3
[@andreasots]: https://github.com/andreasots
[@aria-7553]: https://github.com/aria-7553
[@asherkin]: https://github.com/asherkin
[@B-2U]: https://github.com/B-2U
[@Baev1]: https://github.com/Baev1
[@Bond-009]: https://github.com/Bond-009
[@Bloectasy]: https://github.com/Bloectasy
[@baeuric]: https://github.com/baeuric
[@barzamin]: https://github.com/barzamin
[@bdashore3]: https://github.com/bdashore3
[@benjaminrsherman]: https://github.com/benjaminrsherman
[@ben-brook]: https://github.com/ben-brook
[@bikeshedder]: https://github.com/bikeshedder
[@bippum]: https://github.com/bippum
[@blaenk]: https://github.com/blaenk
[@bumblepie]: https://github.com/bumblepie
[@Caemor]: https://github.com/Caemor
[@CarlGroth]: https://github.com/CarlGroth
[@Celti]: https://github.com/Celti
[@Chronophylos]: https://github.com/Chronophylos
[@Collin-Swish]: https://github.com/Collin-Swish
[@ConcurrentMarxistGC]: https://github.com/ConcurrentMarxistGC
[@cab404]: https://github.com/cab404
[@caoculus]: https://github.com/caoculus
[@casey]: https://github.com/casey
[@chocological00]: https://github.com/chocological00
[@cyril-marpaud]: https://github.com/cyril-marpaud
[@Daggy1234]: https://github.com/Daggy1234
[@DarkKirb]: https://github.com/DarkKirb
[@Dean-Coakley]: https://github.com/Dean-Coakley
[@dclamage]: https://github.com/dclamage
[@Deebster]: https://github.com/Deebster
[@DeltaEvo]: https://github.com/DeltaEvo
[@DimiDimit]: https://github.com/DimiDimit
[@Dinnerbone]: https://github.com/Dinnerbone
[@DoumanAsh]: https://github.com/DoumanAsh
[@dpytaylo]: https://github.com/dpytaylo
[@DRuppFv]: https://github.com/DRuppFv
[@DrBluefall]: https://github.com/DrBluefall
[@dapper-gh]: https://github.com/dapper-gh
[@devtomio]: https://github.com/devtomio
[@dmarcoux]: https://github.com/dmarcoux
[@drklee3]: https://github.com/drklee3
[@drp19]: https://github.com/drp19
[@Elinvynia]: https://github.com/Elinvynia
[@Erk-]: https://github.com/Erk-
[@eLunate]: https://github.com/eLunate
[@eatsfoobars]: https://github.com/eatsfoobars
[@efyang]: https://github.com/efyang
[@elkowar]: https://github.com/elkowar
[@emoticon]: https://github.com/emoticon
[@FallenWarrior2k]: https://github.com/FallenWarrior2k
[@FelixMcFelix]: https://github.com/FelixMcFelix
[@Flat]: https://github.com/Flat
[@float3]: https://github.com/float3
[@ForsakenHarmony]: https://github.com/ForsakenHarmony
[@Friz64]: https://github.com/Friz64
[@fenhl]: https://github.com/fenhl
[@foxbot]: https://github.com/foxbot
[@ftriquet]: https://github.com/ftriquet
[@fwrs]: https://github.com/fwrs
[@Gabriel-Paulucci]: https://github.com/Gabriel-Paulucci
[@GetRektByMe]: https://github.com/GetRektByMe
[@Gentoli]: https://github.com/Gentoli
[@ghost]: https://github.com/ghost
[@GnomedDev]: https://github.com/GnomedDev
[@gradiuscypher]: https://github.com/gradiuscypher
[@HarmoGlace]: https://github.com/HarmoGlace
[@HexPandaa]: https://github.com/HexPandaa
[@hsiW]: https://github.com/hsiW
[@hyarsan]: https://github.com/hyarsan
[@hybras]: https://github.com/hybras
[@icewind1991]: https://github.com/icewind1991
[@iCrawl]: https://github.com/iCrawl
[@ikkerens]: https://github.com/ikkerens
[@imnotbad]: https://github.com/imnotbad
[@indiv0]: https://github.com/indiv0
[@ijks]: https://github.com/ijks
[@ivancernja]: https://github.com/ivancernja
[@JellyWX]: https://github.com/JellyWX
[@Jerald]: https://github.com/Jerald
[@JohnTheCoolingFan]: https://github.com/JohnTheCoolingFan
[@james7132]: https://github.com/james7132
[@jhelwig]: https://github.com/jhelwig
[@jkcclemens]: https://github.com/jkcclemens
[@jmgao]: https://github.com/jmgao
[@joek13]: https://github.com/joek13
[@johnchildren]: https://github.com/johnchildren
[@jontze]: https://github.com/jontze
[@KaDiWa4]: https://github.com/KaDiWa4
[@KamranMackey]: https://github.com/KamranMackey
[@Kroisse]: https://github.com/Kroisse
[@kafinsalim]: https://github.com/kafinsalim
[@kangalio]: https://github.com/kangalio
[@KangarooCoder]: https://github.com/KangarooCoder
[@khazhyk]: https://github.com/khazhyk
[@kristopherbullinger]: https://github.com/kristopherbullinger
[@kotx]: https://github.com/kotx
[@ks129]: https://github.com/ks129
[@kyranet]: https://github.com/kyranet
[@Lakelezz]: https://github.com/Lakelezz
[@LavaToaster]: https://github.com/LavaToaster
[@LeSeulArtichaut]: https://github.com/LeSeulArtichaut
[@Licenser]: https://github.com/Licenser
[@LikeLakers2]: https://github.com/LikeLakers2
[@Lymia]: https://github.com/Lymia
[@lapin-b]: https://github.com/lapin-b
[@legendofmiracles]: https://github.com/legendofmiracles
[@leo-lb]: https://github.com/leo-lb
[@leumasme]: https://github.com/leumasme
[@lhjt]: https://github.com/lhjt
[@lo48576]: https://github.com/lo48576
[@lolzballs]: https://github.com/lolzballs
[@MarkusTheOrt]: https://github.com/MarkusTheOrt
[@MathyouMB]: https://github.com/MathyouMB
[@Max2408]: https://github.com/Max2408
[@MaxOhn]: https://github.com/MaxOhn
[@MelonShooter]: https://github.com/MelonShooter
[@Mendess2526]: https://github.com/Mendess2526
[@Milo123459]: https://github.com/Milo123459
[@Mishio595]: https://github.com/Mishio595
[@MonliH]: https://github.com/MonliH
[@mTvare6]: https://github.com/mTvare6
[@mattfbacon]: https://github.com/mattfbacon
[@mattico]: https://github.com/mattico
[@mbenoukaiss]: https://github.com/mbenoukaiss
[@mdonoughe]: https://github.com/mdonoughe
[@megumisonoda]: https://github.com/megumisonoda
[@mendess]: https://github.com/mendess
[@merlleu]: https://github.com/merlleu
[@miqbalrr]: https://github.com/miqbalrr
[@mjsir911]: https://github.com/mjsir911
[@mkrasnitski]: https://github.com/mkrasnitski
[@molenzwiebel]: https://github.com/molenzwiebel
[@mysteriouspants]: https://github.com/mysteriouspants
[@mshenrick]: https://github.com/mshenrick
[@MOZGIII]: https://github.com/MOZGIII
[@NieDzejkob]: https://github.com/NieDzejkob
[@NinekoTheCat]: https://github.com/NinekoTheCat
[@NilsIrl]: https://github.com/NilsIrl
[@Noituri]: https://github.com/Noituri
[@NotNorom]: https://github.com/NotNorom
[@NovusTheory]: https://github.com/NovusTheory
[@nabijaczleweli]: https://github.com/nabijaczleweli
[@natsukagami]: https://github.com/natsukagami
[@natto1784]: https://github.com/natto1784
[@nickelc]: https://github.com/nickelc
[@nycex]: https://github.com/nycex
[@OnlyCS]: https://github.com/OnlyCS
[@OverHash]: https://github.com/OverHash
[@Polyhistorian]: https://github.com/Polyhistorian
[@PvdBerg1998]: https://github.com/PvdBerg1998
[@Proximyst]: https://github.com/Proximyst
[@Prof-Bloodstone]: https://github.com/Prof-Bloodstone
[@pascalharp]: https://github.com/pascalharp
[@peppizza]: https://github.com/peppizza
[@perryprog]: https://github.com/perryprog
[@Qeenon]: https://github.com/Qeenon
[@qm3ster]: https://github.com/qm3ster
[@rand0m-cloud]: https://github.com/rand0m-cloud
[@Roughsketch]: https://github.com/Roughsketch
[@Rstar284]: https://github.com/Rstar284
[@rasm47]: https://github.com/rasm47
[@rsaihe]: https://github.com/rsaihe
[@SOF3]: https://github.com/SOF3
[@Sei4or]: https://github.com/Sei4or
[@SadiinsoSnowfall]: https://github.com/SadiinsoSnowfall
[@sandlotie]: https://github.com/sandlotie
[@Scetch]: https://github.com/Scetch
[@ShashankKumarSaxena]: https://github.com/ShashankKumarSaxena
[@SimonZehetner]: https://github.com/SimonZehetner
[@SinsofSloth]: https://github.com/SinsofSloth
[@Some-Dood]: https://github.com/Some-Dood
[@Splingush]: https://github.com/Splingush
[@squili]: https://github.com/squili
[@Sreyas-Sreelal]: https://github.com/Sreyas-Sreelal
[@SunDwarf]: https://github.com/SunDwarf
[@s0lst1ce]: https://github.com/s0lst1ce
[@sam-kirby]: https://github.com/sam-kirby
[@sbrocket]: https://github.com/sbrocket
[@shnarazk]: https://github.com/shnarazk
[@sschroe]: https://github.com/sschroe
[@sudomann]: https://github.com/sudomann
[@tahahawa]: https://github.com/tahahawa
[@TehPers]: https://github.com/TehPers
[@ThatsNoMoon]: https://github.com/ThatsNoMoon
[@Th3-M4jor]: https://github.com/Th3-M4jor
[@TheBlackfurGuy]: https://github.com/TheBlackfurGuy
[@TheElec]: https://github.com/TheElec
[@TheUnitedStatesOfAmerica]: https://github.com/TheUnitedStatesOfAmerica
[@TitusEntertainment]: https://github.com/TitusEntertainment
[@tarcieri]: https://github.com/tarcieri
[@tedtramonte]: https://github.com/tedtramonte
[@thelearnerofcode]: https://github.com/thelearnerofcode
[@thebongy]: https://github.com/thebongy
[@timotree3]: https://github.com/timotree3
[@tmcarr]: https://github.com/tmcarr
[@tverghis]: https://github.com/tverghis
[@tweirtx]: https://github.com/tweirtx
[@tylerd008]: https://github.com/tylerd008
[@Unoqwy]: https://github.com/Unoqwy
[@u5surf]: https://github.com/u5surf
[@Vaimer9]: https://github.com/Vaimer9
[@vaporox]: https://github.com/vaporox
[@vicky5124]: https://github.com/vicky5124
[@vityafx]: https://github.com/vityafx
[@vivianhellyer]: https://github.com/vivianhellyer
[@Web-44]: https://github.com/Web-44
[@Wolvereness]: https://github.com/Wolvereness
[@woongzeyi]: https://github.com/woongzeyi
[@xMAC94x]: https://github.com/xMAC94x
[@xSke]: https://github.com/xSke
[@xacrimon]: https://github.com/xacrimon
[@xentec]: https://github.com/xentec
[@xfix]: https://github.com/xfix
[@Zalaxx]: https://github.com/Zalaxx
[@zacck]: https://github.com/zacck
[@zack37]: https://github.com/zack37
[@zackradisic]: https://github.com/zackradisic
[@zeyla]: https://github.com/zeyla
[@zzzzDev4]: https://github.com/zzzzDev4


<!-- COMMITS -->

[c:e690ccd]: https://github.com/serenity-rs/serenity/commit/e690ccd17174f2b9dc3f6a1545f69a209a11241f
[c:bca105a]: https://github.com/serenity-rs/serenity/commit/bca105a0b9bca26d272ddab5c8b99fd93517cbf6
[c:b1c3a62]: https://github.com/serenity-rs/serenity/commit/b1c3a62161a640b7c11a39797d03697e6a4e2ff5
[c:b2ae872]: https://github.com/serenity-rs/serenity/commit/b2ae872f7eba793c06370a58f59e60caceb59d66
[c:888c37f]: https://github.com/serenity-rs/serenity/commit/888c37f044df92b1768661079385bdb1a09e2034
[c:10d9297]: https://github.com/serenity-rs/serenity/commit/10d9297dda9aaa43a02b299ea6e4b05e7ca044d2
[c:475fc1f]: https://github.com/serenity-rs/serenity/commit/475fc1fb1523f3d46b0119c2ba7e9fc1b4b43bfa
[c:39fb310]: https://github.com/serenity-rs/serenity/commit/39fb310247e609ef7baa91fbe34493e16471372b
[c:dc22cf2]: https://github.com/serenity-rs/serenity/commit/dc22cf2af034e521e11971af8f8cda0066d6a87a
[c:0a544c6]: https://github.com/serenity-rs/serenity/commit/0a544c645995c5c8f9cddb0e6240bdb8854bd24b
[c:3e454cc]: https://github.com/serenity-rs/serenity/commit/3e454cc50dfa290d07ea36ad058fd59355c9ff46
[c:8b2326c]: https://github.com/serenity-rs/serenity/commit/8b2326c7523b74d4da39881b30a3a62f7411360d
[c:be17752]: https://github.com/serenity-rs/serenity/commit/be1775240b0739d60653453a16d990500a945b11
[c:79a3cc8]: https://github.com/serenity-rs/serenity/commit/79a3cc85bd014f285c753f370963823c4220ba72
[c:2898af2]: https://github.com/serenity-rs/serenity/commit/2898af206e9ee6fdc383c5b00d876a8bf4b4b2d1
[c:0ba01a7]: https://github.com/serenity-rs/serenity/commit/0ba01a73a25c98c5c32025e60ca15fdd21d7062b
[c:47433de]: https://github.com/serenity-rs/serenity/commit/47433de14f1ef5fd98662bab7e699c187d73fddc
[c:c3ad0b2]: https://github.com/serenity-rs/serenity/commit/c3ad0b2c49aeecddf7f28e324cf35c782245360e
[c:56293bb]: https://github.com/serenity-rs/serenity/commit/56293bbef371e84a183ea413d77d44106578ee57
[c:75df53e]: https://github.com/serenity-rs/serenity/commit/75df53e58fe591525ffdf726dcca3564d06cafdf
[c:e1eaeeb]: https://github.com/serenity-rs/serenity/commit/e1eaeeb2dcf493cd64f6765bf88e5430b22f81e2
[c:dd9d255]: https://github.com/serenity-rs/serenity/commit/dd9d2556cde3a4df0b2b434733fd355e9d4b5de5
[c:628c1ea]: https://github.com/serenity-rs/serenity/commit/628c1ead2c7d091d689a57ca8b44a74594a97db7
[c:889540a]: https://github.com/serenity-rs/serenity/commit/889540a0f336cbe23940acf11b17d2324e6f2a6a
[c:b1cc702]: https://github.com/serenity-rs/serenity/commit/b1cc7022228608563e815c3f82b971d5ca6bfc74
[c:fe53b16]: https://github.com/serenity-rs/serenity/commit/fe53b1681e80386bbb8b9b9e341926504748118e
[c:2f8e2b0]: https://github.com/serenity-rs/serenity/commit/2f8e2b09479a4b5d466e7c89ccba04346729219a
[c:97ef906]: https://github.com/serenity-rs/serenity/commit/97ef906b349c400141104602944186a977a75414
[c:0450dde]: https://github.com/serenity-rs/serenity/commit/0450ddebb541dc5e9cef625b964cf8497b0d6cb7

[c:9f11a3f]: https://github.com/serenity-rs/serenity/commit/9f11a3f2e9908ae33b3ab0dbd38ccf7fb96370ba

[c:440f0fa]: https://github.com/serenity-rs/serenity/commit/440f0fa5204b00ea3200ee3577d1c051862a6041
[c:4dbf3d0]: https://github.com/serenity-rs/serenity/commit/4dbf3d06beaec57e1d235f0469907a5a61a02b04
[c:3b99eeb]: https://github.com/serenity-rs/serenity/commit/3b99eebe1c40ab00ca416c68103c5b1a53eb437a
[c:c18e889]: https://github.com/serenity-rs/serenity/commit/c18e889eca2e481e9168b5450610ad0415e198ae
[c:dc400de]: https://github.com/serenity-rs/serenity/commit/dc400de52a054cf30ea6aa7cc75d530e2c4d4d37
[c:985212a]: https://github.com/serenity-rs/serenity/commit/985212aff1cc73515261846e4e9eeb3867c5bcd4
[c:3a3e399]: https://github.com/serenity-rs/serenity/commit/3a3e3995abb6cc87681d72c88a033ecd9326e66e
[c:283af98]: https://github.com/serenity-rs/serenity/commit/283af983e769b2155b440ab4d610e7fe8ed9b36d
[c:66f8be7]: https://github.com/serenity-rs/serenity/commit/66f8be788c5a9dc7aed355fb074c7dd746fbb60f
[c:a75ac69]: https://github.com/serenity-rs/serenity/commit/a75ac697cb20f31f9a046f9c4e04f9c1bda5b76e
[c:14fc9c6]: https://github.com/serenity-rs/serenity/commit/14fc9c6b5c68a61e37273df6ebb4e82a139efde3
[c:6f28486]: https://github.com/serenity-rs/serenity/commit/6f2848698ed7d15f50d3c3f7c44828f1cf0911d1
[c:a2b388f]: https://github.com/serenity-rs/serenity/commit/a2b388f7baeb2e91800dcdb27d736cefc50dba4f
[c:4cd6caa]: https://github.com/serenity-rs/serenity/commit/4cd6caae070d8840c79ba972cd0918ec6d4213d4
[c:fbfe0cf]: https://github.com/serenity-rs/serenity/commit/fbfe0cf8337db4854778a38bba79c3fd0427ebdf
[c:32cb31c]: https://github.com/serenity-rs/serenity/commit/32cb31c39b736ac3df4464d688c438dce195a7d0
[c:f5458f1]: https://github.com/serenity-rs/serenity/commit/f5458f104c0e6508db2f72b279e5829ccfaacb8e
[c:d245e67]: https://github.com/serenity-rs/serenity/commit/d245e67e4b36e1cdec8cf1d845895644069d672b
[c:10410a2]: https://github.com/serenity-rs/serenity/commit/10410a205b1841f5feab7b0a35c051feaffb9b0f
[c:f4f310d]: https://github.com/serenity-rs/serenity/commit/f4f310ddeef04c0e7aefa064dea9dd7fd324d0df
[c:f8bc937]: https://github.com/serenity-rs/serenity/commit/f8bc937b4c8a50d9ab4aefa95ef5b45fab090edf
[c:a36353b]: https://github.com/serenity-rs/serenity/commit/a36353b45f50ded4d05c4f9895fd0b089d841355
[c:6028072]: https://github.com/serenity-rs/serenity/commit/6028072098a31874fe6059d0001d2a152374acc9
[c:4c97810]: https://github.com/serenity-rs/serenity/commit/4c97810b6d25617fa0a0a4b4769deb751a17d373
[c:c5f9cbe]: https://github.com/serenity-rs/serenity/commit/c5f9cbe731be96aed622838946bf726bad552c6d
[c:81a9bc2]: https://github.com/serenity-rs/serenity/commit/81a9bc2d0b2e179c354192caf23109d501430fc3
[c:c5dc80b]: https://github.com/serenity-rs/serenity/commit/c5dc80bc1642de2a87ce4a2a7f8f28db44c16c11
[c:404ab03]: https://github.com/serenity-rs/serenity/commit/404ab03d4d6fdc1f57daea0e5876fa300029f868
[c:977946f]: https://github.com/serenity-rs/serenity/commit/977946f217d711bbb6958968f011184a09d97e0e
[c:d03809f]: https://github.com/serenity-rs/serenity/commit/d03809f43f62f79060b1ec1b2a5edb5084128a1e
[c:e69bcd3]: https://github.com/serenity-rs/serenity/commit/e69bcd3d06091f6328dcab8ed61ec23d255a6c0f
[c:5a0e8f4]: https://github.com/serenity-rs/serenity/commit/5a0e8f436e84e92468254700c00a31a7de29fc3f
[c:8c9670f]: https://github.com/serenity-rs/serenity/commit/8c9670fbedec8d36259ae2260787f7bf346fb409
[c:cdaa70c]: https://github.com/serenity-rs/serenity/commit/cdaa70cb80d272c6af9ba4c9a7e2e1d2563e5475
[c:63a1000]: https://github.com/serenity-rs/serenity/commit/63a1000fca1a52c4ba0e7432e52d6607582b10ed
[c:8bca94a]: https://github.com/serenity-rs/serenity/commit/8bca94a73a8b9386d1ac77befc312e1d3536f537
[c:d67a21d]: https://github.com/serenity-rs/serenity/commit/d67a21d1d74b80f51e3b4e125dbe7bba0c29f86b
[c:a292c2f]: https://github.com/serenity-rs/serenity/commit/a292c2fcb6826e0305d078b1a08a4535ef2d9c9d
[c:1ce8060]: https://github.com/serenity-rs/serenity/commit/1ce8060b0ae5622d5c698f371988f59c8deb158d
[c:0036620]: https://github.com/serenity-rs/serenity/commit/00366204b19d5585cc0ff1a131f36d42f8257d02
[c:28e6732]: https://github.com/serenity-rs/serenity/commit/28e67324f0a8836538b92305298b31ed858d9eea
[c:aada1b9]: https://github.com/serenity-rs/serenity/commit/aada1b95be893b9ef7f1acecf9245d224149878b
[c:c9e4f27]: https://github.com/serenity-rs/serenity/commit/c9e4f27512cc32aa36ea825223bce1349d9bd79a
[c:787e7d9]: https://github.com/serenity-rs/serenity/commit/787e7d93fd09fe995df412474bb57840cc954487
[c:56cab22]: https://github.com/serenity-rs/serenity/commit/56cab22b4524f7a188e08a1ef4b8156ab138a8aa
[c:36eac42]: https://github.com/serenity-rs/serenity/commit/36eac42126170b41db96479589194f02c7346b88
[c:33d402f]: https://github.com/serenity-rs/serenity/commit/33d402feb0bbc0b47cf28f5bc334a03805a2ffa5
[c:7a8f2cf]: https://github.com/serenity-rs/serenity/commit/7a8f2cfb0826ec358594fa680766cacc7eed659b
[c:d5b5970]: https://github.com/serenity-rs/serenity/commit/d5b597040cd83c8385de5c4c213c65da8bb46f44
[c:4ff348e]: https://github.com/serenity-rs/serenity/commit/4ff348e3ac90c061c1a08d9e14528cd287fdde9d
[c:87bb9a8]: https://github.com/serenity-rs/serenity/commit/87bb9a8aab5dfe80cb2a65c351ed46c70d07938b
[c:9624af0]: https://github.com/serenity-rs/serenity/commit/9624af0def9cd191a51750232edce23970486a0e
[c:1be625b]: https://github.com/serenity-rs/serenity/commit/1be625b1bdc43e109fb34b0ececa29b062c420e6
[c:d3ae156]: https://github.com/serenity-rs/serenity/commit/d3ae156cb80cebda36f2e5a5003dad30825a086e
[c:1dbe9f1]: https://github.com/serenity-rs/serenity/commit/1dbe9f1d839bd3999286c89daabdedc90214e407
[c:f6a3700]: https://github.com/serenity-rs/serenity/commit/f6a3700e89b3dc889aa2a4bf285fb79850b3d8ba
[c:f6c4db0]: https://github.com/serenity-rs/serenity/commit/f6c4db0486254c082761711161ff5c95f60b66a9
[c:a9d1919]: https://github.com/serenity-rs/serenity/commit/a9d191900f926bdeea57faa24f745e3c1f8741ab
[c:11fff2f]: https://github.com/serenity-rs/serenity/commit/11fff2f730aa7de4c8e96986f8a958e542ec82be
[c:1a9bb54]: https://github.com/serenity-rs/serenity/commit/1a9bb549047c3be6a692d1b5df1f569acf4c34d1
[c:2596927]: https://github.com/serenity-rs/serenity/commit/2596927be68466412428d299c94ea630f0c1c999
[c:d7ef273]: https://github.com/serenity-rs/serenity/commit/d7ef273683ac6751b35499505c31da847bf089f3
[c:8d3a079]: https://github.com/serenity-rs/serenity/commit/8d3a079f136dbdd5c640dc19262061551c81484c
[c:e283b51]: https://github.com/serenity-rs/serenity/commit/e283b514428c90c7297b0499397ae2f62805541f
[c:c2b9445]: https://github.com/serenity-rs/serenity/commit/c2b94459e831558c7d0250777e33155b13dc5d2a
[c:16661a3]: https://github.com/serenity-rs/serenity/commit/16661a3abf4786ada95db095a0847a93842c51f3
[c:1ed7ac5]: https://github.com/serenity-rs/serenity/commit/1ed7ac5eab8e2955d9c0f6e913316e55cdff13e5
[c:1976428]: https://github.com/serenity-rs/serenity/commit/1976428ff54976784823c805609a259fe2e5174a
[c:8b21193]: https://github.com/serenity-rs/serenity/commit/8b21193ff61718745018c0245a9048a1ec2f3797
[c:c92acfd]: https://github.com/serenity-rs/serenity/commit/c92acfd89e136b951b61e81b469e76163f286eb2
[c:1e427c6]: https://github.com/serenity-rs/serenity/commit/1e427c62f2126fa22cc95cbafa2a471e4bc319c6
[c:04a57c3]: https://github.com/serenity-rs/serenity/commit/04a57c33c8e785af903860651111624c130a9c63
[c:c1ef6d9]: https://github.com/serenity-rs/serenity/commit/c1ef6d9ac2bfd27538017b9d736e7f4d009f1881
[c:cb7f211]: https://github.com/serenity-rs/serenity/commit/cb7f2110c67336424fbdb7e1a3e43dbbc6d586a6
[c:5c6c72a]: https://github.com/serenity-rs/serenity/commit/5c6c72a0844ae13eb51656a63b5c3b9a99a4b48a
[c:146b1ac]: https://github.com/serenity-rs/serenity/commit/146b1acfc8645e8d274186b545befcd17832eff6
[c:9866a85]: https://github.com/serenity-rs/serenity/commit/9866a8599a4618ab392f7c0cb1a909b3ca6c68b1
[c:dfac5d7]: https://github.com/serenity-rs/serenity/commit/dfac5d722c3688779fa425430926a47ce5c4d3f1
[c:3aca5ca]: https://github.com/serenity-rs/serenity/commit/3aca5cada60065fdb267e521dfe09e84a79d6ca0
[c:6476ed1]: https://github.com/serenity-rs/serenity/commit/6476ed159494d38e79696d31acc1cc2b452a123d
[c:92de3da]: https://github.com/serenity-rs/serenity/commit/92de3da6b422fc853144a2f76836e05440e22d83
[c:71583c3]: https://github.com/serenity-rs/serenity/commit/71583c34de898b29a5e24f16ac304b5be5c0dd07
[c:b4d0765]: https://github.com/serenity-rs/serenity/commit/b4d0765991e84a8890345f04afd581be933d6efc
[c:ad9e987]: https://github.com/serenity-rs/serenity/commit/ad9e9874bd3fc931fa1282cc011d72181dbd6353
[c:0da4ab5]: https://github.com/serenity-rs/serenity/commit/0da4ab54e106b4599f09aeb2891bc778e9bf2f77
[c:b4e4add]: https://github.com/serenity-rs/serenity/commit/b4e4add6019304a464792aecde02888c80a80339
[c:f61fae4]: https://github.com/serenity-rs/serenity/commit/f61fae42d25cb6b38d174ca9dd862ae6e8f7eebf
[c:7e701dc]: https://github.com/serenity-rs/serenity/commit/7e701dc9794d397a35b852f07d839ca451b928cd
[c:2c73618]: https://github.com/serenity-rs/serenity/commit/2c736185494b531ec8ee4f070366d6fdbc0932a2
[c:1fbf1e9]: https://github.com/serenity-rs/serenity/commit/1fbf1e9474a3710687476ffe2b3f4911dca9b4e7
[c:147ed03]: https://github.com/serenity-rs/serenity/commit/147ed038295e3666a73646f6097b30a430958a35
[c:a88673f]: https://github.com/serenity-rs/serenity/commit/a88673f6d01e7b5e115b3cdf366d113772785718
[c:bec660d]: https://github.com/serenity-rs/serenity/commit/bec660d5554a110f9318088b9243255094b4490d
[c:57582d2]: https://github.com/serenity-rs/serenity/commit/57582d215e0d1168117272a0d3b7577cc45cae60
[c:06d101b]: https://github.com/serenity-rs/serenity/commit/06d101b7b0f18cbd20df4222e8c2bbc2de90e271
[c:27bf301]: https://github.com/serenity-rs/serenity/commit/27bf3017be3ee1d7152cdb797f02ba04d7c8c4d7
[c:c2130a8]: https://github.com/serenity-rs/serenity/commit/c2130a8fb395995e08353db9f97c76c0fd51b092
[c:afb3c37]: https://github.com/serenity-rs/serenity/commit/afb3c37184cdcd92d0abd00acb83f141c3b7ea5e
[c:35ee68b]: https://github.com/serenity-rs/serenity/commit/35ee68b36bfe81d6562d8fecc5fbf341f3520caf
[c:e187f73]: https://github.com/serenity-rs/serenity/commit/e187f731ede23e74c378483eea26e3f5c99fd7c5
[c:91ee596]: https://github.com/serenity-rs/serenity/commit/91ee5964b3afbe67ccb4dacaf2fb5c2da21dcb7a
[c:e5073ae]: https://github.com/serenity-rs/serenity/commit/e5073ae7bc157309d37ed4662bec12c0c0fc45eb
[c:16c2089]: https://github.com/serenity-rs/serenity/commit/16c2089bdbe691353d25012e93b99ef01b619582
[c:92fe5bb]: https://github.com/serenity-rs/serenity/commit/92fe5bbf531dbdef62b42bc5161e550188f0af85
[c:57c0826]: https://github.com/serenity-rs/serenity/commit/57c082686371b05ed7834f43b0a506f9b5f7b1e0
[c:3a64da1]: https://github.com/serenity-rs/serenity/commit/3a64da19e75f2c70830beeca9c0963f7d579a992
[c:82c2415]: https://github.com/serenity-rs/serenity/commit/82c2415bfd0f78fb9412b0a805466d85b10319fd
[c:86dbaee]: https://github.com/serenity-rs/serenity/commit/86dbaee11bff1c41ccfbb45716969b0d0b885996
[c:56c40fc]: https://github.com/serenity-rs/serenity/commit/56c40fc53cab1ebda43d1efc0fcd4ab37a73a3d2
[c:7d99bcb]: https://github.com/serenity-rs/serenity/commit/7d99bcb3d7c1359ea48b501e2c57c7aa7c8b6af7
[c:d640238]: https://github.com/serenity-rs/serenity/commit/d6402388180c2bc9e1492202a85ee8738665589a
[c:7823b6e]: https://github.com/serenity-rs/serenity/commit/7823b6e47d4a9d91116244d53954f350ac57128e
[c:5a4fd2a]: https://github.com/serenity-rs/serenity/commit/5a4fd2a077a117f4b5cb097c27beb49d38e2e66a
[c:594a00d]: https://github.com/serenity-rs/serenity/commit/594a00d5a9fcfa028c6aff5f273f1ff9f447795f
[c:74bffd0]: https://github.com/serenity-rs/serenity/commit/74bffd08a9036084afc0baf2e49220a3256024db
[c:6298f67]: https://github.com/serenity-rs/serenity/commit/6298f671cc25e7d341737d57681a43c1fc9937b5
[c:342fdbb]: https://github.com/serenity-rs/serenity/commit/342fdbbe184c30b3afb6dc1cc280d4379cbffd9f
[c:28e0311]: https://github.com/serenity-rs/serenity/commit/28e03116ab470097721d097f584e8b4c0f7cc41e
[c:938936e]: https://github.com/serenity-rs/serenity/commit/938936ed96b215efdfbfa1f7bd616a552f7b2db0
[c:0f77d31]: https://github.com/serenity-rs/serenity/commit/0f77d31cbd29b3dd6f32f5586ae5205d5792a297
[c:89499b2]: https://github.com/serenity-rs/serenity/commit/89499b2f5ea82153a5cfa82df4fa9928b5ced3b5
[c:97ea22f]: https://github.com/serenity-rs/serenity/commit/97ea22fdf394112122f85217916a6b6d50c1a6ab
[c:daa9434]: https://github.com/serenity-rs/serenity/commit/daa9434ad6b800c2fcddd9f7bac38e6551bee8cc
[c:ab4e7fe]: https://github.com/serenity-rs/serenity/commit/ab4e7febb36206faf5998e38c5fd1d6a354a2340
[c:14e747d]: https://github.com/serenity-rs/serenity/commit/14e747db0d8c3157755b16b7ebc8bb57db8400cb
[c:23ed9ea]: https://github.com/serenity-rs/serenity/commit/23ed9eae7c56143b41375ccc2572e2812488e550
[c:4556594]: https://github.com/serenity-rs/serenity/commit/45565946cdc5513adc481d7b0e8e2e5bc7a8c887
[c:8fe7127]: https://github.com/serenity-rs/serenity/commit/8fe7127c195f5f8441b1ae2af32d61b9f3872d04
[c:a29e0f7]: https://github.com/serenity-rs/serenity/commit/a29e0f73755b41c6e613f5765040acc09b76089e
[c:5f6ad2a]: https://github.com/serenity-rs/serenity/commit/5f6ad2a12a31e55710b906ae4566bd8ce22930ee
[c:08b7163]: https://github.com/serenity-rs/serenity/commit/08b71639956faa3908f220159bbd6d7f8bafe2a9
[c:5b43bdb]: https://github.com/serenity-rs/serenity/commit/5b43bdb005415d45d42bc9bff3600a5896c93fc4
[c:dd5661c]: https://github.com/serenity-rs/serenity/commit/dd5661cfc8f6b3200f0ead13e0e88512fb2752c4
[c:1577efc]: https://github.com/serenity-rs/serenity/commit/1577efc2a5a67907ccfbf526dcc24653ba9a3fae
[c:4d5f4f1]: https://github.com/serenity-rs/serenity/commit/4d5f4f1c505101d2c061ce361b38c394d367d03e
[c:fbba88c]: https://github.com/serenity-rs/serenity/commit/fbba88c27392e2d061c1388740fe18683e54fbd0
[c:a3dd38e]: https://github.com/serenity-rs/serenity/commit/a3dd38ef93b1eb4992665f90ba7cbcb9dfc3d0ac
[c:a95ac81]: https://github.com/serenity-rs/serenity/commit/a95ac815f08e4c8ee5fe32f542f654433313f4ab
[c:afba7f5]: https://github.com/serenity-rs/serenity/commit/afba7f578ecb1456988f0542ea5018832e4968a9
[c:701d759]: https://github.com/serenity-rs/serenity/commit/701d7593a1d88ef9e1987efdd6024ae5336115f1
[c:5063de1]: https://github.com/serenity-rs/serenity/commit/5063de1ec89433cf5a0573c4d529dd902432332d
[c:f26fb7f]: https://github.com/serenity-rs/serenity/commit/f26fb7fe1fe075b243b561a7d64d1039ae8f6aea
[c:1715476]: https://github.com/serenity-rs/serenity/commit/1715476d23d7dc22683042d958494276ea080d4d
[c:363a311]: https://github.com/serenity-rs/serenity/commit/363a311e5083956bb5010cc3c832ce81e8aa3c97
[c:bdf99b9]: https://github.com/serenity-rs/serenity/commit/bdf99b950ed1873fb3275a86c7245c9501939638
[c:599efc1]: https://github.com/serenity-rs/serenity/commit/599efc13d7f33b5fe0c5dee0a617d6dd22281805
[c:bc46711]: https://github.com/serenity-rs/serenity/commit/bc46711e59cecdeb3af9fc49bd5c55ab7fc317dd
[c:9a2cce8]: https://github.com/serenity-rs/serenity/commit/9a2cce801a4922bf5162b294eac482c27589cafc
[c:080f5b0]: https://github.com/serenity-rs/serenity/commit/080f5b08a48a045eed09fcafc9b56dba7a0a30c0
[c:f46bbc8]: https://github.com/serenity-rs/serenity/commit/f46bbc8deb98e211c866637d9bdb71baf9fc49da
[c:b1389aa]: https://github.com/serenity-rs/serenity/commit/b1389aa8dfffcbd3dc55004d73aa1ee1fbabbf71
[c:48de0e5]: https://github.com/serenity-rs/serenity/commit/48de0e59eab7ba339de763edbbfc0590cb31f056
[c:f9f6079]: https://github.com/serenity-rs/serenity/commit/f9f607909384cf7f766d992baddeb4d1d462d407
[c:34881b9]: https://github.com/serenity-rs/serenity/commit/34881b9038321f7d6cfe23a59dd1d7be4c2bcf1d
[c:dfe38c2]: https://github.com/serenity-rs/serenity/commit/dfe38c2041cbb82704602f1c940a6fec9cdba4c5
[c:34c484c]: https://github.com/serenity-rs/serenity/commit/34c484c31e4e3ea2d41422251f16394dcc4395fb
[c:25cdfeb]: https://github.com/serenity-rs/serenity/commit/25cdfebcdb4b4ee8b72a5ad84c73bcafa09eab6a
[c:a5f9a4f]: https://github.com/serenity-rs/serenity/commit/a5f9a4fe3a2aafd1239b7379ae810a5406c1cd75
[c:ae93570]: https://github.com/serenity-rs/serenity/commit/ae93570ac70b413e30a8cf056aa673b7872d8921
[c:178f62c]: https://github.com/serenity-rs/serenity/commit/178f62c4ec3575db74914a4fd9f767069eb8765f
[c:68d5049]: https://github.com/serenity-rs/serenity/commit/68d5049161f220dbe084e3850017ae8092d854ce
[c:7a38ad1]: https://github.com/serenity-rs/serenity/commit/7a38ad1c5637c134592ec6cb7acb5de2c4310a01
[c:d504bf2]: https://github.com/serenity-rs/serenity/commit/d504bf2431853c9e7ba43e10eee004639292d5b1
[c:cb5d090]: https://github.com/serenity-rs/serenity/commit/cb5d090904a6039840186c025e6393dbc5f8beb0
[c:5ce8177]: https://github.com/serenity-rs/serenity/commit/5ce817762eacfc8c41b0770098d417dbfad1ca3f
[c:e57433f]: https://github.com/serenity-rs/serenity/commit/e57433fe167100e971b0e794f0707bf3f2128048
[c:173c1a6]: https://github.com/serenity-rs/serenity/commit/173c1a6b010b908bba4b73eea4e984c1233bb80e
[c:590437b]: https://github.com/serenity-rs/serenity/commit/590437ba33fa5ee528fbf10d634da31b21731f16
[c:fa68bb1]: https://github.com/serenity-rs/serenity/commit/fa68bb1cbc79da567165cf2b72744c1095a8f1fe
[c:228ee8e]: https://github.com/serenity-rs/serenity/commit/228ee8eb8093d995f880f632217cb638ee8ee0e1
[c:cf1a897]: https://github.com/serenity-rs/serenity/commit/cf1a89709c235f445797c440d0f16eacdc195854
[c:bc9315c]: https://github.com/serenity-rs/serenity/commit/bc9315c9dfae8eb3e116e4c6061c8df1d790670d
[c:9e2b9df]: https://github.com/serenity-rs/serenity/commit/9e2b9df3d6f26d90b6034c46508d44b276a1aaa8
[c:c889c7e]: https://github.com/serenity-rs/serenity/commit/c889c7eee8917087e245d61cdb36a291f3b6cd6e
[c:a8cd62d]: https://github.com/serenity-rs/serenity/commit/a8cd62dbf426254c4086f5b84f296c5d19b9e3c3
[c:5223ea0]: https://github.com/serenity-rs/serenity/commit/5223ea068f5f907c9a889d6398a07a1978915167
[c:82e5095]: https://github.com/serenity-rs/serenity/commit/82e5095b7a4425dd00b3a23db74b12884041f472
[c:c80084c]: https://github.com/serenity-rs/serenity/commit/c80084c0b92657c830112299a2daea6159070fff
[c:d1a3ded]: https://github.com/serenity-rs/serenity/commit/d1a3ded4a63d350cd82f00e6350a789796f271e0
[c:a168913]: https://github.com/serenity-rs/serenity/commit/a16891342c21807592514233b797b48611e2ff16
[c:36e0909]: https://github.com/serenity-rs/serenity/commit/36e0909fc4c10b07230c2d5ab3245e7f3f39c0e7
[c:87f5bae]: https://github.com/serenity-rs/serenity/commit/87f5baed289b658660204747f62db0179f6da8e7
[c:3f93af8]: https://github.com/serenity-rs/serenity/commit/3f93af86035d48b59902f4b905996af099eb8f7c
[c:1f13d70]: https://github.com/serenity-rs/serenity/commit/1f13d70c408b80eb1ee41ce9f060645761d4e1bf
[c:0632019]: https://github.com/serenity-rs/serenity/commit/0632019a3026d56ae4d4cb1985d96789a8ee4622
[c:bb638c0]: https://github.com/serenity-rs/serenity/commit/bb638c073a812f87c5ed6e89d1c7587cb02a47b8
[c:86a5861]: https://github.com/serenity-rs/serenity/commit/86a586182d00522fb51f5fe922667248ca194c4d
[c:6139a63]: https://github.com/serenity-rs/serenity/commit/6139a63fb91971a54d61b25e769dccc77784897f
[c:f724aec]: https://github.com/serenity-rs/serenity/commit/f724aec70650cfda564610272400da68fa6dfb40
[c:56a9fcb]: https://github.com/serenity-rs/serenity/commit/56a9fcb0dd7e042df76c98ffe1a9eb5999b54ec4
[c:508864d]: https://github.com/serenity-rs/serenity/commit/508864d21df3428dcd07498c0d142e3322f71096
[c:2060cfb]: https://github.com/serenity-rs/serenity/commit/2060cfb9063f33babbfd42c6d10cff6cec740ac3
[c:111f437]: https://github.com/serenity-rs/serenity/commit/111f437000d439da1de34268bb2bf00fe5fec3c9
[c:65c0122]: https://github.com/serenity-rs/serenity/commit/65c01229c65e53668344fc6b0607709fbbdecdfa
[c:6219fec]: https://github.com/serenity-rs/serenity/commit/6219fece673bf93fe4b9a3977a6f665c11c4861a
[c:74f4396]: https://github.com/serenity-rs/serenity/commit/74f439688da3eb53476f765523941900b4ccb34f
[c:ba84c77]: https://github.com/serenity-rs/serenity/commit/ba84c77db1ce4c6c807e0357dc1774e5d3dc4d38
[c:eda64a6]: https://github.com/serenity-rs/serenity/commit/eda64a692910b025345010d2dc7fd48c08deaf12
[c:1725627]: https://github.com/serenity-rs/serenity/commit/172562740d4e77e427da54700c7eb15bbb21039c
[c:1a0cc66]: https://github.com/serenity-rs/serenity/commit/1a0cc66604a5c3ef5fdd029cc063e25307e2ed7b
[c:5114d80]: https://github.com/serenity-rs/serenity/commit/5114d80eb9e15883c8f2965e08244ccb43fcef7a
[c:4b2c4d9]: https://github.com/serenity-rs/serenity/commit/4b2c4d9c4b14bb7b8e288d1f7620db32d1ee6323
[c:c95bcd3]: https://github.com/serenity-rs/serenity/commit/c95bcd3737851de8311e17a77eebc484462bccb2
[c:983d0e4]: https://github.com/serenity-rs/serenity/commit/983d0e402ffa9b8b1af106066795f3afae951e16
[c:584d3a9]: https://github.com/serenity-rs/serenity/commit/584d3a9bfcd324868e30cf93f590d9f0ec0e73b4
[c:faebd0c]: https://github.com/serenity-rs/serenity/commit/faebd0c390a087439c829f23b31243bb45824a58
[c:22a3f64]: https://github.com/serenity-rs/serenity/commit/22a3f64e8a3547e691c5a9cacaadf1e2f6ed7707
[c:3b2b18c]: https://github.com/serenity-rs/serenity/commit/3b2b18c6165128f5acda3163c5d6032fdd7821c3
[c:3f985b5]: https://github.com/serenity-rs/serenity/commit/3f985b5fa65791314bb2d86c96586c0b5e128b26
[c:a3a861c]: https://github.com/serenity-rs/serenity/commit/a3a861c3b930cc0de9b80b47078751ad2c90c3bc
[c:4eee51c]: https://github.com/serenity-rs/serenity/commit/4eee51c2cbbd4f50a6c50bb09b5cf10fa83cffd5
[c:519ea83]: https://github.com/serenity-rs/serenity/commit/519ea837d3e3a01409e94dee99daf20597d78d3f
[c:7a8adfb]: https://github.com/serenity-rs/serenity/commit/7a8adfb4d482291363edff6c2a551474a9ebbedd
[c:48ca506]: https://github.com/serenity-rs/serenity/commit/48ca50627004afdb836b3c9ccb84bb92373eaa25
[c:602bcb6]: https://github.com/serenity-rs/serenity/commit/602bcb65e7a69d9ef6164bb60f1271d34b9448b2
[c:b767215]: https://github.com/serenity-rs/serenity/commit/b76721526493b0bd1f1f2d7946de3a6a659e7a6c
[c:16dcc69]: https://github.com/serenity-rs/serenity/commit/16dcc691b4b78ac84b29196ff1c317fe1bd6e346
[c:63cdeec]: https://github.com/serenity-rs/serenity/commit/63cdeec52b4c7ac83e060d4457af7219df22ca8a
[c:a1cdd7f]: https://github.com/serenity-rs/serenity/commit/a1cdd7f9f976c544f4e423f42abf5b0c54f82a2e
[c:1e11fb4]: https://github.com/serenity-rs/serenity/commit/1e11fb411ab61073ec48712b7b859f7e9ebc3996
[c:75062ae]: https://github.com/serenity-rs/serenity/commit/75062ae7d7c70ec291aad069c8a77f954be77a20
[c:bc77218]: https://github.com/serenity-rs/serenity/commit/bc7721853842658b2ad4e9531a8b9780507042a9
[c:0bb6bb1]: https://github.com/serenity-rs/serenity/commit/0bb6bb1b5365ca94810bdb38241fdb141b521d30
[c:d51b0b1]: https://github.com/serenity-rs/serenity/commit/d51b0b1895168a392c8ece1c106c9839fb4e2dd8
[c:4e7c60a]: https://github.com/serenity-rs/serenity/commit/4e7c60a48218698573ffb71494733c37d7e257b1
[c:0675fd1]: https://github.com/serenity-rs/serenity/commit/0675fd193bc380b15cbe932ca772ee1a1cb0b392
[c:5774572]: https://github.com/serenity-rs/serenity/commit/5774572db780be0b5af7abe0cdf2768226823607
[c:e6039c4]: https://github.com/serenity-rs/serenity/commit/e6039c4ba5b5d0344264fb940ef89e992c309daa
[c:e748deb]: https://github.com/serenity-rs/serenity/commit/e748deb229433df846ee75c3ad870d89c83679e6
[c:0ccb793]: https://github.com/serenity-rs/serenity/commit/0ccb793896dfbfa78a08030e87590352ecdd834b
[c:2f48379]: https://github.com/serenity-rs/serenity/commit/2f4837903b2853bf6c86f746ab93030926b761eb
[c:73ff5ba]: https://github.com/serenity-rs/serenity/commit/73ff5ba654de6765c3dd4efe36ae3d2a1a0a0527
[c:db101fd]: https://github.com/serenity-rs/serenity/commit/db101fda7a54c30ffa456fcf042957d7ba6f6cb4

[c:be5cb54]: https://github.com/serenity-rs/serenity/commit/be5cb544b62562b1d66a48ade8a8882b9bcbae3d
[c:1d05a1e]: https://github.com/serenity-rs/serenity/commit/1d05a1e6ecc078d355618114bfad3cf053016657
[c:69896e0]: https://github.com/serenity-rs/serenity/commit/69896e03cc86b2a3c0261c80540cf59de60ea662
[c:93b66d0]: https://github.com/serenity-rs/serenity/commit/93b66d00454037284c95db07adcda971fef99e9e
[c:9e10d54]: https://github.com/serenity-rs/serenity/commit/9e10d546441ccdaccb1a1ad1d94c915e95869eba
[c:90dae53]: https://github.com/serenity-rs/serenity/commit/90dae534fa3c2438693ad12eae4297e4bc617b37
[c:944cd54]: https://github.com/serenity-rs/serenity/commit/944cd541ee7fcb4476af4ca518390d62c1572927
[c:b026f00]: https://github.com/serenity-rs/serenity/commit/b026f00a65dbe6425d62bf6f20f38766b1dc4216
[c:6bd2f74]: https://github.com/serenity-rs/serenity/commit/6bd2f74067edfe71070509f5ac0ccf389b00437d
[c:7b89775]: https://github.com/serenity-rs/serenity/commit/7b89775858d92a1c8be05f213b92fbe72b083980
[c:ee00e92]: https://github.com/serenity-rs/serenity/commit/ee00e92b21af0cfa6c0e5e245ccccec2463e40ec
[c:47a297b]: https://github.com/serenity-rs/serenity/commit/47a297b453017cf38b8f22d32f6d61f36164c627
[c:cfd518e]: https://github.com/serenity-rs/serenity/commit/cfd518e328697525339175ef6ed4a1f69ec9ba86
[c:6121fd0]: https://github.com/serenity-rs/serenity/commit/6121fd0b388783301146a1f586cc5fda15f4388e
[c:c322657]: https://github.com/serenity-rs/serenity/commit/c322657acaae0624f893ae4b8258b7da30e43501
[c:5b84896]: https://github.com/serenity-rs/serenity/commit/5b84896127e1182341d45042d7148041e44f3d01
[c:6f87d25]: https://github.com/serenity-rs/serenity/commit/6f87d25935b0dd13bd55ef7e53a13bf2bc30a50d
[c:c0463c0]: https://github.com/serenity-rs/serenity/commit/c0463c0d7f04250aab178e823c19f4866ffa9e09
[c:bfb5f13]: https://github.com/serenity-rs/serenity/commit/bfb5f13e72f8694864ec3863d39785f425611dba
[c:5a700f7]: https://github.com/serenity-rs/serenity/commit/5a700f7ea40fc89ff1c5550e673dd37a3b739f06
[c:0bc3ce0]: https://github.com/serenity-rs/serenity/commit/0bc3ce0c9b68d5005948c5624173963aa930a40c
[c:d50628e]: https://github.com/serenity-rs/serenity/commit/d50628ef832296bf6b4f5b11dcc41d230b39335f
[c:02a5e15]: https://github.com/serenity-rs/serenity/commit/02a5e1533c217c4a88d3e6a96f2a1fc21bb54a52
[c:afeb76e]: https://github.com/serenity-rs/serenity/commit/afeb76e0d96e244eab84646310d71ccc55f9b4a2
[c:c74162c]: https://github.com/serenity-rs/serenity/commit/c74162cff0d033736cdae99acaadb156b1bcf9fd
[c:6c5aa4d]: https://github.com/serenity-rs/serenity/commit/6c5aa4dcd83129dbc26f69dac2d46bed934283b0
[c:cf4e216]: https://github.com/serenity-rs/serenity/commit/cf4e216cdaac6887324b7bde753f0681a4c9574a
[c:c85d44a]: https://github.com/serenity-rs/serenity/commit/c85d44a5004f9bea9052908175b98ac2a927c41f
[c:93d6ab8]: https://github.com/serenity-rs/serenity/commit/93d6ab83d5da143044f83c37bcb981988231e9c2
[c:b905554]: https://github.com/serenity-rs/serenity/commit/b90555404a4334110d52c6fa92b0bde04059c7c4
[c:f9de6c5]: https://github.com/serenity-rs/serenity/commit/f9de6c5d97ff55382e8f5396b5ec916cc9048d12
[c:2da46a8]: https://github.com/serenity-rs/serenity/commit/2da46a8402926f2acc90d183a44d370f8708dad0
[c:99eba2b]: https://github.com/serenity-rs/serenity/commit/99eba2b5edb976a1652e47a8d18c4116a330adc3
[c:e87293e]: https://github.com/serenity-rs/serenity/commit/e87293eccbee6c8c04b92c552a99eb654a6d81bd
[c:2a571d3]: https://github.com/serenity-rs/serenity/commit/2a571d3c2af40cbf1007f72886c7651e668fc360
[c:bd1342e]: https://github.com/serenity-rs/serenity/commit/bd1342e41e5357cea5ff7541a53f1a88ae1f21ec
[c:2dcb9b3]: https://github.com/serenity-rs/serenity/commit/2dcb9b350f9498b69dfda5759b329c363b9453c1
[c:5c1e31e]: https://github.com/serenity-rs/serenity/commit/5c1e31e732a59c50d172a50a3f9f815d0d675c8c
[c:32f4430]: https://github.com/serenity-rs/serenity/commit/32f4430b702e7fc2a37008e41e4a7091c0477669
[c:6d476cf]: https://github.com/serenity-rs/serenity/commit/6d476cff91cea4de6394e0307a3ff9765e66ee2e
[c:f630f32]: https://github.com/serenity-rs/serenity/commit/f630f324617e66fb6d3f91ac04e8121e0ee03370
[c:34c2f22]: https://github.com/serenity-rs/serenity/commit/34c2f22bcc521a0ff2d9158678f05532e5be3b55
[c:4f7dad7]: https://github.com/serenity-rs/serenity/commit/4f7dad7c96b0b2111502325090147beacc04b5ce
[c:04fc541]: https://github.com/serenity-rs/serenity/commit/04fc541b0e34d0e16c99039ce94cfe3cb79881b8
[c:a8b4e2f]: https://github.com/serenity-rs/serenity/commit/a8b4e2f9ef8bee24f967944f184d77789b4a9be8
[c:6a0106c]: https://github.com/serenity-rs/serenity/commit/6a0106c92f8b56cc584ff350c0bb3651bb14edd6
[c:fc7c76a]: https://github.com/serenity-rs/serenity/commit/fc7c76a150f667dfea26a4a8a5908f52fa40c88b
[c:57a2e4d]: https://github.com/serenity-rs/serenity/commit/57a2e4dac15ee03dedb994ef683b5d47cf85a60a
[c:5379d3e]: https://github.com/serenity-rs/serenity/commit/5379d3e2874069b17a08c21a9579e45917ad253b
[c:1977e9f]: https://github.com/serenity-rs/serenity/commit/1977e9f8d409b68d31437a5aa4412f55bd82960b
[c:4f2a73b]: https://github.com/serenity-rs/serenity/commit/4f2a73b82cf487e0cef9712a90dc9f37b6e4f2ad
[c:d9c8980]: https://github.com/serenity-rs/serenity/commit/d9c898018d6ce5ff9f2cf325ce6a4392b118b2d3
[c:738465f]: https://github.com/serenity-rs/serenity/commit/738465fc800b91b8f214b7ebb7075a21936e60af
[c:b21f053]: https://github.com/serenity-rs/serenity/commit/b21f053b87d611d0f803cdfbc232b70621daa3ab
[c:921ff6f]: https://github.com/serenity-rs/serenity/commit/921ff6fe8ca25449cf3b94540d9a5ffb58d1f69a
[c:c3358cf]: https://github.com/serenity-rs/serenity/commit/c3358cf6b251d9c3e6b4bfa8aba7b953b33074a9
[c:46f7608]: https://github.com/serenity-rs/serenity/commit/46f760854da7f8f9fe4b930e0ffdf17a96d6d14a
[c:bade6e5]: https://github.com/serenity-rs/serenity/commit/bade6e59c365228ba83e2bd7072c3ca62f8bdfd4
[c:5f81d4a]: https://github.com/serenity-rs/serenity/commit/5f81d4a9ddedcb352424753b7b790153d18bb728
[c:247c073]: https://github.com/serenity-rs/serenity/commit/247c073566880ea3ee8619eee0edb2f930f1c4cf
[c:b2197db]: https://github.com/serenity-rs/serenity/commit/b2197dbbbf01588e06c3597b2079b79180958ef9
[c:6987175]: https://github.com/serenity-rs/serenity/commit/6987175e56140162c32d0fa0724e3fbaac601f70
[c:6085aad]: https://github.com/serenity-rs/serenity/commit/6085aad8c8f82b3d4f661729ab5cb69aea63e381
[c:cf040cb]: https://github.com/serenity-rs/serenity/commit/cf040cb73582cac640c2c7ce050e5bb03da9887a
[c:f915fee]: https://github.com/serenity-rs/serenity/commit/f915fee1e61254307e4845c75370ec3c6c110377
[c:38b11da]: https://github.com/serenity-rs/serenity/commit/38b11da9bc806e342351e32e81bcf4eee69ff1c7
[c:b180f21]: https://github.com/serenity-rs/serenity/commit/b180f217d4f8e381ef1c81fa9fc481cc7e595040
[c:f70112d]: https://github.com/serenity-rs/serenity/commit/f70112da89395e28ac6a791ca9da67246157e9dd
[c:fe946df]: https://github.com/serenity-rs/serenity/commit/fe946df80ed725784ed5b6d5ee233fe82b896861
[c:62b9c84]: https://github.com/serenity-rs/serenity/commit/62b9c841ddd430eebdc3e40c796015e374897cd8
[c:707697e]: https://github.com/serenity-rs/serenity/commit/707697e5b44f09e997b40029533e37e9ccf61dcf
[c:df47df1]: https://github.com/serenity-rs/serenity/commit/df47df1d2ccebcc3dc5fa0866fcb8e877f10a72a
[c:0acabdf]: https://github.com/serenity-rs/serenity/commit/0acabdf71f64d0eaa28ef3992a459b0570fdcd89
[c:37e33bd]: https://github.com/serenity-rs/serenity/commit/37e33bd693dfc4e0ca1ff70ff476b4aee571371e
[c:ccf0d5b]: https://github.com/serenity-rs/serenity/commit/ccf0d5b101b723ba249530da4365edaffbc8991c
[c:2eeb336]: https://github.com/serenity-rs/serenity/commit/2eeb336428263afc71355f9b72430b7aac19d2a2
[c:9bfc1e7]: https://github.com/serenity-rs/serenity/commit/9bfc1e78d47ee55c69ab36f00e743ee35f02a497
[c:f36b4c1]: https://github.com/serenity-rs/serenity/commit/f36b4c12936c29d61a15c6806a5f60efde0ea20e
[c:0611e2c]: https://github.com/serenity-rs/serenity/commit/0611e2c64d9abe436702b89bcfdf27c50222f437
[c:cfbb7b3]: https://github.com/serenity-rs/serenity/commit/cfbb7b32ff87cf2375995aab539bea3de0a33074
[c:7e1a64e]: https://github.com/serenity-rs/serenity/commit/7e1a64e23fedfb28c68604beca2136469cc55c69
[c:5b7a23c]: https://github.com/serenity-rs/serenity/commit/5b7a23c032d2c055dd20da652516c7f1b5fa8294
[c:eb29760]: https://github.com/serenity-rs/serenity/commit/eb2976047c51fd265a03c5a8098225d7363bbead
[c:7392f3f]: https://github.com/serenity-rs/serenity/commit/7392f3f237e3126fc1fa5affbd1359e1984da799
[c:e848af8]: https://github.com/serenity-rs/serenity/commit/e848af85ccaa7b44168a97102e6febdcd598be3d
[c:0d1c88e]: https://github.com/serenity-rs/serenity/commit/0d1c88edb77e5ba41fda4c9d63c4220127574a9c
[c:096d1da]: https://github.com/serenity-rs/serenity/commit/096d1da60290fa9823ccdc72f48958aec1f7de27

[c:dbf4d68]: https://github.com/serenity-rs/serenity/commit/dbf4d685a94f6b7f6f4a2f7f21e12c4e1425c231
[c:ea8ec29]: https://github.com/serenity-rs/serenity/commit/ea8ec292711eefe285eab9ad47322c7d96ff43f1
[c:312ae16]: https://github.com/serenity-rs/serenity/commit/312ae16b9bd9ab5b92e0bcfae6730315cc5116aa
[c:3117f1d]: https://github.com/serenity-rs/serenity/commit/3117f1da55436cd972ef0928d905e34e77692f13
[c:1d446fe]: https://github.com/serenity-rs/serenity/commit/1d446fe825ce5453b47ce31686e25a882b637c2d
[c:0aa1a6c]: https://github.com/serenity-rs/serenity/commit/0aa1a6c232728cb216e7013f2aafe3ed73326a24
[c:e7ba5bf]: https://github.com/serenity-rs/serenity/commit/e7ba5bf211d20ff3b37613973e9553c5682b0894
[c:ca02662]: https://github.com/serenity-rs/serenity/commit/ca026624d07a5913c5b68a05030e187aa2672299
[c:50cd285]: https://github.com/serenity-rs/serenity/commit/50cd28595b007c3501771ae32b6c6f25c08cb834
[c:4c53b48]: https://github.com/serenity-rs/serenity/commit/4c53b48534010c65810da2932d71d889b8bab0b2
[c:13cf056]: https://github.com/serenity-rs/serenity/commit/13cf056a4e9e1b8d4557aaa79dc48a0ed7cfdf2c
[c:3e6d131]: https://github.com/serenity-rs/serenity/commit/3e6d131f1bac92abf367917f487605976e0b22bf
[c:09b523d]: https://github.com/serenity-rs/serenity/commit/09b523d218d62059016013da20a15db8f0b0585b
[c:4f488f8]: https://github.com/serenity-rs/serenity/commit/4f488f86ff38b59a9f38cc8b22711a081260add3
[c:6a400f9]: https://github.com/serenity-rs/serenity/commit/6a400f96721d44b18828ef56025ae3388874f1d6
[c:537753b]: https://github.com/serenity-rs/serenity/commit/537753bbcf2a96a148a0d4f5975967bd3bd4c8c2
[c:eb09d13]: https://github.com/serenity-rs/serenity/commit/eb09d13a371b8c3bcc6287c1162a958c0806c014
[c:e215c9d]: https://github.com/serenity-rs/serenity/commit/e215c9d331681010f98f34242375ba2224328748
[c:59ab451]: https://github.com/serenity-rs/serenity/commit/59ab451f282fa936162cd1c7528a35ca5b6ae659
[c:eb14984]: https://github.com/serenity-rs/serenity/commit/eb14984ad02f1e2cdca4251a957e3e6f59d13358
[c:ce97f9e]: https://github.com/serenity-rs/serenity/commit/ce97f9e72fa81c9c7aa5969051619b28bd8e755b
[c:2b2006c]: https://github.com/serenity-rs/serenity/commit/2b2006c73e6143dd91901e938c6e6a2bb4fe8d2d
[c:46ace1f]: https://github.com/serenity-rs/serenity/commit/46ace1f7dcaca9c993dacc8e920f65728e3e191b
[c:ae09e57]: https://github.com/serenity-rs/serenity/commit/ae09e5704b609807de1fccb723c3d90b591a7e93
[c:0be7d1a]: https://github.com/serenity-rs/serenity/commit/0be7d1a9dab2decf411297ba5621d79b30213bd4
[c:9d18334]: https://github.com/serenity-rs/serenity/commit/9d18334020daff68cd14470b5b496b50b8ca10ac
[c:fb203ae]: https://github.com/serenity-rs/serenity/commit/fb203ae3e71c898aaef9a4805b489fbfbcecbc05
[c:413e3ef]: https://github.com/serenity-rs/serenity/commit/413e3eff769c773e9f000d91a1ad9d502232ba40
[c:6192107]: https://github.com/serenity-rs/serenity/commit/6192107f0b03e5bad5b9d9166fa74e5c782621d4
[c:47b9afb]: https://github.com/serenity-rs/serenity/commit/47b9afb6ac827a699e9af1d7411b78087d5c47cb
[c:5cb9cb9]: https://github.com/serenity-rs/serenity/commit/5cb9cb917fc5b75aa74b33f8416a10b2ad21c0f8
[c:988bfcf]: https://github.com/serenity-rs/serenity/commit/988bfcfadaf3478c3ef478da049598e9c24b26f0
[c:87564a4]: https://github.com/serenity-rs/serenity/commit/87564a4ddbe6b1fc826c9b52b4e03679cca642fe
[c:695bbef]: https://github.com/serenity-rs/serenity/commit/695bbef18e06696b9b9d1f46225099e94001b6c8
[c:fb83066]: https://github.com/serenity-rs/serenity/commit/fb83066eb765a62613780fd831d6672eed7a152d
[c:504f5a4]: https://github.com/serenity-rs/serenity/commit/504f5a4410cd3bc4ea396d5d6eec2bfef2f7fa53
[c:c21f052]: https://github.com/serenity-rs/serenity/commit/c21f0525d42b5bc6ff2b96599aea2f1ff7e3c8a6
[c:d3cf53a]: https://github.com/serenity-rs/serenity/commit/d3cf53a7e026f872f693de055a66760cff4d54d0
[c:7a7e8cc]: https://github.com/serenity-rs/serenity/commit/7a7e8cce663b004b289077e628bca10ddf4e6d52
[c:3fba372]: https://github.com/serenity-rs/serenity/commit/3fba3726b2726c83c0e2a8a4e9862a214e7751da
[c:cb30408]: https://github.com/serenity-rs/serenity/commit/cb304082cbe7340958062def52c8426343e8a072
[c:5d50c56]: https://github.com/serenity-rs/serenity/commit/5d50c560dc8dcf2833b9453b2a72697eb4f57e64

[c:1c7e98b]: https://github.com/serenity-rs/serenity/commit/1c7e98b70d8eedc94ce731276ec76d3ef7505588
[c:5c9c194]: https://github.com/serenity-rs/serenity/commit/5c9c194c82bef5fda93143c12e30173737a12429
[c:47a0a14]: https://github.com/serenity-rs/serenity/commit/47a0a1420ff7d008985dc206fade40d89f6bf0e6
[c:2635f7b]: https://github.com/serenity-rs/serenity/commit/2635f7ba306c61a0e1248d71392106cc694243fb
[c:073c22f]: https://github.com/serenity-rs/serenity/commit/073c22f00d53b78476bddf6bfbd4cd42f1b45aeb
[c:38a39f5]: https://github.com/serenity-rs/serenity/commit/38a39f59b5ad00f8277f9cba59706b0f7bb824fe
[c:c60d504]: https://github.com/serenity-rs/serenity/commit/c60d5042c8117038e3ab14fc2ab1fff54c08b77e
[c:7c50e70]: https://github.com/serenity-rs/serenity/commit/7c50e70fa459c4ef44745f4a498cae4ca1b1db12
[c:b03d14b]: https://github.com/serenity-rs/serenity/commit/b03d14b9a222ff3878b0615822f41cdca4e05896
[c:7b95b41]: https://github.com/serenity-rs/serenity/commit/7b95b41a4ffb1b8c5226d5dd56706cfedb62a82f
[c:132fd68]: https://github.com/serenity-rs/serenity/commit/132fd68aa06a01a2c930270a7193056effe002a0
[c:a788fe2]: https://github.com/serenity-rs/serenity/commit/a788fe25f06728f98f722912bef16abc4c79d714
[c:cbfd92b]: https://github.com/serenity-rs/serenity/commit/cbfd92b6da31958a21344ff5fc50e566b39f25a9
[c:f7907ed]: https://github.com/serenity-rs/serenity/commit/f7907ed75ad105057e5841211a4bf5a963c220fd
[c:1744883]: https://github.com/serenity-rs/serenity/commit/1744883e5a8fd552a9cfb82f1164df8ac8f2f58e
[c:22d2276]: https://github.com/serenity-rs/serenity/commit/22d2276dc31e25ef0680668aab7cb52b6692a06f
[c:97c01ee]: https://github.com/serenity-rs/serenity/commit/97c01ee592b8e0d32986a037c3137a9c241a4af8
[c:d390ca1]: https://github.com/serenity-rs/serenity/commit/d390ca12276b8c70b61ad5d95191fc4dd7f3444e

[c:822406e]: https://github.com/serenity-rs/serenity/commit/822406ef13cdeffd2b199e72b42ffcbda73a0202

[c:aa9748e]: https://github.com/serenity-rs/serenity/commit/aa9748ed44f020a84199e3c71882d76af885ad19
[c:07f15ad]: https://github.com/serenity-rs/serenity/commit/07f15ad8e0c09790f98c04f1b3e978f08f61f467
[c:59bc14c]: https://github.com/serenity-rs/serenity/commit/59bc14cbb35e23f48024ba3eb42525d52e6f44af
[c:a710113]: https://github.com/serenity-rs/serenity/commit/a710113f588864cb7304b251c9aa25904529a6b4
[c:7983a8e]: https://github.com/serenity-rs/serenity/commit/7983a8e3a064990ee3d08bd673f6e25122dddb77
[c:9b8a31a]: https://github.com/serenity-rs/serenity/commit/9b8a31a86b4cf08877d1a7c767de74afd8a11dc0
[c:f88355b]: https://github.com/serenity-rs/serenity/commit/f88355bd2d4d9532cb36d95f55ffb39bcd8416c8
[c:879f193]: https://github.com/serenity-rs/serenity/commit/879f1938c4654a83301a0a77dea1e201df5ab6b7
[c:d6e86df]: https://github.com/serenity-rs/serenity/commit/d6e86df11b1741e075cf2215db7c1143a20e0e78
[c:7b25308]: https://github.com/serenity-rs/serenity/commit/7b253086fe1995222ca05b0a7338c95f1026baf1
[c:badb448]: https://github.com/serenity-rs/serenity/commit/badb44891a73bc27df8b3e39d4baf92675e868a4
[c:3a6eb58]: https://github.com/serenity-rs/serenity/commit/3a6eb585ae625d5e0ca3591b0ec432ce79ca4ee3
[c:1b17732]: https://github.com/serenity-rs/serenity/commit/1b177327b887a00e009493ed0e366238253028b4
[c:f238a46]: https://github.com/serenity-rs/serenity/commit/f238a46b0f8027a8ce67e916d7ecf7f3ef3884da
[c:47d9d1c]: https://github.com/serenity-rs/serenity/commit/47d9d1cbda6951977c503ae439cad21597d77b9b
[c:25ecaa6]: https://github.com/serenity-rs/serenity/commit/25ecaa6f17b9aa4b211a8307272b83a2229203df
[c:d0b404d]: https://github.com/serenity-rs/serenity/commit/d0b404d0481ca537c55dcb3010bec103df46a58b
[c:3b662e0]: https://github.com/serenity-rs/serenity/commit/3b662e023716461ef5b1e073dfe3ee78480b0c6e
[c:7315d78]: https://github.com/serenity-rs/serenity/commit/7315d78943f728336c64d07178e3fb9229088716
[c:cdbd14b]: https://github.com/serenity-rs/serenity/commit/cdbd14ba5e562ce8733571f0ac86adb07357af1d
[c:b2565da]: https://github.com/serenity-rs/serenity/commit/b2565da364dc7c50b10d9991968cb286d17a4f6d
[c:5e5feea]: https://github.com/serenity-rs/serenity/commit/5e5feeafbb12b8d4fb77d94f85ac70095967efc9
[c:fa0bdd8]: https://github.com/serenity-rs/serenity/commit/fa0bdd8a2978af2e757a55077f55f1b8c6df938b
[c:392a534]: https://github.com/serenity-rs/serenity/commit/392a534980dda1706dcd519550fc58306101d8e8
[c:8ab0800]: https://github.com/serenity-rs/serenity/commit/8ab08007680f9612bbab3109b8efe0912c923371
[c:01f70db]: https://github.com/serenity-rs/serenity/commit/01f70dbcad7c31edb5825c70d0e3c049f874abd1
[c:f34c7bb]: https://github.com/serenity-rs/serenity/commit/f34c7bba5b5e0236881e31a96fa1756f63c6129f
[c:6f30b0e]: https://github.com/serenity-rs/serenity/commit/6f30b0e4385663660a95010f00d4f2be4592d928
[c:bab4d78]: https://github.com/serenity-rs/serenity/commit/bab4d78e2d238df7ed830d98942234dc443c5860
[c:d508c35]: https://github.com/serenity-rs/serenity/commit/d508c3583ac57b8ce3d906004925cfbaf9931c07
[c:b995482]: https://github.com/serenity-rs/serenity/commit/b995482142a541cb4294a49fdb6f1687693532ef
[c:c7caedd]: https://github.com/serenity-rs/serenity/commit/c7caedd225acdf3c8ddc60e03c8cc031523a1fc7
[c:8e13e74]: https://github.com/serenity-rs/serenity/commit/8e13e745884035ca56c953d2cbeb7fb16f292e01
[c:cfe5f01]: https://github.com/serenity-rs/serenity/commit/cfe5f01622b7ef49db44eef165e7123be31a953e
[c:dca5c45]: https://github.com/serenity-rs/serenity/commit/dca5c452ef73c94c565d8a5995eb9f7c9a8081d0
[c:9334af5]: https://github.com/serenity-rs/serenity/commit/9334af5bdfa10db20c41258986bbf8acb6c6538e

[c:4018632]: https://github.com/serenity-rs/serenity/commit/40186329f474df0100a9c1ea1ee3422e485d1f10
[c:8d0b307]: https://github.com/serenity-rs/serenity/commit/8d0b307a52f1f2d7ea81a9eef9b283434dcc5827
[c:95c2f78]: https://github.com/serenity-rs/serenity/commit/95c2f781751f3f107ebc734a2ac4342e607abef5
[c:dbc40cb]: https://github.com/serenity-rs/serenity/commit/dbc40cbe6421060933b2d83176e7ec617adf5c8c
[c:cc184a4]: https://github.com/serenity-rs/serenity/commit/cc184a48e81c961e2c530ad6c4a261dfb093637a
[c:b999f46]: https://github.com/serenity-rs/serenity/commit/b999f465180e974a417984bb6b964a5e8655b559
[c:8ab0305]: https://github.com/serenity-rs/serenity/commit/8ab030517e2efcd2ba96fe65ba8c1898291bbf09
[c:3088652]: https://github.com/serenity-rs/serenity/commit/3088652e32aaba535c0d85f42b580f9b808de66d
[c:412f5a9]: https://github.com/serenity-rs/serenity/commit/412f5a950d122534a1329c40645e7684725ee43d
[c:8dfd97d]: https://github.com/serenity-rs/serenity/commit/8dfd97defae0ca9c5ef1cbe18d56784091312aef
[c:eba755c]: https://github.com/serenity-rs/serenity/commit/eba755c61a2effe8983f9b6fd182fdaec65eb663
[c:feda47c]: https://github.com/serenity-rs/serenity/commit/feda47ca0b4b61061d8c02623c0fcdafe1e17c8b
[c:a04291f]: https://github.com/serenity-rs/serenity/commit/a04291f7d43ac78a27483ec54773044272fbd278
[c:4186996]: https://github.com/serenity-rs/serenity/commit/4186996a66f608ffc62efd5b2eb836103dbc4045
[c:859355c]: https://github.com/serenity-rs/serenity/commit/859355c7dd62f7d46f3ff0225742a0977912d87b
[c:909bf8e]: https://github.com/serenity-rs/serenity/commit/909bf8ebc6d736f60033e98ffa0ddc685c75c96d
[c:166c248]: https://github.com/serenity-rs/serenity/commit/166c24802f67174bca4aee7c1f238cf755e2d2c3
[c:17233a2]: https://github.com/serenity-rs/serenity/commit/17233a2ca44b8d6fbcc060427e96d53b7b3a6ea2
[c:03dd250]: https://github.com/serenity-rs/serenity/commit/03dd2509dd62d13dda41632a3d9cf57575832148
[c:16e7556]: https://github.com/serenity-rs/serenity/commit/16e755686181e818658264371b0084a94ad980f3
[c:3aa6aae]: https://github.com/serenity-rs/serenity/commit/3aa6aae837f01d9d7dbc80721c026bf43ff9ec92

[c:b192609]: https://github.com/serenity-rs/serenity/commit/b1926090f0d4a08d9bd112bf6088bea021171f46

[c:1cbc935]: https://github.com/serenity-rs/serenity/commit/1cbc935ac45e956f6aaad7b44e5cd36054df6c2c
[c:8f38985]: https://github.com/serenity-rs/serenity/commit/8f38985620cbd6c9c6e6c1c8a17b31afe24b3458
[c:e5ae947]: https://github.com/serenity-rs/serenity/commit/e5ae947a1478d18fe58a774a0b55f21015417d8b
[c:136adef]: https://github.com/serenity-rs/serenity/commit/136adefc8188feeb8a57a57a878c7919990387ca
[c:53d5007]: https://github.com/serenity-rs/serenity/commit/53d5007a8d119158b5f0eea0a883b88de8861ae5
[c:dcc1ac4]: https://github.com/serenity-rs/serenity/commit/dcc1ac4d0a12f24e998af3949e33ec352153a6af
[c:8931c4c]: https://github.com/serenity-rs/serenity/commit/8931c4c0f2eb4d4c58c0e01dabfc4c91805de57a
[c:810b033]: https://github.com/serenity-rs/serenity/commit/810b033ca31f47f9c9593fee134d60328ebba198
[c:3f93c87]: https://github.com/serenity-rs/serenity/commit/3f93c87095d4a0fc9514540b715c9f3d98effb0d
[c:abfbb9f]: https://github.com/serenity-rs/serenity/commit/abfbb9f45be4dc749b1ba20f4ed1e64e486d92f3
[c:abe955f]: https://github.com/serenity-rs/serenity/commit/abe955f72d21c67fca78f2732a7841402560bb71
[c:0d743fd]: https://github.com/serenity-rs/serenity/commit/0d743fdd3b7cc0ea28500547ff0a9caff4a83c3f
[c:0e2b648]: https://github.com/serenity-rs/serenity/commit/0e2b64878c9630c37cf851c2fb868e098aa70262
[c:835e826]: https://github.com/serenity-rs/serenity/commit/835e8268236817c1eabf03fdbd0a5ff4b2e7fff2
[c:2812236]: https://github.com/serenity-rs/serenity/commit/28122364f433ff848cb092846afba853f90883ed
[c:75b404a]: https://github.com/serenity-rs/serenity/commit/75b404a39266d00649bba0b5cf07a0ebb75c444f
[c:5892db2]: https://github.com/serenity-rs/serenity/commit/5892db2b4824f39ce7f42fa7d12319e29fde7c0a
[c:3db32fb]: https://github.com/serenity-rs/serenity/commit/3db32fb2b6a0d56b91aadc79ae94ff363efdb87c
[c:4ed30c3]: https://github.com/serenity-rs/serenity/commit/4ed30c3ecde327f495cac1100672a02ad6b94854
[c:53b2dc5]: https://github.com/serenity-rs/serenity/commit/53b2dc5d8ee68ea00ab0a17e6be90ff452e8681d
[c:2a21347]: https://github.com/serenity-rs/serenity/commit/2a21347797af68c4ce1a5929aa488078cbd97378
[c:2e50a12]: https://github.com/serenity-rs/serenity/commit/2e50a12a15926859a860f8a68ff1071b944cdb51
[c:7e8620f]: https://github.com/serenity-rs/serenity/commit/7e8620ffc7028cf4084950bbc549f4e0687c9a77
[c:8133a43]: https://github.com/serenity-rs/serenity/commit/8133a43fa65aea7be7a8e9a6f30e21f10da84811
[c:73e6d8a]: https://github.com/serenity-rs/serenity/commit/73e6d8ad9cd0a6caf48cad3f1b195ac16bc5d914
[c:dfd0b32]: https://github.com/serenity-rs/serenity/commit/dfd0b321f30cb6da9549d12bf8e18d37aecca384
[c:9bbb25a]: https://github.com/serenity-rs/serenity/commit/9bbb25aac4d651804286f333eb503a72d41e473b
[c:cb6b528]: https://github.com/serenity-rs/serenity/commit/cb6b528da723f36592b648d77b0f4e8d974ea82d
[c:de72951]: https://github.com/serenity-rs/serenity/commit/de72951996c0c8bf1edc64b933d7bd4e5444460f
[c:6b6e5fe]: https://github.com/serenity-rs/serenity/commit/6b6e5fe0fb405f8dfc4ab7671741d457a822dac2
[c:1ab52a0]: https://github.com/serenity-rs/serenity/commit/1ab52a056f94760a7436927c98c08ea27da3a0ad
[c:83360f8]: https://github.com/serenity-rs/serenity/commit/83360f87bd68d862b710a25916e477b02961ae48
[c:291e4f2]: https://github.com/serenity-rs/serenity/commit/291e4f2f5636a118b1762fac9829bd8936b7753c
[c:2be89e2]: https://github.com/serenity-rs/serenity/commit/2be89e28c4cc516e629dfd9a260f3b960e693938
[c:4477690]: https://github.com/serenity-rs/serenity/commit/44776902ec20f9f185727da499421f79061481e2
[c:10de463]: https://github.com/serenity-rs/serenity/commit/10de463b515f3b703c6d6d86631c7d3860b5751e
[c:5457cda]: https://github.com/serenity-rs/serenity/commit/5457cda04c10eb274eb9d6aa11d164cf93ae6faf
[c:2f835a3]: https://github.com/serenity-rs/serenity/commit/2f835a3316dec3fd97788995194065d476c452c4
[c:2854e6b]: https://github.com/serenity-rs/serenity/commit/2854e6b8ca778455770fca19d0c39757846bb833
[c:5254e2b]: https://github.com/serenity-rs/serenity/commit/5254e2bb2c14fd7833b6d031c8b8bf1cd5d647d5
[c:e59314e]: https://github.com/serenity-rs/serenity/commit/e59314e8576e394deb7ed36ed470c1b9e63a4e9f

[c:580d6de]: https://github.com/serenity-rs/serenity/commit/580d6de400165b8fedd19e359d91d72e782e45d9
[c:9a97f9b]: https://github.com/serenity-rs/serenity/commit/9a97f9b7030e17701fb2887bf86cebde4ec14e8a

[c:ab8c82b]: https://github.com/serenity-rs/serenity/commit/ab8c82bddc2854f17e04efd10ccfd5357a5b415f

[c:2624170]: https://github.com/serenity-rs/serenity/commit/2624170835961e57fe00f027763166d74613b8da
[c:7647e1e]: https://github.com/serenity-rs/serenity/commit/7647e1e7bb4754a5880246312fb807daede31b23
[c:0b1fc27]: https://github.com/serenity-rs/serenity/commit/0b1fc2737efebde01f6f3b781bcdfc83a30e908e
[c:a27d7bb]: https://github.com/serenity-rs/serenity/commit/a27d7bb43977da0f4d10f3405b12e07d56cf9a74
[c:3d56a57]: https://github.com/serenity-rs/serenity/commit/3d56a57d105b1c1177f1874295709ddb18298d44
[c:4f07798]: https://github.com/serenity-rs/serenity/commit/4f077989ef8a4bc54429df4f77197a09f6e678fa
[c:fcb1daa]: https://github.com/serenity-rs/serenity/commit/fcb1daa7bc47f28f250127cd125dca784d1c6f35
[c:b42591c]: https://github.com/serenity-rs/serenity/commit/b42591c0ef97da4da822be2070b8899c89d4a708
[c:3282d77]: https://github.com/serenity-rs/serenity/commit/3282d77966426f3f73bb7892d943b89e9cde475b
[c:aca54ac]: https://github.com/serenity-rs/serenity/commit/aca54acfc9acde6230c2b3cfc08780d69c44a98d
[c:2e8f968]: https://github.com/serenity-rs/serenity/commit/2e8f968fe80ce3747772d194120dd9b41d3694d3
[c:7684323]: https://github.com/serenity-rs/serenity/commit/7684323d1264583d8f3d0424bd8fca1e0abf1941
[c:03a1f97]: https://github.com/serenity-rs/serenity/commit/03a1f97a40c00bf5d8551a6036c58263fd86dc07
[c:66e7add]: https://github.com/serenity-rs/serenity/commit/66e7addd8cddf98d1f279cd4cc51c8d3f62e1ccc
[c:51dc943]: https://github.com/serenity-rs/serenity/commit/51dc943b1fa343b9fd28d6c68de765774553a174
[c:d3ab6a7]: https://github.com/serenity-rs/serenity/commit/d3ab6a7ffe9d72e08c262ed32788be4f7cddb5d0
[c:649f8f2]: https://github.com/serenity-rs/serenity/commit/649f8f2f660efb6c60ee95d725ede0941ca1b6c2
[c:aed3886]: https://github.com/serenity-rs/serenity/commit/aed388613c47d34a4299bd61ceee64b3042688a3
[c:d4caf12]: https://github.com/serenity-rs/serenity/commit/d4caf12ba2c5f8a13b8ac1de2fc87aa0ca981a9d
[c:f1204a8]: https://github.com/serenity-rs/serenity/commit/f1204a8b7bae333fae4cbff2553882e6f2ab9f0d
[c:2ef6da8]: https://github.com/serenity-rs/serenity/commit/2ef6da897b05845f10abdd58fdd52af3154786e8
[c:6240625]: https://github.com/serenity-rs/serenity/commit/6240625c16547eb0bd6d7187b0f09240b86a534f
[c:1589475]: https://github.com/serenity-rs/serenity/commit/1589475665f71ac21295998183e08038d3974bbc
[c:1b4d408]: https://github.com/serenity-rs/serenity/commit/1b4d408f23f0b93829865993d16851c410bf7ab5
[c:fb1f0bf]: https://github.com/serenity-rs/serenity/commit/fb1f0bfdcefada4a08863d5e389df44737a86b12
[c:46c74e9]: https://github.com/serenity-rs/serenity/commit/46c74e9a41e0f3f4e81c2379e54f696cdc2eb200
[c:ce66f8e]: https://github.com/serenity-rs/serenity/commit/ce66f8e5a478611c09e1fa274b2add38537625c4
[c:977fa19]: https://github.com/serenity-rs/serenity/commit/977fa1959281b3e2a61d7c5ba63f4a7b491de13f
[c:2c9c64f]: https://github.com/serenity-rs/serenity/commit/2c9c64f25e7de7b38f37265c2336eaf324503d89
[c:0de77e7]: https://github.com/serenity-rs/serenity/commit/0de77e7a056641e28b729d5d95b03217ce4a5ea8
[c:707752c]: https://github.com/serenity-rs/serenity/commit/707752c303d93a21c0784c2c7984a556fffbabad
[c:35c8f7b]: https://github.com/serenity-rs/serenity/commit/35c8f7b9c1dbf7731134263c53aa5baf789b36e5
[c:e521849]: https://github.com/serenity-rs/serenity/commit/e5218498c1d2c026084d7f7efd6788571bc6170e
[c:4fdbe98]: https://github.com/serenity-rs/serenity/commit/4fdbe989ea2c8c1902cfb35a2ffb731bb3f48f21
[c:7034564]: https://github.com/serenity-rs/serenity/commit/70345647d799703b0c6ab96c49aa505867ab55e5
[c:99c27fe]: https://github.com/serenity-rs/serenity/commit/99c27fed410bc7792c6c6d6979a10fbefa6a1175
[c:f3e4a6b]: https://github.com/serenity-rs/serenity/commit/f3e4a6b4ef04441357e86baf4251fc3d0d9094f7
[c:dbb3669]: https://github.com/serenity-rs/serenity/commit/dbb3669594144a2343a304bba34c4ffe2c942d7b
[c:6aa2841]: https://github.com/serenity-rs/serenity/commit/6aa284124599897bc992dd74e52345da81ca220f
[c:12d1b22]: https://github.com/serenity-rs/serenity/commit/12d1b2209efac3b2848e1099e046e9c051003c79
[c:309bd5d]: https://github.com/serenity-rs/serenity/commit/309bd5ded8a92613ffeb425b273c8a8082fd0234
[c:0ca5813]: https://github.com/serenity-rs/serenity/commit/0ca5813350fa7b171363fc4927b06db15971764b
[c:216d579]: https://github.com/serenity-rs/serenity/commit/216d579084967db08972629a44b0851ffabb0ec1
[c:e74fd6f]: https://github.com/serenity-rs/serenity/commit/e74fd6fa03be314a43da680d13ddddcb5a1bab0c
[c:69cae50]: https://github.com/serenity-rs/serenity/commit/69cae50c73a7edd8224e5954aba738dec5e05574
[c:6e39727]: https://github.com/serenity-rs/serenity/commit/6e39727dd52f7de5fb14cd2abc0b74d7423b86cf
[c:b7b0318]: https://github.com/serenity-rs/serenity/commit/b7b03182543310bdbc9d449fd7da6d76999df6c3
[c:59e09d0]: https://github.com/serenity-rs/serenity/commit/59e09d0f24931bf6b04b480149f59f8c27381557
[c:08bd87f]: https://github.com/serenity-rs/serenity/commit/08bd87f10882d26a22b4364322ff5fac08d33d61
[c:be6eefe]: https://github.com/serenity-rs/serenity/commit/be6eefec915e375ac3172a6bca57cd1012e7520e
[c:b3a71db]: https://github.com/serenity-rs/serenity/commit/b3a71db7e70dc8411c498d88e5fd74361b761f49
[c:6dd66e1]: https://github.com/serenity-rs/serenity/commit/6dd66e168be3725ebfc53a1553da1ac487791395
[c:1606e09]: https://github.com/serenity-rs/serenity/commit/1606e0940fa1390c2bfd4ecde350dea2ea29f39a
[c:2587e6a]: https://github.com/serenity-rs/serenity/commit/2587e6a0bfdba2ada6d4bcc7ef87eaf85332b12f
[c:49be2ab]: https://github.com/serenity-rs/serenity/commit/49be2abe91036a48b93bc1a8ac7e3add78723417
[c:879ae4b]: https://github.com/serenity-rs/serenity/commit/879ae4beeacc1b26c9dbe2e0b83c346ea46aa18e
[c:477925f]: https://github.com/serenity-rs/serenity/commit/477925fc04e3363e25f11fec39d152061d137fbd
[c:2b1e2fc]: https://github.com/serenity-rs/serenity/commit/2b1e2fc3d2525f64380fda8f888a6d5b1f0ef50c
[c:49ebbd7]: https://github.com/serenity-rs/serenity/commit/49ebbd77e2c004283a1608ad20af93d7cc6e3eee
[c:d74a0a6]: https://github.com/serenity-rs/serenity/commit/d74a0a61b818eb819c88e216e78dfb6ee1cb495f
[c:fd6f08b]: https://github.com/serenity-rs/serenity/commit/fd6f08b7ca8239cd4c17865b023cb06867c1ff98
[c:a836180]: https://github.com/serenity-rs/serenity/commit/a8361804e324d3739734736debd676bc87c5aab9
[c:8471f5b]: https://github.com/serenity-rs/serenity/commit/8471f5b1caffbff2c17e2fac79772ebb852ca4a8
[c:692e984]: https://github.com/serenity-rs/serenity/commit/692e984aeed7477777c8d18c29e5677117fac8e5
[c:20043d4]: https://github.com/serenity-rs/serenity/commit/20043d485b1ae56d68c772071891ed8af6a59fcb
[c:02916dc]: https://github.com/serenity-rs/serenity/commit/02916dcc7f454b5e72da9e9a9d677577b8028153
[c:e49fae4]: https://github.com/serenity-rs/serenity/commit/e49fae4bdea674173e9d2973a93542bd977510f4
[c:a7ab03e]: https://github.com/serenity-rs/serenity/commit/a7ab03e31cf7ec5767595ba69588226555072807
[c:7ad3abe]: https://github.com/serenity-rs/serenity/commit/7ad3abe1f4168efdd0fffde2fe352fe8d661d86f
[c:a42bba2]: https://github.com/serenity-rs/serenity/commit/a42bba2f76b85dc6fc51a10bb5824b6a1f5e4c95
[c:ba85299]: https://github.com/serenity-rs/serenity/commit/ba852999c1eea1d73832a6bae6d74fba649fd636
[c:ac23e19]: https://github.com/serenity-rs/serenity/commit/ac23e19f5ab75c281ce64067ab37d3951201c03d
[c:5fea8e4]: https://github.com/serenity-rs/serenity/commit/5fea8e41c29155b24adedf4275585eff91b4c1ab
[c:177adbf]: https://github.com/serenity-rs/serenity/commit/177adbf9e9b78e8f81f4ab1b265d7c396a2f14ac
[c:e8ae00f]: https://github.com/serenity-rs/serenity/commit/e8ae00fa94c426332011fa111f3433ae5a6ff091
[c:2225c8d]: https://github.com/serenity-rs/serenity/commit/2225c8dd1814a6c21d2358a0e7f566ff7a1af198
[c:79c506e]: https://github.com/serenity-rs/serenity/commit/79c506e810f739f52a577c07be4329300946dd1f
[c:8fb18e6]: https://github.com/serenity-rs/serenity/commit/8fb18e615993d1480ac080bf5d025c8ddc5a9115
[c:b1bcc67]: https://github.com/serenity-rs/serenity/commit/b1bcc675df237e466a12f4c80ef8f08c0973e0c2
[c:9cc30c3]: https://github.com/serenity-rs/serenity/commit/9cc30c3d0d5d8c00f09c4a16a310d8717e2e0407
[c:52f9dca]: https://github.com/serenity-rs/serenity/commit/52f9dca368853f07ef580e3528a82452d4f9e7fa
[c:d6984a8]: https://github.com/serenity-rs/serenity/commit/d6984a831c0e0ef13948e96f8c7dc6e9a0bd74d5
[c:4729f26]: https://github.com/serenity-rs/serenity/commit/4729f2603396be768daee56ff12414331893651c
[c:ac66a54]: https://github.com/serenity-rs/serenity/commit/ac66a54b87a743eb9f85a7f1dd064769792b3c29

[c:b03df91]: https://github.com/serenity-rs/serenity/commit/b03df91f50a554cd9ab3dcb78d74eba46f2601e8

[c:1cc66b3]: https://github.com/serenity-rs/serenity/commit/1cc66b3d4ba70f75823cb59c5db4e4b0ef162ac7
[c:1047eac]: https://github.com/serenity-rs/serenity/commit/1047eac908eb28c69691292db8e35e089985a09e
[c:fac94f2]: https://github.com/serenity-rs/serenity/commit/fac94f2559abe1cfdafe0ca998e011ba08a1e84c
[c:5399d44]: https://github.com/serenity-rs/serenity/commit/5399d443f4e9962b5a40fd9635eb6e3441600769
[c:75feff2]: https://github.com/serenity-rs/serenity/commit/75feff2869fcb4a59b19a40794182e497925d37f
[c:8f5a2e9]: https://github.com/serenity-rs/serenity/commit/8f5a2e9e6073ba85519f531e8f2c6324bc07ca3b
[c:1dba16a]: https://github.com/serenity-rs/serenity/commit/1dba16a7281a6c367e636f71abd89772ab6d74ab
[c:8b42790]: https://github.com/serenity-rs/serenity/commit/8b42790306e9165acd3e092c71a3d280353185a4

[c:bc78991]: https://github.com/serenity-rs/serenity/commit/bc78991f7cdbdeaeba308785a42953423f5e89aa
[c:85e5722]: https://github.com/serenity-rs/serenity/commit/85e5722c997d5cc024cb3f687a8fda3a368db2cd
[c:757e0d6]: https://github.com/serenity-rs/serenity/commit/757e0d60067553ea49fa0dc180c6e64843a7ea13
[c:e6fd09b]: https://github.com/serenity-rs/serenity/commit/e6fd09b15550654a6f2dd16742b9703539d0ff03

[c:7ea8098]: https://github.com/serenity-rs/serenity/commit/7ea80988604ce1a9c772dabd8b150b5f89ca355c
[c:182ee78]: https://github.com/serenity-rs/serenity/commit/182ee78f685595d43b951dab2d0f8a59c9a3f84d
[c:58e3d49]: https://github.com/serenity-rs/serenity/commit/58e3d495185eee497555faaaa311e6c69b9b35cc
[c:a836180]: https://github.com/serenity-rs/serenity/commit/a8361804e324d3739734736debd676bc87c5aab9
[c:61597d6]: https://github.com/serenity-rs/serenity/commit/61597d66bbf44511bf4b92337ea7236c63cfba1f

[c:c9971b6]: https://github.com/serenity-rs/serenity/commit/c9971b6e60f3b5f5ac9c3e8931f6a7ecbf8194db
[c:0d9b821]: https://github.com/serenity-rs/serenity/commit/0d9b821953268edeeea8b1c01fe0d33447ba08cb
[c:50d9643]: https://github.com/serenity-rs/serenity/commit/50d96436612b18c5bb807287b10a68f62747b7e4
[c:d9e9bf7]: https://github.com/serenity-rs/serenity/commit/d9e9bf77aff4770e22aacc5153947e1ecf1a9862
[c:85b5489]: https://github.com/serenity-rs/serenity/commit/85b5489bc8f05a98ad8aeacc20e220ad08206ff8

[c:7e55a0e]: https://github.com/serenity-rs/serenity/commit/7e55a0e6eec2106b0ca540822772706232aa2a1f

[c:5bb8342]: https://github.com/serenity-rs/serenity/commit/5bb8342b018bc60def13f36218fd9ee31ce18c0d
[c:a1b3c8d]: https://github.com/serenity-rs/serenity/commit/a1b3c8ddbfde9fdcd6484d99a6f4ad6bc25a742c
[c:8c30b40]: https://github.com/serenity-rs/serenity/commit/8c30b402d7c6259380c8f989d37c1d5df7a3ed9e
[c:c0c2137]: https://github.com/serenity-rs/serenity/commit/c0c2137eb346ac8ef599a619413646fe590501d8
[c:2b66828]: https://github.com/serenity-rs/serenity/commit/2b66828454a4ea4e751fdb3ad09bab62c7c3933b
[c:2386690]: https://github.com/serenity-rs/serenity/commit/2386690af923be1ff2c6ea030d7adf7644e7f30a
[c:b38cdf5]: https://github.com/serenity-rs/serenity/commit/b38cdf550aee8c87f57ddf8265014655d9aaaa80
[c:3fe8056]: https://github.com/serenity-rs/serenity/commit/3fe80560243b376284ccf8c1bb7fdbe9c31463c2
[c:7ef12ee]: https://github.com/serenity-rs/serenity/commit/7ef12ee050c80aeb18a840134921693db9c22e00
[c:bcf8249]: https://github.com/serenity-rs/serenity/commit/bcf824953b13de97ba8c9d366f103712f6cec2d3
[c:860a2c5]: https://github.com/serenity-rs/serenity/commit/860a2c503cb87515057a5de0c2381186235888d7
[c:bba7d55]: https://github.com/serenity-rs/serenity/commit/bba7d55be4079abbd43eb879d0a8dacc1439e8a6
[c:35e2a40]: https://github.com/serenity-rs/serenity/commit/35e2a40b761e6c662d2034eb5cc5ad46b4aac9e7
[c:6577838]: https://github.com/serenity-rs/serenity/commit/65778380953a87f910e1bd65751bfb3949a6d531
[c:b1187ce]: https://github.com/serenity-rs/serenity/commit/b1187cefa2a9e3118174491e1f5f58a189258616
[c:c6150ae]: https://github.com/serenity-rs/serenity/commit/c6150aecbf5c5a129a6946dfe03df2d30fc2e6c3
[c:5cc67c3]: https://github.com/serenity-rs/serenity/commit/5cc67c37ff2a931d04f1a565a957e16ecb833578
[c:47e2f34]: https://github.com/serenity-rs/serenity/commit/47e2f34a6dd93742c94e8101f314580452589228
[c:519b67e]: https://github.com/serenity-rs/serenity/commit/519b67ed881ef90340fbe308b1ec3f849d3caa82
[c:b522e83]: https://github.com/serenity-rs/serenity/commit/b522e83777ae185d3f09300ebb01c3b55ca65d10
[c:aa589da]: https://github.com/serenity-rs/serenity/commit/aa589da8d6d69780745f7fe7028b24ca4c5580a0
[c:d514806]: https://github.com/serenity-rs/serenity/commit/d514806e6a92ed578d3de96603c4e3777cd4ba15
[c:ebd1241]: https://github.com/serenity-rs/serenity/commit/ebd1241f4fee31b0a63e1fb6c376a75369aa89d8
[c:de085ff]: https://github.com/serenity-rs/serenity/commit/de085ff4d0d759865a345d7f4e92b89143f12243
[c:87df95f]: https://github.com/serenity-rs/serenity/commit/87df95f0a4354b87260483c5421f64df0e85aa4f
[c:f0cfc0e]: https://github.com/serenity-rs/serenity/commit/f0cfc0eec8156dd70dd8d3db021e74bb57126acb
[c:186fd6c]: https://github.com/serenity-rs/serenity/commit/186fd6ce1b3e835b75bb950bf60797aeed3c6900
[c:3467ddc]: https://github.com/serenity-rs/serenity/commit/3467ddc6819ed5481489fcf46e0a2b81af7d70d9
[c:b044b6d]: https://github.com/serenity-rs/serenity/commit/b044b6d5d4ae0a9472fb783797516f9becaee848
[c:42da202]: https://github.com/serenity-rs/serenity/commit/42da202a5b43d1648f524a07e3eb5e529c94ee36
[c:b9252a5]: https://github.com/serenity-rs/serenity/commit/b9252a5f833a66c5c3ab061f6e4a8088ef08c396
[c:83e1d12]: https://github.com/serenity-rs/serenity/commit/83e1d12fa4f530071b0cbe3d7f38e125c5a6c054
[c:ffc2997]: https://github.com/serenity-rs/serenity/commit/ffc2997f8c76285b03dc31e62d653b40a553acf0
[c:2d365d0]: https://github.com/serenity-rs/serenity/commit/2d365d00e00acc307c772abfdc433a000f6140d5
[c:f0a3947]: https://github.com/serenity-rs/serenity/commit/f0a3947cebd71b8ac5f6ba03e18ed4504bb7150c
[c:626bdfd]: https://github.com/serenity-rs/serenity/commit/626bdfd7ff6dd41965531f4bfe1befd8d8fb7d4b

[c:ebc2cc7]: https://github.com/serenity-rs/serenity/commit/ebc2cc7c027c63f3a5e5e16484efe751bd16957d
[c:fb44a15]: https://github.com/serenity-rs/serenity/commit/fb44a15aea10bddca0f730fc6837a9f838a3ba94
[c:b137f51]: https://github.com/serenity-rs/serenity/commit/b137f5123c772edd0885b8630d78df24f5cb40cb
[c:f2191d5]: https://github.com/serenity-rs/serenity/commit/f2191d5b6f093d5d5fd99ed81f86e185ce8562e6
[c:7146fd7]: https://github.com/serenity-rs/serenity/commit/7146fd7842f9e17561043ea2f05c0824afae3b2b
[c:a3f9186]: https://github.com/serenity-rs/serenity/commit/a3f9186993ceeb3f9db6b4ed380857f090a35517
[c:bab4b24]: https://github.com/serenity-rs/serenity/commit/bab4b247bbb0e38b2aad6b83f101a145beb1aa7f
[c:3f5882c]: https://github.com/serenity-rs/serenity/commit/3f5882c3630d26d7dfa75b46de4551f675284b91
[c:9ee42f1]: https://github.com/serenity-rs/serenity/commit/9ee42f1cd9c4c3eea1c14e1848413f28f898d5ac
[c:78dd862]: https://github.com/serenity-rs/serenity/commit/78dd8624fad69392129098f8b18ae210b3ea04e1
[c:66e05ea]: https://github.com/serenity-rs/serenity/commit/66e05ea56d2f08a29dbbf6ffd59994512efebd4c
[c:7c2d4f7]: https://github.com/serenity-rs/serenity/commit/7c2d4f7b6339c526834214d1a6086391daea2aa5
[c:49c1603]: https://github.com/serenity-rs/serenity/commit/49c1603a9d6b9856fac05a59ac58833c4b44eeb0
[c:9e77154]: https://github.com/serenity-rs/serenity/commit/9e7715463b26c1e943a71e9a8579f28087f9cdf9
[c:88af7cf]: https://github.com/serenity-rs/serenity/commit/88af7cfa5a1dab9c8e5763bc5fb81176f25528ba
[c:2eac499]: https://github.com/serenity-rs/serenity/commit/2eac49923894c7c494714836818ba828d48214ad
[c:323875f]: https://github.com/serenity-rs/serenity/commit/323875f5a9ca9b22202e6e291bfca0c78794cfdb
[c:5c0c045]: https://github.com/serenity-rs/serenity/commit/5c0c04548622360a5f9baf14182a2bba4faa644e
[c:5805539]: https://github.com/serenity-rs/serenity/commit/5805539aad346dd5e644b0b06d987a76dd454262
[c:659448b]: https://github.com/serenity-rs/serenity/commit/659448ba5598ed3da00be8cd70be666070a8ab5c
[c:fa03b75]: https://github.com/serenity-rs/serenity/commit/fa03b75b7ac435c8138375848fbbe9a55439d86a
[c:41698b6]: https://github.com/serenity-rs/serenity/commit/41698b6e999557c8b784ce2f51dff3c806246258

[c:c8dacf2]: https://github.com/serenity-rs/serenity/commit/c8dacf2d2ff48ce25eebe3608a6617929df33e37
[c:a7e2f74]: https://github.com/serenity-rs/serenity/commit/a7e2f746930a0b4d3cfcf1937951d41ca7786480
[c:1eb2714]: https://github.com/serenity-rs/serenity/commit/1eb2714fa27ea4df133c2ef96db2ec4bb5c63ad0
[c:2a1c385]: https://github.com/serenity-rs/serenity/commit/2a1c3852fb90f09951d4b13191c87f4f15d22ba2
[c:a9c9caa]: https://github.com/serenity-rs/serenity/commit/a9c9caabe9c7cd760274628261d0ff13e8a088d2
[c:59fb7b9]: https://github.com/serenity-rs/serenity/commit/59fb7b9cc7d62270c45528aca9d20ac36ca95b4d
[c:7d8c806]: https://github.com/serenity-rs/serenity/commit/7d8c8063c43d81c567de3e1011217d63148e6cb3
[c:f00f1c0]: https://github.com/serenity-rs/serenity/commit/f00f1c0ec0736999ffe2619840ae89690daa41f9
[c:13bf356]: https://github.com/serenity-rs/serenity/commit/13bf3567f94e74cc9f00c46128a5c83cc2000a16
[c:ed7b18e]: https://github.com/serenity-rs/serenity/commit/ed7b18e637c19bb5d33b84a6b7620a7997bc4b25
[c:cfbd0be]: https://github.com/serenity-rs/serenity/commit/cfbd0be692dfe3cd687fb75322cf0fd4c4373237
[c:3d44d66]: https://github.com/serenity-rs/serenity/commit/3d44d66b9d98ad04fc059675ede3d9f1c46528dd
[c:b414f29]: https://github.com/serenity-rs/serenity/commit/b414f297f4bc0690f8fb96ade4a3a4e741bd4c43

[c:28f9f8a]: https://github.com/serenity-rs/serenity/commit/28f9f8addea5aae5d0c59f6682e47a49c534d5ed
[c:5a6979a]: https://github.com/serenity-rs/serenity/commit/5a6979a27745f2c8ae7d981ae1249dc3dc7add4e
[c:e049bc1]: https://github.com/serenity-rs/serenity/commit/e049bc1b9bdb2b0a7bd37f1d5ebac29e70415e9a
[c:614f99a]: https://github.com/serenity-rs/serenity/commit/614f99a72b5c206d4f5c8379ad41ae084d4350d5
[c:c60f397]: https://github.com/serenity-rs/serenity/commit/c60f39740afc8df2df29cfad25890ad5d05641f7
[c:5931d1c]: https://github.com/serenity-rs/serenity/commit/5931d1c6a940b61908f591c49337d065223ab374
[c:a92ca08]: https://github.com/serenity-rs/serenity/commit/a92ca08d45609b137a6bebba3e527dd0b7027016
[c:1594a3e]: https://github.com/serenity-rs/serenity/commit/1594a3e9aed2f4f51079d87d853e6c794c573c19
[c:9eadffa]: https://github.com/serenity-rs/serenity/commit/9eadffae6ec6009d1c65196d3bf8289dc6443c20
[c:4e8e40b]: https://github.com/serenity-rs/serenity/commit/4e8e40b4d7694a0d0cbe0d0f84b74248aa402191
[c:d99bee9]: https://github.com/serenity-rs/serenity/commit/d99bee900e2d5a61a218f7107a3ce3d8642e8ee0
[c:3dda20d]: https://github.com/serenity-rs/serenity/commit/3dda20d0ec5dc2c68be572e52b71e7c07e918d24
[c:5df4c6d]: https://github.com/serenity-rs/serenity/commit/5df4c6d1d82ad1de14442b468de8f989c2459c3c
[c:802df92]: https://github.com/serenity-rs/serenity/commit/802df92dcd33610789af678d54136591dd1b1e04
[c:c1c7300]: https://github.com/serenity-rs/serenity/commit/c1c730038d49b1e3e46356ed8a26557bf9d6eb7c
[c:287245a]: https://github.com/serenity-rs/serenity/commit/287245a6d0c693ed059ee463034db28180e105fa
[c:00683ef]: https://github.com/serenity-rs/serenity/commit/00683ef1a5f311ac3c007d0941eff0f84f5b5dab
[c:0d72b6e]: https://github.com/serenity-rs/serenity/commit/0d72b6e2f517624d59c2eae69032bdc1d8ad2d53
[c:4472dec]: https://github.com/serenity-rs/serenity/commit/4472dec8bf3544776552029ed555ab3f04f23f9d
[c:e7eb52d]: https://github.com/serenity-rs/serenity/commit/e7eb52dc96a8ee23505160a7de80e5baf09ffe66
[c:94d22c3]: https://github.com/serenity-rs/serenity/commit/94d22c3ac338a5b68c942b7a3e71d90d291d7f27
[c:a406d48]: https://github.com/serenity-rs/serenity/commit/a406d48533330e1f875225c1d1e47ce9265f2c55
[c:9f75c09]: https://github.com/serenity-rs/serenity/commit/9f75c09f6616cd61fd3f26500d1e114a5a0fe448
[c:8628b2a]: https://github.com/serenity-rs/serenity/commit/8628b2ae80e0b74030afbcd80fc5b1f87599e977
[c:5edd11d]: https://github.com/serenity-rs/serenity/commit/5edd11daea250d887fe6cf323c7e6776509fdb70
[c:3e4294b]: https://github.com/serenity-rs/serenity/commit/3e4294b5e20296526e55bca82aac92eccda13d74
[c:dd580fb]: https://github.com/serenity-rs/serenity/commit/dd580fb9881e1025b1d11c0e3bd817e8b63806f3
[c:8040fa5]: https://github.com/serenity-rs/serenity/commit/8040fa5912d488b56addc6f2ec60d12a8540f8c1
[c:760ed71]: https://github.com/serenity-rs/serenity/commit/760ed7173cb87622cecb52c53a0f95d41560b02a
[c:21a5d8e]: https://github.com/serenity-rs/serenity/commit/21a5d8e98a09a7b4db8d906618b90687baad44e8
[c:ed61459]: https://github.com/serenity-rs/serenity/commit/ed6145979c0d428937925b27bd832ef9787e8581
[c:4227a52]: https://github.com/serenity-rs/serenity/commit/4227a527e749133758bb84d4c18c73d6351946da
[c:9cfc410]: https://github.com/serenity-rs/serenity/commit/9cfc410b232f75f0b0663b06fc6e3497892949ce
[c:2418edd]: https://github.com/serenity-rs/serenity/commit/2418edd02ca7bcf4f0763216cc7e986a77b6aede
[c:3135e0f]: https://github.com/serenity-rs/serenity/commit/3135e0f3ccaf80d4ac8eabbed4040ed10e8a7863

[c:eaead53]: https://github.com/serenity/serenity/commit/eaead536db0327fde24c87aebe28470633db4a4c
[c:b96529e]: https://github.com/serenity/serenity/commit/b96529e92b8d79cb580e15a90506e6637e6809aa
[c:bb04fad]: https://github.com/serenity/serenity/commit/bb04fad0a8716472a5ba5f7429c18c123f9e650f

[c:72f287c]: https://github.com/serenity-rs/serenity/commit/72f287c5b4ebbd9e9fbaae9afec99187387db0dd

[c:6b1021f]: https://github.com/serenity-rs/serenity/commit/6b1021f85ea8590b2aa50e4ea986b598575c6abb
[c:9f848af]: https://github.com/serenity-rs/serenity/commit/9f848aff0fda841d7ca827447ac115bcb0d18d1a
[c:912d759]: https://github.com/serenity-rs/serenity/commit/912d759c31bf6ba69697fe5dac451b1f3b299a91
[c:1b195d7]: https://github.com/serenity-rs/serenity/commit/1b195d73870e7561b869aaf9ff795a85ccdc6b6f
[c:08d0608]: https://github.com/serenity-rs/serenity/commit/08d06082f4df7c6897aa5fe3e481819bca47aa7f
[c:3965e00]: https://github.com/serenity-rs/serenity/commit/3965e003040a385124c5809bd2df85533db4b801
[c:46db84a]: https://github.com/serenity-rs/serenity/commit/46db84a64364b20f1276db9b7909f3fbd318a87d
[c:f98e61e]: https://github.com/serenity-rs/serenity/commit/f98e61eed83f7c51c26c41ea84f0712888cc7dcd
[c:9c198ed]: https://github.com/serenity-rs/serenity/commit/9c198ed7b36d9a808a3e477d2450d5a8b5ae8409
[c:550a63c]: https://github.com/serenity-rs/serenity/commit/550a63c0209ca5b6d01312fa4cf7f3c171932938
[c:2a54410]: https://github.com/serenity-rs/serenity/commit/2a54410f79d0918925fa0ca02607e5b073f0e247
[c:7dd01eb]: https://github.com/serenity-rs/serenity/commit/7dd01eb95754dcdc71cea8e744e2fedacd3ab9dd
[c:638780e]: https://github.com/serenity-rs/serenity/commit/638780e35c8b21688c2c9dec87e92906251ef1fa
[c:84b04ba]: https://github.com/serenity-rs/serenity/commit/84b04ba8ca0e3ff4cef3a52775beaadb0af5bd94
[c:e000833]: https://github.com/serenity-rs/serenity/commit/e0008330ff5a706a1fc0ac827e52623517e2387a
[c:9356a5c]: https://github.com/serenity-rs/serenity/commit/9356a5c9b44aef0aee1185634a8f60f25ae09a16

[c:b8e2a7d]: https://github.com/serenity-rs/serenity/commit/b8e2a7d14bd3abca3fb8ae0dcf5e1889180ff037

[c:13d5481]: https://github.com/serenity-rs/serenity/commit/13d54818d4bc7cc9cd999d0e56ef713c159ce848

[c:003c8e1]: https://github.com/serenity-rs/serenity/commit/003c8e1d2519610601a6f800be6bc8faddac8b1b

[c:0959f15]: https://github.com/serenity-rs/serenity/commit/0959f15e3605a2fea1e945bd4d8ef0e8052e6d65

[c:f543b57]: https://github.com/serenity-rs/serenity/commit/f543b579ca10d4fc439f1a899361a2edff5f04f3

[c:f708bec]: https://github.com/serenity-rs/serenity/commit/f708bec4932a7bec2b53e4b4450d9567b44a4138

[c:7f04ecf]: https://github.com/serenity-rs/serenity/commit/7f04ecf967b3de4a8e83a2441fa9cb99e7f5913d
[c:9ebd779]: https://github.com/serenity-rs/serenity/commit/9ebd7791b5b266d2877ece8cad19f333bcacc606

[c:f5dd8bf]: https://github.com/serenity-rs/serenity/commit/f5dd8bf42a7a952c1093925ecd60b46b8f716f60

[c:e005ef1]: https://github.com/serenity-rs/serenity/commit/e005ef19b695fb444f921e84741b97fb2a9d0687
[c:f27c7c1]: https://github.com/serenity-rs/serenity/commit/f27c7c148b14239b7170b3acd6a17137c6986737
[c:d6b0038]: https://github.com/serenity-rs/serenity/commit/d6b003832a37a3ca8d473e5253a1dd656fcc49c8
[c:683ff27]: https://github.com/serenity-rs/serenity/commit/683ff27169cc62a2f7fcfcc84acd9169dd4b3e9c
[c:1361b33]: https://github.com/serenity-rs/serenity/commit/1361b3377ebf00fb31375f5010863f59b66c3f3a
[c:820d50e]: https://github.com/serenity-rs/serenity/commit/820d50ee4fbc72a41a2040f6ced240df7aaa6fa8
[c:581eb2f]: https://github.com/serenity-rs/serenity/commit/581eb2fbbd2a5d401b30f68168fcb2f9d7776452
[c:48c4b59]: https://github.com/serenity-rs/serenity/commit/48c4b5920d41278c741c8ccdb988f24e12dd89af
[c:f2d0ad5]: https://github.com/serenity-rs/serenity/commit/f2d0ad5182c6321aee97fa43896450cd7cefaa85
[c:3ca41fd]: https://github.com/serenity-rs/serenity/commit/3ca41fd1afed11469f623850536263ad97c34e77
[c:a23acc7]: https://github.com/serenity-rs/serenity/commit/a23acc7ad6a78205a04c054b50f96425beb55747
[c:a692bcd]: https://github.com/serenity-rs/serenity/commit/a692bcdc66e0f294486c4c006d55274903e83e6c
[c:a44f16d]: https://github.com/serenity-rs/serenity/commit/a44f16de8f11d108eaa15d3b136ae4e0eef89d05
[c:ec306ee]: https://github.com/serenity-rs/serenity/commit/ec306ee98cccbee0f68ba9dc929dee26f3b13135
[c:5f9a27a]: https://github.com/serenity-rs/serenity/commit/5f9a27a49abb95b9fe100241ed5725b2cf5c6f59
[c:61bcfbc]: https://github.com/serenity-rs/serenity/commit/61bcfbcd42e186b66b670e374ac42fc73f8397b7
[c:91f10dd]: https://github.com/serenity-rs/serenity/commit/91f10dd7ded8120f1364bd0be65ca7b347d79713
[c:d2b19a2]: https://github.com/serenity-rs/serenity/commit/d2b19a27128e80ad298bc87414d6cb0a10060bc6
[c:c3d5264]: https://github.com/serenity-rs/serenity/commit/c3d5264f384429050d64989587ef6b9ba045b7c2
[c:3a313c8]: https://github.com/serenity-rs/serenity/commit/3a313c8c9f8ab3a03992cb6222794b010128b899
[c:4b67d8e]: https://github.com/serenity-rs/serenity/commit/4b67d8e0fd908f9373cb89b14ecb72a84eb84e03
[c:d851fea]: https://github.com/serenity-rs/serenity/commit/d851fea1c012b8a6a62b46ac38311909d0b10989
[c:8bba7b0]: https://github.com/serenity-rs/serenity/commit/8bba7b0269e10dc9859da13a51f5bc9101546bc3
[c:5b0e9f3]: https://github.com/serenity-rs/serenity/commit/5b0e9f3f4ed4975e0b0085ec652dd624b67178f3

[c:05044b6]: https://github.com/serenity-rs/serenity/commit/05044b6f1bdf9047ebfc447e19d199f4a35816e6
[c:8326dc7]: https://github.com/serenity-rs/serenity/commit/8326dc761e0c86672e37a78eecfd8cab22589c82
[c:d189624]: https://github.com/serenity-rs/serenity/commit/d18962468010b7beed1b6311b8b1df42edff4530
[c:5dbe078]: https://github.com/serenity-rs/serenity/commit/5dbe078c5395d0d39bda6a202e1dde367bf0de33
[c:28a91c6]: https://github.com/serenity-rs/serenity/commit/28a91c61f13e08e15adc2b0cb7f211e4cf610c86
[c:b2c951d]: https://github.com/serenity-rs/serenity/commit/b2c951d90acc2061824926573cc1dfe331d0c1e5
[c:d4b45f4]: https://github.com/serenity-rs/serenity/commit/d4b45f4758e8545e913c70dcbf41a122e487ce5d
[c:3a3f9b3]: https://github.com/serenity-rs/serenity/commit/3a3f9b380bfda440c002876eb0726d2f0a8a7d14
[c:08d894e]: https://github.com/serenity-rs/serenity/commit/08d894ef5937c1877c1209816f103421b4c67b80
[c:5b01290]: https://github.com/serenity-rs/serenity/commit/5b0129018301a2235d7cb97b52c9f89cff75da39
[c:6916bfc]: https://github.com/serenity-rs/serenity/commit/6916bfc4d7a30ff331acc4635cd3f30a19c80f80
[c:b5deb39]: https://github.com/serenity-rs/serenity/commit/b5deb391bf0521d049152218a8d774c8db474e5a
[c:3c2f9ad]: https://github.com/serenity-rs/serenity/commit/3c2f9adf4d540597b620866773ac4b574eb71d12
[c:3a449ee]: https://github.com/serenity-rs/serenity/commit/3a449ee8a11cadf1c09d93f860c97ef2dfb522a8
[c:e807288]: https://github.com/serenity-rs/serenity/commit/e8072881af7cf8f826c2f3ae77684ad2a4893841
[c:ab7f631]: https://github.com/serenity-rs/serenity/commit/ab7f6316b551ed0485dac99d328321d8363405e2
[c:db5a09e]: https://github.com/serenity-rs/serenity/commit/db5a09e8c513befd5f41894baededd259f8b6df7
[c:cc2f918]: https://github.com/serenity-rs/serenity/commit/cc2f918fba4b3a1dc8f0bb939a24bf020007de42
[c:240d3e6]: https://github.com/serenity-rs/serenity/commit/240d3e63432f6a1d35ef0a40cbe48d5e9826409c
[c:0b3ad00]: https://github.com/serenity-rs/serenity/commit/0b3ad00853d23db316e2bd0b5759617b598e61aa
[c:173f7fa]: https://github.com/serenity-rs/serenity/commit/173f7fad1486e31c473293ed2ef7ff25a6ce2a08
[c:aed4b24]: https://github.com/serenity-rs/serenity/commit/aed4b24e8e8c511748ae28f33a5ff81280ad1069
[c:ae0acd0]: https://github.com/serenity-rs/serenity/commit/ae0acd0c6442e937c90e49843d66dfd11c544cfa
[c:f897a8d]: https://github.com/serenity-rs/serenity/commit/f897a8dfc39598e055a838e92550de915aa4ef50
[c:346a7fe]: https://github.com/serenity-rs/serenity/commit/346a7feb0da9167760bd69d1d1cc3478c6c379b6
[c:d6699c2]: https://github.com/serenity-rs/serenity/commit/d6699c26c2ae2c29be1e1463b254e104fbd80064
[c:69f2dff]: https://github.com/serenity-rs/serenity/commit/69f2dffdb3caa24e3387a2702593bcb57ba5e690
[c:caeeda1]: https://github.com/serenity-rs/serenity/commit/caeeda155d1853125d2100f3d55b7060d9a27888
[c:487fa04]: https://github.com/serenity-rs/serenity/commit/487fa0413c05f5c1ad688bb534dcedb15a428de8

[c:e6ed1b5]: https://github.com/serenity-rs/serenity/commit/e6ed1b5987814174fcf66dff084be45386a68136
[c:42937e9]: https://github.com/serenity-rs/serenity/commit/42937e9b7414455a9baefeb0c902ba81ff242de4
[c:918273b]: https://github.com/serenity-rs/serenity/commit/918273b8e936796e6424b0c28c0a929f7ce6bf03

[c:2734e27]: https://github.com/serenity-rs/serenity/commit/2734e27a163a1cc585bd7f4f7b5aa0855792ed58
[c:c4b1c60]: https://github.com/serenity-rs/serenity/commit/c4b1c6033c7b21ac314ce6845be957ca69b1d223

[c:de9e8a6]: https://github.com/serenity-rs/serenity/commit/de9e8a673f906311957bb2f6e31026cc57fd86b1
[c:445810f]: https://github.com/serenity-rs/serenity/commit/445810f0673319462b685d849c6ac87ab739f44d
[c:2d3e585]: https://github.com/serenity-rs/serenity/commit/2d3e585506d20c4ffab34ff015679a1dcca30575
[c:65837f5]: https://github.com/serenity-rs/serenity/commit/65837f54a671a30a869fe09e2a1abc70d64a5226
[c:730c959]: https://github.com/serenity-rs/serenity/commit/730c959c73b0e3227a42dc2373aed646e286c3a4
[c:e762ea9]: https://github.com/serenity-rs/serenity/commit/e762ea948d6ee3fdf76991f60e743adcb8c3d8ae
[c:711882b]: https://github.com/serenity-rs/serenity/commit/711882baabde1127b9bf6e2e39116306961f671a
[c:0183714]: https://github.com/serenity-rs/serenity/commit/0183714d450b2285cfae3c619063965783af95c1
[c:21c95fd]: https://github.com/serenity-rs/serenity/commit/21c95fdfd9b4fe8a98d3a0e459e8ab94ceecaa23
[c:ebdeb4e]: https://github.com/serenity-rs/serenity/commit/ebdeb4e456c206ea0cccd94318e4eb19660241a0
[c:d427da4]: https://github.com/serenity-rs/serenity/commit/d427da4a17dd78fe5f4f681855e028abb3fbccee
[c:96b49f9]: https://github.com/serenity-rs/serenity/commit/96b49f97c080ea6fdc2e1bbd1cd1e90958adceb1

[c:50d2a76]: https://github.com/serenity-rs/serenity/commit/50d2a7654e0aa5248c16941b68da30d758262419

[c:aa1070d]: https://github.com/serenity-rs/serenity/commit/aa1070d05f23ea2a7a57857ee47e7b41af36815b
[c:720d9ad]: https://github.com/serenity-rs/serenity/commit/720d9adda4d432cf3fb5ceb890fc0aa751f927bb
[c:c45c1d4]: https://github.com/serenity-rs/serenity/commit/c45c1d47ec70168e90091e676d3fdf0a0d4e4c8c
[c:8594c29]: https://github.com/serenity-rs/serenity/commit/8594c29a2e993da7960d0c63a571bae203e07ea3
[c:85dd1a0]: https://github.com/serenity-rs/serenity/commit/85dd1a011593c293319c26a1fd5e7a45ba0c693d
[c:6d06632]: https://github.com/serenity-rs/serenity/commit/6d066322e1a6a2fd0d2a577b2f7f0b59b842789f
[c:22f3d2a]: https://github.com/serenity-rs/serenity/commit/22f3d2a32e16ef0a12a17ec67415e27a531b095d
[c:8e926f9]: https://github.com/serenity-rs/serenity/commit/8e926f97bccf53e0a2637f81fa8fa6913ed96f9a
[c:3a4e2ed]: https://github.com/serenity-rs/serenity/commit/3a4e2eda25dde94d377fee2bdc088a8c8a2d4e8e
[c:6f7797e]: https://github.com/serenity-rs/serenity/commit/6f7797e45cb9cb887dd0f89eb2f5063fb04d32ee
[c:2439275]: https://github.com/serenity-rs/serenity/commit/2439275d57630fd4e325efe149646c5ef25442bf
[c:ec687ad]: https://github.com/serenity-rs/serenity/commit/ec687adbe0eeba513a462bfa26f779d3bcd4e63e
[c:ccbba0a]: https://github.com/serenity-rs/serenity/commit/ccbba0a67da7514bf0abbdd976beebd0f3a6e30c
[c:09c1e01]: https://github.com/serenity-rs/serenity/commit/09c1e015c2b4ce3c3ed94ca7a44988caf2aff187
[c:e8da420]: https://github.com/serenity-rs/serenity/commit/e8da420e8bdb47da950f8344d7000c5a9d543460

[c:26192fa]: https://github.com/serenity-rs/serenity/commit/26192fa1e8df9a7bd7be6065657890a200432661
[c:e6c5d41]: https://github.com/serenity-rs/serenity/commit/e6c5d418390a90632fb2dee75ddcfd5cc1cc2672
[c:d2df2b9]: https://github.com/serenity-rs/serenity/commit/d2df2b9db9dd23bec2bb3bf8f217a8e437582e2f
[c:d280ed1]: https://github.com/serenity-rs/serenity/commit/d280ed18fedf324699c1173432fd63fd5d0dd657
[c:81d5af1]: https://github.com/serenity-rs/serenity/commit/81d5af16d0d4262f71fe9a3480ad57543d7e2d10
[c:a8f0387]: https://github.com/serenity-rs/serenity/commit/a8f03870c423b4633d6490adb140f5be5d150f40
[c:abd84c2]: https://github.com/serenity-rs/serenity/commit/abd84c202f4932bda8924349126757d1cee93e2a
[c:67f5e3d]: https://github.com/serenity-rs/serenity/commit/67f5e3d970e23da320c091b19fc90314e15db83a
[c:1a209e8]: https://github.com/serenity-rs/serenity/commit/1a209e8815e8319711e012639c90d3de9d322791
[c:45d44cb]: https://github.com/serenity-rs/serenity/commit/45d44cbd65938a8d1f8f65ff865b45316f11a48e
[c:8cdfd7c]: https://github.com/serenity-rs/serenity/commit/8cdfd7cd9a618d4a0edf6e5738979082462cea62
[c:4cf4b21]: https://github.com/serenity-rs/serenity/commit/4cf4b21365bfcd17130867abcae42cee4ca4803b
[c:b7b3a85]: https://github.com/serenity-rs/serenity/commit/b7b3a855c29a036b452f0fbf5ee3f19395bb42f1
[c:eddef7b]: https://github.com/serenity-rs/serenity/commit/eddef7b57f9d1f1380d77ed42e7497015b97ba49
[c:cd4ca1b]: https://github.com/serenity-rs/serenity/commit/cd4ca1b98071b3617e55407a30a3837c2dcfaebc

[c:17f1dc2]: https://github.com/serenity-rs/serenity/commit/17f1dc214f95be129d0ade54ebe8e4e7ab93fbdc
[c:13595ff]: https://github.com/serenity-rs/serenity/commit/13595ff25d35fcaf9bd69f7fe8d75c67f72e676e
[c:e628614]: https://github.com/serenity-rs/serenity/commit/e62861464d96d42a150cce1678cd4afdbea6f121
[c:b28716c]: https://github.com/serenity-rs/serenity/commit/b28716cf09a66b0fc717643c4c6a3e0e8c4afb57

[c:335701e]: https://github.com/serenity-rs/serenity/commit/335701ee06f0083ab98cc245a59a0a77f6d6bd62
[c:3a72058]: https://github.com/serenity-rs/serenity/commit/3a72058d3ef3aa7324c1348e05435575f46f7211
[c:e5081db]: https://github.com/serenity-rs/serenity/commit/e5081db9f8adf370f193340f645f6ab54612b413
[c:759a278]: https://github.com/serenity-rs/serenity/commit/759a2788896f08c79972f1e8fa91ca212c104e52
[c:030bb4d]: https://github.com/serenity-rs/serenity/commit/030bb4d76df5a40fe90bc531d8cd05c4b99599f0
[c:7a0d169]: https://github.com/serenity-rs/serenity/commit/7a0d1698576ecae1159b1079e5689ce0d483b85f
[c:1527838]: https://github.com/serenity-rs/serenity/commit/1527838bfd8d3984ce6c57686d3aac70493e6c55
[c:3e15bb8]: https://github.com/serenity-rs/serenity/commit/3e15bb8ad240431d4351a1ab00d5aed249434fd5

[c:90b7829]: https://github.com/serenity-rs/serenity/commit/90b78294c74bb4fe7f861fad0a1896a5b1ee280f
[c:638b642]: https://github.com/serenity-rs/serenity/commit/638b642c853e0567fe008298691daaa765ef4a5f
[c:c8a8d4f]: https://github.com/serenity-rs/serenity/commit/c8a8d4f2f5b351def970b344045a16d0504d9d8f
[c:0daaac1]: https://github.com/serenity-rs/serenity/commit/0daaac1519e7b583c9d1ea9e31779d7e6d00e5a0
[c:dddd417]: https://github.com/serenity-rs/serenity/commit/dddd417c1b55b4a4908fd65e2cfd2a0010b31e0d
[c:d1addff]: https://github.com/serenity-rs/serenity/commit/d1addff0dff6f199cacb0ed161ca013cb96d7d02
[c:638bb1a]: https://github.com/serenity-rs/serenity/commit/638bb1af7711898267b67b1513d512d55de97d80
[c:5e77718]: https://github.com/serenity-rs/serenity/commit/5e77718d93c97d0b118e4ad77842f311c9382ba9

[c:ab34f75]: https://github.com/serenity-rs/serenity/commit/ab34f75281750ddca64ada640515fef4e01bdf23
[c:58fa50c]: https://github.com/serenity-rs/serenity/commit/58fa50cd4d1fd6660ed6d3692e125cc4f292097b
[c:7aea26c]: https://github.com/serenity-rs/serenity/commit/7aea26c00ffbca9b18f0ac633df8f54252150614
[c:9783b35]: https://github.com/serenity-rs/serenity/commit/9783b354d313e8753134ad1bfd9c53f3aa966684
[c:c970f44]: https://github.com/serenity-rs/serenity/commit/c970f443e645121033b2b3605ba4f15d33b144f6
[c:c8a5f69]: https://github.com/serenity-rs/serenity/commit/c8a5f6999559f3cf3ebb776b31a445b6b7078968
[c:3cf673e]: https://github.com/serenity-rs/serenity/commit/3cf673e670ecac615a1565ed4c96a513327f7e05
[c:8e01ff6]: https://github.com/serenity-rs/serenity/commit/8e01ff64fe5ed4e15a56dce2acd85574ae5a9d0c
[c:c6ae140]: https://github.com/serenity-rs/serenity/commit/c6ae1402d5be9fad62c348549141e06aa08cb43a
[c:13fae29]: https://github.com/serenity-rs/serenity/commit/13fae29e053dda813448dca97c667d4b5a0519a4
[c:9450d4b]: https://github.com/serenity-rs/serenity/commit/9450d4b55e824bd841577f7bb58f9916d98b9c09
[c:b1eff27]: https://github.com/serenity-rs/serenity/commit/b1eff278bdb876612f5ab99a566e680ffc1db11a
[c:4793a84]: https://github.com/serenity-rs/serenity/commit/4793a8482915b6a4438b5e209144d0fa5f0948ca
[c:aae22a2]: https://github.com/serenity-rs/serenity/commit/aae22a2011f3fc65eecd7a89f473df9de9fd5232
[c:10b9cc2]: https://github.com/serenity-rs/serenity/commit/10b9cc23bd22bc50732fc698c7af4c12c306f695
[c:186e914]: https://github.com/serenity-rs/serenity/commit/186e9148a616f19fdbd526b8a6c7191268ee2936
[c:86ec810]: https://github.com/serenity-rs/serenity/commit/86ec810e06e4f5fad11e10a787aad3f33d7fe9a1
[c:eca204a]: https://github.com/serenity-rs/serenity/commit/eca204a8cc4828eecbe914cabe099fbc50901656
[c:cd7d07e]: https://github.com/serenity-rs/serenity/commit/cd7d07e02aef5810806c8fea09cceb58d7c92578
[c:5e66cd1]: https://github.com/serenity-rs/serenity/commit/5e66cd13a2459bd6b93bffde9827929112443c25
[c:9162929]: https://github.com/serenity-rs/serenity/commit/916292909e9b1cc9db9bd96536632999d9777fdc
[c:6a37535]: https://github.com/serenity-rs/serenity/commit/6a3753589946f9cdd1915aa4277cf61212347025
[c:33f8383]: https://github.com/serenity-rs/serenity/commit/33f83838942cd89f6ba3f981575c20a2f19039a0
[c:05254c8]: https://github.com/serenity-rs/serenity/commit/05254c8376b6b198aff6734aa8c0b58560e3a756
[c:0e55b73]: https://github.com/serenity-rs/serenity/commit/0e55b73f244f8903878f384cddaaf7d67feb0530
[c:0a640a4]: https://github.com/serenity-rs/serenity/commit/0a640a43122a24125f05f7610934dd09d267177c
[c:2ae3a48]: https://github.com/serenity-rs/serenity/commit/2ae3a48cdb6a62066c72c08fcfd31300f70943ea
[c:26b072f]: https://github.com/serenity-rs/serenity/commit/26b072f67f9662214738f0c0db7856c7fe7ef4b7
[c:15e7fde]: https://github.com/serenity-rs/serenity/commit/15e7fdee8de2068c5023a7a6d5b372117ba0b4c5
[c:709c9e4]: https://github.com/serenity-rs/serenity/commit/709c9e45d59b30797062cc32afa910a6b0da7476
[c:3902caf]: https://github.com/serenity-rs/serenity/commit/3902caf9881c9bc2a007e6f417002caef81a3ae5
[c:ddf7a3]: https://github.com/serenity-rs/serenity/commit/ddf7a3f960d06e666288f68a36567f9c73a9fde8
[c:6586830]: https://github.com/serenity-rs/serenity/commit/6586830e9737a6fea256c8b48e83760898e33285
[c:7706475]: https://github.com/serenity-rs/serenity/commit/77064758e63b21825f33569d10008edaa6bcd4d5
[c:fc3a1f6]: https://github.com/serenity-rs/serenity/commit/fc3a1f6f911d2a9615c9647b252624c5bdeffb97
[c:7f9c4e1]: https://github.com/serenity-rs/serenity/commit/7f9c4e1b4d829ce99614271c278a06eb678b778e
[c:f2ff97a]: https://github.com/serenity-rs/serenity/commit/f2ff97aed2c2640a99205ab9f947237aa2ccf01e
[c:0fcb43c]: https://github.com/serenity-rs/serenity/commit/0fcb43c06895efde905482e4b9120c1bde3a671c
[c:82d97c2]: https://github.com/serenity-rs/serenity/commit/82d97c2513e547dd5275f07fe7327e0433f18860
[c:d91594b]: https://github.com/serenity-rs/serenity/commit/d91594baadf49c725a2478b85509194c17a3ba67
[c:4541935]: https://github.com/serenity-rs/serenity/commit/4541935243d794cc760f46520f0cfa3f4994a9a3
[c:7c61f95]: https://github.com/serenity-rs/serenity/commit/7c61f958b139159a6c4595f6c99d7812d69d339b
[c:5f7231d]: https://github.com/serenity-rs/serenity/commit/5f7231db550ba5233773b801710fd593642cbb2f
[c:f7109ee]: https://github.com/serenity-rs/serenity/commit/f7109ee74d62652569f860ac39b70962bc08bbeb
[c:5375827]: https://github.com/serenity-rs/serenity/commit/5375827523034e5074a3490a70f002caefd77b7b
[c:3de5378]: https://github.com/serenity-rs/serenity/commit/3de537875734a9a847ec1ac986430371f1f27d4a
[c:8d50840]: https://github.com/serenity-rs/serenity/commit/8d508401371ea7595030a7a2bafa76c7b1c76251
[c:98532da]: https://github.com/serenity-rs/serenity/commit/98532da727b8f813985f333290a5d954d0a654c6
[c:d27d391]: https://github.com/serenity-rs/serenity/commit/d27d391afaf55ba3b96c13701a71beb0ce5a2844
[c:8c83fec]: https://github.com/serenity-rs/serenity/commit/8c83fec4d85dee65078bad65b9abc51daf564af1
[c:ef15739]: https://github.com/serenity-rs/serenity/commit/ef15739329809b0c3c07d696b5bce4faf2a0e345
[c:62e19a7]: https://github.com/serenity-rs/serenity/commit/62e19a721c1b04bf9e5cd2f9ab3429b4f6634a6f
[c:c14ca32]: https://github.com/serenity-rs/serenity/commit/c14ca326ab49ae01f13df3a08c7a64d23a1429ae

[c:23bed41]: https://github.com/serenity-rs/serenity/commit/23bed417864a53c6e050ec732f72cf175ef293df
[c:1bd5bbc]: https://github.com/serenity-rs/serenity/commit/1bd5bbc8004dab8150ba4c631b8323440634cd35
[c:c63eaea]: https://github.com/serenity-rs/serenity/commit/c63eaeafe5c23006712021cc3efde9988efb2fd2
[c:64e97c5]: https://github.com/serenity-rs/serenity/commit/64e97c51367630f04d1d58ab917d584cd6e9c15a
[c:b425ceb]: https://github.com/serenity-rs/serenity/commit/b425ceb51ac34e55ce055ad981569818fa2558ea
[c:b1559bc]: https://github.com/serenity-rs/serenity/commit/b1559bc38f99a65cdf3231ddafa84c51b5829b1a
[c:61ac765]: https://github.com/serenity-rs/serenity/commit/61ac765c9d5ad75a848322418d4d2c4ad0c54236
[c:decbc04]: https://github.com/serenity-rs/serenity/commit/decbc04c01cbb24755a0ac1219c9778b9c53d63c

[c:5405ac2]: https://github.com/serenity-rs/serenity/commit/5405ac2a46c5f90451de0a2c68f6f6d323ce299b
[c:f7360e6]: https://github.com/serenity-rs/serenity/commit/f7360e6675d100c7af9c0a67ed47d76e64672e37
[c:7512c19]: https://github.com/serenity-rs/serenity/commit/7512c19ff3b395e57fccaa8f4cfb6e5923138414
[c:25cb595]: https://github.com/serenity-rs/serenity/commit/25cb5959dea20803a236ff151d38b015554e3ea4
[c:4026d77]: https://github.com/serenity-rs/serenity/commit/4026d77a229027170f516ceee14763422d1b5ba2
[c:2f613c0]: https://github.com/serenity-rs/serenity/commit/2f613c0e817cd880941b9d72f4aaed3f67d6722a
[c:0a58e85]: https://github.com/serenity-rs/serenity/commit/0a58e858ea8370a90c465f47b6cf8b4c83263c65
[c:59b4c60]: https://github.com/serenity-rs/serenity/commit/59b4c60a1db57663428c4ea476dc099af1af75a7
[c:d2233e2]: https://github.com/serenity-rs/serenity/commit/d2233e25e1badb9379bd1b91896afd579f7f0106
[c:2969561]: https://github.com/serenity-rs/serenity/commit/2969561517ff4a0625e4a22230dd807ab62c4aa8
[c:faa773a]: https://github.com/serenity-rs/serenity/commit/faa773a301ca1a277f4912f2bcc62abf7caeda31
[c:1074b28]: https://github.com/serenity-rs/serenity/commit/1074b28560c3e1a9bb9ec4796d693136b7f6714b
[c:2b453c3]: https://github.com/serenity-rs/serenity/commit/2b453c365c0169475c67977f2e081f67083b734a
[c:965fa7b]: https://github.com/serenity-rs/serenity/commit/965fa7bf088e5ee5a79efb0b7167478bb0fe719c
[c:15e2c41]: https://github.com/serenity-rs/serenity/commit/15e2c41ca95bcc9666e45eea542f7c712ede9949
[c:00f465c]: https://github.com/serenity-rs/serenity/commit/00f465ceb93f6f4809e121ea00dd9c7bba630e62
[c:393a5ae]: https://github.com/serenity-rs/serenity/commit/393a5aec2a9823549ac7cc1d376991651a61f33d
[c:1546171]: https://github.com/serenity-rs/serenity/commit/15461712914708b51b19f8cf0ddfd0851b63f93e
[c:6d87d71]: https://github.com/serenity-rs/serenity/commit/6d87d7105deda09dc2c08f554afd1371dc2eadf3
[c:b7a6fee]: https://github.com/serenity-rs/serenity/commit/b7a6feee7c9896f0ad4fecc31f3cba4ec5d40429
[c:b012ab7]: https://github.com/serenity-rs/serenity/commit/b7a6feee7c9896f0ad4fecc31f3cba4ec5d40429
[c:1546171]: https://github.com/serenity-rs/serenity/commit/15461712914708b51b19f8cf0ddfd0851b63f93e
[c:1e50d30]: https://github.com/serenity-rs/serenity/commit/1e50d30d405050751a91e3cbc3d8b4aaeef9217a
[c:3f81cf3]: https://github.com/serenity-rs/serenity/commit/3f81cf3392436ca82d9eb55949fb7c7f5557d820
[c:87bc6ca]: https://github.com/serenity-rs/serenity/commit/87bc6ca529e211bbb427f05a58519724eed5c443
[c:86a8b60]: https://github.com/serenity-rs/serenity/commit/86a8b60b4c84cf2239d6870454dda3c5abab98eb
[c:a5aa2a9]: https://github.com/serenity-rs/serenity/commit/a5aa2a9c16e96741e59524df78be3ae3d4c5788c
[c:21518c8]: https://github.com/serenity-rs/serenity/commit/21518c8590c055d3eab6c99ebdd824721b7b0a73
[c:712cfa5]: https://github.com/serenity-rs/serenity/commit/712cfa58c3e426c698b90a0bc49de3f81258c59b
[c:3f0ea69]: https://github.com/serenity-rs/serenity/commit/3f0ea6985e2333d3b04af174a811357d96aa3e02
[c:661d778]: https://github.com/serenity-rs/serenity/commit/661d7787ecb399803ae8794adffdea7df44f6839
[c:40bf272]: https://github.com/serenity-rs/serenity/commit/40bf272cee3d9bcded6598a830c2b54dfad2d504
[c:fa0376c]: https://github.com/serenity-rs/serenity/commit/fa0376c543f545f53c67213cc3d3ee4aebe26ea8
[c:51b48f4]: https://github.com/serenity-rs/serenity/commit/51b48f498cba54e3d05b8bcd79d370e429501f9a
[c:5d6dc37]: https://github.com/serenity-rs/serenity/commit/5d6dc37082412fb0f254000cc19211f78bbb7327
[c:625b764]: https://github.com/serenity-rs/serenity/commit/625b7643fddb6c1a13eb5dab6ae3c536e44f2780
[c:c472ddd]: https://github.com/serenity-rs/serenity/commit/c472ddd8aad713c0a378af3d7740c799f36d95ab
[c:cc81e47]: https://github.com/serenity-rs/serenity/commit/cc81e47d518402de79e287b6d0e80a3a59a74d26
[c:9b591ec]: https://github.com/serenity-rs/serenity/commit/9b591ec0219f62c3c84fc9355c3746e73ce76042
[c:5dff7eb]: https://github.com/serenity-rs/serenity/commit/5dff7eb3d9d59cf8ead692b4ca00bf69e888649f
[c:d955df4]: https://github.com/serenity-rs/serenity/commit/d955df401a3e7b91cb22c037965a272978e5a575
[c:fa11a30]: https://github.com/serenity-rs/serenity/commit/fa11a30bafa050a56df7138275556ddd54895b93
[c:bd45e42]: https://github.com/serenity-rs/serenity/commit/bd45e42ce75b25f3fd9abff9098e08d82b288c17
[c:a713b40]: https://github.com/serenity-rs/serenity/commit/a713b400c79995daa306ec975ac7a99dcabc3c65
[c:4af7a98]: https://github.com/serenity-rs/serenity/commit/4af7a986579b67d56bdbf6254256187184aa4897
[c:9cc8816]: https://github.com/serenity-rs/serenity/commit/9cc8816ec56295541230c87992500fee0aaa8696
[c:6cfc0e1]: https://github.com/serenity-rs/serenity/commit/6cfc0e18181a8c93906ed4ea25eb68796e0f4720
[c:d995fa0]: https://github.com/serenity-rs/serenity/commit/d995fa0e08c67e1a7f217b427d4d9b4dcedaa45b
[c:0a77330]: https://github.com/serenity-rs/serenity/commit/0a773302eb57c5b7e024f91336cc0547a8746616
[c:d01eeae]: https://github.com/serenity-rs/serenity/commit/d01eeae4fbd7676ceb2bb903b3933b7e939ba64e
[c:a3477a2]: https://github.com/serenity-rs/serenity/commit/a3477a2cad7d36110acb0316df927bf8611ebece
[c:bc0d82e]: https://github.com/serenity-rs/serenity/commit/bc0d82eb73f1d5c277dbe9865540b7a623d373b2
[c:70720ae]: https://github.com/serenity-rs/serenity/commit/70720aeeee44d67a4cb2d58a0c375a54c9be95a2

[c:f648d90]: https://github.com/serenity-rs/serenity/commit/f648d9093f87354bbec03228fa647f6dd9afb03a
[c:33f4adf]: https://github.com/serenity-rs/serenity/commit/33f4adfe0f6303ac6b39e8c3db6f413e2289c81b
[c:1705338]: https://github.com/serenity-rs/serenity/commit/17053381b1481e753abdcd319143ddd63467605d
[c:e40758e]: https://github.com/serenity-rs/serenity/commit/e40758eefdbe5a2b62f252cde69e7dec04898a09
[c:a7ee6a6]: https://github.com/serenity-rs/serenity/commit/a7ee6a674ae158839466db58ad7e090bb64dc797
[c:68c4f5c]: https://github.com/serenity-rs/serenity/commit/68c4f5c907993c70fc4a590a6f7d06ee0dba98ee

[c:201dab8]: https://github.com/serenity-rs/serenity/commit/201dab8fa4c31d6e840f88b691772c5b0961780f
[c:201bc56]: https://github.com/serenity-rs/serenity/commit/201bc56f79ab913db32b3453bc8d61a7bfa0ee17
[c:68263ac]: https://github.com/serenity-rs/serenity/commit/68263acc637010f854d986bdcd838593b0dc02f0
[c:bca2f4b]: https://github.com/serenity-rs/serenity/commit/bca2f4be1e603122e681423410b7f30562912727
[c:3b8ae67]: https://github.com/serenity-rs/serenity/commit/3b8ae6712cb1a6d771704de97b2ea7af3f1f7138
[c:a0b1dd8]: https://github.com/serenity-rs/serenity/commit/a0b1dd8b2f8dc4c4a3b924d6ed686e5ba006e60d
[c:bd48ac5]: https://github.com/serenity-rs/serenity/commit/bd48ac5106c540358babff1e31a81e1e1ab7cec0
[c:00990c0]: https://github.com/serenity-rs/serenity/commit/00990c05bb5bfaf0e0ee5e52ed1faefe142277a3
[c:bc3d978]: https://github.com/serenity-rs/serenity/commit/bc3d978b65ae6d07342bfba4618c249d0beae98e
[c:e94388]: https://github.com/serenity-rs/serenity/commit/e94388115845aba533eb7d08f13fddd46ef30f69
[c:cfcd342]: https://github.com/serenity-rs/serenity/commit/cfcd342baf254d35da948cce9fe1c39a284ce6cb
[c:07e81b0]: https://github.com/serenity-rs/serenity/commit/07e81b02143d57f5abced7e0d4fe13d40da7a5de
[c:498e41c]: https://github.com/serenity-rs/serenity/commit/498e41c91453a1d895e5c46e76310f92b44127c0
[c:3899547]: https://github.com/serenity-rs/serenity/commit/3899547968abbdf3a51071a5d9eccdfaad4cc0c4
[c:b469611]: https://github.com/serenity-rs/serenity/commit/b4696115c05eefaabff8b0f5ceb3b12b900dda2b
[c:7c09cdd]: https://github.com/serenity-rs/serenity/commit/7c09cdd27d28264444f34ab8157bc9aa78cbc096
[c:88d914e]: https://github.com/serenity-rs/serenity/commit/88d914e32071ef9cefcfbc4cd74df8f664b66377
[c:e6694f2]: https://github.com/serenity-rs/serenity/commit/e6694f27d74069c80f642ab17ce73d45962fd547
[c:62a1aa2]: https://github.com/serenity-rs/serenity/commit/62a1aa2abcf0919bf38ef90590aaa363eb03aae0
[c:23ae9d8]: https://github.com/serenity-rs/serenity/commit/23ae9d83f96e0d1ad28905deb73eb4ad4fed61a8

[c:2cb67df]: https://github.com/serenity-rs/serenity/commit/2cb67df6a0441b70462434429030cef759c7c57c
[c:794393c]: https://github.com/serenity-rs/serenity/commit/794393c9656e2de916ca72fd63393b26d4fef7a0
[c:b11b4e2]: https://github.com/serenity-rs/serenity/commit/b11b4e23ca576b0b23d67c0b28c7578340762b44
[c:a56d014]: https://github.com/serenity-rs/serenity/commit/a56d0146c6c287f912d65882cc3fa7516355d156
[c:7066ed2]: https://github.com/serenity-rs/serenity/commit/7066ed24a9d4f3386972f9173673034109546378
[c:8bf39a7]: https://github.com/serenity-rs/serenity/commit/8bf39a7a6971543d2deacec4ec77bd871e046ff2
[c:dd75410]: https://github.com/serenity-rs/serenity/commit/dd75410c977dd21fe540e01c3b68731c0718e941
[c:065f55b]: https://github.com/serenity-rs/serenity/commit/065f55b05b174acdf37ea78e9f00b5ed9b10dd28
[c:bca1530]: https://github.com/serenity-rs/serenity/commit/bca1530a6cebafe028d4a3a58f332b7985d5229f
[c:e8d0628]: https://github.com/serenity-rs/serenity/commit/e8d0628a35adeca44386e7a9a5e58714a66a4ac2
[c:11d5b72]: https://github.com/serenity-rs/serenity/commit/11d5b724a95e47001028a5d0d75d91380463b1bc
[c:98bece3]: https://github.com/serenity-rs/serenity/commit/98bece30c6f4ddb265c635b8c285aec1ea42c6d9
[c:ae0fc14]: https://github.com/serenity-rs/serenity/commit/ae0fc14793e6e55a09ba05f2769441f95461f327
[c:3c166e3]: https://github.com/serenity-rs/serenity/commit/3c166e38e00fa9363504eba69d2e2a15ccf046c7
[c:204e0b9]: https://github.com/serenity-rs/serenity/commit/204e0b94a9a4ea2b7f9c0fc88e3a7b097f2c00bc

[c:16bc3815]: https://github.com/serenity-rs/serenity/commit/16bc3815b3420ae2224667e6e1bbdf768760fd87
[c:5f9ed749]: https://github.com/serenity-rs/serenity/commit/5f9ed7497dc08f238fad8e06d041e0b84ac1d00a
[c:ed17114c]: https://github.com/serenity-rs/serenity/commit/ed17114c3d5052eb88b95217bd248bba9a294e6a
[c:99b72358]: https://github.com/serenity-rs/serenity/commit/99b7235888fcabf6952812eb54150ee53989fa4a
[c:d6c4beea]: https://github.com/serenity-rs/serenity/commit/d6c4beeaf89940731c3f2fff14199414dc478342

[c:08511dae]: https://github.com/serenity-rs/serenity/commit/08511dae397f4d99e222d24f546035108a885da0
[c:9f834b2b]: https://github.com/serenity-rs/serenity/commit/9f834b2ba32444fdc6efebd601d062a7f71b3fcb
[c:3b050f49]: https://github.com/serenity-rs/serenity/commit/3b050f49fbc2055c3940b4cacd05b3068856c7b5
[c:e763d80b]: https://github.com/serenity-rs/serenity/commit/e763d80b7697f92e84c2d84ace5ea9fc50a9f9f0
[c:d529cf79]: https://github.com/serenity-rs/serenity/commit/d529cf79af4e493700aa9c69bbb690dbc47a80b8
[c:ccfa7fdc]: https://github.com/serenity-rs/serenity/commit/ccfa7fdc750f567f713e01e6f8e8ccfd0cdec2fb
[c:69931fe3]: https://github.com/serenity-rs/serenity/commit/69931fe349a55eee6ef2735dcfa3823a5988df69
[c:9d141bfc]: https://github.com/serenity-rs/serenity/commit/9d141bfcb23894c5017bae823faa3b144734d42e
[c:8e401f03]: https://github.com/serenity-rs/serenity/commit/8e401f03683b8f2cbce161be669cbd85c791e798
[c:9865d9cc]: https://github.com/serenity-rs/serenity/commit/9865d9ccd727a7f6c5c9a6094b87af0f6353831b
[c:093a1bab]: https://github.com/serenity-rs/serenity/commit/093a1babec70fc331ec08ba14f23f1c14a08178f
[c:5b6574c3]: https://github.com/serenity-rs/serenity/commit/5b6574c351daa8c28c0fbcf666a14289b4505be3
[c:e32f9b57]: https://github.com/serenity-rs/serenity/commit/e32f9b57d22c37a3238e83d086694304eb6c0cd5
[c:b2362dbb]: https://github.com/serenity-rs/serenity/commit/b2362dbb0014781bd7757a9e322ae3b8d5f2fadf
[c:c5285ae1]: https://github.com/serenity-rs/serenity/commit/c5285ae1824dd26adbbd2f0b876eed607f64baa1
[c:6a68f68e]: https://github.com/serenity-rs/serenity/commit/6a68f68e6cb95af38666a4f5d9a6ad4b39fa88c6
[c:75fb5c04]: https://github.com/serenity-rs/serenity/commit/75fb5c041511077e60e577e55039acc33d624569
[c:176fde29]: https://github.com/serenity-rs/serenity/commit/176fde296b143e230ea8889073c69b34a95d41f6
[c:12534348]: https://github.com/serenity-rs/serenity/commit/12534348792edf78cddccb3169e068da27cfcb5e
[c:41ff44ba]: https://github.com/serenity-rs/serenity/commit/41ff44ba4e4bdd7aa77bfbfce201b89c7990d9e4
[c:867a7447]: https://github.com/serenity-rs/serenity/commit/867a744720c46c0b04a2d34c2119ad366aa440ef

[c:12bbc1a]: https://github.com/serenity-rs/serenity/commit/12bbc1ab79176d39c2528cae3c762404b0d5f8ab
[c:14c6099]: https://github.com/serenity-rs/serenity/commit/14c6099ced49623b0c3a373b9a21d0574f9294c9
[c:19c65bd]: https://github.com/serenity-rs/serenity/commit/19c65bd27f27192cc9a8a04c1d38ec08b62652c8
[c:28cdc53]: https://github.com/serenity-rs/serenity/commit/28cdc5328687b74772e37da89caff5751e30a2a5
[c:30a325e]: https://github.com/serenity-rs/serenity/commit/30a325ea840755cf74f376657d9a1e9ac363e92e
[c:3fbab76]: https://github.com/serenity-rs/serenity/commit/3fbab7638be44914a7a28ac366ca03d4d1df9bba
[c:41b6e24]: https://github.com/serenity-rs/serenity/commit/41b6e247b0484fc2ff3e254bb61d104b2e94cbdb
[c:4778e69]: https://github.com/serenity-rs/serenity/commit/4778e6940131e97691f5e1e3d04a28480a9066cc
[c:6157f61]: https://github.com/serenity-rs/serenity/commit/6157f61600d656219491f21f533f63c8f362bd1b
[c:669da40]: https://github.com/serenity-rs/serenity/commit/669da407111f924a5dc498c15c0c0b43f7b42411
[c:6ca4bea]: https://github.com/serenity-rs/serenity/commit/6ca4bea21ac83034c3ff1d4adf79754c80df85ca
[c:7295079]: https://github.com/serenity-rs/serenity/commit/729507947c05c313d37b4b31059f41ba8e6f147a
[c:75f6516]: https://github.com/serenity-rs/serenity/commit/75f6516fceb6d8e124f91ae25a10f74f183337ad
[c:79d8843]: https://github.com/serenity-rs/serenity/commit/79d8843e3640bcc6ffffc0101f3ef458f6770684
[c:823b829]: https://github.com/serenity-rs/serenity/commit/823b8299bb88013ce900e2f8d4b5745556380c72
[c:82dbff2]: https://github.com/serenity-rs/serenity/commit/82dbff282d4eefe7a7125f4393eef2d2eee3beb5
[c:966cb3e]: https://github.com/serenity-rs/serenity/commit/966cb3e00a7c8a803a299db8f792d42542d5896a
[c:c49e02c]: https://github.com/serenity-rs/serenity/commit/c49e02ca024b0263d2b7e23e67338558555101ea
[c:ce79f01]: https://github.com/serenity-rs/serenity/commit/ce79f0183d9fc457ce0fc10fa94e3a1350f33f66
[c:dec3f13]: https://github.com/serenity-rs/serenity/commit/dec3f13ac10b7d22a45ae8393dda95f0a796aee7
[c:e59f766]: https://github.com/serenity-rs/serenity/commit/e59f766c24b53b9c98109e8cfeafdec36feed161
[c:e66812a]: https://github.com/serenity-rs/serenity/commit/e66812aa3b8458634901ca7226e5547f0e4be9eb
[c:ebbc324]: https://github.com/serenity-rs/serenity/commit/ebbc32438e1cca94da80b00ae753e3cde86fb73f
[c:f01e6e3]: https://github.com/serenity-rs/serenity/commit/f01e6e35c42372984f52d53ae8a7d4fa4712047b
[c:fe69ef0]: https://github.com/serenity-rs/serenity/commit/fe69ef034c2d6561e05ff67f6a419b7b4a42c04e

[c:0bbe5f5]: https://github.com/serenity-rs/serenity/commit/0bbe5f5dde6989a8d6a4d4910bf026b1b801fef9
[c:40053a7]: https://github.com/serenity-rs/serenity/commit/40053a71931bb63c43eb6f469ee3c94383c9e90a
[c:46b4194]: https://github.com/serenity-rs/serenity/commit/46b419460254edc2343b5a184952ab5c6e53b287
[c:516ede3]: https://github.com/serenity-rs/serenity/commit/516ede3649b74bca8631d05397e330cde0632fee
[c:71edc3a]: https://github.com/serenity-rs/serenity/commit/71edc3a11ac450728bca19ca7cab7c84079d59f0
[c:7b0cff6]: https://github.com/serenity-rs/serenity/commit/7b0cff66f483687b26f3129e7b093f6a87fb1383
[c:826220f]: https://github.com/serenity-rs/serenity/commit/826220f351a688b2a6f1c6ec527e65a996861d22
[c:8bec4af]: https://github.com/serenity-rs/serenity/commit/8bec4af635c3e50b111d19f6c20d56eafbb81193

[c:04b410e]: https://github.com/serenity-rs/serenity/commit/04b410ee75b2eb29f32e66fc137d3992a4972f1d
[c:3a58090]: https://github.com/serenity-rs/serenity/commit/3a580909c489c328f3faa10741debd4b063e7fbd
[c:d1266fc]: https://github.com/serenity-rs/serenity/commit/d1266fc3051a436f87a4778c5081c2228eb50b1c

[c:01e3c33]: https://github.com/serenity-rs/serenity/commit/01e3c331ed188e2b95bafa2fa0fc63d5c0c03905
[c:02de778]: https://github.com/serenity-rs/serenity/commit/02de7789d72141434264e8bd7cee7e1fc65a043f
[c:0501020]: https://github.com/serenity-rs/serenity/commit/05010204eaded91b29aef0561fc8fb668b522760
[c:0d55363]: https://github.com/serenity-rs/serenity/commit/0d553630c1a9da216e42e7c0a9bedaccfedf678d
[c:12d5321]: https://github.com/serenity-rs/serenity/commit/12d53214f39211a4c02026d9389b9aa2bfa8a5ee
[c:1de3937]: https://github.com/serenity-rs/serenity/commit/1de39377a2e428f9652d887627f420349337c5b1
[c:2179623]: https://github.com/serenity-rs/serenity/commit/2179623ebf12f7d8e16cc87e193ecd4de0f7b1fe
[c:21eb42f]: https://github.com/serenity-rs/serenity/commit/21eb42f96f9721d4e004dbc70aedf60e6d1ae7c4
[c:2a6c3b1]: https://github.com/serenity-rs/serenity/commit/2a6c3b1d1e24ec7dc3b1f19baf87594e362ded27
[c:4648f58]: https://github.com/serenity-rs/serenity/commit/4648f58e8ddc878d06a5a4a1d2840180c359ddf0
[c:602c5a7]: https://github.com/serenity-rs/serenity/commit/602c5a7b78dda42b9c3d5426c39099d48e74bca5
[c:73ab20f]: https://github.com/serenity-rs/serenity/commit/73ab20f271c9cc6dadb7bb76938ae64d19cee71e
[c:7a93557]: https://github.com/serenity-rs/serenity/commit/7a935574ffe0b7d19c1ed5c5befe1b7e7e4f0e0d
[c:8301333]: https://github.com/serenity-rs/serenity/commit/830133377a5832784c311302e543f86f85194e3b
[c:869fff5]: https://github.com/serenity-rs/serenity/commit/869fff566ca7a3669f7f08461a6bd481af3649d3
[c:8918201]: https://github.com/serenity-rs/serenity/commit/891820102ff7b9025c67e03ac59f5ecd75959aac
[c:8c0e5a3]: https://github.com/serenity-rs/serenity/commit/8c0e5a377ad7db3c40e37740123c0ebf3d7e36ae
[c:8f128b2]: https://github.com/serenity-rs/serenity/commit/8f128b2c041d5f708378082af3653ff1ee2df919
[c:90c7ec4]: https://github.com/serenity-rs/serenity/commit/90c7ec45d6cc01b25296de9619b7d3a6288244fe
[c:9568e3b]: https://github.com/serenity-rs/serenity/commit/9568e3b24816bb180740789d1e30c29f3658dc8b
[c:9a863bd]: https://github.com/serenity-rs/serenity/commit/9a863bd78e8edc5849e56e979888f1191b1d5845
[c:a0b0dd2]: https://github.com/serenity-rs/serenity/commit/a0b0dd226f9ad2476729fa79dbc680bd08aa44b3
[c:a4c3fec]: https://github.com/serenity-rs/serenity/commit/a4c3fec493d3b85ad1b43f3a5c4927d0d5cdc717
[c:aa437d4]: https://github.com/serenity-rs/serenity/commit/aa437d4dbc4a59ffa65f80c7eafa6efc37eedc86
[c:b324774]: https://github.com/serenity-rs/serenity/commit/b3247749f745c524b1eb0f44118c8358868e722a
[c:bbbf638]: https://github.com/serenity-rs/serenity/commit/bbbf63868a8ef3c0f21c1896f7afb96f4d8fbcc1
[c:c458099]: https://github.com/serenity-rs/serenity/commit/c45809973f9ed333d9c13905a376af14a73d920b
[c:db21036]: https://github.com/serenity-rs/serenity/commit/db210367f3752d8e8ad018742ea0b590ddc54009
[c:e1332a5]: https://github.com/serenity-rs/serenity/commit/e1332a54af46eff6051097ff4989c8d0fde4ca37
[c:e2873c8]: https://github.com/serenity-rs/serenity/commit/e2873c820c1134ea7cc4cfbe99467aac350fa892
[c:e290b03]: https://github.com/serenity-rs/serenity/commit/e290b038242cec6d4465f96c22cff24578f1a068
[c:e5ea6c1]: https://github.com/serenity-rs/serenity/commit/e5ea6c176ba96988efc612a8e14eea90f9c293e1
[c:f064d65]: https://github.com/serenity-rs/serenity/commit/f064d65486d0c8a3c510ee398e7d0bbf6b283bdb
[c:f3f22d7]: https://github.com/serenity-rs/serenity/commit/f3f22d7e072477028c9853d467dd18cf50e1589f

[c:0067c33]: https://github.com/serenity-rs/serenity/commit/0067c3335929325f54a3a0fe3693703e16de219c
[c:04b0be1]: https://github.com/serenity-rs/serenity/commit/04b0be18b101186d618f9593fc8d2569ee845487
[c:0d6e019]: https://github.com/serenity-rs/serenity/commit/0d6e019c258a8f2e743bcab196acab50b01e3958
[c:10bbffe]: https://github.com/serenity-rs/serenity/commit/10bbffe9332edf8b8835d98cfffb8ec411162145
[c:10f7548]: https://github.com/serenity-rs/serenity/commit/10f7548d4d57864b599dd7a760d2609144a2ec63
[c:1ec1086]: https://github.com/serenity-rs/serenity/commit/1ec1086026971c903858128a8d38c5143f3f0f6f
[c:1f3a57e]: https://github.com/serenity-rs/serenity/commit/1f3a57eb6c0a1419614927d52bd3e798db36b043
[c:29480e5]: https://github.com/serenity-rs/serenity/commit/29480e5eeccc12afc0e9020373647786736aabc7
[c:2ef660e]: https://github.com/serenity-rs/serenity/commit/2ef660e34c4cca96ec30049e42c79e899c573be0
[c:2ff765b]: https://github.com/serenity-rs/serenity/commit/2ff765bbe74e2dc36a6c0c221c7ab06aac74462a
[c:305d200]: https://github.com/serenity-rs/serenity/commit/305d2008216b5351d9fdd357381027ea42f4740b
[c:3121f90]: https://github.com/serenity-rs/serenity/commit/3121f90a9f98e82fab48d62cf95cd316ae9f0496
[c:3a647e3]: https://github.com/serenity-rs/serenity/commit/3a647e3b7f6762fa6a078bc539e5b3e8012b37d4
[c:40c8248]: https://github.com/serenity-rs/serenity/commit/40c8248d107b3c6cad785502e6d619669aba6431
[c:4cf83d0]: https://github.com/serenity-rs/serenity/commit/4cf83d0d6b2a4fe156d3c54c06db4ce32293efb0
[c:4e4dcb1]: https://github.com/serenity-rs/serenity/commit/4e4dcb11586520f798c831956dc42778c0205386
[c:530ea76]: https://github.com/serenity-rs/serenity/commit/530ea76cfd05ffa64a826e6afa342860c730fd00
[c:55555b8]: https://github.com/serenity-rs/serenity/commit/55555b88dd44366e27d2c7cc02166995a3835a69
[c:5abc7d1]: https://github.com/serenity-rs/serenity/commit/5abc7d1d7fe7130e73e4848c6333627d9881cb9e
[c:5dab87b]: https://github.com/serenity-rs/serenity/commit/5dab87b0ff0097eb78abc1089c6a51ea05aa2273
[c:5b66ace]: https://github.com/serenity-rs/serenity/commit/5b66ace77b55c3d7272aab9b49db919c180ec33f
[c:5ffdcea]: https://github.com/serenity-rs/serenity/commit/5ffdceafcbc75947365004107e640783ec033335
[c:614402f]: https://github.com/serenity-rs/serenity/commit/614402f7b963a713bfa98bc5b1cfa968e8d6c103
[c:6ddfef8]: https://github.com/serenity-rs/serenity/commit/6ddfef8359a619be9a49be7b33b466724eed0ecb
[c:703d135]: https://github.com/serenity-rs/serenity/commit/703d13564f9081839eb77e4e4699d711b1de895a
[c:7937025]: https://github.com/serenity-rs/serenity/commit/7937025a484955cc8d74fb10004ba8b49dcc2bb0
[c:7b9764c]: https://github.com/serenity-rs/serenity/commit/7b9764cf1097b0620d871fabe67b5593f0cd4a4a
[c:7eac4d5]: https://github.com/serenity-rs/serenity/commit/7eac4d5fcf6c16db64e118de3d69825909979d5b
[c:8114a7a]: https://github.com/serenity-rs/serenity/commit/8114a7ace3ad51b9903a6017993aa526742bd72d
[c:8aefde0]: https://github.com/serenity-rs/serenity/commit/8aefde08465a050ad7bae12e6003fe514f43af5f
[c:8ce8234]: https://github.com/serenity-rs/serenity/commit/8ce82346846f235357b8dc53cb3ff399e70fcb4a
[c:93f453b]: https://github.com/serenity-rs/serenity/commit/93f453b07b9e8f813e6bfb0ddd2648a8e626d136
[c:9b2cd75]: https://github.com/serenity-rs/serenity/commit/9b2cd75baf1fa7ee063f47e966ee3f6566a6d45c
[c:9da7669]: https://github.com/serenity-rs/serenity/commit/9da766976929417c4b8f487f8ec05b6f8b3f43ef
[c:9e45642]: https://github.com/serenity-rs/serenity/commit/9e456427ccd496c4128bde841df0c0af7a262047
[c:9e56062]: https://github.com/serenity-rs/serenity/commit/9e560628deb1cf66e0c5029f41a79404fadffb40
[c:a9a2c27]: https://github.com/serenity-rs/serenity/commit/a9a2c27d7aefa6061dd9ca58a96c5ba617a78a6a
[c:a9e8626]: https://github.com/serenity-rs/serenity/commit/a9e8626c4cd642087f828c5b32481bee9e4d368b
[c:aeb89af]: https://github.com/serenity-rs/serenity/commit/aeb89af4eff59bb3ea9eb7623685bf7ad7520496
[c:b520ec7]: https://github.com/serenity-rs/serenity/commit/b520ec708c375e09838b9f25fd285790b856bb97
[c:bbfc8e2]: https://github.com/serenity-rs/serenity/commit/bbfc8e2d0250f41d5bf4230b6efb428419133de8
[c:bd4aa0a]: https://github.com/serenity-rs/serenity/commit/bd4aa0aabda4a2986e6145e3a793e8b2a391f8dd
[c:caeab28]: https://github.com/serenity-rs/serenity/commit/caeab28059d029a92b784f3b5ae1f79c412c8404
[c:ccd2506]: https://github.com/serenity-rs/serenity/commit/ccd250649665b1726b0ca852b2375c113da6ed57
[c:ce8da79]: https://github.com/serenity-rs/serenity/commit/ce8da793d3142cb001d9b155ff4224c15fe833ce
[c:d0d363f]: https://github.com/serenity-rs/serenity/commit/d0d363fb2a3475c68d40b02ec22ab728059fd55e
[c:d11d916]: https://github.com/serenity-rs/serenity/commit/d11d916a94b8a96fde218db4550d6c2428b4bc2a
[c:dd3744b]: https://github.com/serenity-rs/serenity/commit/dd3744b08887debba0d44fd0bceddef5f8ed1356
[c:e602630]: https://github.com/serenity-rs/serenity/commit/e6026308b33c80aa33f0001c89cd271cc5cb6687
[c:eae624e]: https://github.com/serenity-rs/serenity/commit/eae624e3f18681971a654c95624d917afe00695a
[c:f09b661]: https://github.com/serenity-rs/serenity/commit/f09b661be9085c7525a6c9f6929b50deebffae9b
[c:f0f06b7]: https://github.com/serenity-rs/serenity/commit/f0f06b7d3b890d2ddcb84e00b3f62e195da80090

[c:0324e01]: https://github.com/serenity-rs/serenity/commit/0324e011f1ea0eed0709c92fe86319c812a42206
[c:08a7110]: https://github.com/serenity-rs/serenity/commit/08a71106748e356d2618e48d8797e6da60d7eb54
[c:0e1e8fb]: https://github.com/serenity-rs/serenity/commit/0e1e8fbbe564c23530a709a7ec407b08f63944e2
[c:1162e68]: https://github.com/serenity-rs/serenity/commit/1162e686592f23f4dc5ad509051858e453c82d1f
[c:152fe3d]: https://github.com/serenity-rs/serenity/commit/152fe3ded89c71580a9ab9d3bb05587abee97e72
[c:23c5398]: https://github.com/serenity-rs/serenity/commit/23c5398d8c6b0a3e5ad28cb43fadd48002195d3c
[c:2603063]: https://github.com/serenity-rs/serenity/commit/26030630bee7750c047b155708a62a03a6a5edf3
[c:32c3bed]: https://github.com/serenity-rs/serenity/commit/32c3bed1afa65d14a93d4e3d4e9e8471cfd77ced
[c:457a17e]: https://github.com/serenity-rs/serenity/commit/457a17e059395aab3d1a23bd1cfe6e01ea0b5a61
[c:4a24c90]: https://github.com/serenity-rs/serenity/commit/4a24c9004a1aac31eb552e5cdef6e986a6e903b3
[c:5ba521b]: https://github.com/serenity-rs/serenity/commit/5ba521b4e8bb5ff10bd83303436cd454f27c93ab
[c:6346975]: https://github.com/serenity-rs/serenity/commit/63469753a960431e5edacf520589493121f0e62d
[c:89a18aa]: https://github.com/serenity-rs/serenity/commit/89a18aa919d8c08cf9fba9a98ebe32c9fd59d5d4
[c:63fe032]: https://github.com/serenity-rs/serenity/commit/63fe032eaa29bbee51b6efd3f19392cc41b992e0
[c:a80aab2]: https://github.com/serenity-rs/serenity/commit/a80aab264b3d81c2457eb6bef755a5b0d1a88e4b
[c:af7f176]: https://github.com/serenity-rs/serenity/commit/af7f176101aea9bcf43551fbcd3261469bbc0b43
[c:c659bbd]: https://github.com/serenity-rs/serenity/commit/c659bbd756391fc26e9e862937ef77113ee892ed
[c:cc6b567]: https://github.com/serenity-rs/serenity/commit/cc6b5677d8e5ba61a53f7aeae26b66207db9fd3d
[c:ff9edc0]: https://github.com/serenity-rs/serenity/commit/ff9edc063fd987b9ba14c8d211ed26f6b480f751

[c:36d7a54]: https://github.com/serenity-rs/serenity/commit/36d7a541ec53051007fee74f621919bea721c8f2
[c:40db3c0]: https://github.com/serenity-rs/serenity/commit/40db3c024dd5e13c5a02e10ab4f7a7e9c31a6876
[c:42063a2]: https://github.com/serenity-rs/serenity/commit/42063a240682f0cfa93b7dce9fcb79b2dfe7ef99
[c:526c366]: https://github.com/serenity-rs/serenity/commit/526c366eb355ff2cdfd5769621448d35926fe123
[c:64dcced]: https://github.com/serenity-rs/serenity/commit/64dccedc999b6f2cdf4b60830d4c5fb1126bd37c
[c:7b6b601]: https://github.com/serenity-rs/serenity/commit/7b6b6016078c3492d2873d3eed0ddb39323079ab
[c:7f9c01e]: https://github.com/serenity-rs/serenity/commit/7f9c01e4e4d413979e2e66eea1d3cdf9157c4dc5
[c:83a0c85]: https://github.com/serenity-rs/serenity/commit/83a0c85b0bf87cb4272b5d6e189d139fc31a6d23
[c:a481df6]: https://github.com/serenity-rs/serenity/commit/a481df6e67d83216617a40d07991ba8ea04d0075
[c:e546fa2]: https://github.com/serenity-rs/serenity/commit/e546fa2a32a05a9bbc351b9aa789233ee71e88f0
[c:fd77a91]: https://github.com/serenity-rs/serenity/commit/fd77a91f2ba7c782f3e0e67ecee19df17bb31e26

[c:003dc2e]: https://github.com/serenity-rs/serenity/commit/003dc2eed0f09cd214373f1581b7794d1483a689
[c:02dc506]: https://github.com/serenity-rs/serenity/commit/02dc5064d9402f73ef514c9b8ffa318f5d4235ff
[c:0d779ba]: https://github.com/serenity-rs/serenity/commit/0d779ba85ad43eb7e95be3844ad3fcd74335b47c
[c:21fe999]: https://github.com/serenity-rs/serenity/commit/21fe999d23cb0e4e76812537b48edadeab5a1540
[c:217e1c6]: https://github.com/serenity-rs/serenity/commit/217e1c6ebd9577fa5a538b4ecd3c1303ee034462
[c:24d2233]: https://github.com/serenity-rs/serenity/commit/24d2233ba583221b35eca02cf321e7a4a1adf76d
[c:2791ed7]: https://github.com/serenity-rs/serenity/commit/2791ed78df18f2721264352611b1ba26b3077196
[c:2937792]: https://github.com/serenity-rs/serenity/commit/29377922d3d79848efcb8d3bd0fbd52c21e81c5d
[c:2ab714f]: https://github.com/serenity-rs/serenity/commit/2ab714f9c3683db83eba1f6fe7bb3887a47d4f3f
[c:2e1eb4c]: https://github.com/serenity-rs/serenity/commit/2e1eb4c35b488ac68a33e76502d0c8f56a72c4b6
[c:3d67a4e]: https://github.com/serenity-rs/serenity/commit/3d67a4e2cd33b17747c7499e07d0a0e05fe73253
[c:5166a0a]: https://github.com/serenity-rs/serenity/commit/5166a0a81bac494b0e337d00f946a85d65e4bbbd
[c:77c399b]: https://github.com/serenity-rs/serenity/commit/77c399ba7b3bff0cf3180df5edad5d6ff6dcb10d
[c:7d162b9]: https://github.com/serenity-rs/serenity/commit/7d162b96686a56eed052984c18f22f54ad76f780
[c:7e0d908]: https://github.com/serenity-rs/serenity/commit/7e0d9082fe0262c7b4c4668ca1e1208dffa4796f
[c:7f09642]: https://github.com/serenity-rs/serenity/commit/7f09642fa66517fc983f63bb2e414638382a6512
[c:80dfcb0]: https://github.com/serenity-rs/serenity/commit/80dfcb0539c88e9434dc4875d5af55df1aafa725
[c:82e21a6]: https://github.com/serenity-rs/serenity/commit/82e21a61ae17d8466834f486108d1ace2791efc4
[c:9baf167]: https://github.com/serenity-rs/serenity/commit/9baf1675b0d1837fe010cfbadac8e80fd9d53898
[c:a4cc582]: https://github.com/serenity-rs/serenity/commit/a4cc5821c0ca194d321141985cfaf30241f54acf
[c:b71d99f]: https://github.com/serenity-rs/serenity/commit/b71d99fde84135fa66f73c4817d340ffbe8bddae
[c:b9fa745]: https://github.com/serenity-rs/serenity/commit/b9fa7457c2011130b28f5eca063f88bdf72be544
[c:bd195de]: https://github.com/serenity-rs/serenity/commit/bd195dea4422f516b727868209116ff868e3941b
[c:beebff5]: https://github.com/serenity-rs/serenity/commit/beebff50eb4f447f41162299fde81cb6fa9b336d
[c:c6a5fe4]: https://github.com/serenity-rs/serenity/commit/c6a5fe4ef9dc07b9b14f742440a35ba6ea02058b
[c:d0ae9bb]: https://github.com/serenity-rs/serenity/commit/d0ae9bba89d5e66129da7bed7faf39abfd4fb17d
[c:d8c9d89]: https://github.com/serenity-rs/serenity/commit/d8c9d89c80ffb0ae95b91c4dcc593fd70fa976d8
[c:dbfc06e]: https://github.com/serenity-rs/serenity/commit/dbfc06e9c6df506839fb178eaeb9db704aefd357
[c:e506e9f]: https://github.com/serenity-rs/serenity/commit/e506e9f30584d0fd67bf28a33b1033c4f1e5cd8b
[c:e5bcee7]: https://github.com/serenity-rs/serenity/commit/e5bcee7b2c2d2ff873982c6a3bf39ab16ec4e1e3
[c:e814e9a]: https://github.com/serenity-rs/serenity/commit/e814e9aa9b6117defa4f885ef0027e61706112d5
[c:f115c17]: https://github.com/serenity-rs/serenity/commit/f115c177def1a35fc532c896713a187bb468088e
[c:fdcf44e]: https://github.com/serenity-rs/serenity/commit/fdcf44e1463e708cd8b612c183e302db9af0febd
[c:ffc5ea1]: https://github.com/serenity-rs/serenity/commit/ffc5ea1da38cb7d9c302fa8d5614253c1f361311

[c:03a7e3e]: https://github.com/serenity-rs/serenity/commit/03a7e3e1d82ca667ca065367d2cf21b847f984ac
[c:13b0de1]: https://github.com/serenity-rs/serenity/commit/13b0de121eda30e59849fd442c8a0926a63df2b8
[c:27c83e8]: https://github.com/serenity-rs/serenity/commit/27c83e8ef0def0a62e8a5ce5bfd4849892749c83
[c:324a288]: https://github.com/serenity-rs/serenity/commit/324a288fbb0dd7d135aa9aab876cf39dabb6a02e
[c:5a0b8a6]: https://github.com/serenity-rs/serenity/commit/5a0b8a68c133c3093260a5aeb08b02eb3595c18d
[c:55fa37a]: https://github.com/serenity-rs/serenity/commit/55fa37ade187aa68ef3eec519d22767920aae4ab
[c:76bcf7d]: https://github.com/serenity-rs/serenity/commit/76bcf7dcef91fd2658fb3348acf6df0ecc33fcdf
[c:7912f23]: https://github.com/serenity-rs/serenity/commit/7912f23bed7ddc540c46aee0ecd64c6b60daa0f4
[c:8578d5f]: https://github.com/serenity-rs/serenity/commit/8578d5fe6e3bdc2842cda9417c242169f93b1a99
[c:92c91b8]: https://github.com/serenity-rs/serenity/commit/92c91b81490b621de4519e0d87830dbce53dd689
[c:ab1f11a]: https://github.com/serenity-rs/serenity/commit/ab1f11a37d64166c08f833042d7b3bcde2ea586d
[c:aba1ba6]: https://github.com/serenity-rs/serenity/commit/aba1ba67dc78a0c14e5de3c8ac650829e436e96f
[c:b9a7e50]: https://github.com/serenity-rs/serenity/commit/b9a7e50579718a20e60a19f0c0d410661ee3e77a
[c:ce2952a]: https://github.com/serenity-rs/serenity/commit/ce2952ad0d783b4a256171e48602c6caf1125c61
[c:d193975]: https://github.com/serenity-rs/serenity/commit/d1939756f6bf4b4bb3c60fbb81a397c218492d62
[c:d240074]: https://github.com/serenity-rs/serenity/commit/d2400742f657d9f8c432a440810d49e63339f5aa
[c:e4612ac]: https://github.com/serenity-rs/serenity/commit/e4612acf58dc42fdc32094426c14274bd61203dd
[c:eee3168]: https://github.com/serenity-rs/serenity/commit/eee3168b4ed266001571abe4e1a6ae4ef06b93e0

[c:031fc92]: https://github.com/serenity-rs/serenity/commit/031fc92e02c314cce9fc8febcc7900fa2d885941
[c:032c5a7]: https://github.com/serenity-rs/serenity/commit/032c5a75620c3ff185a749113d93fb3051b38acb
[c:0525ede]: https://github.com/serenity-rs/serenity/commit/0525ede086ccffa5781c9a1876a368ac3531813e
[c:05f6ed4]: https://github.com/serenity-rs/serenity/commit/05f6ed429aeac1920307ea692fef122bbd2dffff
[c:0881e18]: https://github.com/serenity-rs/serenity/commit/0881e18c07113cc7b2f6cec38cadcb1ea03dda12
[c:08db9fa]: https://github.com/serenity-rs/serenity/commit/08db9fa2adef141743ab9681c46dd91489278063
[c:08febb0]: https://github.com/serenity-rs/serenity/commit/08febb0d8d95bbbae9861130a756e842eae40eef
[c:0aa55a2]: https://github.com/serenity-rs/serenity/commit/0aa55a2b9b757321d5b8bb9e512813aa9d0a62ca
[c:0b1f684]: https://github.com/serenity-rs/serenity/commit/0b1f684106031657d6bf581206c06e5443d06da9
[c:10c56a9]: https://github.com/serenity-rs/serenity/commit/10c56a9385c0d410241d33525f8f49242daced2d
[c:114e43a]: https://github.com/serenity-rs/serenity/commit/114e43a3d0079072593588ad7b2de9f8588e041d
[c:12317b9]: https://github.com/serenity-rs/serenity/commit/12317b98a0cc145e717e9da3cdbe8bc1ff8fc4f1
[c:141bbfc]: https://github.com/serenity-rs/serenity/commit/141bbfcb1e4843eaeb55bf07e10e2c0aa4bbe1e4
[c:143fddd]: https://github.com/serenity-rs/serenity/commit/143fddd83f1fc93c070e36bf31906d2631c68f97
[c:14b9222]: https://github.com/serenity-rs/serenity/commit/14b92221184fcaca0f4a60a3b31d5a9470b14b1f
[c:1735e57]: https://github.com/serenity-rs/serenity/commit/1735e57ea57bcd4d75b73ac9398e13bee5198c5b
[c:1fa83f7]: https://github.com/serenity-rs/serenity/commit/1fa83f73577d926664518d849bc26e46087611b4
[c:1fd652b]: https://github.com/serenity-rs/serenity/commit/1fd652be41f4de96d26efaf20055cf7a80e42bf1
[c:2032a40]: https://github.com/serenity-rs/serenity/commit/2032a402c387b1310f2ae62621f3e07c86b76aef
[c:222382c]: https://github.com/serenity-rs/serenity/commit/222382ca48cb9786aaf5d0b5fc16958e482e7c5f
[c:25d79ac]: https://github.com/serenity-rs/serenity/commit/25d79ac7e07654dbf77166d46db33d186faf9885
[c:25dddb6]: https://github.com/serenity-rs/serenity/commit/25dddb6695109eeead9e19593cb85a22096c2c7a
[c:26fe139]: https://github.com/serenity-rs/serenity/commit/26fe139363a847542bbe609fe4d15accbf4fef14
[c:2abeea5]: https://github.com/serenity-rs/serenity/commit/2abeea53745b5ddfce33d9e1160c682888850344
[c:2c9b682]: https://github.com/serenity-rs/serenity/commit/2c9b6824a7bf6231a08d5c5465c1db5417ea5d8a
[c:2d23d8b]: https://github.com/serenity-rs/serenity/commit/2d23d8b50386e38fece6987286bd0b3d56d1cada
[c:2edba81]: https://github.com/serenity-rs/serenity/commit/2edba816f6901db46e7fc0f6664058033a56d5e7
[c:39a1435]: https://github.com/serenity-rs/serenity/commit/39a1435be57335e99039ddea731032221eb6d96e
[c:3a0c890]: https://github.com/serenity-rs/serenity/commit/3a0c8908ce837f6fe64f865a1a7a9de63cbd237c
[c:3a4cb18]: https://github.com/serenity-rs/serenity/commit/3a4cb18be8ca33d507cbfc88fec79b2a6c5d8bfc
[c:3b9f0f8]: https://github.com/serenity-rs/serenity/commit/3b9f0f8501f7581d710e3f7ebbfcd3176d14a9b1
[c:3ca7e15]: https://github.com/serenity-rs/serenity/commit/3ca7e15e55de640200edb3898a33b838946a506c
[c:3d24033]: https://github.com/serenity-rs/serenity/commit/3d24033f550623f78ad71a37f6ec847e7d0a2c78
[c:3e14067]: https://github.com/serenity-rs/serenity/commit/3e1406764cf655694fef0e04e43324d58499bba3
[c:40c5c12]: https://github.com/serenity-rs/serenity/commit/40c5c12373e2a2c7acd3501f43c79f9bf3e7c685
[c:45c1f27]: https://github.com/serenity-rs/serenity/commit/45c1f27edbeedcb30aa3e9daa78eba44817f7260
[c:470f366]: https://github.com/serenity-rs/serenity/commit/470f366000b3d3f8080e02b185f0f7fef592a736
[c:4bd223a]: https://github.com/serenity-rs/serenity/commit/4bd223a88cfacc335814ef3ddc0c1aaa88fc05f7
[c:4e20277]: https://github.com/serenity-rs/serenity/commit/4e20277de4f164705074ba41199e4530332056b3
[c:524b8f8]: https://github.com/serenity-rs/serenity/commit/524b8f8ab5153e20ad86be2df7fba6bbed159b7c
[c:551f166]: https://github.com/serenity-rs/serenity/commit/551f16673fe775a80a1da788fd7e1db20f6eae29
[c:5d4301b]: https://github.com/serenity-rs/serenity/commit/5d4301bbd2aaa4abe47fbbc2a7a2853ba9b728f2
[c:60613ef]: https://github.com/serenity-rs/serenity/commit/60613ef696b093dbbac3a4e9e033c226c5d358ea
[c:612e973]: https://github.com/serenity-rs/serenity/commit/612e973f286ba6b711824333551b07b88df6740c
[c:62647f5]: https://github.com/serenity-rs/serenity/commit/62647f53fd01a670cf5ad01c7d0a68cb69bf92cf
[c:65233ad]: https://github.com/serenity-rs/serenity/commit/65233ad6f3d002f72942aaf811514fa9d29ad068
[c:65e3279]: https://github.com/serenity-rs/serenity/commit/65e3279ce7b3c4807e8b1310551e9493d3868b94
[c:68156c9]: https://github.com/serenity-rs/serenity/commit/68156c9ce93edc86a70f50cf10986615cfb9f93a
[c:7566f32]: https://github.com/serenity-rs/serenity/commit/7566f32c194bc4e62e89adc57bfb6104cc99458e
[c:78c6df9]: https://github.com/serenity-rs/serenity/commit/78c6df9ed3640c097ef088519ec99a6a01796bfc
[c:7a5aa3c]: https://github.com/serenity-rs/serenity/commit/7a5aa3c57951ee5c7267fabf38f2729b06629b34
[c:7c911d5]: https://github.com/serenity-rs/serenity/commit/7c911d57eb3db3ac51cfc51cf9b3a5884e0f4ea3
[c:7cf1e52]: https://github.com/serenity-rs/serenity/commit/7cf1e523f0c0bee1b7ec11ff6e6565c68f9d173e
[c:7e46d8f]: https://github.com/serenity-rs/serenity/commit/7e46d8f3ac5a968df9a05f8f0006522ad14891ef
[c:82b87f1]: https://github.com/serenity-rs/serenity/commit/82b87f196425ff8553bc9dcb84ddac9764b971e4
[c:84706f1]: https://github.com/serenity-rs/serenity/commit/84706f1fc0a934a851d57f524697da5b177b9be8
[c:84ff27b]: https://github.com/serenity-rs/serenity/commit/84ff27be8455d9ec885b190150a2b592cffdf2a2
[c:85d7d5f]: https://github.com/serenity-rs/serenity/commit/85d7d5f6a6df9841659bc7ad8e392f31c1aae46c
[c:8c83866]: https://github.com/serenity-rs/serenity/commit/8c83866748bf7bf339df9a234c3297c8008ffa46
[c:8c85664]: https://github.com/serenity-rs/serenity/commit/8c85664a94f7439ab4bc3a132f313a9e26d94fe7
[c:8c9baa7]: https://github.com/serenity-rs/serenity/commit/8c9baa74c2716d62c405d909bb453ffea636c94d
[c:8d68503]: https://github.com/serenity-rs/serenity/commit/8d685039d89fd2130e88c9a9e0421492a3e99da6
[c:91c8ec4]: https://github.com/serenity-rs/serenity/commit/91c8ec4ae7540956a714ce9584074538b45467cc
[c:9232b8f]: https://github.com/serenity-rs/serenity/commit/9232b8f065deb4637a74e7f85ab617bb527c51be
[c:934eb3a]: https://github.com/serenity-rs/serenity/commit/934eb3aa0b1f9c0aaad003627bd65932114654c1
[c:93e0a42]: https://github.com/serenity-rs/serenity/commit/93e0a4215c915b98cf433ac6d0bcfbc60f0168ec
[c:9908999]: https://github.com/serenity-rs/serenity/commit/9908999a6bae1585bb70b7814f13b49bf99b6c32
[c:99d17d2]: https://github.com/serenity-rs/serenity/commit/99d17d2975143b0d588c969f7ae6f8d11e62a9e1
[c:9aaa555]: https://github.com/serenity-rs/serenity/commit/9aaa55542d6bee1e953a080612ee6af765b8a5a5
[c:9aad1aa]: https://github.com/serenity-rs/serenity/commit/9aad1aa375168d6131cb6f68d6998b2af6fb00a3
[c:9da642a]: https://github.com/serenity-rs/serenity/commit/9da642a5bea8b4ac2d291058ad22e4cbe27b1f94
[c:a17fea7]: https://github.com/serenity-rs/serenity/commit/a17fea783cd91b2adcd1330b7038cf3ca2d79a85
[c:a359f77]: https://github.com/serenity-rs/serenity/commit/a359f77d1fd03def94fc08367132a616ec2ea599
[c:a7b67df]: https://github.com/serenity-rs/serenity/commit/a7b67df6d77f5acacf83710807b231866397d551
[c:a96be90]: https://github.com/serenity-rs/serenity/commit/a96be90385b58a9098b918e0fd17288d89229752
[c:aad4744]: https://github.com/serenity-rs/serenity/commit/aad4744fb751e3e1147f085781323172755d4ef2
[c:ad0dcb3]: https://github.com/serenity-rs/serenity/commit/ad0dcb305d959a2bb273a63dd2dd1b5594f5c49d
[c:ae50886]: https://github.com/serenity-rs/serenity/commit/ae50886a1a8f69c114d9e99a0913c878aaaaabe2
[c:b146501]: https://github.com/serenity-rs/serenity/commit/b14650193342297746f985f8794e4b93ceeac52b
[c:b19b031]: https://github.com/serenity-rs/serenity/commit/b19b031a5052a268f323a116403ea66ca71ea575
[c:b215457]: https://github.com/serenity-rs/serenity/commit/b215457ab46c9d10bf47300d6525f9a2641d3b17
[c:b328b3e]: https://github.com/serenity-rs/serenity/commit/b328b3e09b0095abb54530dc4d50db6b4e3e1779
[c:b52eb9f]: https://github.com/serenity-rs/serenity/commit/b52eb9f108fb7af182e0cf29259cd4d522ed7f89
[c:b60d037]: https://github.com/serenity-rs/serenity/commit/b60d0378548a53ffefda17aab403c073d3438cf6
[c:b62dfd4]: https://github.com/serenity-rs/serenity/commit/b62dfd431668b4bdb6021d21120da05d17ab77d5
[c:b7542f4]: https://github.com/serenity-rs/serenity/commit/b7542f44306fedb7f79f7b8cd5c8d6afd6ccb7ad
[c:b8efeaf]: https://github.com/serenity-rs/serenity/commit/b8efeaf5e920cbfc775cdee70f23aa41ab7b9dd5
[c:bcd16dd]: https://github.com/serenity-rs/serenity/commit/bcd16dddb8cc3086a13524c79676f3a8bebbc524
[c:be43836]: https://github.com/serenity-rs/serenity/commit/be43836839a31714f58e3ffe81dd4b0aeab2ab59
[c:c3aa63f]: https://github.com/serenity-rs/serenity/commit/c3aa63faee8b3ae6d5126aa27a74876766c61e4c
[c:d1113c0]: https://github.com/serenity-rs/serenity/commit/d1113c07f061149b5d090c1f15c3c03806f8b0cf
[c:d264cc3]: https://github.com/serenity-rs/serenity/commit/d264cc3496f56d2757cf9c1735d5d8a68577c2f5
[c:d5a9aa8]: https://github.com/serenity-rs/serenity/commit/d5a9aa8b1e0a94094ef5bda98a76dd259a6e7a3a
[c:d90b90c]: https://github.com/serenity-rs/serenity/commit/d90b90c7f3d8a368acbab46150f199866562229a
[c:e02a842]: https://github.com/serenity-rs/serenity/commit/e02a842fb76b1e591287395ac223cc1c04913820
[c:e0e7617]: https://github.com/serenity-rs/serenity/commit/e0e76173f63b6071b9df3ff8f53371b4b6c4ee1e
[c:e5a6f3a]: https://github.com/serenity-rs/serenity/commit/e5a6f3a8ed367bd3d780fd23a0a27f8a80567879
[c:e611776]: https://github.com/serenity-rs/serenity/commit/e6117760e3c020ed41d643a8b34d7bfeb62d3bfa
[c:e678883]: https://github.com/serenity-rs/serenity/commit/e6788838556d13d4a4f19253ce297ca2f72168ee
[c:e748d1f]: https://github.com/serenity-rs/serenity/commit/e748d1ff80dbbeb82b23f8ac9fec9cf8c7e4a69e
[c:eb9e8df]: https://github.com/serenity-rs/serenity/commit/eb9e8dfbc9d778de405d7369579d90c49a2bf90c
[c:ee207b3]: https://github.com/serenity-rs/serenity/commit/ee207b331d571d5afb5c35c8f119937d0196663a
[c:ee2bbca]: https://github.com/serenity-rs/serenity/commit/ee2bbcaa0b62c09a6c0df352bfddcbf99d06e61d
[c:f0a56f4]: https://github.com/serenity-rs/serenity/commit/f0a56f46ce7ef6c2a65d64d8953ac43e0b7b5b4d
[c:f0ee805]: https://github.com/serenity-rs/serenity/commit/f0ee805a8ee20b6180b9f54d5096a8a9b73b4be2
[c:f10b9d7]: https://github.com/serenity-rs/serenity/commit/f10b9d77f0b94864fa20688e3c99de6cec7ca6f9
[c:f26dad8]: https://github.com/serenity-rs/serenity/commit/f26dad86aea82070aab9cc081f50d0144ee4c778
[c:f2c21ef]: https://github.com/serenity-rs/serenity/commit/f2c21ef5b15ef1f345cdc30f4b793e55905f15f4
[c:f2fa349]: https://github.com/serenity-rs/serenity/commit/f2fa349d831c1db59993341284049ffbd56dde3b
[c:f61816c]: https://github.com/serenity-rs/serenity/commit/f61816ca141add5024e36e073764b7c824872ca4
[c:fd19446]: https://github.com/serenity-rs/serenity/commit/fd19446fcc4c7ad2c9f634c97fa1c056440a6abd

[c:52403a5]: https://github.com/serenity-rs/serenity/commit/52403a5084ed7f0589bde3351844907a92de2d62
[c:795eaa1]: https://github.com/serenity-rs/serenity/commit/795eaa15bca61116fbde9c2482c765f2d47a7696

[c:77f462e]: https://github.com/serenity-rs/serenity/commit/77f462ea2044ef7d2d12fd1289ea75a6a33cb5dd

[c:1b7101f]: https://github.com/serenity-rs/serenity/commit/1b7101fe71335c0e18bf855c0703acc23d87e427
[c:2ba4d03]: https://github.com/serenity-rs/serenity/commit/2ba4d03f15d57d9f0fb1cc4d4f4355ebbc483d0a
[c:3be6e2e]: https://github.com/serenity-rs/serenity/commit/3be6e2e28b0c3e9baaef19f405c463e3a41fed25
[c:800e58f]: https://github.com/serenity-rs/serenity/commit/800e58f4603ce99ab69569b30cbec756301a6a63
[c:c99091d]: https://github.com/serenity-rs/serenity/commit/c99091d241f240c6b76ac969655a8ec4423aaf80
[c:d3eddc6]: https://github.com/serenity-rs/serenity/commit/d3eddc68e07bbc31e2043577cbf48741f0547ed3
[c:dcac271]: https://github.com/serenity-rs/serenity/commit/dcac27168915b4f22745950ec0ef0c0af696774e
[c:e219a6a]: https://github.com/serenity-rs/serenity/commit/e219a6a9d6a890b008fc390a909ae504a0c1a329

[c:002ce3a]: https://github.com/serenity-rs/serenity/commit/002ce3aa272fa51b84e820f12db39cb87a461a83
[c:022e35d]: https://github.com/serenity-rs/serenity/commit/022e35d5b12322bd77bbe74a1a3b2ad319977390
[c:05f158f]: https://github.com/serenity-rs/serenity/commit/05f158fc89f2adc82e31cf4b93706dc7d25e11d8
[c:08d390c]: https://github.com/serenity-rs/serenity/commit/08d390c19f187986fd2856fe5cbb9035a0877e0f
[c:09a8a44]: https://github.com/serenity-rs/serenity/commit/09a8a444f5bcefaee8b83dc129a3cea2de8792f9
[c:0d1c0f1]: https://github.com/serenity-rs/serenity/commit/0d1c0f1356fd3a891232498c2230d0bb4d2ed4ff
[c:0df77b9]: https://github.com/serenity-rs/serenity/commit/0df77b933ff5e98725252116069afad2dec9f89b
[c:0ed1972]: https://github.com/serenity-rs/serenity/commit/0ed19727debf28a8aa0818b44713090e97dd6eee
[c:11b85ca]: https://github.com/serenity-rs/serenity/commit/11b85ca6799b9984481119851f983d8e3c84cdc0
[c:1b167b5]: https://github.com/serenity-rs/serenity/commit/1b167b5496ce816cbcacb0e4f6e63399dffaa25c
[c:1bf4d9c]: https://github.com/serenity-rs/serenity/commit/1bf4d9cb9823dca8c4bb77147c66eac2d53f609f
[c:1d4ecb2]: https://github.com/serenity-rs/serenity/commit/1d4ecb2f13258d286378c44d59c2ee4b1c68349d
[c:21e194b]: https://github.com/serenity-rs/serenity/commit/21e194bffc37f396f007d390170f5b60e22f5d02
[c:3b2c246]: https://github.com/serenity-rs/serenity/commit/3b2c2462cb34b5ae5190ebc4a9e04968dc8d5335
[c:483b069]: https://github.com/serenity-rs/serenity/commit/483b069cc0c821ec673ac475b168809e3a41525a
[c:55167c3]: https://github.com/serenity-rs/serenity/commit/55167c300598536a852b3596fcf1c420aeb96c3a
[c:683691f]: https://github.com/serenity-rs/serenity/commit/683691f762bbf58e3abf3bc67381e18112f5c8ad
[c:6b9dcf5]: https://github.com/serenity-rs/serenity/commit/6b9dcf5272458499c1caef544cb82d5a8624258b
[c:71f709d]: https://github.com/serenity-rs/serenity/commit/71f709d0aceedb6d3091d0c28c9535e281270f71
[c:7945094]: https://github.com/serenity-rs/serenity/commit/794509421f21bee528e582a7b109d6a99284510a
[c:7befcd5]: https://github.com/serenity-rs/serenity/commit/7befcd5caa9ccdf44d90ecc12014c335b1bd2be7
[c:8109619]: https://github.com/serenity-rs/serenity/commit/8109619184867fc843a1e73d18d37726a34f7fbf
[c:8565fa2]: https://github.com/serenity-rs/serenity/commit/8565fa2cb356cf8cbccfeb09828c9d136ad3d614
[c:8572943]: https://github.com/serenity-rs/serenity/commit/857294358d5f3029850dc79c174b831c0b0c161c
[c:86d8bdd]: https://github.com/serenity-rs/serenity/commit/86d8bddff3e3242186d0c2607b34771e5422ba5b
[c:917dd30]: https://github.com/serenity-rs/serenity/commit/917dd3071dc8a145b9c379cb3a8a84731c690340
[c:9b0c053]: https://github.com/serenity-rs/serenity/commit/9b0c053725e04c60eb7ddcfeb847be4189b3dbf6
[c:b3aa441]: https://github.com/serenity-rs/serenity/commit/b3aa441c2d61ba324396deaf70f2c5818fd3f528
[c:c98cae4]: https://github.com/serenity-rs/serenity/commit/c98cae4e838147eaa077bbc68ffebf8834ff7b6b
[c:cf40386]: https://github.com/serenity-rs/serenity/commit/cf403867400110f446720fc20fad6781cf8c6b13
[c:d7621aa]: https://github.com/serenity-rs/serenity/commit/d7621aa4dfb2a3dea22e7848eb97e2b4cc1ade14

[c:005437f]: https://github.com/serenity-rs/serenity/commit/005437f56869e846ff677b6516605def0c4de7bc
[c:0186754]: https://github.com/serenity-rs/serenity/commit/01867549709ef73ee09ed442e1d5ea938fd7f74d
[c:0240717]: https://github.com/serenity-rs/serenity/commit/02407175e463b2b75295364d6b0e182fe34966ed
[c:03b6d78]: https://github.com/serenity-rs/serenity/commit/03b6d78885b3a59ffa781ded3682c2dd24e65aa7
[c:05162aa]: https://github.com/serenity-rs/serenity/commit/05162aa18aa737c05fbc13917fed1c8c218064d5
[c:051d23d]: https://github.com/serenity-rs/serenity/commit/051d23d60d4898d331d046861035165bf2e6cd23
[c:069df4f]: https://github.com/serenity-rs/serenity/commit/069df4f85d8c462df58c1fce00595462f2825337
[c:078947e]: https://github.com/serenity-rs/serenity/commit/078947edc2b7036b2a0b49afc3cc54b12a39af18
[c:0810ab7]: https://github.com/serenity-rs/serenity/commit/0810ab7a6aa37ca684b10c22dde8f0e03d3f8ea2
[c:092f288]: https://github.com/serenity-rs/serenity/commit/092f288fdd22ae39b019e61a6f12420b6ca3b67c
[c:0d6965f]: https://github.com/serenity-rs/serenity/commit/0d6965f647396c84b2570e92b63244c3afaea863
[c:106a4d5]: https://github.com/serenity-rs/serenity/commit/106a4d5f8ff22a829a9486ce88fa8326184828fa
[c:125c1b8]: https://github.com/serenity-rs/serenity/commit/125c1b8feff65ed86136ca0c3b75cdfa073aefc3
[c:14fd41b]: https://github.com/serenity-rs/serenity/commit/14fd41b0d62ab441b6600028792641d813f09cd8
[c:16a5828]: https://github.com/serenity-rs/serenity/commit/16a5828394c21baf799366136f5d48e20447a49e
[c:192ac8a]: https://github.com/serenity-rs/serenity/commit/192ac8aec0afb33055352ed6e6838c506cbbbf8c
[c:1a08904]: https://github.com/serenity-rs/serenity/commit/1a089048138e85607bd298ebc07e30f57fb4ac53
[c:1ab8b31]: https://github.com/serenity-rs/serenity/commit/1ab8b31a19c6782b867b518c01bad9fbbdd06241
[c:1fad3dd]: https://github.com/serenity-rs/serenity/commit/1fad3dd60a0a9a0959f6e7e55896bef151bf3e9d
[c:25d4931]: https://github.com/serenity-rs/serenity/commit/25d49316133e2a8b7c4b26d3b6a44efdf5ad8834
[c:25e91da]: https://github.com/serenity-rs/serenity/commit/25e91dabd2380bd8fd98acbb7cb220dd90d238bd
[c:266411c]: https://github.com/serenity-rs/serenity/commit/266411cd6fc9ee96310da52c68264f303bcf5938
[c:26919cf]: https://github.com/serenity-rs/serenity/commit/26919cf9aad1d7bc5f0f8042b4caf6bfcddbd7d8
[c:29ee627]: https://github.com/serenity-rs/serenity/commit/29ee627207e0c2a0d3f5310ac00d90b232d910c0
[c:2b053ea]: https://github.com/serenity-rs/serenity/commit/2b053ea007d6ca9cc820cb910597e8b5dad89d70
[c:2fb12e2]: https://github.com/serenity-rs/serenity/commit/2fb12e2b3782fff211a41cb27cd316afc4320a7b
[c:3017f6d]: https://github.com/serenity-rs/serenity/commit/3017f6dbc02e6189c69491993e828e2a7595cbed
[c:32de2cb]: https://github.com/serenity-rs/serenity/commit/32de2cb941e8d4fdffde7b8b82599fcd78ab4c2f
[c:3582691]: https://github.com/serenity-rs/serenity/commit/35826915a174c7f3e5d82bbc320d3238ae308d8c
[c:3c2716b]: https://github.com/serenity-rs/serenity/commit/3c2716bbaeb71eca8cb2c7fca0dfd0b00cd34ba5
[c:3db42c9]: https://github.com/serenity-rs/serenity/commit/3db42c96c98fdd6d332347767cb1c276858da98b
[c:3e0b103]: https://github.com/serenity-rs/serenity/commit/3e0b1032d80a1847558a752e8316d97f9ae58f04
[c:40031d9]: https://github.com/serenity-rs/serenity/commit/40031d9ec55b1a4dd6e350a7566ea230751a54ed
[c:420f9bd]: https://github.com/serenity-rs/serenity/commit/420f9bdaa5a5022ff1d769f1d44a689a6fea12a4
[c:421c709]: https://github.com/serenity-rs/serenity/commit/421c709bbd706d4f04453baacf0ec6a88759f8cd
[c:428cbb9]: https://github.com/serenity-rs/serenity/commit/428cbb94de239e87d3258891591e1464cb9d2e06
[c:4532e4a]: https://github.com/serenity-rs/serenity/commit/4532e4a1e87d7b4f09446b1f10db178931eb314a
[c:45d72ef]: https://github.com/serenity-rs/serenity/commit/45d72eff173d87b1353d8b5d001775cc49129dab
[c:47ea8f7]: https://github.com/serenity-rs/serenity/commit/47ea8f79b4e980e38fb337b2f3cefc5c7d92fb33
[c:485ad29]: https://github.com/serenity-rs/serenity/commit/485ad299fec218ed3fd354f7207ce6160d803b06
[c:4be6b9d]: https://github.com/serenity-rs/serenity/commit/4be6b9d5008ff8bb3d1fdddff5647a6bb307513c
[c:4d4e9dc]: https://github.com/serenity-rs/serenity/commit/4d4e9dcf4b559423dd5b169ecef46efe6a0d1fca
[c:4e360cf]: https://github.com/serenity-rs/serenity/commit/4e360cf86a74051e2d4f98758c65ae29b97b7b8b
[c:4efe1d1]: https://github.com/serenity-rs/serenity/commit/4efe1d1271515e9ffecd318e368f127becfe273f
[c:4f2e47f]: https://github.com/serenity-rs/serenity/commit/4f2e47f399a10b281a1638fd7fcd3b945154d52c
[c:50d7f00]: https://github.com/serenity-rs/serenity/commit/50d7f00f1b01f4e0d9c86dbdd05a4d4f7b41f8b1
[c:511ec87]: https://github.com/serenity-rs/serenity/commit/511ec87280e8ddec6589f48fec8260bf2e598bdb
[c:52b8e29]: https://github.com/serenity-rs/serenity/commit/52b8e29193801aa254ac7ab105331fb6b0e8eec1
[c:561b0e3]: https://github.com/serenity-rs/serenity/commit/561b0e38b4cda6661425f76c8d707d58d0f12d09
[c:562ce49]: https://github.com/serenity-rs/serenity/commit/562ce49698a39d5da68d3ac58a3d8cf401aa9e42
[c:5a96724]: https://github.com/serenity-rs/serenity/commit/5a967241efabd49116a6d6d5a6eeb95d3281d93b
[c:5e5f161]: https://github.com/serenity-rs/serenity/commit/5e5f161f83b48367bc65d83f8d3cb7f4b1b61f0a
[c:5fd3509]: https://github.com/serenity-rs/serenity/commit/5fd3509c8cfe25370ca4fa66a8468bd2a9679ef5
[c:60c33db]: https://github.com/serenity-rs/serenity/commit/60c33db56bb3754bb0d2196d5f48fee63adf7730
[c:619a91d]: https://github.com/serenity-rs/serenity/commit/619a91de7a2d3e882cbcb8d8566ffeee3bc8192f
[c:64bfc54]: https://github.com/serenity-rs/serenity/commit/64bfc5471808cff59c9b4b5eef80a756f13ff5be
[c:6572580]: https://github.com/serenity-rs/serenity/commit/657258040376be45a8be0ef0e3bd762a23babb0a
[c:68c5be8]: https://github.com/serenity-rs/serenity/commit/68c5be8b6beec57618abea4d8b5bcca34489746e
[c:6a101c4]: https://github.com/serenity-rs/serenity/commit/6a101c4a409ae3abe4038f96dcd51f0788d4c0e4
[c:6c43fed]: https://github.com/serenity-rs/serenity/commit/6c43fed3702be3fdc1eafed26a2f6335acd71843
[c:6d6063f]: https://github.com/serenity-rs/serenity/commit/6d6063fc8334a4422465d30e938a045fd7a09d17
[c:6f147e1]: https://github.com/serenity-rs/serenity/commit/6f147e182b60817dd16e7868326b8cfa1f89ac88
[c:710fa02]: https://github.com/serenity-rs/serenity/commit/710fa02405d8d740c4ee952822d856af0e845aa8
[c:78e7b1b]: https://github.com/serenity-rs/serenity/commit/78e7b1b0624edce9bf69ff6d1d652f9cdfd3f841
[c:7c4b052]: https://github.com/serenity-rs/serenity/commit/7c4b052d7b5a50f234721249bd0221f037e48ea9
[c:7e8da0c]: https://github.com/serenity-rs/serenity/commit/7e8da0c6574ed051de5a9d51001ead0779dfb1de
[c:7e913b6]: https://github.com/serenity-rs/serenity/commit/7e913b6185468d2dd3740c711d418a300584b5bb
[c:824f8cb]: https://github.com/serenity-rs/serenity/commit/824f8cb63271ac3907a9c8223b08b7ee6ff0d746
[c:870a2a5]: https://github.com/serenity-rs/serenity/commit/870a2a5f821c9b0624cad03d873d04a8aad47082
[c:878684f]: https://github.com/serenity-rs/serenity/commit/878684f61fb48a25e117ed32548f78869cb027fc
[c:88765d0]: https://github.com/serenity-rs/serenity/commit/88765d0a978001ff88a1ee12798a725b7f5a90e9
[c:8a33329]: https://github.com/serenity-rs/serenity/commit/8a333290365f1304ad84a8e8f17c0d60728241c2
[c:8bf77fa]: https://github.com/serenity-rs/serenity/commit/8bf77fa431308451411670f20896e36f920997c5
[c:8cc2300]: https://github.com/serenity-rs/serenity/commit/8cc2300f7f2992ae858808033137440ee7e22cd8
[c:8d51ead]: https://github.com/serenity-rs/serenity/commit/8d51ead1747296eac5f2880332ae3e6de048ea4f
[c:8e1435f]: https://github.com/serenity-rs/serenity/commit/8e1435f29a2051f3f481131399fedf5528cb96e4
[c:8e29694]: https://github.com/serenity-rs/serenity/commit/8e296940b7e40879dcfbb185282b906804ba7e3d
[c:8e3b4d6]: https://github.com/serenity-rs/serenity/commit/8e3b4d601ffb78909db859640482f7e0bb10131f
[c:8f37f78]: https://github.com/serenity-rs/serenity/commit/8f37f78af0b9fda4cb0c4bf41e4c047958aa5a40
[c:924c447]: https://github.com/serenity-rs/serenity/commit/924c44759a79a8467cbf9f616a6aaa54c0e746cb
[c:948b27c]: https://github.com/serenity-rs/serenity/commit/948b27ce74e8dce458d427d8159f2a821d4d7cec
[c:97e84fe]: https://github.com/serenity-rs/serenity/commit/97e84fe136c5649ca3529c11790d9988dfe3bb92
[c:9900b20]: https://github.com/serenity-rs/serenity/commit/9900b20bf5cd4036cd8d8ba28bdcd852f2c89d2f
[c:9ccf388]: https://github.com/serenity-rs/serenity/commit/9ccf388e89b0cedddbf76a2236254d4d6ba0dd02
[c:9f02720]: https://github.com/serenity-rs/serenity/commit/9f02720d53ea117b1f6505a061b42fd7044219b9
[c:aa307b1]: https://github.com/serenity-rs/serenity/commit/aa307b160a263fb4d091d4aed06076b6c7f744b6
[c:aace5fd]: https://github.com/serenity-rs/serenity/commit/aace5fdb7f6eb71c143414c491005e378e299221
[c:ab67c1d]: https://github.com/serenity-rs/serenity/commit/ab67c1dd60b5f49541815b2527e8a3cb7712e182
[c:af1061b]: https://github.com/serenity-rs/serenity/commit/af1061b5e82ed1bf4e71ff3146cb98bc6cbb678c
[c:b249c82]: https://github.com/serenity-rs/serenity/commit/b249c8212ecd37cf3d52188fcc56f45268b3400e
[c:b602805]: https://github.com/serenity-rs/serenity/commit/b602805501df003d1925c2f0d0c80c2bac6d32a2
[c:b6af867]: https://github.com/serenity-rs/serenity/commit/b6af86779701110f7f21da26ae8712f4daf4ee3b
[c:bc3491c]: https://github.com/serenity-rs/serenity/commit/bc3491cf3a70a02ce5725e66887746567ae4660c
[c:bd05bda]: https://github.com/serenity-rs/serenity/commit/bd05bdad1765ad2038dcc4650e1ad4da8a2e020c
[c:bd9fcf7]: https://github.com/serenity-rs/serenity/commit/bd9fcf73a7912c900d194a0bebae586fb0d96d79
[c:bfdb57c]: https://github.com/serenity-rs/serenity/commit/bfdb57cdf35721f4953d436a819745ac5d44295e
[c:c2cf691]: https://github.com/serenity-rs/serenity/commit/c2cf6910b6a77c40d543d8950fca45c0d49b6073
[c:c68d4d5]: https://github.com/serenity-rs/serenity/commit/c68d4d5230e60ab48c5620f3d7daff666ded4a11
[c:c7b8ab8]: https://github.com/serenity-rs/serenity/commit/c7b8ab89c33c72b36b789dcc0648c164df523b1b
[c:ca0f113]: https://github.com/serenity-rs/serenity/commit/ca0f113324c1ed64a8646c42ed742dd8021fbccd
[c:caf69d6]: https://github.com/serenity-rs/serenity/commit/caf69d66893c2688f0856cc33f03702071d1314a
[c:cb18d42]: https://github.com/serenity-rs/serenity/commit/cb18d4207c3b9cf942bd561e76ae4059dd50979d
[c:cdedf36]: https://github.com/serenity-rs/serenity/commit/cdedf36330aa6da9e59d296164090f54b651b874
[c:d35d719]: https://github.com/serenity-rs/serenity/commit/d35d719518a48b1cf51c7ecb5ed9c717893784dc
[c:d8027d7]: https://github.com/serenity-rs/serenity/commit/d8027d7a3b9521565faa829f865c6248b3ba26c5
[c:d925f92]: https://github.com/serenity-rs/serenity/commit/d925f926c0f9f5b8010a998570441258417fc89a
[c:dbcb351]: https://github.com/serenity-rs/serenity/commit/dbcb3514f20409b3c4c4054fe51aaa2bd1792b96
[c:dbd6727]: https://github.com/serenity-rs/serenity/commit/dbd672783ef6f647664d3b1aa97957af9321d55c
[c:dc3a4df]: https://github.com/serenity-rs/serenity/commit/dc3a4dfafb1ee096b56c78d2506743e4012323f7
[c:deee38d]: https://github.com/serenity-rs/serenity/commit/deee38d87d71a918b6d8270dbfaffeb0a7234508
[c:e1912c2]: https://github.com/serenity-rs/serenity/commit/e1912c22fc806f97d9eb9025aa2432e785003f3b
[c:e1a8fe3]: https://github.com/serenity-rs/serenity/commit/e1a8fe3e9f619fbb94dd54993c8f5d25fd5dc375
[c:e2053dd]: https://github.com/serenity-rs/serenity/commit/e2053dd53f7c85175901ee57f7c028ba369487a9
[c:e218ce0]: https://github.com/serenity-rs/serenity/commit/e218ce0ec78b7b480e9a83628378dc9670e2cf4a
[c:e5889ed]: https://github.com/serenity-rs/serenity/commit/e5889ed1a62ddcb6bc11364800cd813329eb3ece
[c:e72e25c]: https://github.com/serenity-rs/serenity/commit/e72e25cf8b0160a3ec0de0be98dd8f1467d3b505
[c:e7a5ba3]: https://github.com/serenity-rs/serenity/commit/e7a5ba3e6c7e914c952408828f0cc71e15acea61
[c:ea1eba8]: https://github.com/serenity-rs/serenity/commit/ea1eba89087825e526e54fffdb27642fe72f9602
[c:ea432af]: https://github.com/serenity-rs/serenity/commit/ea432af97a87b8a3d673a1f40fe06cde4d84e146#diff-2e7fe478bd2e14b5b3306d2c679e4b5a
[c:eb47559]: https://github.com/serenity-rs/serenity/commit/eb47559fa00c13c8fdc8f40a8fe3d06690c0570c
[c:ebc4e51]: https://github.com/serenity-rs/serenity/commit/ebc4e51fe3b1e5bc61dc99da25a22d2e2277ffc6
[c:eee857a]: https://github.com/serenity-rs/serenity/commit/eee857a855831851599e5196750b27b26151eb16
[c:f05efce]: https://github.com/serenity-rs/serenity/commit/f05efce7af0cb7020e7da08c7ca58fa6f786d4ef
[c:f16af97]: https://github.com/serenity-rs/serenity/commit/f16af97707edfc36f52fa836791d07512e5d41ef
[c:f5a97d4]: https://github.com/serenity-rs/serenity/commit/f5a97d43b467130fd97af8c8a0dd1bbf0e7f5326
[c:f830f31]: https://github.com/serenity-rs/serenity/commit/f830f31f046b39124877a65fa1a95f789d125809
[c:fb2a1a9]: https://github.com/serenity-rs/serenity/commit/fb2a1a9262b481af62f9c0025a0f180626d19241
[c:fbc1ac7]: https://github.com/serenity-rs/serenity/commit/fbc1ac740e769e624637c490b6a959ed86ec3839
[c:fc9eba3]: https://github.com/serenity-rs/serenity/commit/fc9eba3d6d6a600f7d45a6f4e5918aae1191819d
[c:fd47b86]: https://github.com/serenity-rs/serenity/commit/fd47b865f3c32f5bbfce65162023898a6ecd29a1
[c:fd89d09]: https://github.com/serenity-rs/serenity/commit/fd89d09d3397eba21d1b454d3b6155ba9c3a829e
[c:fdbfbe0]: https://github.com/serenity-rs/serenity/commit/fdbfbe098c9d59000c234a0893496751744fd31e
[c:fdfb184]: https://github.com/serenity-rs/serenity/commit/fdfb1846083165629feca81b5169ceaf331289c5
[c:f6fcf32]: https://github.com/serenity-rs/serenity/commit/f6fcf32e7f62dfc207ac2f9f293f804446ea3423
[c:fdfd5bc]: https://github.com/serenity-rs/serenity/commit/fdfd5bcf708b6633b564fc58fb86935536310314

[c:00fb61b]: https://github.com/serenity-rs/serenity/commit/00fb61b5f306aebde767cc21a498a8ca0742d0be
[c:0102706]: https://github.com/serenity-rs/serenity/commit/0102706321a00cfb39b356bdf2cf8d523b93a8ec
[c:01f6872]: https://github.com/serenity-rs/serenity/commit/01f687204dd9d5564ec4bdc860f11bfd5e01454f
[c:04cfaa9]: https://github.com/serenity-rs/serenity/commit/04cfaa9a69dc1638e9cd1904a9b8e94c1a97f832
[c:060b06e]: https://github.com/serenity-rs/serenity/commit/060b06ec62b1f4e4cc2c11b877fd988b7dcfe627
[c:063a52f]: https://github.com/serenity-rs/serenity/commit/063a52f8c028c7432ee556380d2bd5c652d75d22
[c:0708ccf]: https://github.com/serenity-rs/serenity/commit/0708ccf85bac347e59053133a2b8b6f2eabe99ba
[c:096b0f5]: https://github.com/serenity-rs/serenity/commit/096b0f57aae04a5e0ea28414f5016eeafc5b9e0a
[c:0a2f5ab]: https://github.com/serenity-rs/serenity/commit/0a2f5ab525022fbf0055649f2262573fb07cf18c
[c:0b95db9]: https://github.com/serenity-rs/serenity/commit/0b95db916580b8b7eb8bf7e81e6051f849a9c0c8
[c:0b9bf91]: https://github.com/serenity-rs/serenity/commit/0b9bf91f62eef85a4eca703902077f4c04b3b6d1
[c:0c9ec37]: https://github.com/serenity-rs/serenity/commit/0c9ec377aa7281fb3d4bc390c896b426660a5387
[c:0d218e0]: https://github.com/serenity-rs/serenity/commit/0d218e02e043c043d7274c7169607b11c9897a5a
[c:0ec4dfb]: https://github.com/serenity-rs/serenity/commit/0ec4dfb785459c0d04c295f84a1c33e71c016eba
[c:0f41ffc]: https://github.com/serenity-rs/serenity/commit/0f41ffc811827fdd45e4e631884909e33fa8769e
[c:11a02db]: https://github.com/serenity-rs/serenity/commit/11a02db8e70c18a152bad9de6491817efc1d2f54
[c:13de5c2]: https://github.com/serenity-rs/serenity/commit/13de5c2e50410c3a68435dc774537b490bb7115c
[c:143337a]: https://github.com/serenity-rs/serenity/commit/143337ae717773f59562d67f85d0e9c44063a45b
[c:147cf01]: https://github.com/serenity-rs/serenity/commit/147cf01d4f13e3ee15eb03705ab2b7a006851cdd
[c:1561f9e]: https://github.com/serenity-rs/serenity/commit/1561f9e36384a215d2b866a752996f80d36a3ede
[c:1594961]: https://github.com/serenity-rs/serenity/commit/159496188b2c841a65829328cddafef620c517af
[c:16bd765]: https://github.com/serenity-rs/serenity/commit/16bd765112befd5d81818cab7b97ac59bd8a1b75
[c:16d1b3c]: https://github.com/serenity-rs/serenity/commit/16d1b3cad3982accd805f64ef93e51d921b3da55
[c:1700a4a]: https://github.com/serenity-rs/serenity/commit/1700a4a9090789d485c190c2a6ccd2c48986f5dd
[c:175d3a3]: https://github.com/serenity-rs/serenity/commit/175d3a3ef585f6fede959183138d507886192a4e
[c:2416813]: https://github.com/serenity-rs/serenity/commit/24168137ff7b1ec44d3ecdec0f516455fd3785a7
[c:268f356]: https://github.com/serenity-rs/serenity/commit/268f356a25f27175a5d72458fff92b0f770d0a5a
[c:2844ae1]: https://github.com/serenity-rs/serenity/commit/2844ae158f3d8297b17a584ff9a75f1f51116f48
[c:2845681]: https://github.com/serenity-rs/serenity/commit/28456813f6f05e9bdaf08e8cad641df1e3dfaff7
[c:2a743ce]: https://github.com/serenity-rs/serenity/commit/2a743cedaf08f7eb532e3c4b795cfc5f85bc96af
[c:2afab7c]: https://github.com/serenity-rs/serenity/commit/2afab7c6eb828e491721e15f11a76ae36e34796d
[c:2b237e7]: https://github.com/serenity-rs/serenity/commit/2b237e7de221beab9c516d6de29f83188ef63840
[c:2cb607d]: https://github.com/serenity-rs/serenity/commit/2cb607d72a39aa7ab3df866b23de4c9798e69a0f
[c:2d09152]: https://github.com/serenity-rs/serenity/commit/2d091528287b7f5dfd678e9bc77c25bf53b0f420
[c:2eaa415]: https://github.com/serenity-rs/serenity/commit/2eaa4159955260e7c9ade66803d69865f1f76018
[c:302d771]: https://github.com/serenity-rs/serenity/commit/302d771182308f907423ed73be9b736f268737fe
[c:3062981]: https://github.com/serenity-rs/serenity/commit/3062981bfc1412e93450b30fa9405e555624ce1e
[c:31aae7d]: https://github.com/serenity-rs/serenity/commit/31aae7d12763f94a7a08ea9fd0102921e8402241
[c:31becb1]: https://github.com/serenity-rs/serenity/commit/31becb16f184cd7d396b383ad4abed8095451fcb
[c:32e07e4]: https://github.com/serenity-rs/serenity/commit/32e07e4ac822d5cc1118f0db0fc92b549c1aaf81
[c:3348178]: https://github.com/serenity-rs/serenity/commit/3348178f151d8e1d7aa0432984a2dd23fa7b9e89
[c:345e140]: https://github.com/serenity-rs/serenity/commit/345e1401142d21a0fdabb2accd1f33e3a07c02c8
[c:38a484d]: https://github.com/serenity-rs/serenity/commit/38a484d0fec91e290bc1633fc871131f9decd0ca
[c:38db32e]: https://github.com/serenity-rs/serenity/commit/38db32e2cbb9dc8504e0dfbc2366b17596836da0
[c:39a28d3]: https://github.com/serenity-rs/serenity/commit/39a28d3bf5d7005c3549a09542d27c08660f49cb
[c:3c7c575]: https://github.com/serenity-rs/serenity/commit/3c7c575d988f4dc793678880560aee48456f4526
[c:3ca7ad9]: https://github.com/serenity-rs/serenity/commit/3ca7ad92507f056054d081485f437c08505bc7e5
[c:3f03f9a]: https://github.com/serenity-rs/serenity/commit/3f03f9adc97315bb61a5c71f52365306cb8e2d1a
[c:404a089]: https://github.com/serenity-rs/serenity/commit/404a089af267c5d5c33025a3d74826e02b6f8ca1
[c:4229034]: https://github.com/serenity-rs/serenity/commit/42290348bc05c876b7e70c570a485160e594e098
[c:4267bdb]: https://github.com/serenity-rs/serenity/commit/4267bdbae05d5516774ca72fe92789651cfa7230
[c:43a5c5d]: https://github.com/serenity-rs/serenity/commit/43a5c5d7eb8bffb8c9ca450ab1bc377d602fb8c3
[c:46b79dd]: https://github.com/serenity-rs/serenity/commit/46b79ddb45d03bfbe0eb10a9d5e1c53c9a15f55b
[c:494cc50]: https://github.com/serenity-rs/serenity/commit/494cc50ff3dcf8553a5588fa868754d27c237055
[c:49a6841]: https://github.com/serenity-rs/serenity/commit/49a684134df32427e9502192122c4fb22ef1a735
[c:4a14b92]: https://github.com/serenity-rs/serenity/commit/4a14b92ff58173acb98c7e0a135b4989a87a7529
[c:4cf8338]: https://github.com/serenity-rs/serenity/commit/4cf8338e364b0feefef26ece6649077e87962ff3
[c:4de39da]: https://github.com/serenity-rs/serenity/commit/4de39da887248e374b4d824472a6422c7e46dacc
[c:4f5fbb5]: https://github.com/serenity-rs/serenity/commit/4f5fbb54ae930dd56aa9a53878cf1b5e123de038
[c:51c15d0]: https://github.com/serenity-rs/serenity/commit/51c15d088054dd42c66fee10deed1431df931ec9
[c:543b604]: https://github.com/serenity-rs/serenity/commit/543b60421d1c6acd77e02cdd11c7dd2157399821
[c:55ccaca]: https://github.com/serenity-rs/serenity/commit/55ccaca57051b3fbd47cf7fa288014d9c36f6952
[c:57c060f]: https://github.com/serenity-rs/serenity/commit/57c060fa2fccfbb3b3d4b2d18aad2faa5929deb3
[c:585af23]: https://github.com/serenity-rs/serenity/commit/585af231028e46788d689f94e14e110c072a578e
[c:5918d01]: https://github.com/serenity-rs/serenity/commit/5918d01ed69541e43aed0e62ee6eadbf5ebb20d2
[c:5b275fc]: https://github.com/serenity-rs/serenity/commit/5b275fc425d4ef1c1a9eaa9d915db1f873f9c11d
[c:5bf6c2d]: https://github.com/serenity-rs/serenity/commit/5bf6c2d2cf0491951eddb10ab2641d02d0e730a1
[c:5c40e85]: https://github.com/serenity-rs/serenity/commit/5c40e85001b9b2620a76fcc57d8f0cddfb6f9b34
[c:5ee5fef]: https://github.com/serenity-rs/serenity/commit/5ee5feff615565b6f661ee3598fe19bb98bd6a88
[c:5fe6a39]: https://github.com/serenity-rs/serenity/commit/5fe6a3956d39e9b5caef19df78e8b392898b6908
[c:601704a]: https://github.com/serenity-rs/serenity/commit/601704acb94601a134ae43e795474afe8574b2ae
[c:626ffb2]: https://github.com/serenity-rs/serenity/commit/626ffb25af35f5b91a76fdccf6788382a1c39455
[c:62ed564]: https://github.com/serenity-rs/serenity/commit/62ed564e5f67f3e25d2307fbbf950d0489a28de8
[c:6355288]: https://github.com/serenity-rs/serenity/commit/635528875c59d34f0d7b2f2b0a3bd61d762f0e9c
[c:6502ded]: https://github.com/serenity-rs/serenity/commit/6502dedfcced471aaf17b7d459da827a1867807a
[c:651c618]: https://github.com/serenity-rs/serenity/commit/651c618f17cb92d3ea9bbd1d5f5c92a015ff64e0
[c:6579b1f]: https://github.com/serenity-rs/serenity/commit/6579b1fb0409410f303a4df5e7246c507a80f27b
[c:66546d3]: https://github.com/serenity-rs/serenity/commit/66546d36749f6c78a4957a616524fab734d5c972
[c:6853daf]: https://github.com/serenity-rs/serenity/commit/6853daf4d04719a9a8a081151bd85336e160a752
[c:68c473d]: https://github.com/serenity-rs/serenity/commit/68c473dd17a2098f97808b3d1f2a200621f67c9d
[c:69ec62a]: https://github.com/serenity-rs/serenity/commit/69ec62a42bcb143cdde056ad8ffce81922e88317
[c:6a887b2]: https://github.com/serenity-rs/serenity/commit/6a887b25f2712d70c65fc85b5cfbd8b6d4b41260
[c:6b0b9b2]: https://github.com/serenity-rs/serenity/commit/6b0b9b2491fa895bd7dd8e065f067470ea08639d
[c:6e11a10]: https://github.com/serenity-rs/serenity/commit/6e11a103f7a6a4ab43b1aa511aad9e04b1fd8c5a
[c:6f33a35]: https://github.com/serenity-rs/serenity/commit/6f33a35c4f85a06c45c4ce9e118db203c4951475
[c:70bf22a]: https://github.com/serenity-rs/serenity/commit/70bf22a00cd19651a0d994cc43e8d8c4bd8947fc
[c:70d4e75]: https://github.com/serenity-rs/serenity/commit/70d4e7538cefc21dd0e06d5451888b82f53acf38
[c:71f3dbb]: https://github.com/serenity-rs/serenity/commit/71f3dbb650f4b0d6434630137ae9eea502a1ebef
[c:760a47a]: https://github.com/serenity-rs/serenity/commit/760a47aa4d34160f44048e775afeb30f08891c99
[c:76f9095]: https://github.com/serenity-rs/serenity/commit/76f9095c012a8769c7bd27aca6540b7018574c28
[c:77b5b48]: https://github.com/serenity-rs/serenity/commit/77b5b480d67e747908f8f4fb9f910bab23b761b5
[c:7914274]: https://github.com/serenity-rs/serenity/commit/79142745cb571ba2d4284fd1dcbe53c14a0ed623
[c:7990381]: https://github.com/serenity-rs/serenity/commit/799038187d903a75d60f0c98d013ae87fb665d02
[c:7b45f16]: https://github.com/serenity-rs/serenity/commit/7b45f16f063a47dc8a302dce5b016cf43a3edcc1
[c:7b4b154]: https://github.com/serenity-rs/serenity/commit/7b4b1544603a70dd634b51593ea5173b4515889a
[c:7dbae6b]: https://github.com/serenity-rs/serenity/commit/7dbae6b5261b8f53200090c9eb1bf39a7498f07d
[c:7e254c5]: https://github.com/serenity-rs/serenity/commit/7e254c5c6098bb1a47bac26c9895098a46cdc53f
[c:7f04179]: https://github.com/serenity-rs/serenity/commit/7f041791aa95e38a0cacd2ab64f0423524c60052
[c:7fc49d8]: https://github.com/serenity-rs/serenity/commit/7fc49d8dd9e253b066ab1b82446d0344f800e2d7
[c:c832009]: https://github.com/serenity-rs/serenity/commit/c832009eae235881815186f740b716e0b7e63951
[c:8360f32]: https://github.com/serenity-rs/serenity/commit/8360f329eae1751a8a413a6f6838486f3a0bba01
[c:83b1d96]: https://github.com/serenity-rs/serenity/commit/83b1d967f4cc2040f94d67dd987302347f227d6a
[c:83b29d5]: https://github.com/serenity-rs/serenity/commit/83b29d5f839cd2ea6cd150aa7b8ccbbc677c1fad
[c:858bbf2]: https://github.com/serenity-rs/serenity/commit/858bbf298d08ada3ae6c5b24105bf751bc938d5e
[c:86a4e00]: https://github.com/serenity-rs/serenity/commit/86a4e008ca7acf23d920e344463df801a774d5ce
[c:86cd00f]: https://github.com/serenity-rs/serenity/commit/86cd00f20d6f218e524deed040d3c209f0361a86
[c:8b504ad]: https://github.com/serenity-rs/serenity/commit/8b504ad7f6e10fecb27583a949262eb61cfd266d
[c:8c04d31]: https://github.com/serenity-rs/serenity/commit/8c04d318e273e9bcb3af6ddd820ad067048e95c6
[c:8c0aeac]: https://github.com/serenity-rs/serenity/commit/8c0aeacadb93d3b56fb98beb882eaef1f79cd652
[c:8c5ee70]: https://github.com/serenity-rs/serenity/commit/8c5ee70b28b42ac92f899932ab2ddafeb9c6f913
[c:8e2c052]: https://github.com/serenity-rs/serenity/commit/8e2c052a55e5e08c6e7ed643b399f1a7f69a2b25
[c:8effc91]: https://github.com/serenity-rs/serenity/commit/8effc918cc3d269b0d4cf34ef4f2053cecad2606
[c:8f24aa3]: https://github.com/serenity-rs/serenity/commit/8f24aa391f6b8a9103a9c105138c7610288acb05
[c:8f88c6b]: https://github.com/serenity-rs/serenity/commit/8f88c6b0613199492ebca8cd9f2bf4dd5c97add7
[c:8f8a059]: https://github.com/serenity-rs/serenity/commit/8f8a05996c5b47ec9401aabb517d96ed2af5c36b
[c:9114963]: https://github.com/serenity-rs/serenity/commit/9114963daf708cfaeaf54d8c788206ccfbae5df8
[c:921f7f4]: https://github.com/serenity-rs/serenity/commit/921f7f42d87e7c727b5a87802d7738f8081b600a
[c:92309b2]: https://github.com/serenity-rs/serenity/commit/92309b2fb8ffd96292fd2edaa7c223a2ba774a56
[c:9268f9c]: https://github.com/serenity-rs/serenity/commit/9268f9c10ef47ffeaeb3d5040e65b1093e04b866
[c:92f4ec2]: https://github.com/serenity-rs/serenity/commit/92f4ec204d10a8d60af9ce3cc7433be8117a711d
[c:933ee89]: https://github.com/serenity-rs/serenity/commit/933ee8914509e52c5119ced9f5d9d8f9644cfa63
[c:93416cd]: https://github.com/serenity-rs/serenity/commit/93416cdebff12a3f85e694c8cb28350a5c14c50f
[c:9392f61]: https://github.com/serenity-rs/serenity/commit/9392f61f8857b6ab2a04781c2d9c92a582a1577b
[c:93f3c60]: https://github.com/serenity-rs/serenity/commit/93f3c60b23cb8ffd16666bdc01b3502ca7ba5f47
[c:9969be6]: https://github.com/serenity-rs/serenity/commit/9969be60cf320797c37b317da24d9a08fd5eafa5
[c:97f9bd1]: https://github.com/serenity-rs/serenity/commit/97f9bd10c16eb24d54a0ab00c52f19eb51a88675
[c:990e611]: https://github.com/serenity-rs/serenity/commit/990e611a56f37f64fbce74fbc487c7dcc4aa4e28
[c:9aa357f]: https://github.com/serenity-rs/serenity/commit/9aa357f0c8f504b53b49824cc20561c8501d2dda
[c:9c04a19]: https://github.com/serenity-rs/serenity/commit/9c04a19015cf579d343d81a7fa50e6f4b18b4a5b
[c:9c1ed6c]: https://github.com/serenity-rs/serenity/commit/9c1ed6ca933f81bc0254d9d52159b9190b50a3ea
[c:9dae9e6]: https://github.com/serenity-rs/serenity/commit/9dae9e67b992cea4c18f1c685f5185abd9428887
[c:9ec05e7]: https://github.com/serenity-rs/serenity/commit/9ec05e701bdbadad39847f0dcc18d5156ecdde02
[c:9ef5522]: https://github.com/serenity-rs/serenity/commit/9ef55224757dff6dec8576bd1ad11db24a10891e
[c:a0bb306]: https://github.com/serenity-rs/serenity/commit/a0bb30686c1a9431aef23c2e8594791f64035194
[c:a2cbeb6]: https://github.com/serenity-rs/serenity/commit/a2cbeb6ece9ef56e2082b6eabbabe5fe536ab3ec
[c:a39647d]: https://github.com/serenity-rs/serenity/commit/a39647d3ba1650a4dd4c92bd40001959828000c7
[c:a8acd61]: https://github.com/serenity-rs/serenity/commit/a8acd6138741a6e5268141ac4ce902561931d353
[c:ab778f8]: https://github.com/serenity-rs/serenity/commit/ab778f8a9cf47c4e27fe688a61effb0caa4f8a6e
[c:ab7f113]: https://github.com/serenity-rs/serenity/commit/ab7f113a9e3acd000dbf69b7c4bd8d2d766b39f1
[c:abd22d2]: https://github.com/serenity-rs/serenity/commit/abd22d289599530cbd1bc9cf1b739420f0d22372
[c:ada07fa]: https://github.com/serenity-rs/serenity/commit/ada07fae09f3521f44d81613f26839d69c1fc7ef
[c:ae352ea]: https://github.com/serenity-rs/serenity/commit/ae352ea3df86eb2d853d5b1af048a95409aafc38
[c:ae395f4]: https://github.com/serenity-rs/serenity/commit/ae395f44361a9a9b488b31d6ac0cb54e0ee9e7a1
[c:aea9885]: https://github.com/serenity-rs/serenity/commit/aea98851e86c0f36be231c0a3b763f769c76e061
[c:afc571f]: https://github.com/serenity-rs/serenity/commit/afc571fd67c294cc10682db5c579d10645aec437
[c:b001234]: https://github.com/serenity-rs/serenity/commit/b0012349cca2a5c7c62bb6d2c99106d245b6c55a
[c:b468cbf]: https://github.com/serenity-rs/serenity/commit/b468cbffa0db341987d1dc397582b3edd3944d09
[c:b4bd771]: https://github.com/serenity-rs/serenity/commit/b4bd7714a155381cc16ece51acb0c4dc6cde96a2
[c:b7cbf75]: https://github.com/serenity-rs/serenity/commit/b7cbf75103939b0b7834c808050b19ba4fbc4b17
[c:b96f85c]: https://github.com/serenity-rs/serenity/commit/b96f85c224b9c0478b7f1b5c5b76761e23ff7edf
[c:bad9ac3]: https://github.com/serenity-rs/serenity/commit/bad9ac3d28bb0417dedcdddf10cf764c08d1d6ae
[c:bb97211]: https://github.com/serenity-rs/serenity/commit/bb97211b2b107943dd6fabb7a0a344d4fe236780
[c:bcb70e8]: https://github.com/serenity-rs/serenity/commit/bcb70e85384a16b2440788a73241f507aaeba4dc
[c:bceb049]: https://github.com/serenity-rs/serenity/commit/bceb049bb2b804dac975567bb7eac6afcfc28574
[c:c00f349]: https://github.com/serenity-rs/serenity/commit/c00f3490f2fb0c045c2da72d850f70da8e2cdb95
[c:c01f238]: https://github.com/serenity-rs/serenity/commit/c01f238a34ad846f8732c8bf97fbbd96fbf6a7ae
[c:c032fbe]: https://github.com/serenity-rs/serenity/commit/c032fbe7a5c65fb6824a5eb36daf327134b854cf
[c:c050c59]: https://github.com/serenity-rs/serenity/commit/c050c59da25b9093a75bda22baa81be3b267c688
[c:c2e8b69]: https://github.com/serenity-rs/serenity/commit/c2e8b69702cf81a1cf149c420aec999124f398e2
[c:c36841d]: https://github.com/serenity-rs/serenity/commit/c36841dd1c3f80141251ba01130333f43ff363d7
[c:c74cc15]: https://github.com/serenity-rs/serenity/commit/c74cc15f8969c8db68119d07a4f273a0d3fc44f4
[c:c8536c1]: https://github.com/serenity-rs/serenity/commit/c8536c111117f26833fb1bceff734ac1abc55479
[c:c8c6b83]: https://github.com/serenity-rs/serenity/commit/c8c6b83ca685a3e503c853d4154a17761790954e
[c:cd914f5]: https://github.com/serenity-rs/serenity/commit/cd914f503c8f0ada7473b5b56e4ad7830370ea45
[c:d033909]: https://github.com/serenity-rs/serenity/commit/d03390968ec7a5e1e93dbcc508c3b8a5f44b792d
[c:d0b64cd]: https://github.com/serenity-rs/serenity/commit/d0b64cd64a18a6116267fa09a837d62c19cced42
[c:d144136]: https://github.com/serenity-rs/serenity/commit/d1441363364970b749d57b8a4863b284239488d1
[c:d3389be]: https://github.com/serenity-rs/serenity/commit/d3389be3042fd7977350a08152d177ac6cdcd37f
[c:d367a70]: https://github.com/serenity-rs/serenity/commit/d367a704985bbb127f410770125c160f90561937
[c:d37461b]: https://github.com/serenity-rs/serenity/commit/d37461b5b705e0cdf802925c59113898a71676df
[c:d4fc8b6]: https://github.com/serenity-rs/serenity/commit/d4fc8b6188627ae8d553cf282b1371e3de7b01f9
[c:d58c544]: https://github.com/serenity-rs/serenity/commit/d58c54425a18bbbdc8e66e8eebfb8191bad06901
[c:d9118c0]: https://github.com/serenity-rs/serenity/commit/d9118c081742d6654dc0a4f60228a7a212ca436e
[c:daf92ed]: https://github.com/serenity-rs/serenity/commit/daf92eda815b8f539f6d759ab48cf7a70513915f
[c:db0f025]: https://github.com/serenity-rs/serenity/commit/db0f025d154e4b6212dd9340c1b789b3c711a24a
[c:dc73d1a]: https://github.com/serenity-rs/serenity/commit/dc73d1a4bad07b453a9d60a6c8f8c187a7e42450
[c:e033ff3]: https://github.com/serenity-rs/serenity/commit/e033ff33b94e024fe5f55a8c93c65c3e885f821b
[c:e1079e9]: https://github.com/serenity-rs/serenity/commit/e1079e9a03473f9ec67414628d5b84e7ea1b5b38
[c:e2557ac]: https://github.com/serenity-rs/serenity/commit/e2557ac794068c1a6a5c4c674ed9f7b7a806068e
[c:e4b484f]: https://github.com/serenity-rs/serenity/commit/e4b484f1c823ccb0aa2be7c54e0def07e5a01806
[c:e5a83dd]: https://github.com/serenity-rs/serenity/commit/e5a83dd1873e5af2df18835d960fe19286c70f1e
[c:e6712c9]: https://github.com/serenity-rs/serenity/commit/e6712c9459c367cf9ba3e5d9bf1c0831357a20b5
[c:e7110ad]: https://github.com/serenity-rs/serenity/commit/e7110adb1e5659b7395588381c2e56c2aa06d1fa
[c:e85e901]: https://github.com/serenity-rs/serenity/commit/e85e901062e8b9ea717ec6c6253c9c7a300448d3
[c:e891ebe]: https://github.com/serenity-rs/serenity/commit/e891ebeba43eb87c985db4e031b8bf76dcaca67b
[c:e8a9086]: https://github.com/serenity-rs/serenity/commit/e8a90860d1e451e21d3bf728178957fe54cf106d
[c:e9282d3]: https://github.com/serenity-rs/serenity/commit/e9282d3373158b6e9792a5484ae3dfb9212eb6f7
[c:e92b667]: https://github.com/serenity-rs/serenity/commit/e92b667058138ffd01587e28e9d8551cd59df160
[c:e9aae9c]: https://github.com/serenity-rs/serenity/commit/e9aae9c043b206b15bd5429126ded62259d6731b
[c:eb09f2d]: https://github.com/serenity-rs/serenity/commit/eb09f2d3389b135978e0671a0e7e4ed299014f94
[c:eb43b9c]: https://github.com/serenity-rs/serenity/commit/eb43b9c4a4e43a8e097ea71fdc7584c8108b52a3
[c:ec9b1c7]: https://github.com/serenity-rs/serenity/commit/ec9b1c79abeb2a4eff9f013ba8f0e430979dbc56
[c:ef6eba3]: https://github.com/serenity-rs/serenity/commit/ef6eba37636a487c0d6f3b93b8e76c94f28abbab
[c:f00e165]: https://github.com/serenity-rs/serenity/commit/f00e1654e8549ec6582c6f3a8fc4af6aadd56015
[c:f0d1157]: https://github.com/serenity-rs/serenity/commit/f0d1157212397ae377e11d4205abfebc849ba9d8
[c:f3f74ce]: https://github.com/serenity-rs/serenity/commit/f3f74ce43f8429c4c9e38ab7b905fb5a24432fd4
[c:f53124e]: https://github.com/serenity-rs/serenity/commit/f53124ec952124f5b742f204cdf7e1dc00a168ab
[c:f57a187]: https://github.com/serenity-rs/serenity/commit/f57a187d564bdcd77f568e77a102d6d261832ee0
[c:f69512b]: https://github.com/serenity-rs/serenity/commit/f69512beaa157775accd4392295dba112adcf1df
[c:f695174]: https://github.com/serenity-rs/serenity/commit/f695174287e3999cbcbabc691a86302fa8269900
[c:f6b27eb]: https://github.com/serenity-rs/serenity/commit/f6b27eb39c042e6779edc2d5d4b6e6c27d133eaf
[c:f847638]: https://github.com/serenity-rs/serenity/commit/f847638859423ffaaecfdb77ee5348a607ad3293
[c:f894cfd]: https://github.com/serenity-rs/serenity/commit/f894cfdc43a708f457273e1afb57ed1c6e8ebc58
[c:f96b6cc]: https://github.com/serenity-rs/serenity/commit/f96b6cc5e1e0383fd2de826c8ffd95565d5ca4fb
[c:fafa363]: https://github.com/serenity-rs/serenity/commit/fafa3637e760f0c72ae5793127bc2f70dcf2d0e2
[c:fb07751]: https://github.com/serenity-rs/serenity/commit/fb07751cfc1efb657cba7005c38ed5ec6b192b4f
[c:fb4d411]: https://github.com/serenity-rs/serenity/commit/fb4d411054fa44928b4fa052b19de19fce69d7cf
[c:ff4437a]: https://github.com/serenity-rs/serenity/commit/ff4437addb01e5c6c3ad8c5b1830db0d0a86396b

[c:f47a0c8]: https://github.com/serenity-rs/serenity/commit/f47a0c831efe5842ca38cb1067de361ae42f6edc
[c:d50b129]: https://github.com/serenity-rs/serenity/commit/d50b12931404946e219d3ff0878f0632445ef35f
[c:41f26b3]: https://github.com/serenity-rs/serenity/commit/41f26b3757c7a5fba1f09f34e3192e2fd9702a4a
[c:f9e5e76]: https://github.com/serenity-rs/serenity/commit/f9e5e76585a1f6317dadb67e440765b0070ca131
[c:9428787]: https://github.com/serenity-rs/serenity/commit/9428787abb6126ba05bfef96cd2b8d2a217fdf5d
[c:a58de97]: https://github.com/serenity-rs/serenity/commit/a58de97e6089aa98f04d2cdc7312ed38a9f72b22
[c:fbd6258]: https://github.com/serenity-rs/serenity/commit/fbd625839e6a2e01b16e6c3814cb9b9f31dc7caa
[c:292ceda]: https://github.com/serenity-rs/serenity/commit/292cedaa3462f7532efda98722354afa8e213b6a
[c:d3015a0ff]: https://github.com/serenity-rs/serenity/commit/d3015a0ff0c0c87888437f991945453b92296875
[c:585ac6e]: https://github.com/serenity-rs/serenity/commit/585ac6e6ca792facf29063776c83262fa849161b
[c:3616585]: https://github.com/serenity-rs/serenity/commit/361658510f3e2eb9aefbe66232b9b1f1a1ebb80f
[c:e694766]: https://github.com/serenity-rs/serenity/commit/e694766bb6c93d5f6a75ad9871cfdefbd0309a17
[c:e02d5fb]: https://github.com/serenity-rs/serenity/commit/e02d5fb8171b11214e1502c6754fef1972bbf1b9
[c:b7cdf15]: https://github.com/serenity-rs/serenity/commit/b7cdf1542cb9199c61c0b17bdd381d4f117f635e
[c:c7aa27d]: https://github.com/serenity-rs/serenity/commit/c7aa27dbb64e64d70c7f13725c79017c4bba1c95
[c:2219bb3]: https://github.com/serenity-rs/serenity/commit/2219bb37a80c4c2b4ff5a24d72b82737eb241195
[c:74ec713]: https://github.com/serenity-rs/serenity/commit/74ec713825b2b4c55382fb76fa57bd967e66b3aa
[c:5829c67]: https://github.com/serenity-rs/serenity/commit/5829c673c13655b86d317ab65d204067a2b1a7a4
[c:ce4f8c2]: https://github.com/serenity-rs/serenity/commit/ce4f8c2ac8dd2c472ab537a60bf92579d078073b
[c:fcc4e2c]: https://github.com/serenity-rs/serenity/commit/fcc4e2ce2e523248ed33c9f4853d3485cbc9b6e6
[c:23ff6f]: https://github.com/serenity-rs/serenity/commit/23ff6f21019bc94f8dc32355fa34691b881bfb69
[c:e57b510]: https://github.com/serenity-rs/serenity/commit/e57b510edd640abb243664337a1c163924313612
[c:c149e36]: https://github.com/serenity-rs/serenity/commit/c149e368ae4bb1be5d0392b9cae282fc530831c5
