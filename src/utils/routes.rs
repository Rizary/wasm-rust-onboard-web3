use serde::{Deserialize, Serialize};

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::Url;

#[derive(Debug, Clone)]
pub enum Route {
    NotFound,
    Home(HomeRoute),
}

#[derive(Debug, Clone)]
pub enum HomeRoute {
    Home,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl Route {
    pub fn redirect(self) {
        let location = web_sys::window().unwrap().location();
        let s: String = self.into();
        location.set_href(&s).unwrap();
    }

    pub fn push_state(self) {
        let history = web_sys::window().unwrap().history().unwrap();
        let url: String = self.into();
        let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
    }
    
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        let paths = url.pathname();
        let paths = paths.split('/').into_iter().skip(1).collect::<Vec<_>>();
        let paths = paths.as_slice();
        match paths {
            [""] => Self::Home(HomeRoute::Home),
            _ => Self::NotFound,
        }
    }
}

impl From<Route> for String {
    fn from(route: Route) -> Self {
        (&route).into()
    }
}

impl From<&Route> for String {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home(route) => match route {
                HomeRoute::Home => "/".to_string(),
            },
            Route::NotFound => "/404".to_string(),
        }
    }
}

// pub fn get_param_bool(param: &str) -> bool {
//     match get_param(param) {
//         None => false,
//         Some(value) => value == "true",
//     }
// }
// pub fn get_param_index(param: &str) -> Option<usize> {
//     get_param(param).and_then(|x| x.parse().ok())
// }

// pub fn get_param(param: &str) -> Option<String> {
//     let url: String = dominator::routing::url().get_cloned();
//     let url: web_sys::Url = web_sys::Url::new(&url).unwrap();
//     let params = url.search_params();

//     params.get(param)
// }
