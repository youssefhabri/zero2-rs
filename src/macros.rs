#[macro_export]
macro_rules! page_content {
    ($context:expr, $message_id:expr, $embed_fn:expr) => {{
        let data = $context.data.read();
        let paginator = data.get::<MessagePaginator>().unwrap();
        match paginator.get(&($message_id)) {
            Some(pagination) => {
                let content = pagination.pages[pagination.current_page].clone();
                let prefix = format!(
                    "Page: {}/{} | ",
                    pagination.current_page + 1,
                    pagination.pages.len()
                );

                match serde_json::from_str(&content) {
                    Ok(item) => Some($embed_fn(&item, prefix)),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }};
}

#[macro_export]
macro_rules! match_send {
    ($context:expr, $message:expr, $data:expr, $embed_builder:expr) => {
        if let Some(data) = $data {
            let _sending = ($message).channel_id.send_message(&($context).http, |m| {
                m.embed(|embed| {
                    embed.clone_from(&($embed_builder(&data, "".into())));

                    embed
                })
            });
        }
    };
}
