extern crate core;

use std::os::raw::{c_char, c_float, c_int};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::{Data, Parameters, Position};

use svg::node::element::path::Command;

#[repr(C)]
#[derive(Debug)]
pub struct SvgData {
    pub data_idx: c_int,
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
pub struct SvgDataPath {
    pub data_path_idx: c_int,
    pub data_path_size: c_int,
}


fn main() {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();
    //println!("{:?}", &data);

    let svg_command_i = 0;

    let mut vec: Vec<SvgData> = Vec::new();
    let mut vec_point: Vec<SvgPoint> = Vec::new();

    let mut closure = |i: usize, c: char, param: &Parameters| -> SvgData {
        let point_size: c_int = param.len() as c_int;
        for j in 0..point_size as usize {
            let point: c_float = *&param[j];
            vec_point.push(SvgPoint {
                point_idx: i as c_int,
                point
            })
        };
        SvgData {
            data_idx: svg_command_i,
            data: c as c_char,
            point_idx: i as c_int,
            point_size
        }
    };

    for (i, x) in data.iter().enumerate().into_iter() {
        let svg_command = match x {
            Command::Move(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'M' // Move to
                    }
                    Position::Relative => {
                        'm' // Move by
                    }
                };
                closure(i, c, param)
            }
            Command::Line(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'L' // Line to
                    }
                    Position::Relative => {
                        'l' // Line by
                    }
                };
                closure(i, c, param)
            }
            Command::HorizontalLine(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'H'
                    }
                    Position::Relative => {
                        'h'
                    }
                };
                closure(i, c, param)
            }
            Command::VerticalLine(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'V'
                    }
                    Position::Relative => {
                        'v'
                    }
                };
                closure(i, c, param)
            }
            Command::QuadraticCurve(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'Q'
                    }
                    Position::Relative => {
                        'q'
                    }
                };
                closure(i, c, param)
            }
            Command::SmoothQuadraticCurve(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'T'
                    }
                    Position::Relative => {
                        't'
                    }
                };
                closure(i, c, param)
            }
            Command::CubicCurve(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'C'
                    }
                    Position::Relative => {
                        'c'
                    }
                };
                closure(i, c, param)
            }
            Command::SmoothCubicCurve(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'S'
                    }
                    Position::Relative => {
                        's'
                    }
                };
                closure(i, c, param)
            }
            Command::EllipticalArc(pos, param) => {
                let c = match pos {
                    Position::Absolute => {
                        'A'
                    }
                    Position::Relative => {
                        'a'
                    }
                };
                closure(i, c, param)
            },
            Command::Close => {
                SvgData {
                    data_idx: svg_command_i,
                    data: 'Z' as c_char,
                    point_idx: i as c_int,
                    point_size: 0,
                }
            }
        };
        vec.push(svg_command);
    }
    //println!("{:?}", &vec);
    let size_usize = vec.len();
    let svg_command_size: c_int = size_usize as c_int;
    let svg_command_slice = vec.as_slice();
    let svg_command: *const SvgData = svg_command_slice.as_ptr();

    let mut data_str: String = "".to_string();

    // SvgDataCommand
    let cmd = SvgDataPath {
        data_path_idx: svg_command_i,
        data_path_size: svg_command_size
    };
    //println!("{:?}", &cmd);
    for i in 0..cmd.data_path_size as isize {
        // SvgCommand
        let svg_command_extract = unsafe { svg_command.offset(i).as_ref().unwrap() };
        let c = svg_command_extract.data;
        let point_i = svg_command_extract.point_idx;
        let point_size = svg_command_extract.point_size;
        let cstr = char::from_u32(c as u32).unwrap();
        //println!("{:?} - {}", &svg_command_extract, cstr);

        // SvgPoint
        let svg_point_slice = vec_point.as_slice();
        let svg_point: *const SvgPoint = svg_point_slice.as_ptr();
        let mut vec_point_sortie: Vec<SvgPoint> = Vec::new();
        for j in 0..vec_point.len() as isize {
            let e = unsafe { svg_point.offset(j).as_ref().unwrap() };
            if e.point_idx == point_i {
                vec_point_sortie.push(SvgPoint {
                    point_idx: e.point_idx,
                    point: e.point
                });
            }
        }
        //println!("{:?}", vec_point_sortie);
        data_str += format!("{}", cstr.to_string()).as_str();
        for (j, x) in vec_point_sortie.iter().enumerate() {
            if point_size == j as c_int {
                data_str += format!("{}", x.point).as_str();
            } else {
                data_str += format!("{},", x.point).as_str();
            }
        }
    }

    let data2 = Data::parse(data_str.as_str()).unwrap();
    //println!("data2: {:?}", data2);


    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data2);

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path);

    //svg::save("image.svg", &document).unwrap();
    println!("{:?}", &document.to_string());
}