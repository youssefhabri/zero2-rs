#[macro_export]
macro_rules! ok_or_return {
    ($e:expr) => {
        match $e {
            Ok(value) => value,
            Err(why) => {
                error!("{}", why);
                return;
            }
        };
    };
}

#[macro_export]
macro_rules! match_send {
    ($context:expr, $message:expr, $embed:expr) => {
        let sending = $message
            .channel_id
            .send_message($context, |m| {
                m.embed(|embed| {
                    embed.clone_from($embed);

                    embed
                })
            })
            .await;
        if let Err(why) = sending {
            error!("{}", why);
        }
    };
}
