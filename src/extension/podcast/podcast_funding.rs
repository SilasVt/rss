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

use crate::error::Error;
use crate::toxml::ToXml;
use crate::util::{attr_value, decode, skip};

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

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// use rss::extension::podcast::PodcastFunding;
    ///
    /// let mut extension = PodcastChannelExtension::default();
    /// extension.set_url("patreon.com/example".to_string());
    /// ```
    pub fn set_url<V>(&mut self, url: V)
    where
        V: Into<Option<String>>,
    {
        self.url = url.into();
    }

    pub fn set_description<V>(&mut self, description: V)
    where
        V: Into<Option<String>>,
    {
        self.description = description.into();
    }
}



impl ToXml for PodcastFunding {
    fn to_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), XmlError> {
        let name = "podcast:funding";
        let mut element = BytesStart::new(name);

        element.push_attribute(("url", self.url.as_str()));
        writer.write_event(Event::Start(element))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(&self.description)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(b"podcast:funding")))?;

        Ok(())
    }
}

