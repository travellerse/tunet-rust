use std::{
    os::windows::prelude::{AsRawHandle, FromRawHandle, OwnedHandle},
    ptr::null_mut,
};
use widestring::{u16str, U16CString, U16String};
use windows::{
    core::*,
    Win32::{
        Foundation::HANDLE,
        Security::{DuplicateTokenEx, SecurityIdentification, TokenPrimary, TOKEN_ALL_ACCESS},
        System::{
            Environment::CreateEnvironmentBlock,
            RemoteDesktop::{
                WTSGetActiveConsoleSessionId, WTSQueryUserToken, WTSSendMessageW,
                WTS_CURRENT_SERVER_HANDLE,
            },
            Threading::{
                CreateProcessAsUserW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION,
                STARTF_USESHOWWINDOW, STARTUPINFOW,
            },
        },
        UI::WindowsAndMessaging::{MB_OK, MESSAGEBOX_RESULT, SW_HIDE},
    },
};

pub fn notify() -> Result<()> {
    unsafe {
        let session = WTSGetActiveConsoleSessionId();
        let mut token = HANDLE::default();
        WTSQueryUserToken(session, &mut token).ok()?;
        let mut dup_token = HANDLE::default();
        DuplicateTokenEx(
            token,
            TOKEN_ALL_ACCESS,
            None,
            SecurityIdentification,
            TokenPrimary,
            &mut dup_token,
        )
        .ok()?;
        let dup_token = OwnedHandle::from_raw_handle(dup_token.0 as _);
        let mut env = null_mut();
        CreateEnvironmentBlock(&mut env, HANDLE(dup_token.as_raw_handle() as _), false).ok()?;
        let mut si = STARTUPINFOW::default();
        si.cb = std::mem::size_of_val(&si) as _;
        si.dwFlags = STARTF_USESHOWWINDOW;
        si.wShowWindow = SW_HIDE.0 as _;
        let mut pi = PROCESS_INFORMATION::default();
        let app_name =
            U16CString::from_os_str(std::env::current_exe().unwrap().into_os_string()).unwrap();
        // Need to set the first arg as the exe itself.
        let mut command_line = U16CString::from_str("tunet-service.exe run-once").unwrap();
        let app_dir =
            U16CString::from_os_str(std::env::current_dir().unwrap().into_os_string()).unwrap();
        CreateProcessAsUserW(
            HANDLE(dup_token.as_raw_handle() as _),
            PCWSTR(app_name.as_ptr()),
            PWSTR(command_line.as_mut_ptr()),
            None,
            None,
            false,
            CREATE_UNICODE_ENVIRONMENT,
            Some(env),
            PCWSTR(app_dir.as_ptr()),
            &si,
            &mut pi,
        )
        .ok()?;
        let _thread = OwnedHandle::from_raw_handle(pi.hThread.0 as _);
        let _process = OwnedHandle::from_raw_handle(pi.hProcess.0 as _);
    }
    Ok(())
}

pub fn error(s: impl AsRef<str>) -> Result<()> {
    let title = u16str!("tunet-service");
    let msg = U16String::from_str(s.as_ref());
    let mut res = MESSAGEBOX_RESULT(0);
    unsafe {
        WTSSendMessageW(
            WTS_CURRENT_SERVER_HANDLE,
            WTSGetActiveConsoleSessionId(),
            PCWSTR(title.as_ptr()),
            title.len() as _,
            PCWSTR(msg.as_ptr()),
            msg.len() as _,
            MB_OK,
            0,
            &mut res,
            false,
        )
        .ok()?;
    }
    Ok(())
}
