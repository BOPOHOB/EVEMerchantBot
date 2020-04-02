extern crate dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    run().await;
}

fn get_env_variable(key: &str)->String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("{} {}", key, e),
    }
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting guess_a_number_bot!");
    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                let eve_client_id: String = get_env_variable("EVE_CLIENT_ID");
                let redirect_uri = get_env_variable("REDIRECT_URI");
                let url = format!("https://login.eveonline.com/oauth/authorize?response_type=code&redirect_uri={}&state={}&client_id={}&scope=esi-markets.read_character_orders.v1", redirect_uri, message.update.chat_id(), eve_client_id);
                let text = format!("Привет. Чтобы бот мог следить за ситуацией на рынке позволь ему получать информацию из магазина от твоего лица. Для этого перейди по [ссылке]({}) и разреши приложению авторизоваться", url);
                message.answer(text).parse_mode(teloxide::types::ParseMode::Markdown).send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}

