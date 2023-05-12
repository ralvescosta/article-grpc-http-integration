use crate::controllers::basic;
use actix_web::web::{self, ServiceConfig};
use http_components::CustomServiceConfigure;

pub fn basic_route() -> CustomServiceConfigure {
    CustomServiceConfigure::new(|cfg: &mut ServiceConfig| {
        cfg.service(web::scope("v1/basic").service(basic::create));
    })
}
