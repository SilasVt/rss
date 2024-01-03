// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.
// Modified for Podcast namespace in 2023 by Silas Vogt
use std::io::{BufRead, Write};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::BytesText;
use quick_xml::events::BytesEnd;


use crate::error::Error;
use crate::toxml::ToXml;
use crate::util::{attr_value, decode, skip};
use crate::toxml::{ToXml, WriterExt};

/// Represents a funding tag in the podcast namespace.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]

pub struct PodcastFunding {
    /// The URL for the funding/donation pages.
    pub url: String,
    /// The description text..
    pub description: String,
}

impl PodcastFunding {
    // Getter and setter methods for `url` and `description`.

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// use rss::extension::podcast::PodcastFunding;
    ///
    /// let mut extension = PodcastChannelExtension::default();
    /// extension.set_url("patreon.com/example".to_string());
    /// ```
    pub fn set_url<V>(&mut self, url: V)
    where
        V: Into<String>,
    {
        self.url = url.into();
    }

    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<String>,
    {
        self.description = description.into();
    }
}


 //Space for to and from XML

