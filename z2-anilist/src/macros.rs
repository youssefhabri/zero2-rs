#[macro_export]
macro_rules! make_response {
    ($name:ident, $inner:ident, $func:ident) => {
        pub type $name = AniListResponse<Single<$inner>>;

        impl $name {
            pub fn $func(&self) -> &Option<$inner> {
                &self.data.item
            }
        }
    };
}

#[macro_export]
macro_rules! make_paged_response {
    ($name:ident, $inner:ident, $func:ident) => {
        pub type $name = AniListResponse<Paged<$inner>>;

        impl $name {
            pub fn $func(&self) -> &Vec<$inner> {
                &self.data.page.items
            }
        }
    };
}
