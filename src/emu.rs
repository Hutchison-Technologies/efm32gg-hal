//! EMU (Energy Management Unit)

use registers;

pub trait EMUExt {
    fn constrain(self) -> Emu;
}

impl EMUExt for registers::EMU {
    fn constrain(self) -> Emu {
        Emu { _private: () }
    }
}

pub struct Ems {
    pub power_cfg: PwrCfg,
    pub dcdc_ctrl: DCDCCtrl,
    pub status: Status,
}

pub struct Emu {
    // Just make sure this can't be created from outside; becomes obsolete when there are other
    // non-pub fields.
    _private: (),
}

pub struct DCDCCtrl {
    _private: (),
}

impl DCDCCtrl {
    pub fn bypass_mode(&mut self) {
        unsafe {
            let emu = &*registers::EMU::ptr();
            emu.dcdcctrl.write(|w| w.dcdcmode().bypass());
        }
    }
}

pub struct PwrCfg {
    _private: (),
}

impl PwrCfg {
    pub fn enable(&mut self) {
        unsafe {
            let emu = &*registers::EMU::ptr();
            emu.pwrcfg.write(|w| w.pwrcfg().dcdctodvdd());
        }
    }
}

pub struct Status {
    _private: (),
}

impl Status {
    pub fn is_vscale_busy(&self) -> bool {
        unsafe {
            let emu = &*registers::EMU::ptr();
            emu.status.read().vscalebusy().bit()
        }
    }

    pub fn is_vscale2(&self) -> bool {
        unsafe {
            let emu = &*registers::EMU::ptr();
            emu.status.read().vscale().is_vscale2()
        }
    }
}

impl Emu {
    pub fn split(self) -> Ems {
        Ems {
            power_cfg: PwrCfg { _private: () },
            dcdc_ctrl: DCDCCtrl { _private: () },
            status: Status { _private: () },
        }
    }
}
