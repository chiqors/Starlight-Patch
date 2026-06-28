use std::ffi::c_void;
use windows::core::s;
use windows::Win32::System::Diagnostics::Debug::{WriteProcessMemory, IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS};
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

pub unsafe fn disable() {
    unsafe {
        let module = s!("ntdll.dll");
        let Ok(module) = GetModuleHandleA(module) else {
            tracing::error!("failed to get module handle for ntdll.dll");
            return;
        };

        let Some(nt_qs) = ({
            let proc_name = s!("NtQuerySection");
            GetProcAddress(module, proc_name)
        }) else {
            tracing::error!("failed to resolve NtQuerySection");
            return;
        };

        let Some(nt_pvm) = ({
            let proc_name = s!("NtProtectVirtualMemory");
            GetProcAddress(module, proc_name)
        }) else {
            tracing::error!("failed to resolve NtProtectVirtualMemory");
            return;
        };

        let nt_qs = nt_qs as *const u8;
        let immediate_value = (*nt_qs.offset(4)).wrapping_sub(1);
        patch_memory(
            nt_pvm as *const c_void,
            &[
                0x4C, 0x8B, 0xD1,
                0xB8,
                immediate_value
            ]
        );

        do_rwx();
    }
}

pub unsafe fn do_rwx() {
    unsafe {
        let base = GetModuleHandleA(None);
        let base_usize = base.unwrap().0 as usize;

        let dos_header = &*(base_usize as *const IMAGE_DOS_HEADER);
        let nt_headers = &*((base_usize + dos_header.e_lfanew as usize) as *const IMAGE_NT_HEADERS64);

        let section_count = nt_headers.FileHeader.NumberOfSections;
        let section_headers = (base_usize
            + dos_header.e_lfanew as usize
            + size_of::<IMAGE_NT_HEADERS64>()) as *const IMAGE_SECTION_HEADER;

        for i in 0..section_count {
            let sec = &*section_headers.add(i as usize);

            let sec_addr = base_usize + sec.VirtualAddress as usize;
            let sec_size = sec.Misc.VirtualSize as usize;

            if sec_size == 0 {
                continue;
            }

            let mut old: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);

            VirtualProtect(
                sec_addr as *mut c_void,
                sec_size,
                PAGE_EXECUTE_READWRITE,
                &mut old,
            ).unwrap();
        }
    }
}

fn patch_memory(addr: *const c_void, new: &[u8]) {
    unsafe {
        let mut flags: PAGE_PROTECTION_FLAGS = Default::default();
        if let Err(err) = VirtualProtect(addr, new.len(), PAGE_EXECUTE_READWRITE, &mut flags) {
            tracing::error!(?err, "VirtualProtect (initial) failed");
            return;
        }

        let process_id = std::process::id();
        let Ok(process) = OpenProcess(PROCESS_ALL_ACCESS, false, process_id) else {
            tracing::error!("OpenProcess failed for pid {}", process_id);
            return;
        };

        let bytes: *const u8 = new.as_ptr();
        let bytes = bytes as *const c_void;

        if let Err(err) = WriteProcessMemory(process, addr, bytes, new.len(), None) {
            tracing::error!(?err, "WriteProcessMemory failed");
            return;
        }

        if let Err(error) = VirtualProtect(addr, new.len(), flags, &mut flags) {
            tracing::error!(?error, "VirtualProtect (restore) failed");
            return;
        }
    }
}