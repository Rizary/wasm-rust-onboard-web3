use dominator::{html, Dom};

pub fn render(slot: Option<&str>) -> Dom {
    html!("page-footer", {
        .child(html!("div", {
            .text("FOOTER")
        }))
    })
}