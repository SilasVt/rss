// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.
// Modified for Podcast namespace in 2023 by Silas Vogt

use std::collections::BTreeMap;
use std::io::Write;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Error as XmlError;
use quick_xml::Writer;
/*
use super::{parse_categories, parse_image, parse_owner, NAMESPACE};
use crate::extension::itunes::{ITunesCategory, ITunesOwner};
use crate::extension::util::remove_extension_value;
use crate::extension::Extension;
use crate::toxml::{ToXml, WriterExt};

/// An iTunes channel element extension.
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
*/
pub struct PodcastChannelExtension {
    ///Phase 1 tags:
    /// Specifies if the feed is locked or not. Telling platforms if they are allowed to import the feed or not.
    pub locked: Option<String>,
    // This tag lists possible donation links for the podcast.
    //pub funding: Option<String>,


    //Phase 2 tags:
    // Specifies people like for example the host, guest, or other roles on the podcast.
    //pub person: Option<String>,
    // Specifies the location the podcast is about.
    //pub location: Option<String>,
    // Specifies the location of an audio or video file to be used as a trailer


    //Phase 3 tags:
    //Defines trailers for the feed.
    //pub trailer: Vec<PodcastTrailer>,
    // Defines a license that is applied to the audio/video content of the podcast as a whole.
    //pub license: Option<PodcastLicense>,
    // The Unique Identifier code for the podcast.
    //pub guid: Option<String>,

    //Phase 4 tags:
    // Define the medium of the feed, examples are podcast, music, video, film, audiobook and more.
    //pub medium: Option<String>,
    //Adds one or more square images to the channel
    //pub images Option<podcast_images>,

    // Define a payment layer, transport method and suggested amount of
    //pub value: Option<String>,
    //Essentially the same as a regular item except for livestreamed content.
    //pub liveItem: Vec<PodcastLiveItem>,

    //Phase 5 tags:
    // Allows the podcaster to define which services are not allowed to list the feed.
    //pub block: Vec<PodcastBlock>,

    //Phase 6 tags:
    //free-form text modeled after DNS TXT record. Meant for niche usecases that dont need special tags.
    //pub txt: Vec<PodcastTxt>,
    // Allows for the podcaster to point to another feed or an item in another feed.
    //pub remoteItem: Vec<PodcastRemoteItem>,
    //Lets the podcaster recommend other shows to the listeners using feedGuids.
    //pub podRoll: Option<PodcastPodRoll>,
    //Specifies the upload freqeuncy or status of the feed.
    //pub updateFrequency: Option<PodcastUpdateFrequency>,
    // Does the feed use the podping notification network.
    //pub podping: Option<PodcastPodping>,

}


