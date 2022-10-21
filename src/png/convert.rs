use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use usvg::{FitTo, Options, Tree};
use tiny_skia::Pixmap;
use crate::B64;

pub fn create_png_ff(svg_file: *const c_char) -> Result<Vec<u8>, B64> {
    let svg_file_cstr: &CStr = unsafe { CStr::from_ptr(svg_file) };
    let svg_file_str = svg_file_cstr.to_str().unwrap();
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(svg_file_str.clone()).ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));
    //opt.fontdb.load_system_fonts();
    let svg_data = std::fs::read(svg_file_str).unwrap();
    let rtree = match Tree::from_data(&svg_data, &opt.to_ref()) {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return Err(B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible de lire le svg. {}", err)).unwrap().into_raw()
            });
        }
    };

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&rtree, FitTo::Original, tiny_skia::Transform::default(), pixmap.as_mut());
    let png_res = match pixmap.encode_png() {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return Err(B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible d'encoder le png en base64. {}", err)).unwrap().into_raw()
            });
        }
    };
    Ok(png_res)
}

pub fn create_png(svg_v_u8: Vec<u8>) -> Result<Vec<u8>, B64> {
    let svg_slice = svg_v_u8.as_slice();
    let opt = Options::default();
    //let mut opt = Options::default();
    //opt.fontdb.load_system_fonts();
    let rtree = match Tree::from_data(svg_slice, &opt.to_ref()) {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return Err(B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible de lire le svg. {}", err)).unwrap().into_raw()
            });
        }
    };

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&rtree, FitTo::Original, tiny_skia::Transform::default(), pixmap.as_mut());
    let png_res = match pixmap.encode_png() {
        Ok(ok) => {
            ok
        },
        Err(err) => {
            return Err(B64 {
                b_64: CString::new("").unwrap().into_raw(),
                sw: false,
                err: CString::new(format!("Impossible d'encoder le png en base64. {}", err)).unwrap().into_raw()
            });
        }
    };
    Ok(png_res)
}