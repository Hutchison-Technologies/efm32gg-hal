//! MSC (Mass Storage Controller)

use registers;

pub struct Msc {
    pub status: Status,
    pub lock: Lock,
    pub write_ctrl: WriteCtrl,
    pub write_cmd: WriteCmd,
    pub addrb: AddrB,
    pub wdata: WData,
    pub cachecmd: CacheCmd,
    pub ctrl: Ctrl,
}

pub trait MSCExt {
    fn split(self) -> Msc;
}

impl MSCExt for registers::MSC {
    fn split(self) -> Msc {
        Msc {
            status: Status { _private: () },
            lock: Lock { _private: () },
            write_ctrl: WriteCtrl { _private: () },
            write_cmd: WriteCmd { _private: () },
            addrb: AddrB { _private: () },
            wdata: WData { _private: () },
            cachecmd: CacheCmd { _private: () },
            ctrl: Ctrl { _private: () },
        }
    }
}

pub struct Ctrl {
    _private: (),
}

impl Ctrl {
    pub fn init(&mut self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.ctrl.write(|w| w.bits(1u32  << 8));
        }
    }
}

pub struct CacheCmd {
    _private: (),
}

impl CacheCmd {
    pub fn invalidate_cache(&self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.cachecmd.write(|w| w.invcache().bit(true));
        }
    }
}

pub struct Status {
    _private: (),
}

impl Status {
    pub fn is_storage_busy(&self) -> bool {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.status.read().busy().bit()
        }
    }

    pub fn is_invalid_addr(&self) -> bool {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.status.read().invaddr().bit()
        }
    }

    pub fn is_wdata_ready(&self) -> bool {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.status.read().wdataready().bit()
        }
    }

    pub fn is_word_timeout(&self) -> bool {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.status.read().wordtimeout().bit()
        }
    }

    pub fn is_storage_locked(&self) -> bool {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.status.read().locked().bit()
        }
    }
}

pub struct Lock {
    _private: (),
}

impl Lock {
    pub fn unlock(&mut self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.lock.write(|w| w.lockkey().unlocked());
        }
    }

    pub fn lock(&mut self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.lock.write(|w| w.lockkey().locked());
        }
    }

    pub fn is_locked(&self) -> bool {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.lock.read().lockkey().is_locked()
        }
    }
}

pub struct WriteCtrl {
    _private: (),
}

impl WriteCtrl {
    pub fn disable(&mut self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writectrl.write(|w| w.wren().bit(false));
        }
    }

    pub fn enable(&mut self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writectrl.write(|w| w.wren().bit(true));
        }
    }
}

pub struct AddrB {
    _private: (),
}

impl AddrB {
    pub fn set_addr(&mut self, addr: u32) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.addrb.write(|w| w.addrb().bits(addr));
        }
    }
}

pub struct WriteCmd {
    _private: (),
}

impl WriteCmd {
    pub fn load_addr(&self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writecmd.write(|w| w.laddrim().bit(true));
        }
    }

    pub fn write_once(&self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writecmd.write(|w| w.writeonce().bit(true));
        }
    }

    pub fn write_trig(&self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writecmd.write(|w| w.writetrig().bit(true));
        }
    }

    pub fn write_end(&self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writecmd.write(|w| w.writeend().bit(true));
        }
    }

    pub fn erase_page(&self) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.writecmd.write(|w| w.erasepage().bit(true));
        }
    }
}

pub struct WData {
    _private: (),
}

impl WData {
    pub fn write_data(&mut self, value: u32) {
        unsafe {
            let msc = &*registers::MSC::ptr();
            msc.wdata.write(|w| w.wdata().bits(value));
        }
    }
}
