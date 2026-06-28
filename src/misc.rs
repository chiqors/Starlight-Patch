use std::ffi::CString;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;
use colored::{Color, Colorize};
use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{GetModuleFileNameW, GetModuleHandleA, GetProcAddress};
use crate::GAME_BASE;
use crate::scanner::scan_ga_section;

type PtrToStringAnsiFn = unsafe extern "C" fn(*const u8) -> usize;

const PTR_TO_STRING_ANSI: &str = "56 48 83 EC 20 48 85 C9 74 ? 48 89 CE E8 ? ? ? ? 48 89 F1 89 C2";
const LOGO: &str = r#"
      _____ __             ___       __    __
     / ___// /_____ ______/ (_)___ _/ /_  / /_
     \__ \/ __/ __ `/ ___/ / / __ `/ __ \/ __/
    ___/ / /_/ /_/ / /  / / / /_/ / / / / /_
   /____/\__/\__,_/_/  /_/_/\__, /_/ /_/\__/
                           /____/
"#;

pub fn print_banner() {
    for line in LOGO.lines() {
        if is_wine() {
            println!("{}", line);
        } else {
            println!("{}", line.color(Color::TrueColor {
                r: 134,
                g: 121,
                b: 247,
            }));
        }
    }
}

pub fn is_wine() -> bool {
    unsafe {
        let ntdll = GetModuleHandleA(PCSTR(b"ntdll.dll\0".as_ptr()))
            .expect("ntdll.dll should always be loaded");

        GetProcAddress(ntdll, PCSTR(b"wine_get_version\0".as_ptr())).is_some()
    }
}

pub fn exe_dir() -> io::Result<PathBuf> {
    let mut buf = vec![0u16; 260];

    let len = unsafe { GetModuleFileNameW(Option::from(HMODULE::default()), &mut buf) } as usize;

    if len == 0 {
        return Err(io::Error::last_os_error());
    }

    buf.truncate(len);

    let mut path = PathBuf::from(String::from_utf16_lossy(&buf));
    path.pop();
    Ok(path)
}

pub fn create_il2cpp_string(text: &str) -> usize {
    static PTR_TO_STRING_ANSI_FN: OnceLock<PtrToStringAnsiFn> = OnceLock::new();

    let func = *PTR_TO_STRING_ANSI_FN.get_or_init(|| unsafe {
        std::mem::transmute::<usize, PtrToStringAnsiFn>(
            scan_ga_section(PTR_TO_STRING_ANSI).unwrap() + *GAME_BASE,
        )
    });

    let text = CString::new(text).unwrap();
    unsafe { func(text.as_c_str().to_bytes_with_nul().as_ptr()) }
}