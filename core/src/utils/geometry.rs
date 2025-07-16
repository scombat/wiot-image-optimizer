use crate::{
    models::gravity_options::GravityOptions, models::options::CropOptions,
    utils::coordinates::Coord,
};
use image::{DynamicImage, GenericImageView};

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    pub image: DynamicImage,
}

impl Geometry {
    pub fn new(image: &DynamicImage) -> Geometry {
        Self {
            image: image.clone(),
        }
    }

    pub fn area(&self, opts: &CropOptions, gravity: &GravityOptions) -> Area {
        let (img_w, img_h) = self.image.dimensions();
        let img_w_i = img_w as i32;
        let img_h_i = img_h as i32;

        let off_x_px_opt = opts.offset_x.as_ref().map(|c| match c {
            Coord::Px(n) => *n,
            Coord::Percent(p) => ((p / 100.0) * img_w_i as f32).round() as i32,
        });
        let off_y_px_opt = opts.offset_y.as_ref().map(|c| match c {
            Coord::Px(n) => *n,
            Coord::Percent(p) => ((p / 100.0) * img_h_i as f32).round() as i32,
        });

        let width_i = if let Some(c) = &opts.width {
            match c {
                Coord::Px(n) => n.abs(),
                Coord::Percent(p) => ((p.abs() / 100.0) * img_w_i as f32).round() as i32,
            }
        } else if let Some(off_x_px) = off_x_px_opt {
            if off_x_px >= 0 {
                img_w_i - off_x_px
            } else {
                img_w_i + off_x_px
            }
        } else {
            img_w_i
        };

        let height_i = if let Some(c) = &opts.height {
            match c {
                Coord::Px(n) => n.abs(),
                Coord::Percent(p) => ((p.abs() / 100.0) * img_h_i as f32).round() as i32,
            }
        } else if let Some(off_y_px) = off_y_px_opt {
            if off_y_px >= 0 {
                img_h_i - off_y_px
            } else {
                img_h_i + off_y_px
            }
        } else {
            img_h_i
        };

        let raw_x = if let Some(off_x_px) = off_x_px_opt {
            if off_x_px >= 0 {
                off_x_px
            } else if opts.width.is_some() {
                img_w_i + off_x_px - width_i
            } else {
                0
            }
        } else {
            let (gx, _) = gravity.as_coef();
            ((img_w_i - width_i) as f32 * gx).round() as i32
        };

        let raw_y = if let Some(off_y_px) = off_y_px_opt {
            if off_y_px >= 0 {
                off_y_px
            } else if opts.height.is_some() {
                img_h_i + off_y_px - height_i
            } else {
                0
            }
        } else {
            let (_, gy) = gravity.as_coef();
            ((img_h_i - height_i) as f32 * gy).round() as i32
        };

        let max_x = (img_w_i - width_i).max(0);
        let max_y = (img_h_i - height_i).max(0);

        let x = raw_x.max(0).min(max_x) as u32;
        let y = raw_y.max(0).min(max_y) as u32;
        let width = width_i.max(0).min(img_w_i) as u32;
        let height = height_i.max(0).min(img_h_i) as u32;

        Area {
            x,
            y,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::options::{CropOptions, GravityOptions};
    use image::DynamicImage;
    use once_cell::sync::Lazy;

    static GEO: Lazy<Geometry> = Lazy::new(|| Geometry::new(&DynamicImage::new_rgb8(200, 100)));

    #[tokio::test]
    async fn test_no_crop() {
        let crop = CropOptions {
            width: None,
            height: None,
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_only_width() {
        let crop = CropOptions {
            width: Some(Coord::Px(50)),
            height: None,
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 75,
            y: 0,
            width: 50,
            height: 100,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_only_height() {
        let crop = CropOptions {
            width: None,
            height: Some(Coord::Px(40)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 0,
            y: 30,
            width: 200,
            height: 40,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_width_height_gravity_top_left() {
        let crop = CropOptions {
            width: Some(Coord::Px(50)),
            height: Some(Coord::Px(50)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        let area = GEO.area(&crop, &GravityOptions::TopLeft);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_width_height_gravity_center() {
        let crop = CropOptions {
            width: Some(Coord::Px(50)),
            height: Some(Coord::Px(50)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 75,
            y: 25,
            width: 50,
            height: 50,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_width_height_gravity_bottom_right() {
        let crop = CropOptions {
            width: Some(Coord::Px(50)),
            height: Some(Coord::Px(50)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 150,
            y: 50,
            width: 50,
            height: 50,
        };
        let area = GEO.area(&crop, &GravityOptions::BottomRight);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_only_offsets() {
        let crop = CropOptions {
            width: None,
            height: None,
            offset_x: Some(Coord::Px(10)),
            offset_y: Some(Coord::Px(20)),
        };
        let expected = Area {
            x: 10,
            y: 20,
            width: 190,
            height: 80,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_only_negative_offsets() {
        let crop = CropOptions {
            width: None,
            height: None,
            offset_x: Some(Coord::Px(-10)),
            offset_y: Some(Coord::Px(-20)),
        };
        let expected = Area {
            x: 0,
            y: 0,
            width: 190,
            height: 80,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_width_height_negative_offsets() {
        let crop = CropOptions {
            width: Some(Coord::Px(50)),
            height: Some(Coord::Px(50)),
            offset_x: Some(Coord::Px(-10)),
            offset_y: Some(Coord::Px(-20)),
        };
        let expected = Area {
            x: 140,
            y: 30,
            width: 50,
            height: 50,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_percent_width_height_gravity_center() {
        let crop = CropOptions {
            width: Some(Coord::Percent(50.0)),
            height: Some(Coord::Percent(20.0)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 50,
            y: 40,
            width: 100,
            height: 20,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_percent_offsets_only() {
        let crop = CropOptions {
            width: None,
            height: None,
            offset_x: Some(Coord::Percent(10.0)),
            offset_y: Some(Coord::Percent(20.0)),
        };
        let expected = Area {
            x: 20,
            y: 20,
            width: 180,
            height: 80,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_percent_width_height_offset_y_pixel_offset_x() {
        let crop = CropOptions {
            width: Some(Coord::Percent(20.0)),
            height: Some(Coord::Percent(40.0)),
            offset_x: Some(Coord::Px(10)),
            offset_y: Some(Coord::Percent(10.0)),
        };
        let expected = Area {
            x: 10,
            y: 10,
            width: 40,
            height: 40,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_percent_negative_offsets_percent_height() {
        let crop = CropOptions {
            width: None,
            height: Some(Coord::Percent(30.0)),
            offset_x: Some(Coord::Percent(-10.0)),
            offset_y: None,
        };
        let expected = Area {
            x: 0,
            y: 0,
            width: 180,
            height: 30,
        };
        let area = GEO.area(&crop, &GravityOptions::TopLeft);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_width_height_oversized() {
        let crop = CropOptions {
            width: Some(Coord::Px(300)),
            height: Some(Coord::Px(150)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
        };
        let area = GEO.area(&crop, &GravityOptions::TopLeft);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_no_dimensions_gravity_center() {
        let crop = CropOptions {
            width: Some(Coord::Px(0)),
            height: Some(Coord::Px(0)),
            offset_x: None,
            offset_y: None,
        };
        let expected = Area {
            x: 100,
            y: 50,
            width: 0,
            height: 0,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_width_offset_x_only() {
        let crop = CropOptions {
            width: Some(Coord::Px(50)),
            height: None,
            offset_x: Some(Coord::Px(10)),
            offset_y: None,
        };
        let expected = Area {
            x: 10,
            y: 0,
            width: 50,
            height: 100,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
    #[tokio::test]
    async fn test_height_offset_y_only() {
        let crop = CropOptions {
            width: None,
            height: Some(Coord::Px(30)),
            offset_x: None,
            offset_y: Some(Coord::Px(20)),
        };
        let expected = Area {
            x: 0,
            y: 20,
            width: 200,
            height: 30,
        };
        let area = GEO.area(&crop, &GravityOptions::Center);
        assert_eq!(area, expected);
    }
}
