use chrono::Weekday;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AniListMediaView {
    Overview,
    Stats,
    Recommendations,
    // TODO add Relations (give it the "F" emoji)
}

impl Default for AniListMediaView {
    fn default() -> Self {
        AniListMediaView::Overview
    }
}

impl FromStr for AniListMediaView {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let view = match s.as_str() {
            "-o" | "-overview" => AniListMediaView::Overview,
            "-r" | "-recommendations" => AniListMediaView::Recommendations,
            "-s" | "-stats" => AniListMediaView::Stats,
            _ => return Err(()),
        };

        Ok(view)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AniListUserView {
    Overview,
    Stats,
    Favourites,
}

impl Default for AniListUserView {
    fn default() -> Self {
        AniListUserView::Overview
    }
}

impl FromStr for AniListUserView {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let view = match s.as_str() {
            "-o" | "-overview" => AniListUserView::Overview,
            "-f" | "-favourites" => AniListUserView::Favourites,
            "-s" | "-stats" => AniListUserView::Stats,
            _ => return Err(()),
        };

        Ok(view)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AniListCharacterView {
    Overview,
    RelatedAnime,
    RelatedManga,
}

impl Default for AniListCharacterView {
    fn default() -> Self {
        AniListCharacterView::Overview
    }
}

impl FromStr for AniListCharacterView {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let view = match s.as_str() {
            "-o" | "-overview" => AniListCharacterView::Overview,
            "-a" | "-anime" => AniListCharacterView::RelatedAnime,
            "-m" | "-manga" => AniListCharacterView::RelatedManga,
            _ => return Err(()),
        };

        Ok(view)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AniListStaffView {
    Overview,
    RelatedAnime,
    RelatedManga,
}

impl Default for AniListStaffView {
    fn default() -> Self {
        AniListStaffView::Overview
    }
}

impl FromStr for AniListStaffView {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let view = match s.as_str() {
            "-o" | "-overview" => AniListStaffView::Overview,
            "-a" | "-anime" => AniListStaffView::RelatedAnime,
            "-m" | "-manga" => AniListStaffView::RelatedManga,
            _ => return Err(()),
        };

        Ok(view)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ALAiringScheduleView {
    Main,
    Schedule,
}

impl Default for ALAiringScheduleView {
    fn default() -> Self {
        ALAiringScheduleView::Main
    }
}

impl From<Option<Weekday>> for ALAiringScheduleView {
    fn from(weekday: Option<Weekday>) -> Self {
        match weekday {
            Some(_) => Self::Schedule,
            None => Self::Main,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AniListPaginationKind {
    AiringSchedule(ALAiringScheduleView),
    Character(AniListCharacterView),
    Media(AniListMediaView),
    User(AniListUserView),
    Staff(AniListStaffView),
    Studio,
}

impl AniListPaginationKind {
    pub(crate) fn airing_schedule_view(&self) -> Option<ALAiringScheduleView> {
        match &self {
            AniListPaginationKind::AiringSchedule(view) => Some(view.clone()),
            _ => None,
        }
    }
}
