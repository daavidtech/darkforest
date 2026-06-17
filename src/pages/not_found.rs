use crate::context::SharedContext;
use crate::db::DarkforestDb;
use async_trait::async_trait;
use std::sync::Arc;
use wgui::wgui_controller;
use wgui::wui::runtime::{Component, Ctx, MountResult, RouteContext};

#[derive(Debug, Clone, wgui::WguiModel)]
pub struct NotFoundState {
    pub path: String,
}

pub struct NotFound {
    ctx: Arc<Ctx<SharedContext, DarkforestDb>>,
}

#[wgui_controller]
impl NotFound {
    pub fn new(ctx: Arc<Ctx<SharedContext, DarkforestDb>>) -> Self {
        Self { ctx }
    }

    pub fn state(&self) -> NotFoundState {
        NotFoundState {
            path: self
                .ctx
                .route()
                .map(|route| route.path)
                .unwrap_or_else(|| "/".to_string()),
        }
    }

    pub fn title(&self) -> String {
        "Darkforest | Not Found".to_string()
    }

    pub fn open_overview(&mut self) {
        self.ctx.push_state("/");
    }

    pub fn open_map(&mut self) {
        self.ctx.push_state("/map");
    }

    pub fn open_buildings(&mut self) {
        self.ctx.push_state("/buildings");
    }

    pub fn open_resources(&mut self) {
        self.ctx.push_state("/resources");
    }

    pub fn open_population(&mut self) {
        self.ctx.push_state("/population");
    }

    pub fn open_units(&mut self) {
        self.ctx.push_state("/units");
    }

    pub fn open_technology(&mut self) {
        self.ctx.push_state("/technology");
    }

    pub fn open_mailbox(&mut self) {
        self.ctx.push_state("/mailbox");
    }
}

#[async_trait]
impl Component for NotFound {
    type Context = SharedContext;
    type Db = DarkforestDb;
    type Model = NotFoundState;

    async fn mount(
        ctx: Arc<Ctx<SharedContext, DarkforestDb>>,
        _route: RouteContext,
    ) -> MountResult<Self> {
        MountResult::Ready(Self::new(ctx))
    }

    fn render(&self, _ctx: &Ctx<SharedContext, DarkforestDb>) -> Self::Model {
        self.state()
    }

    fn unmount(self, _ctx: Arc<Ctx<SharedContext, DarkforestDb>>) {}
}
