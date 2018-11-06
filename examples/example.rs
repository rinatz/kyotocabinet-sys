extern crate kyotocabinet_sys as ffi;

use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let db = ffi::kcdbnew();
        let path = CString::new("casket.kch").unwrap();

        let ok = ffi::kcdbopen(db, path.as_ptr(), ffi::KCOWRITER | ffi::KCOCREATE);

        if ok == 0 {
            eprintln!("{:?}", CStr::from_ptr(ffi::kcecodename(ffi::kcdbecode(db))));
            return;
        }

        let key = CString::new("foo").unwrap();
        let val = CString::new("hop").unwrap();

        let ok = ffi::kcdbset(db, key.as_ptr(), 3, val.as_ptr(), 3);

        if ok == 0 {
            eprintln!("{:?}", CStr::from_ptr(ffi::kcecodename(ffi::kcdbecode(db))));
            return;
        }

        let mut size = 0usize;
        let ptr = ffi::kcdbget(db, key.as_ptr(), 3, &mut size);

        println!("{:?}:{:?}", key, CStr::from_ptr(ptr));

        ffi::kcfree(ptr as _);

        ffi::kcdbclose(db);
        ffi::kcdbdel(db);
    }
}
