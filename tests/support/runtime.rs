use std::future::Future;

pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("test runtime should be created")
        .block_on(future)
}
