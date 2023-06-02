use crate::get_f64_from_str;
use crate::prelude::DstvElement;

pub struct Cut {
    pub nor_vec_x: f64,
    pub nor_vec_y: f64,
    pub nor_vec_z: f64,
    pub sp_point_x: f64,
    pub sp_point_y: f64,
    pub sp_point_z: f64,
}

impl DstvElement for Cut {
    fn from_str(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        if iter.clone().count() < 6 {
            return Err("Illegal data vector format (SC): too short");
        }
        let sp_point_x = get_f64_from_str(iter.next(), "sp_point_x");
        let sp_point_y = get_f64_from_str(iter.next(), "sp_point_y");
        let sp_point_z = get_f64_from_str(iter.next(), "sp_point_z");
        let nor_vec_x = get_f64_from_str(iter.next(), "nor_vec_x");
        let nor_vec_y = get_f64_from_str(iter.next(), "nor_vec_y");
        let nor_vec_z = get_f64_from_str(iter.next(), "nor_vec_z");
        Ok(Self {
            nor_vec_x,
            nor_vec_y,
            nor_vec_z,
            sp_point_x,
            sp_point_y,
            sp_point_z,
        })
    }

    fn to_svg(&self) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" />",
            self.sp_point_x,
            self.sp_point_y,
            self.sp_point_x + self.nor_vec_x,
            self.sp_point_y + self.nor_vec_y
        )
    }
}
