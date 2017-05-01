use env;

pub fn get_env_var(key: String) -> Option<String> {
    for (k, v) in env::vars() {
        if k == key {
            return Some(v);
        }
    }
    None
}