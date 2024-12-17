use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{
    httprequest::{self, HttpRequest},
    httpresponse::HttpResponse,
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        let mut _res = HttpResponse::default();
        match req.method {
            httprequest::Method::Get => {
                let route: Vec<&str> = req.path.split("/").collect();
                match route[1] {
                    "api" => _res = WebServiceHandler::handle(&req),
                    _ => _res = StaticPageHandler::handle(&req),
                }
            }
            _ => _res = PageNotFoundHandler::handle(&req),
        }
        let _ = _res.send_response(stream);
    }
}
