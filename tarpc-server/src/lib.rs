#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn hello(req: i32) -> i32;
}