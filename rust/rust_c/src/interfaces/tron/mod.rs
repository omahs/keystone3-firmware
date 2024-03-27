pub mod structs;

use crate::interfaces::companion_app;
use crate::interfaces::errors::RustCError;
use crate::interfaces::structs::{SimpleResponse, TransactionCheckResult, TransactionParseResult};
use crate::interfaces::tron::structs::DisplayTron;
use crate::interfaces::types::{PtrBytes, PtrString, PtrT, PtrUR};
use crate::interfaces::ur::UREncodeResult;
use crate::interfaces::utils::{convert_c_char, recover_c_char};
use alloc::boxed::Box;

use alloc::slice;
use cty::c_char;

#[no_mangle]
pub extern "C" fn tron_check_companion_app(
    ptr: PtrUR,
    master_fingerprint: PtrBytes,
    length: u32,
    x_pub: PtrString,
) -> PtrT<TransactionCheckResult> {
    companion_app::check(ptr, master_fingerprint, length, x_pub)
}

#[no_mangle]
pub extern "C" fn tron_parse_companion_app(
    ptr: PtrUR,
    master_fingerprint: PtrBytes,
    length: u32,
    x_pub: PtrString,
) -> *mut TransactionParseResult<DisplayTron> {
    if length != 4 {
        return TransactionParseResult::from(RustCError::InvalidMasterFingerprint).c_ptr();
    }
    companion_app::build_payload(ptr).map_or_else(
        |e| TransactionParseResult::from(e).c_ptr(),
        |payload| {
            companion_app::build_parse_context(master_fingerprint, x_pub).map_or_else(
                |e| TransactionParseResult::from(e).c_ptr(),
                |context| {
                    app_tron::parse_raw_tx(payload, context).map_or_else(
                        |e| TransactionParseResult::from(e).c_ptr(),
                        |res| {
                            TransactionParseResult::success(Box::into_raw(Box::new(
                                DisplayTron::from(res),
                            )))
                            .c_ptr()
                        },
                    )
                },
            )
        },
    )
}

#[no_mangle]
pub extern "C" fn tron_sign_companion_app(
    ptr: PtrUR,
    master_fingerprint: PtrBytes,
    length: u32,
    x_pub: PtrString,
    cold_version: i32,
    seed: PtrBytes,
    seed_len: u32,
) -> *mut UREncodeResult {
    let seed = unsafe { slice::from_raw_parts(seed, seed_len as usize) };
    companion_app::sign(ptr, master_fingerprint, length, x_pub, cold_version, seed)
}

#[no_mangle]
pub extern "C" fn tron_get_address(
    hd_path: PtrString,
    x_pub: PtrString,
) -> *mut SimpleResponse<c_char> {
    let x_pub = recover_c_char(x_pub);
    let hd_path = recover_c_char(hd_path);
    let address = app_tron::get_address(hd_path, &x_pub);
    match address {
        Ok(result) => SimpleResponse::success(convert_c_char(result) as *mut c_char).simple_c_ptr(),
        Err(e) => SimpleResponse::from(e).simple_c_ptr(),
    }
}