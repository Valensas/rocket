use rocket::{ get, http::Method, route::BoxFuture, Data, Request, Route };
use vaultrs::error::ClientError;

pub struct Endpoint {
    endpoint: String,
    method: String,
    function: for<'x, 'a> fn(&'x Request<'a>, Data<'x>) -> BoxFuture<'x>,
}

impl Clone for Endpoint {
    fn clone(&self) -> Self {
        Self {
            endpoint: self.endpoint.clone(),
            method: self.method.clone(),
            function: self.function.clone(),
        }
    }
}

impl Endpoint {
    pub fn new(
        endpoint: String,
        method: String,
        function: for<'x, 'a> fn(&'x Request<'a>, Data<'x>) -> BoxFuture<'x>
    ) -> Self {
        Self {
            endpoint,
            method,
            function,
        }
    }

    pub fn get_route(self) -> Route {
        match self.method.as_str() {
            "POST" => Route::new(Method::Post, self.endpoint.as_str(), slm),
            "GET" => Route::new(Method::Get, self.endpoint.as_str(), self.function),
            "DELETE" => Route::new(Method::Delete, self.endpoint.as_str(), self.function),
            "PUT" => Route::new(Method::Put, self.endpoint.as_str(), self.function),
            "PATCH" => Route::new(Method::Patch, self.endpoint.as_str(), self.function),
            met => { panic!("Method: {} is not valid", met) }
        }
    }
}
