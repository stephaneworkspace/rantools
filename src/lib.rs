mod node;
mod png;
mod pdf;
mod svg;
mod tests;

extern crate base64;
extern crate core;

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::{c_char, c_float, c_int};
use base64::{encode, decode, DecodeError};
use crate::pdf::create_pdf;
use crate::png::{create_png, create_png_ff};
use crate::svg::{circle, document, image, line, path_data};
pub use crate::node::read_template;
pub use crate::pdf::create_pdf_numerologie;

#[repr(C)]
#[derive(Debug)]
pub struct SvgData {
    pub data: c_char,
    pub point_idx: c_int,
    pub point_size: c_int
}

#[repr(C)]
#[derive(Debug)]
pub struct SvgPoint {
    pub point_idx: c_int,
    pub point: c_float,
}

#[repr(C)]
#[derive(Debug)]
pub struct SvgProperties {
    pub fill: SvgFill,
    pub stroke: SvgStroke,
}

#[repr(C)]
#[derive(Debug)]
pub struct SvgFill {
    pub fill: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct SvgStroke {
    pub stroke: *const c_char,
    pub stroke_width: c_float,
}

#[repr(C)]
#[derive(Debug)]
pub struct B64 {
    pub b_64: *const c_char,
    sw: bool,
    pub err: *const c_char,
}

#[no_mangle]
pub extern "C" fn create_png_b64(p_svg_b64: *const c_char) -> B64 {
    let cstr_svg_b64: &CStr = unsafe { CStr::from_ptr(p_svg_b64) };
    let base64_svg = match cstr_svg_b64.to_str() {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Paramètre d'entrée \"p_svg_b64\" invalide {}", err)).unwrap().into_raw()
            }
        }
    };
    let svg_res: Result<Vec<u8>, DecodeError> = decode(base64_svg);
    let svg_v_u8 = match svg_res {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible de décoder le svg. {}", err)).unwrap().into_raw()
            };
        }
    };
    match create_png(svg_v_u8) {
        Ok(ok) => {
            B64 {
                b_64: CString::new(encode(ok)).unwrap().into_raw(),
                sw: true,
                err: CString::new("").unwrap().into_raw()
            }
        },
        Err(err) => {
            err
        }
    }
}

#[no_mangle]
pub extern "C" fn create_png_from_file(svg_file: *const c_char) -> B64 {
    match create_png_ff(svg_file) {
        Ok(ok) => {
            B64 {
                b_64: CString::new(encode(ok)).unwrap().into_raw(),
                sw: true,
                err: CString::new("").unwrap().into_raw()
            }
        },
        Err(err) => {
            err
        }
    }
}

#[no_mangle]
pub extern "C" fn create_pdf_b64(p_svg_b64: *const c_char) -> B64 {
    let cstr_svg_b64: &CStr = unsafe { CStr::from_ptr(p_svg_b64) };
    let base64_svg = match cstr_svg_b64.to_str() {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Paramètre d'entrée \"p_svg_b64\" invalide {}", err)).unwrap().into_raw()
            };
        }
    };

    let svg_res: Result<Vec<u8>, DecodeError> = decode(base64_svg);
    let svg_v_u8 = match svg_res {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible de décoder le svg. {}", err)).unwrap().into_raw()
            };
        }
    };
    let data = match create_png(svg_v_u8) {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return err;
        }
    };

    match create_pdf(data) {
        Ok(ok) => {
            B64 {
                b_64: CString::new(ok).unwrap().into_raw(),
                sw: true,
                err: CString::new("").unwrap().into_raw()
            }
        },
        Err(err) => {
            err
        }
    }
}

#[no_mangle]
pub extern "C" fn create_pdf_b64_from_png_b64(p_png_b64: *const c_char) -> B64 {
    let cstr_png_b64: &CStr = unsafe { CStr::from_ptr(p_png_b64) };
    let base64_png = match cstr_png_b64.to_str() {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Paramètre d'entrée \"p_png_b64\" invalide {}", err)).unwrap().into_raw()
            };
        }
    };

    let mut f = match File::open(base64_png.clone()) {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible de lire le png. {}", err)).unwrap().into_raw()
            };
        }
    };
    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(_) => {
        },
        Err(err) => {
            return B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible de lire le buffer du png. {}", err)).unwrap().into_raw()
            };
        }
    }
    match create_pdf(buffer) {
        Ok(ok) => {
            B64 {
                b_64: CString::new(ok).unwrap().into_raw(),
                sw: true,
                err: CString::new("").unwrap().into_raw()
            }
        },
        Err(err) => {
            err
        }
    }
}



#[no_mangle]
pub extern "C" fn svg_path_data(data: *const SvgData,
                                point: *const SvgPoint,
                                data_size: c_int,
                                point_size: c_int,
                                properties: SvgProperties) -> *const c_char {
    let res= path_data(data, point, data_size as isize, point_size as isize, properties);
    let res_c_str = CString::new(res).unwrap();
    let res_ptr = res_c_str.into_raw();
    res_ptr
}

#[no_mangle]
pub extern "C" fn svg_circle(x: c_float,
                             y: c_float,
                             r: c_float,
                             properties: SvgProperties) -> *const c_char {
    let res = circle(x, y, r, properties);
    let res_c_str = CString::new(res).unwrap();
    let res_ptr = res_c_str.into_raw();
    res_ptr
}

#[no_mangle]
pub extern "C" fn svg_line(x1: c_float,
                           y1: c_float,
                           x2: c_float,
                           y2: c_float,
                           stroke: SvgStroke) -> *const c_char {
    let res = line(x1, y1, x2, y2, stroke);
    let res_c_str = CString::new(res).unwrap();
    let res_ptr = res_c_str.into_raw();
    res_ptr
}

#[no_mangle]
pub extern "C" fn svg_image(width: c_float,
                            height: c_float,
                            x: c_float,
                            y: c_float,
                            href: *const c_char) -> *const c_char {
    let res = image(width, height, x, y, href);
    let res_c_str = CString::new(res).unwrap();
    let res_ptr = res_c_str.into_raw();
    res_ptr
}

#[no_mangle]
pub extern "C" fn svg_document(width: c_float,
                               height: c_float,
                               content: *const c_char) -> *const c_char {
    let res = document(width, height, content);
    let res_c_str = CString::new(res).unwrap();
    let res_ptr = res_c_str.into_raw();
    res_ptr
}