use ilhook::x64::Registers;
use lazy_static::lazy_static;
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey;
use crate::patches::{Fireable, Patch};
use crate::{misc, scanner, GAME_BASE};
use crate::config::CONFIG;

pub struct Encryption;

const MHYRSA_PERFORM_CRYPTO_ACTION: &str = "E8 ? ? ? ? 48 83 C4 20 66 41 C7 06 30 82";
const KEY_SIGN_CHECK: &str = "E8 ? ? ? ? 48 83 C4 30 84 C0 74 ? 41 8B 04 24";
const KEY_SIGN_CHECK_OFFSET: usize = 0x5;
const SDK_UTIL_RSA_ENCRYPT: &str = "41 57 41 56 41 55 41 54 56 57 55 53 48 83 EC ? 49 89 D6 48 89 CE 48 8B 0D ? ? ? ? E8 ? ? ? ? 49 89 C5";
const KEY_SIZE: usize = 268;

lazy_static! {
    pub static ref PUBLIC_KEY: Vec<u8> = {
        let key = RsaPublicKey::from_public_key_pem(
            CONFIG.get().unwrap().encryption.check_sign_key.as_str(),
        )
        .unwrap();

        let der = key.to_pkcs1_der().unwrap();
        der.as_bytes()[2..].to_vec()
    };
}

impl Fireable for Patch<Encryption> {
    unsafe fn fire(&mut self) -> Result<(), ()> {
        let mhyrsa_perform_crypto_action = scanner::scan_ga_section_raw(MHYRSA_PERFORM_CRYPTO_ACTION)
            .expect("MHYRSA_PERFORM_CRYPTO_ACTION failed! Does the patch need to be updated?")
            + *GAME_BASE;

        let key_sign_check = scanner::scan_ga_section_raw(KEY_SIGN_CHECK)
            .expect("KEY_SIGN_CHECK failed! Does the patch need to be updated?")
            + *GAME_BASE
            + KEY_SIGN_CHECK_OFFSET;

        let sdk_util_rsa_encrypt = scanner::scan_ga_section(SDK_UTIL_RSA_ENCRYPT)
            .expect("SDKUtil::RSAEncrypt failed! Does the patch need to be updated?")
            + *GAME_BASE;

        self.hook_mgr.hook(mhyrsa_perform_crypto_action, on_perform_action);
        self.hook_mgr.hook(key_sign_check, after_key_sign_check);
        self.hook_mgr.hook(sdk_util_rsa_encrypt, on_sdk_rsa_encrypt);

        tracing::info!("Set up encryption patches!");
        Ok(())
    }
}

unsafe extern "win64" fn on_perform_action(reg: *mut Registers, _: usize) {
    if ((*reg).r8 as usize) - 3 == KEY_SIZE {
        std::ptr::copy_nonoverlapping(
            PUBLIC_KEY.as_ptr(),
            (*reg).r13 as *mut u8,
            PUBLIC_KEY.len()
        );

        tracing::debug!("Replaced signing key!")
    }
}

unsafe extern "win64" fn on_sdk_rsa_encrypt(reg: *mut Registers, _: usize) {
    (*reg).rcx =
        misc::create_il2cpp_string(&*CONFIG.get().unwrap().encryption.sdk_key) as u64;
}

unsafe extern "win64" fn after_key_sign_check(reg: *mut Registers, _: usize) {
    (*reg).rax = 1;
}