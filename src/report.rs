#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DualSenseReport {
    pub report_id: u8,      // 0
    pub flags: u8,          // 1
    pub enable_bits: u8,    // 2
    _reserved0: u8,         // 3
    pub rumble_right: u8,   // 4
    pub rumble_left: u8,    // 5
    pub lightbar_red: u8,   // 6
    pub lightbar_green: u8, // 7
    pub lightbar_blue: u8,  // 8
    _reserved1: [u8; 2],    // 9-10
    pub l2_effect: [u8; 9], // 11-19
    _reserved2: [u8; 2],    // 20-21
    pub r2_effect: [u8; 9], // 22-30
    _reserved3: [u8; 47],   // 31-77
}

impl Default for DualSenseReport {
    fn default() -> Self {
        Self {
            report_id: 0x02,
            flags: 0x00,
            enable_bits: 0x00,
            _reserved0: 0,
            rumble_right: 0,
            rumble_left: 0,
            lightbar_red: 0,
            lightbar_green: 0,
            lightbar_blue: 0,
            _reserved1: [0; 2],
            l2_effect: [0; 9],
            _reserved2: [0; 2],
            r2_effect: [0; 9],
            _reserved3: [0; 47],
        }
    }
}

impl DualSenseReport {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const DualSenseReport as *const u8,
                std::mem::size_of::<DualSenseReport>(),
            )
        }
    }

    pub fn clear_triggers(&mut self) {
        self.l2_effect = [0; 9];
        self.r2_effect = [0; 9];
    }
}