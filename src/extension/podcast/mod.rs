// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use std::collections::BTreeMap;

use crate::extension::Extension;


mod podcast_channel_extension;
mod podcast_item_extension;
mod podcast_locked;
mod podcast_transcript;
mod podcast_funding;
mod podcast_chapters;
mod podcast_soundbite;

pub use self::podcast_channel_extension::*;
pub use self::podcast_locked::*;


/// The Podcast XML namespace.
pub const NAMESPACE: &str = "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md";
