mod doc;

pub use doc::Doc;

// logging for tests
#[cfg(test)]
pub(crate) mod logging {
    use std::sync::Once;

    static INIT_LOGGER: Once = Once::new();

    pub fn init_logger() {
        INIT_LOGGER.call_once(|| {
            env_logger::builder()
                .format_timestamp(None)
                .filter_level(log::LevelFilter::Debug)
                .is_test(true)
                .init();
        });
    }
}
