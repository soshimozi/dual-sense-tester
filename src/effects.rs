// src/effects.rs

pub mod trigger {
    #[derive(Copy, Clone, Debug)]
    pub enum TriggerMode {
        Off,
        Rigid { start: u8, force: u8 },
        Pulse { start: u8, force: u8 },
        Slope { start: u8, end: u8 },
    }

    #[derive(Clone, Debug)]
    pub struct TriggerEffect {
        pub l2: [u8; 9],
        pub r2: [u8; 9],
    }

    impl TriggerEffect {
        pub fn new_shared(mode: TriggerMode) -> Self {
            let effect = build_effect(mode);
            Self {
                l2: effect,
                r2: effect,
            }
        }

        pub fn new_split(l2_mode: TriggerMode, r2_mode: TriggerMode) -> Self {
            Self {
                l2: build_effect(l2_mode),
                r2: build_effect(r2_mode),
            }
        }
    }

    fn build_effect(mode: TriggerMode) -> [u8; 9] {
        match mode {
            TriggerMode::Off => [0u8; 9],
            TriggerMode::Rigid { start, force } => [0x01, 0x01, start, force, 0, 0, 0, 0, 0],
            TriggerMode::Pulse { start, force } => [0x01, 0x02, start, force, 0, 0, 0, 0, 0],
            TriggerMode::Slope { start, end } => [0x01, 0x26, start, end, 0xFF, 0, 0, 0, 0],
        }
    }
}

pub mod led {
    #[derive(Copy, Clone, Debug)]
    pub struct LedEffect {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl LedEffect {
        pub fn new(red: u8, green: u8, blue: u8) -> Self {
            Self { red, green, blue }
        }

        pub fn purple() -> Self {
            Self { red: 128, green: 0, blue: 128 }
        }

        pub fn off() -> Self {
            Self { red: 0, green: 0, blue: 0 }
        }

        pub fn from_rgb_slice(rgb: &[u8; 3]) -> Self {
            Self {
                red: rgb[0],
                green: rgb[1],
                blue: rgb[2],
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ControllerEffect {
    pub triggers: trigger::TriggerEffect,
    pub led: led::LedEffect,
}

impl ControllerEffect {
    pub fn new_shared_trigger(mode: trigger::TriggerMode, led: led::LedEffect) -> Self {
        Self {
            triggers: trigger::TriggerEffect::new_shared(mode),
            led,
        }
    }

    pub fn new_split_trigger(
        l2: trigger::TriggerMode,
        r2: trigger::TriggerMode,
        led: led::LedEffect,
    ) -> Self {
        Self {
            triggers: trigger::TriggerEffect::new_split(l2, r2),
            led,
        }
    }

    pub fn apply_to_report(&self, report: &mut crate::report::DualSenseReport) {
        report.l2_effect.copy_from_slice(&self.triggers.l2);
        report.r2_effect.copy_from_slice(&self.triggers.r2);
        report.lightbar_red = self.led.red;
        report.lightbar_green = self.led.green;
        report.lightbar_blue = self.led.blue;
    }    
}
