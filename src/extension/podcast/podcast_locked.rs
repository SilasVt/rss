// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.
// Modified for Podcast namespace in 2023 by Silas Vogt


pub struct PodcastLocked {
    pub locked: String, // "yes" or "no"
    pub owner: Option<String>, // Optional owner email
}

impl PodcastLocked {
    // Getter and setter for the 'locked' field
    pub fn locked(&self) -> &str {
        &self.locked
    }

    pub fn set_locked(&mut self, locked: String) {
        self.locked = locked;
    }

    // Getter and setter for the optional 'owner' field
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }

    pub fn set_owner<V>(&mut self, owner: V)
    where
        V: Into<Option<String>>,
    {
        self.owner = owner.into();
    }
}


