extern "C" {
    pub fn init_device();

    pub fn terminate_device();
}

pub fn k_init_device() {
    unsafe {
        init_device();
    }
}

pub fn k_terminate_device() {
    unsafe {
        terminate_device();
    }
}
