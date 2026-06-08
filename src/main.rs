use backend_bootstrap::bootstrap::error::BootstrapError;



fn main() -> Result<(), BootstrapError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(backend_bootstrap::bootstrap::app::run())
        
}
