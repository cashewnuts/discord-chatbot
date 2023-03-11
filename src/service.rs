use lambda_runtime::tower;
use std::{
    fmt,
    future::Future,
    task::{Context, Poll},
};

#[derive(Copy, Clone)]
pub struct ServiceFn<'a, T, S> {
    f: T,
    service: &'a S,
}

impl<'a, T, S> ServiceFn<'a, T, S> {
    pub fn new(f: T, service: &'a S) -> Self {
        Self { f, service }
    }
}

impl<'a, T, S> fmt::Debug for ServiceFn<'a, T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceFn")
            .field("f", &format_args!("{}", std::any::type_name::<T>()))
            .finish()
    }
}

impl<'a, T, F, Request, R, E, S> tower::Service<Request> for ServiceFn<'a, T, S>
where
    T: FnMut(Request, &'a S) -> F,
    F: Future<Output = Result<R, E>>,
{
    type Response = R;
    type Error = E;
    type Future = F;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), E>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request) -> Self::Future {
        (self.f)(req, self.service)
    }
}
