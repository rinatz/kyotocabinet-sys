# kyotocabinet-sys

`libkyotocabinet` bindings for Rust.

## Cargo.toml

```toml
[dependencies]
kyotocabinet-sys = "0.1.0"
```

## Example

```rust
extern crate kyotocabinet_sys as ffi;
use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let db = ffi::kcdbnew();
        let path = CString::new("casket.kch").unwrap();

        ffi::kcdbopen(db, path.as_ptr(), ffi::KCOWRITER | ffi::KCOCREATE);

        let key = CString::new("key").unwrap();
        let val = CString::new("val").unwrap();

        ffi::kcdbset(db, key.as_ptr(), 3, val.as_ptr(), 3);

        let mut len = 0usize;
        let ptr = ffi::kcdbget(db, key.as_ptr(), 3, &mut len);

        println!("{:?}:{:?}", key, CStr::from_ptr(ptr));

        ffi::kcfree(ptr as _);
        ffi::kcdbclose(db);
        ffi::kcdbdel(db);
    }
}
```

## License

MIT
