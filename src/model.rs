use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum ObjAttribute {
    VertexPos,
    TexturePos,
    VertexNormal,
    PolygonIndex,
    MaterialName,
    ObjectName,
    Comment,
    Other,
}

fn detect_obj_attribute(str_: &str) -> ObjAttribute {
    match str_ {
        "v" => ObjAttribute::VertexPos,
        "vt" => ObjAttribute::TexturePos,
        "vn" => ObjAttribute::VertexNormal,
        "f" => ObjAttribute::PolygonIndex,
        "mtllib" => ObjAttribute::MaterialName,
        "o" => ObjAttribute::ObjectName,
        "#" => ObjAttribute::Comment,
        _ => ObjAttribute::Other,
    }
}

enum LoadError{
    OpenFileFailed,
    ReadError,
}

enum Polygon {
    Triangle = 3,
    Quadrangle = 4,
    Pentagon = 5,
    Hexagon = 6,
    Polygon,
}

fn read_one_line(
    input_str: &Vec<&str>,
    out_buf: &mut Vec<f32>,
    error_msg: &str,
    error_pos: i32,
) -> Result<(), Box<dyn Error>> {
    for i in 1..input_str.len() {
        match input_str.get(i) {
            Some(str_) => out_buf.push(str_.parse::<f32>()?),
            None => {
                let error_msg = format!("{} Error line number:{}", error_msg, error_pos);
                return Err(error_msg.into());
            }
        }
    }
    Ok(())
}

pub struct Indices {
    pub vertex_indices: Vec<u32>,
    pub texture_indices: Vec<u32>,
    pub normal_indices: Vec<u32>,
}
impl Indices {
    pub fn new() -> Indices {
        let v_indices = Vec::new();
        let tex_indices = Vec::new();
        let normal_indices = Vec::new();
        Indices {
            vertex_indices: v_indices,
            texture_indices: tex_indices,
            normal_indices: normal_indices,
        }
    }
}
pub struct Model {
    pub vertex: Vec<f32>,
    pub vertex_tex: Vec<f32>,
    pub vertex_normal: Vec<f32>,
    pub vertex_normal_tmp: Vec<f32>,
    pub indices: Indices,
}
impl Model {
    pub fn new() -> Model {
        Model {
            vertex: Vec::new(),
            vertex_tex: Vec::new(),
            vertex_normal: Vec::new(),
            vertex_normal_tmp: Vec::new(),
            indices: Indices::new(),
        }
    }

    fn load_indices(&mut self, input_str: &Vec<&str>, counter: i32) -> Result<(), Box<dyn Error>> {
        '_units: for i in 0..=2 {
            match input_str.get(i) {
                Some(t) => {
                    // is empty?
                    if t.is_empty() {
                        continue '_units;
                    }
                    let index = t.parse::<u32>()? - 1;
                    // println!("index :{}",index);
                    match i {
                        0 => self.indices.vertex_indices.push(index),
                        1 => self.indices.texture_indices.push(index),
                        2 => self.indices.normal_indices.push(index),
                        _ => (),
                    }
                }
                None => {
                    let error_text = format!("Polygon index is invalid. Line number:{}", counter);
                    return Err(error_text.into());
                }
            }
        }
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut counter = 1;

        for line in buf_reader.lines() {
            if let Ok(str) = line {
                // ex) str = "v 0.0 0.1 0.2"
                // ex) ss = "v" "0.0" "0.1" "0.2"
                let ss: Vec<&str> = str.split(' ').collect();
                let data_attr = detect_obj_attribute(ss[0]); // first str is data attribute
                match data_attr {
                    ObjAttribute::VertexPos => {
                        read_one_line(
                            &ss,
                            &mut self.vertex,
                            "Vertex position is invalid.",
                            counter,
                        )?;
                    }
                    ObjAttribute::TexturePos => {
                        read_one_line(
                            &ss,
                            &mut self.vertex_tex,
                            "Vertex texture position is invalid.",
                            counter,
                        )?;
                    }
                    ObjAttribute::VertexNormal => {
                        read_one_line(
                            &ss,
                            &mut self.vertex_normal_tmp,
                            "Vertex normal is invalid.",
                            counter,
                        )?;
                    }
                    ObjAttribute::ObjectName => {
                        // println!("Object name: {}", str);
                    }
                    ObjAttribute::MaterialName => {
                        // println!("Material name: {}", str);
                    }
                    ObjAttribute::PolygonIndex => {
                        // ss= f ?/?/? ?/?/? ?/?/? ?/?/?
                        let polygon = match ss.len() - 1 {
                            3 => Polygon::Triangle,
                            4 => Polygon::Quadrangle,
                            5 => Polygon::Pentagon,
                            6 => Polygon::Hexagon,
                            _ => Polygon::Polygon,
                        };

                        match polygon {
                            Polygon::Triangle => {
                                for i in 1..=3 {
                                    let separate_index =
                                        ss.get(i).ok_or("Polygon index is invalid.")?;
                                    let separate_index: Vec<&str> =
                                        separate_index.split('/').collect();
                                    self.load_indices(&separate_index, counter)?;
                                }
                            }
                            Polygon::Quadrangle => {
                                let mut ss_str: Vec<Vec<&str>> = Vec::new();
                                ss_str.resize(polygon as usize, Vec::new());
                                for i in 1..ss.len() {
                                    // str_=?/?/?
                                    let str_ = ss.get(i).ok_or("Polygon index is invalid.")?;
                                    let arr_index = i - 1;
                                    ss_str[arr_index] = str_.split('/').collect();
                                }
                                let index_for_quad = [[0, 1, 2], [2, 3, 0]];
                                for row in index_for_quad.iter() {
                                    for col in row.iter() {
                                        self.load_indices(ss_str.get(*col).ok_or("err")?, counter)?;
                                    }
                                }
                            }
                            Polygon::Pentagon => {
                                let mut ss_str: Vec<Vec<&str>> = Vec::new();
                                ss_str.resize(polygon as usize, Vec::new());
                                for i in 1..ss.len() {
                                    // str_=?/?/?
                                    let str_ = ss.get(i).ok_or("Polygon index is invalid.")?;
                                    let arr_index = i - 1;
                                    ss_str[arr_index] = str_.split('/').collect();
                                }
                                let index_for_penta = [[0, 1, 2], [2, 3, 4], [4, 0, 1]];
                                for row in index_for_penta.iter() {
                                    for col in row.iter() {
                                        self.load_indices(ss_str.get(*col).ok_or("err")?, counter)?;
                                    }
                                }
                            }
                            Polygon::Hexagon => {
                                let mut ss_str: Vec<Vec<&str>> = Vec::new();
                                ss_str.resize(polygon as usize, Vec::new());
                                for i in 1..ss.len() {
                                    // str_=?/?/?
                                    let str_ = ss.get(i).ok_or("Polygon index is invalid.")?;
                                    let arr_index = i - 1;
                                    ss_str[arr_index] = str_.split('/').collect();
                                }
                                let index_for_hexa = [[0, 1, 2], [2, 3, 4], [4, 5, 0], [0, 2, 4]];
                                for row in index_for_hexa.iter() {
                                    for col in row.iter() {
                                        self.load_indices(ss_str.get(*col).ok_or("err")?, counter)?;
                                    }
                                }
                            }
                            _ => {
                                // println!("{}",counter);
                            }
                        }
                        // self.vertex_normal[self.vertex[separate_index[0]as usize]]=self.vertex_normal_tmp[separate_index[2]];
                    }
                    ObjAttribute::Comment => {
                        // println!("Comment: {}", str);
                    }
                    ObjAttribute::Other => {
                        // println!("Other: {}", str);
                    }
                };
            }
            counter += 1;
        }
        Ok(())
    }

    pub fn create_vertex_normal(&mut self) {
        if self.vertex.len() != self.vertex_normal.len() {
            self.vertex_normal.resize(self.vertex.len(), 0.0);
        }
        for i in 0..self.vertex_normal.len() {
            self.vertex_normal[i] = self.vertex_normal_tmp[self.indices.normal_indices[i] as usize];
            // self.vertex_normal[i] = self.vertex_normal_tmp[self.indices.normal_indices[i] as usize];
            // self.vertex_normal[i] = self.vertex_normal_tmp[self.indices.vertex_indices[self.indices.normal_indices[i] as usize] as usize];
        }
    }

    pub fn create_concat_vertex(&self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        let vertex_num = self.vertex.len() / 3;
        for i in 0..vertex_num {
            vec.push(self.vertex[i * 3 + 0]);
            vec.push(self.vertex[i * 3 + 1]);
            vec.push(self.vertex[i * 3 + 2]);
            vec.push(self.vertex_normal[i * 3 + 0]);
            vec.push(self.vertex_normal[i * 3 + 1]);
            vec.push(self.vertex_normal[i * 3 + 2]);
        }
        vec
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_detect_obj_attribute() {
        let actual = detect_obj_attribute("v");
        let expected = ObjAttribute::VertexPos;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute("vt");
        let expected = ObjAttribute::TexturePos;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute("vn");
        let expected = ObjAttribute::VertexNormal;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute("f");
        let expected = ObjAttribute::PolygonIndex;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute("mtllib");
        let expected = ObjAttribute::MaterialName;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute("o");
        let expected = ObjAttribute::ObjectName;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute("#");
        let expected = ObjAttribute::Comment;
        assert_eq!(expected as i32, actual as i32);

        let actual = detect_obj_attribute(" ");
        let expected = ObjAttribute::Other;
        assert_eq!(expected as i32, actual as i32);
    }

    #[test]
    fn test_ok_read_one_line() {
        let input_str = vec!["v", "0.0", "1.0", "2.0"];
        let mut actual: Vec<f32> = Vec::new();
        if let Err(e) = read_one_line(&input_str, &mut actual, "error", 1) {
            println!("{}", e);
        };
        let expected = vec![0.0, 1.0, 2.0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ng_read_one_line() {
        let input_str = vec!["v", "sample", "0.0", "1.0", "2.0"];
        let mut actual: Vec<f32> = Vec::new();
        if let Err(e) = read_one_line(&input_str, &mut actual, "error", 1) {
            println!("{}", e);
        };
        let expected: Vec<f32> = Vec::new();
        assert_eq!(expected, actual);

        let input_str = vec!["v", "0.0", "sample", "1.0", "2.0"];
        let mut actual: Vec<f32> = Vec::new();
        if let Err(e) = read_one_line(&input_str, &mut actual, "error", 1) {
            println!("{}", e);
        };
        let expected = vec![0.0];
        assert_eq!(expected, actual);

        let input_str = vec!["v", "0.0", "1.0", "sample", "2.0"];
        let mut actual: Vec<f32> = Vec::new();
        if let Err(e) = read_one_line(&input_str, &mut actual, "error", 1) {
            println!("{}", e);
        };
        let expected = vec![0.0, 1.0];
        assert_eq!(expected, actual);

        let input_str = vec!["v", "0.0", "1.0", "2.0", "sample"];
        let mut actual: Vec<f32> = Vec::new();
        if let Err(e) = read_one_line(&input_str, &mut actual, "error", 1) {
            println!("{}", e);
        };
        let expected = vec![0.0, 1.0, 2.0];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ok_load() {
        let mut model = Model::new();
        model.load("/home/twmoca/Documents/3d_obj/BaseSpiderMan.obj");
    }
}
