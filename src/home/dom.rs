use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use super::Home;
use crate::{router::Router, common::header};

use std::sync::Arc;

impl Home {
    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self.clone();
        html!("div", {
            .children(&mut [
                html!("div", {
                    .child(
                        header::render(Arc::new(header::Header::new(self.app.clone())))
                    )
                }),
                self.render_hero(),
                self.render_showcase(),
                self.render_about(),
                self.render_roadmap(),
                self.render_community(),
                self.render_newsletter(),
                self.render_team(),
            ])
        })
    }
    
    fn render_hero(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("hero")
                })
            ])
            
        })
    }

    fn render_showcase(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("showcase")
                })
            ])
            
        })
    }

    fn render_about(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("about")
                })
            ])
            
        })
    }

    fn render_roadmap(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("roadmap")
                })
            ])
            
        })
    }

    fn render_community(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("community")
                })
            ])
            
        })
    }

    fn render_newsletter(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("newsletter")
                })
            ])
            
        })
    }

    fn render_team(self: &Arc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("div", {
                    .text("team")
                })
            ])
            
        })
    }
}