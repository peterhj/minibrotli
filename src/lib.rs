use crate::ffi::*;

use std::ptr::{null_mut};

pub mod ffi;

pub struct BrotliDecoder {
  state: *mut BrotliDecoderState,
}

impl Drop for BrotliDecoder {
  fn drop(&mut self) {
    assert!(!self.state.is_null());
    unsafe { BrotliDecoderDestroyInstance(self.state) };
  }
}

pub struct BrotliEncoder {
  state: *mut BrotliEncoderState,
}

impl Drop for BrotliEncoder {
  fn drop(&mut self) {
    assert!(!self.state.is_null());
    unsafe { BrotliEncoderDestroyInstance(self.state) };
  }
}

impl BrotliEncoder {
  pub fn new() -> Result<BrotliEncoder, ()> {
    let state = unsafe { BrotliEncoderCreateInstance(
        None,
        None,
        null_mut(),
    ) };
    if state.is_null() {
      return Err(());
    }
    Ok(BrotliEncoder{state})
  }

  pub fn max_output_size(&mut self, input_sz: usize) -> usize {
    unsafe { BrotliEncoderMaxCompressedSize(input_sz) }
  }

  pub fn compress(&mut self, input_buf: &[u8], output_buf: &mut [u8]) -> Result<usize, ()> {
    // TODO: compression parameters.
    let mut enc_output_sz: usize = 0;
    let ret = unsafe { BrotliEncoderCompress(
        6, BROTLI_DEFAULT_WINDOW as i32, BROTLI_MODE_GENERIC,
        input_buf.len(),
        input_buf.as_ptr(),
        &mut enc_output_sz as *mut usize,
        output_buf.as_mut_ptr(),
    ) };
    assert!(enc_output_sz <= output_buf.len());
    match ret {
      0 => Err(()),
      1 => Ok(enc_output_sz),
      _ => panic!(),
    }
  }
}
