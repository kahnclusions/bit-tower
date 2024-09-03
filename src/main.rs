#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    server::serve().await;
}

#[cfg(not(feature = "ssr"))]
fn main() {
    println!("I'm not in SSR mode. Did you mean to run me in SSR mode? Exiting...");
}
