pub struct InterruptController {
    interrupt_master_enable: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            interrupt_master_enable: false,
        }
    }
}
