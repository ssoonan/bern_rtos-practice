//! Bern RTOS kernel configuration.
//!
//! This is the default kernel config. To apply your own config clone the this
//! crate into your project and apply a cargo path to override the default
//! config:
//! ```toml
//! # `Cargo.toml`
//! [patch.crates-io]
//! bern-conf = { path = "conf" }
//! ```
//!
//! [Example Configuration](../src/bern_conf/conf.rs.html#18-44)

#![no_std]

use bern_conf_type::*;
use bern_units::memory_size::Byte;

pub const CONF: Conf<1> = Conf {
    kernel: Kernel {
        priorities: 8,
        memory_size: Byte::from_kB(2),
    },

    shared: Shared {
        size: Byte::from_kB(64), // Shared memory 크기를 줄임
    },

    memory_map: MemoryMap {
        flash: Memory {
            link_name: "FLASH",
            start_address: 0x0800_0000,
            size: Byte::from_kB(512), // Flash 크기를 512KB로 수정
        },
        sram: Memory {
            link_name: "RAM",
            start_address: 0x2000_0000,
            size: Byte::from_kB(128), // RAM 크기를 128KB로 수정
        },
        peripheral: Memory {
            link_name: "",
            start_address: 0x4000_0000,
            size: Byte::from_MB(512), // 주변 장치 메모리는 유지
        },
        additional: [
            OptionalMemory {
                memory_type: MemoryType::Ram,
                location: MemoryLocation::External,
                link_name: "XRAM",
                start_address: 0xC000_0000,
                size: Byte::from_MB(32),
            }
        ], // 외부 RAM을 사용하지 않는다면 제거
    },

    data_placement: DataPlacement {
        kernel: "RAM",
        processes: "RAM",
        shared: "RAM",
    },
};
