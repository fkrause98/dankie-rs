use crate::{
    contexts::fields::{self, Album, AnyText, Caption},
    types::{self, message::Text},
};

media_message! {
    struct Video {
        /// The video.
        video: types::Video,
        /// The caption of the video.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<String>,
    } -> EventLoop::video

    fn new(caption: Text, media_group_id: Option<String>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}

impl fields::Video for Video {
    #[must_use]
    fn video(&self) -> &types::Video {
        &self.video
    }
}

impl Caption for Video {
    #[must_use]
    fn caption(&self) -> &Text {
        &self.caption
    }
}

impl AnyText for Video {
    #[must_use]
    fn text(&self) -> &Text {
        &self.caption
    }
}

impl Album for Video {
    #[must_use]
    fn media_group_id(&self) -> Option<&str> {
        self.media_group_id.as_ref().map(String::as_ref)
    }
}
