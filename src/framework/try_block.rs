pub fn try_block<T>(callback: impl FnOnce() -> anyhow::Result<T>) -> anyhow::Result<T> {
    callback()
}
