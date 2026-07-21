use std::{
    env,
    ffi::{c_int, c_ulong},
};

#[link(name = "hello", kind = "static")]
#[link(name = "z")]
unsafe extern "C" {
    fn my_compress(
        dest: *mut u8,
        destLen: *mut c_ulong,
        source: *const u8,
        sourceLen: c_ulong,
    ) -> c_int;
}

fn main() {
    let data = b"hello world";
    let mut compressed = vec![0u8; 1024];
    let mut compressed_len = compressed.len() as c_ulong;

    unsafe {
        my_compress(
            compressed.as_mut_ptr(),
            &mut compressed_len,
            data.as_ptr(),
            data.len() as c_ulong,
        );
    }
    println!("压缩成功！");
    println!("{:?}", compressed);
}
