use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use printpdf::Color;

//use pdf_writer::types::{ActionType, BorderType};
//use pdf_writer::{Content, Finish, Name, PdfWriter, Rect, Ref, Str, TextStr};
use crate::node::{AttrEnum, NodeElement, NodeEnum};
// Set the size to A4
const MAX_WIDTH: f64 = 595.0;
const MAX_HEIGHT: f64 = 842.0;
const MARGIN_WIDTH: f64 = 90.0;
const MARGIN_HEIGHT: f64 = 90.0;
const TR_HEIGHT:f64 = 18.0;

// Color
const RGB_COLOR_WHITE: (f64, f64, f64) = (255.0, 255.0, 255.0);
const RGB_COLOR_BLACK: (f64, f64, f64) = (0.0, 0.0, 0.0);

#[derive(Debug)]
pub struct LayerAttr {
    pub fill_color: (f64, f64, f64),
    pub stroke_color: (f64, f64, f64),
    pub stroke: i32
}


impl NodeElement {
    fn read_root_template_draw_rectangle(template: &[NodeElement], mut current_y: f64) -> Vec<(LayerAttr, Line)> {
        let mut vec: Vec<(LayerAttr, Line)> = Vec::new();
        for x in template.iter() {
            match &x.node {
                NodeEnum::Table(_table) => {
                    let vec_res = x.tr(current_y);
                    current_y -= TR_HEIGHT;
                    for y in vec_res.into_iter() {
                        vec.push(y);
                    }
                },
                _=> {
                    unreachable!();
                }
            }
        }
        vec.into_iter().collect()
    }

    fn td(&self, current_y: f64) -> Vec<(LayerAttr, Line)> {
        let filter = |x: &NodeElement| {
            match &x.node {
                NodeEnum::Td(_) => {
                    true
                },
                _ => {
                    false
                }
            }
        };
        let td = self.child.iter().filter(|&x| filter(x)).count();
        let table_width_min = MARGIN_WIDTH;
        let table_width_max = MAX_WIDTH - MARGIN_WIDTH;
        let td_size: f64 = (table_width_max - table_width_min) / td as f64;
        // ici ça ne tient pas compte de width pour le moment, ça divise
        let vec = self.child.iter()
            .filter(|&x| filter(x))
            .enumerate()
            .map(|(i, node_element)| {
                let mut layer_attr = LayerAttr {
                    fill_color: (RGB_COLOR_WHITE.0, RGB_COLOR_WHITE.1, RGB_COLOR_WHITE.2),
                    stroke_color: (RGB_COLOR_BLACK.0, RGB_COLOR_BLACK.1, RGB_COLOR_BLACK.2),
                    stroke: 1
                };
                match &node_element.node {
                    NodeEnum::Td(vec_attr) => {
                        for x in vec_attr.iter() {
                            match x {
                                AttrEnum::Width(_) => {}
                                AttrEnum::Height(_) => {}
                                AttrEnum::Border(stroke) => {
                                    layer_attr.stroke = *stroke;
                                }
                                AttrEnum::Align(_) => {}
                                AttrEnum::Text(_) => {}
                            }
                        }
                    }
                    _ => { 
                        unreachable!()
                    }
                }
                let width_begin = table_width_min + (i as f64 * td_size);
                let width_end = width_begin + td_size;
                let points = vec![(Point::new(Mm(width_begin), Mm(current_y)), false),
                                  (Point::new(Mm(width_begin), Mm(current_y - TR_HEIGHT)), false),
                                  (Point::new(Mm(width_end), Mm(current_y - TR_HEIGHT)), false),
                                  (Point::new(Mm(width_end), Mm(current_y)), false)];
                let line = Line {
                    points: points,
                    is_closed: true,
                    has_fill: true,
                    has_stroke: true,
                    is_clipping_path: false,
                };
                (layer_attr, line)
            }).collect();
        vec
    }

    fn tr(&self, mut current_y: f64) -> Vec<(LayerAttr, Line)> {
        let filter = |x: &NodeElement| {
            match &x.node {
                NodeEnum::Tr(_) => {
                    true
                },
                _ => {
                    false
                }
            }
        };
        let mut vec: Vec<(LayerAttr, Line)> = Vec::new();
        for x in self.child.iter().filter(|&x| filter(x)) {
            let res_vec = x.td(current_y);
            for y in res_vec.into_iter() {
                vec.push(y);
            }
            current_y -= TR_HEIGHT;
        }
        vec
    }
}

pub fn create_pdf(template: &[NodeElement]) {
    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(MAX_WIDTH), Mm(MAX_HEIGHT), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let current_y = MAX_HEIGHT - MARGIN_HEIGHT;
    for x in NodeElement::read_root_template_draw_rectangle(template, current_y) {
        current_layer.set_fill_color(Color::Rgb(Rgb::new(x.0.fill_color.0,
                                                         x.0.fill_color.1,
                                                         x.0.fill_color.2,
                                                         None)));
        current_layer.set_outline_color(Color::Rgb(Rgb::new(x.0.stroke_color.0,
                                                            x.0.stroke_color.1,
                                                            x.0.stroke_color.2,
                                                            None)));
        current_layer.set_outline_thickness(x.0.stroke as f64);
        current_layer.add_shape(x.1);
    }

    let text = "Lorem ipsum";
    let text2 = "abcdefgh";

    let font = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
    current_layer.use_text(text, 48 as f64, Mm(200.0), Mm(200.0), &font);
    current_layer.use_text(text2, 48 as f64, Mm(200.0), Mm(200.0), &font);

    current_layer.begin_text_section();
    current_layer.set_font(&font, 33 as f64);
    current_layer.set_text_cursor(Mm(MARGIN_WIDTH + 5.0), Mm(MAX_HEIGHT - MARGIN_HEIGHT - TR_HEIGHT + 5.0));
    current_layer.set_word_spacing(3000 as f64);
    current_layer.set_character_spacing(10 as f64);
    current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    current_layer.write_text(text.clone(), &font);
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font, 33 as f64);
    current_layer.set_text_cursor(Mm(MARGIN_WIDTH + 5.0), Mm(MAX_HEIGHT - MARGIN_HEIGHT - (TR_HEIGHT * 2.0) + 5.0));
    current_layer.set_word_spacing(3000 as f64);
    current_layer.set_character_spacing(10 as f64);
    current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    current_layer.write_text(text2.clone(), &font);
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("target/hello.pdf").unwrap())).unwrap();
}