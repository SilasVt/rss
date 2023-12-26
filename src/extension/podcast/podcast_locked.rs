// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.
// Modified for Podcast namespace in 2023 by Silas Vogt



pub struct PodcastLocked {
    pub locked: String, // If podcast:locked is present, this is required to be yes or no
    pub owner: Option<String>, // Optional attribute
}


