extern crate futures;
extern crate regex;
extern crate telegram_bot;
extern crate tokio_core;

use futures::Stream;
use regex::Regex;
use std::env;
use telegram_bot::*;
use tokio_core::reactor::Core;

fn process(api: &Api, message: Message, users: Vec<User>, re: &Regex) {
    let chat_id = message.to_source_chat();

    users.into_iter().for_each(|user| {
        if re.is_match(&user.first_name) {
            api.spawn(DeleteMessage::new(chat_id, &message));
            api.spawn(KickChatMember::new(chat_id, &user.id));
        }
    });
}

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let name_re = env::var("NAME_REGEX").unwrap();

    let api = Api::configure(token).build(core.handle()).unwrap();
    let re = Regex::new(&name_re).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::NewChatMembers { data } = message.clone().kind {
                process(&api, message, data, &re);
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
