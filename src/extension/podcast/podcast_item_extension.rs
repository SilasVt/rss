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


pub struct PodcastItemExtension {

    //Phase 1 tags:
    //Transcript or closed captions file for episode.
    //pub transcript: Option<PodcastTranscript>,
    //Link to external chapters file.
    //pub chapters: Option<PodcastChapters>,
    //Points to a part of an episode to be used as a trailer/soundbite.
    //pub soundbite: Option<Podcast>,

    //Phase 2 tags:
    //Identify and display people related to an episode.
    //pub person: Vec<PodcastPerson>,
    //Describe the location the episode is focused on.
    //pub location: Option<PodcastLocation>,
    //Identify the season an episode is in.
    //pub season: Option<PodcastSeason>,
    //Identify which episode it is.
    //pub episode: Option<PodcastEpisode>,

    //Phase 3 tags:
    //// Defines a license that is applied to the audio/video content of the podcast as a whole.
    //pub license: Option<PodcastLicense>,
    //Alternate Enclosures for an item. (other codec, video, etc)
    //pub alternate_enclosure Vec<PodcastAlternateEnclosure>,

    //Phase 4 tags:
    //Adds one or more square images to a item
    //pub images Option<podcast_images>,

    //Phase 5 tags:
    //Define a social media post for people to comment on the episode.
    //pub Vec<PodcastSocialInteract>,

    //Phase 6 tags:
    // free-form text modeled after DNS TXT record. Meant for niche usecases that dont need special tags.
    //pub txt: Vec<PodcastTxt>,

}
