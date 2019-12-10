use core::convert::From;

use cortex_m::{interrupt::Nr, peripheral::scb::SystemHandler, self};

const SCB_AIRCR_VECTKEY_POS: u32 = 16;
const SCB_AIRCR_VECTKEY_MSK: u32 = 0xFFFF << SCB_AIRCR_VECTKEY_POS;
const SCB_AIRCR_PRIGROUP_POS: u32 = 8;
const SCB_AIRCR_PRIGROUP_MSK: u32 = 7 << SCB_AIRCR_PRIGROUP_POS;

const NVIC_PRIO_BITS: u32 = 3;

struct IRQ(u8);
impl From<i32> for IRQ {
    fn from(other: i32) -> Self {
        IRQ(other as u8)
    }
}

unsafe impl Nr for IRQ {
    fn nr(&self) -> u8 {
        self.0
    }
}

pub fn set_priority_grouping(periph: &cortex_m::Peripherals, priority_group: u32) {
    let mut reg_value = periph.SCB.aircr.read();
    let priority_group = priority_group & 0x07; // only 0-7 are used
    reg_value &= !(SCB_AIRCR_VECTKEY_MSK | SCB_AIRCR_PRIGROUP_MSK);
    reg_value =
        reg_value | (0x5FA << SCB_AIRCR_VECTKEY_POS) | (priority_group << SCB_AIRCR_PRIGROUP_POS);
    unsafe {
        periph.SCB.aircr.write(reg_value);
    }
}

pub fn set_priority(periph: &mut cortex_m::Peripherals, irq: i32, prio: u32) {
    if irq >= 0 {
        unsafe {
            let irq: IRQ = irq.into();
            periph.NVIC.set_priority(irq, prio as u8)
        }
    } else {
        unsafe {
            // values are from cortex_m::peripheral::scb::SystemHandler::index()
            let handler: SystemHandler = match (irq as u32) & 0xf {
                4 => SystemHandler::MemoryManagement,
                5 => SystemHandler::BusFault,
                6 => SystemHandler::UsageFault,
                11 => SystemHandler::SVCall,
                12 => SystemHandler::DebugMonitor,
                14 => SystemHandler::PendSV,
                15 => SystemHandler::SysTick,
                _ => panic!("Can't be any other value"),
            };
            periph.SCB.set_priority(handler, prio as u8)
        }
    }
}

pub fn encode_priority(priority_group: u32, preempt_priority: u32, sub_priority: u32) -> u32 {
    let priority_group = priority_group & 0x7;
    let preempt_priority_bits = if (7 - priority_group) > NVIC_PRIO_BITS {
        NVIC_PRIO_BITS
    } else {
        7 - priority_group
    };
    let sub_priority_bits = if (priority_group + NVIC_PRIO_BITS) < 7 {
        0
    } else {
        (priority_group - 7) + NVIC_PRIO_BITS
    };
    ((preempt_priority & (1 << (preempt_priority_bits)) - 1) << sub_priority_bits)
        | (sub_priority & (1 << sub_priority_bits) - 1)
}
