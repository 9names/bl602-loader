#![no_std]
#![no_main]

use panic_never as _;
use bl602_rom_wrapper::sflash;

use sflash::{
    sf_cfg::SF_Ctrl_Set_Owner, SF_Ctrl_Mode_Type_SF_CTRL_QPI_MODE,
    SF_Ctrl_Owner_Type_SF_CTRL_OWNER_IAHB, SF_Ctrl_Owner_Type_SF_CTRL_OWNER_SAHB,
};

/// Erase the sector at the given address in flash
///
/// `Return` - 0 on success, 1 on failure.
#[no_mangle]
#[inline(never)]
pub extern "C" fn EraseSector(adr: u32) -> i32 {
    let mut cfg = sflash::flashconfig::winbond_80_ew_cfg();
    let adr = adr >> 12;
    match sflash::SFlash_Sector_Erase(&mut cfg, adr) {
        0 => 0,
        _ => 1,
    }
}

/// Initializes the microcontroller for Flash programming. Returns 0 on Success, 1 otherwise
///
/// This is invoked whenever an attempt is made to download the program to Flash.
///
///  # Arguments
///
/// `adr` - specifies the base address of the device.
///
/// `clk` - specifies the clock frequency for prgramming the device.
///
/// `fnc` - is a number: 1=Erase, 2=Program, 3=Verify, to perform different init based on command
#[no_mangle]
#[inline(never)]
pub extern "C" fn Init(_adr: u32, _clk: u32, _fnc: u32) -> i32 {
    // disable memory-mapped flash
    sflash::SFlash_Cache_Read_Disable();
    SF_Ctrl_Set_Owner(SF_Ctrl_Owner_Type_SF_CTRL_OWNER_SAHB);

    0
}

/// Write code into the Flash memory. Call this to download a program to Flash. Returns 0 on Success, 1 otherwise
///
/// As Flash memory is typically organized in blocks or pages, parameters must not cross alignment boundaries of flash pages.
/// The page size is specified in the struct FlashDevice with the value Program Page Size.
/// # Arguments
///
/// `adr` - specifies the start address of the page that is to be programmed. It is aligned by the host programming system to a start address of a flash page
///
/// `sz` -  specifies the data size in the data buffer. The host programming system ensures that page boundaries are not crossed
///
/// `buf` - points to the data buffer containing the data to be programmed
#[no_mangle]
#[inline(never)]
pub extern "C" fn ProgramPage(adr: u32, sz: u32, buf: *mut u8) -> i32 {
    let mut cfg = sflash::flashconfig::winbond_80_ew_cfg();
    match sflash::SFlash_Program(&mut cfg, SF_Ctrl_Mode_Type_SF_CTRL_QPI_MODE, adr, buf, sz) {
        0 => 0,
        _ => 1,
    }
}

/// De-initializes the microcontroller after Flash programming. Returns 0 on Success, 1 otherwise
///
/// This is invoked at the end of an erasing, programming, or verifying step.
///
///  # Arguments
///
/// `fnc` - is a number: 1=Erase, 2=Program, 3=Verify, to perform different de-init based on command
#[no_mangle]
#[inline(never)]
pub extern "C" fn UnInit(_fnc: u32) -> i32 {
    // Should we care about this? It should be set during reset anyway...
    SF_Ctrl_Set_Owner(SF_Ctrl_Owner_Type_SF_CTRL_OWNER_IAHB);

    0
}

const fn sectors() -> [FlashSector; 512] {
    let mut sectors = [FlashSector::default(); 512];

    sectors[0] = FlashSector {
        size: 0x1000,
        address: 0x0,
    };
    sectors[1] = SECTOR_END;

    sectors
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
#[link_section = "DeviceData"]
pub static FlashDevice: FlashDeviceDescription = FlashDeviceDescription {
    vers: 0x0101,
    dev_name: [0u8; 128],
    dev_type: 5,
    dev_addr: 0x2300_0000,
    device_size: 0x1e8480,
    page_size: 256,
    _reserved: 0,
    empty: 0xff,
    program_time_out: 5,
    erase_time_out: 20000,
    flash_sectors: sectors(),
};

#[repr(C)]
pub struct FlashDeviceDescription {
    vers: u16,
    dev_name: [u8; 128],
    dev_type: u16,
    dev_addr: u32,
    device_size: u32,
    page_size: u32,
    _reserved: u32,
    empty: u8,
    program_time_out: u32,
    erase_time_out: u32,

    flash_sectors: [FlashSector; 512],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FlashSector {
    size: u32,
    address: u32,
}

impl FlashSector {
    const fn default() -> Self {
        FlashSector {
            size: 0,
            address: 0,
        }
    }
}

const SECTOR_END: FlashSector = FlashSector {
    size: 0xffff_ffff,
    address: 0xffff_ffff,
};
