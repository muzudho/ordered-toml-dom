use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn random_name() -> String {
    format!(
        "snapshot_{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect::<String>()
    )
}

/*
pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
*/
