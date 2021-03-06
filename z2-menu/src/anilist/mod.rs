mod airing_schedule;
mod character;
pub mod embeds;
mod media;
pub mod pagination;
mod staff;
mod studio;
mod types;
mod user;

pub use pagination::AniListPagination;
pub use types::{
    AniListCharacterView, AniListMediaView, AniListPaginationKind, AniListStaffView,
    AniListUserView,
};
