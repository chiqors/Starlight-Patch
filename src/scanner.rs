use std::sync::OnceLock;
use memchr::memmem;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;

pub fn module_slice() -> &'static [u8] {
    static SLICE: OnceLock<&'static [u8]> = OnceLock::new();
    *SLICE.get_or_init(|| unsafe {
        let module = GetModuleHandleW(None).unwrap();
        let mut module_info = MODULEINFO {
            lpBaseOfDll: std::ptr::null_mut(),
            SizeOfImage: 0,
            EntryPoint: std::ptr::null_mut(),
        };

        GetModuleInformation(
            GetCurrentProcess(),
            module,
            &mut module_info,
            size_of::<MODULEINFO>() as u32,
        )
            .unwrap();

        std::slice::from_raw_parts(module.0 as *const u8, module_info.SizeOfImage as usize)
    })
}

unsafe fn find_section(base: *const u8, name: &[u8]) -> Option<(usize, usize)> {
    let e_lfanew = *(base.add(0x3C) as *const i32);
    let nt_headers = base.add(e_lfanew as usize);

    let signature = *(nt_headers as *const u32);
    if signature != 0x0000_4550 {
        return None;
    }

    let file_header = nt_headers.add(4);
    let number_of_sections = *(file_header.add(2) as *const u16);
    let size_of_optional_header = *(file_header.add(16) as *const u16);

    let section_table = file_header.add(20).add(size_of_optional_header as usize);

    for i in 0..number_of_sections as usize {
        let section = section_table.add(i * 40);
        let section_name = std::slice::from_raw_parts(section, 8);
        if &section_name[..name.len()] == name {
            let virtual_size = *(section.add(8) as *const u32) as usize;
            let virtual_address = *(section.add(12) as *const u32) as usize;
            return Some((virtual_address, virtual_size));
        }
    }
    None
}

fn find_named_section(name: &'static [u8]) -> Option<(usize, &'static [u8])> {
    unsafe {
        let full = module_slice();
        let base = full.as_ptr();
        find_section(base, name).map(|(rva, size)| {
            let size = size.min(full.len().saturating_sub(rva));
            (rva, std::slice::from_raw_parts(base.add(rva), size))
        })
    }
}

fn text_section() -> (usize, &'static [u8]) {
    static SECTION: OnceLock<(usize, &'static [u8])> = OnceLock::new();
    *SECTION.get_or_init(|| find_named_section(b".text").unwrap_or_else(|| (0, module_slice())))
}

fn il2cpp_section() -> Option<(usize, &'static [u8])> {
    static SECTION: OnceLock<Option<(usize, &'static [u8])>> = OnceLock::new();
    *SECTION.get_or_init(|| find_named_section(b"il2cpp"))
}

struct CompiledPattern {
    bytes: Vec<Option<u8>>,
    anchor_offset: usize,
    anchor: Vec<u8>,
}

fn compile_pattern(pat: &str) -> CompiledPattern {
    let bytes: Vec<Option<u8>> = pat
        .split_whitespace()
        .map(|tok| match tok {
            "?" | "??" => None,
            hex => Some(u8::from_str_radix(hex, 16).expect("invalid pattern byte")),
        })
        .collect();

    let (mut best_start, mut best_len, mut cur_start, mut cur_len) = (0, 0, 0, 0);
    for (i, b) in bytes.iter().enumerate() {
        if b.is_some() {
            if cur_len == 0 {
                cur_start = i;
            }
            cur_len += 1;
            if cur_len > best_len {
                best_len = cur_len;
                best_start = cur_start;
            }
        } else {
            cur_len = 0;
        }
    }

    let anchor = bytes[best_start..best_start + best_len]
        .iter()
        .map(|b| b.unwrap())
        .collect();

    CompiledPattern { bytes, anchor_offset: best_start, anchor }
}

#[inline]
fn matches_at(haystack: &[u8], start: usize, pattern: &[Option<u8>]) -> bool {
    pattern.iter().enumerate().all(|(i, b)| match b {
        Some(expected) => haystack[start + i] == *expected,
        None => true,
    })
}

fn find_pattern(haystack: &[u8], pat: &str) -> Option<usize> {
    let compiled = compile_pattern(pat);

    if compiled.anchor.is_empty() {
        return (0..=haystack.len().saturating_sub(compiled.bytes.len()))
            .find(|&start| matches_at(haystack, start, &compiled.bytes));
    }

    let finder = memmem::Finder::new(&compiled.anchor);
    let mut search_from = 0;

    while let Some(rel) = finder.find(&haystack[search_from..]) {
        let anchor_pos = search_from + rel;
        if anchor_pos >= compiled.anchor_offset {
            let start = anchor_pos - compiled.anchor_offset;
            if start + compiled.bytes.len() <= haystack.len()
                && matches_at(haystack, start, &compiled.bytes)
            {
                return Some(start);
            }
        }
        search_from = anchor_pos + 1;
        if search_from >= haystack.len() {
            break;
        }
    }
    None
}

#[inline]
fn resolve_call_target(slice: &[u8], local: usize) -> usize {
    match slice.get(local) {
        Some(&0xE8) => {
            let offset = i32::from_le_bytes(slice[local + 1..local + 5].try_into().unwrap());
            (local as isize + 5 + offset as isize) as usize
        }
        Some(&0x48) if slice.get(local + 1) == Some(&0x8B) => {
            let offset = i32::from_le_bytes(slice[local + 3..local + 7].try_into().unwrap());
            (local as isize + 7 + offset as isize) as usize
        }
        _ => local,
    }
}

pub fn scan_ga_section_raw(pat: &str) -> Option<usize> {
    let (text_rva, text_slice) = text_section();
    if let Some(local) = find_pattern(text_slice, pat) {
        return Some(text_rva + local);
    }

    if let Some((il2cpp_rva, il2cpp_slice)) = il2cpp_section() {
        if let Some(local) = find_pattern(il2cpp_slice, pat) {
            return Some(il2cpp_rva + local);
        }
    }

    None
}

pub fn scan_ga_section(pat: &str) -> Option<usize> {
    let (text_rva, text_slice) = text_section();
    if let Some(local) = find_pattern(text_slice, pat) {
        return Some(text_rva + resolve_call_target(text_slice, local));
    }

    if let Some((il2cpp_rva, il2cpp_slice)) = il2cpp_section() {
        if let Some(local) = find_pattern(il2cpp_slice, pat) {
            return Some(il2cpp_rva + resolve_call_target(il2cpp_slice, local));
        }
    }

    None
}