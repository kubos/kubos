/// Definitions of external C interfaces
/// and safe Rust wrappers

/// Bring in C functions from kubos-hal-iobc
extern "C" {
    pub fn supervisor_reset() -> bool;
}

/// Create safe Rust wrapper
pub fn k_supervisor_reset() -> bool {
    unsafe { supervisor_reset() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset() {
        assert_eq!(k_supervisor_reset(), false);
    }
}
