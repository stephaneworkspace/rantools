#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_float, c_int};
    use svg::node::element::path::{Command, Data, Parameters, Position};
    use crate::{svg_path_data, svg_circle, svg_line, svg_image, svg_document, SvgData, SvgPoint, SvgProperties, SvgFill, SvgStroke};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
   /* fn convert_svg() {
        let cstring = CString::new("assets/svg/mysvg.svg").unwrap();
        let file = cstring.as_ptr();
        create_png_ff_nsvg(file).expect("Impossible de convertir");
    }*/

    #[test]
    fn svg_path_data_works() {
        let data = Data::new()
            .move_to((10, 10))
            .line_by((0, 50))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();

        let mut vec_data: Vec<SvgData> = Vec::new();
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
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
                    closure(i, c, &param)
                },
                Command::Close => {
                    SvgData {
                        data: 'Z' as c_char,
                        point_idx: i as c_int,
                        point_size: 0,
                    }
                }
            };
            vec_data.push(svg_command);
        }

        let data = vec_data.as_ptr();
        let point = vec_point.as_ptr();
        let data_size = vec_data.len();
        let point_size = vec_point.len();

        let fill_cstring = CString::new("none").unwrap();
        let fill = fill_cstring.as_ptr();
        let stroke_cstring = CString::new("black").unwrap();
        let stroke = stroke_cstring.as_ptr();
        let properties = SvgProperties {
            fill: SvgFill {
                fill
            },
            stroke: SvgStroke {
                stroke,
                stroke_width: 3.0
            },
        };

        let res = svg_path_data(data, point, data_size as c_int, point_size as c_int, properties);

        let res_cstr = unsafe { CStr::from_ptr(res) };
        let res_str = res_cstr.to_str().unwrap();

        let assert = r#"<path d="M10,10 l0,50 l50,0 l0,-50 z" fill="none" stroke="black" stroke-width="3"/>"#;
        assert_eq!(assert, res_str);
    }

    #[test]
    fn svg_circle_works() {
        let fill_cstring = CString::new("none").unwrap();
        let fill = fill_cstring.as_ptr();
        let stroke_cstring = CString::new("black").unwrap();
        let stroke = stroke_cstring.as_ptr();
        let properties = SvgProperties {
            fill: SvgFill {
                fill
            },
            stroke: SvgStroke {
                stroke,
                stroke_width: 3.0
            },
        };
        let res = svg_circle(10.0, 50.0, 100.0, properties);

        let res_cstr = unsafe { CStr::from_ptr(res) };
        let res_str = res_cstr.to_str().unwrap();

        let assert = r#"<circle cx="10" cy="50" fill="none" r="100" stroke="black" stroke-width="3"/>"#;
        assert_eq!(assert, res_str);
    }

    #[test]
    fn svg_line_works() {
        let stroke_cstring = CString::new("black").unwrap();
        let stroke_ptr= stroke_cstring.as_ptr();
        let stroke = SvgStroke {
            stroke: stroke_ptr,
            stroke_width: 3.0
        };
        let res = svg_line(10.0, 50.0, 100.0, 120.0, stroke);

        let res_cstr = unsafe { CStr::from_ptr(res) };
        let res_str = res_cstr.to_str().unwrap();

        let assert = r#"<line stroke="black" stroke-width="3" x1="10" x2="100" y1="50" y2="120"/>"#;
        assert_eq!(assert, res_str);
    }

    #[test]
    fn svg_image_works() {
        let href_cstring = CString::new("data:image/svg+xml;base64,").unwrap();
        let href_ptr= href_cstring.as_ptr();

        let res = svg_image(40.0, 30.0, 10.0, 5.0, href_ptr);

        let res_cstr = unsafe { CStr::from_ptr(res) };
        let res_str = res_cstr.to_str().unwrap();

        let assert = r#"<image height="30" href="data:image/svg+xml;base64," width="40" x="10" y="5"/>"#;
        assert_eq!(assert, res_str);
    }

    #[test]
    fn svg_document_works() {
        let content_cstring = CString::new("<p/>").unwrap();
        let content_ptr= content_cstring.as_ptr();

        let res = svg_document(40.0, 30.0, content_ptr);

        let res_cstr = unsafe { CStr::from_ptr(res) };
        let res_str = res_cstr.to_str().unwrap();

        let assert = r#"<svg viewBox="0 0 40 30" xmlns="http://www.w3.org/2000/svg"><p/></svg>"#;
        assert_eq!(assert, res_str);
    }
}
