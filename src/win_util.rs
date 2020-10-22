use winapi::um::winnt::HRESULT;
use winapi::shared::winerror::SUCCEEDED;
use wio::com::ComPtr;

#[inline]
pub fn check_hr(hr: HRESULT) -> Result<(), HRESULT> {
    if SUCCEEDED(hr) {
        Ok(())
    } else {
        Err(hr)
    }
}

pub unsafe fn get_comptr<T: winapi::Interface, F: FnOnce(*mut *mut T)>(f: F) -> ComPtr<T> {
    let mut tmp: *mut T = std::ptr::null_mut();
    f(&mut tmp);
    ComPtr::<T>::from_raw(tmp)
}

pub unsafe fn get_comptr_with_result<T: winapi::Interface, F: FnOnce(*mut *mut T) -> HRESULT>(
    f: F,
) -> Result<ComPtr<T>, HRESULT> {
    let mut tmp: *mut T = std::ptr::null_mut();
    let hr = f(&mut tmp);
    if SUCCEEDED(hr) {
        Ok(ComPtr::<T>::from_raw(tmp))
    } else {
        Err(hr)
    }
}

