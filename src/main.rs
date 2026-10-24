use darkforest::{context, db, engine, pages};
use log::Level;
use std::sync::{Arc, Mutex};
use wgui::Wgui;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let engine = Arc::new(Mutex::new(engine::Darkforest::starter()));
    let db = {
        let engine = engine.lock().expect("darkforest actor mutex poisoned");
        db::DarkforestDb::from_game_state(engine.state())
    };
    let mut wgui = Wgui::new("0.0.0.0:4200".parse().unwrap()).with_db(db);
    wgui.mount_static_file("/favicon.ico", "server/assets/favicon.ico");
    wgui.set_ctx_state(context::SharedContext::new(engine));
    wgui.add_page::<pages::overview::Overview>("/");
    wgui.add_page::<pages::map::Map>("/map");
    wgui.add_page::<pages::buildings::Buildings>("/buildings");
    wgui.add_page::<pages::resources::Resources>("/resources");
    wgui.add_page::<pages::population::Population>("/population");
    wgui.add_page::<pages::units::Units>("/units");
    wgui.add_page::<pages::technology::Technology>("/technology");
    wgui.add_page::<pages::mailbox::Mailbox>("/mailbox");
    wgui.add_page::<pages::not_found::NotFound>("/*");
    wgui.run().await;
}
