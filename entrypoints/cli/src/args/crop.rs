use clap::Args;
use wiot_core::models::crop_options::CropOptions;
use wiot_core::utils::coordinates::Coord;

#[derive(Args, Debug)]
pub struct CropArgs {
    #[arg(
        long = "crop-width",
        value_name = "WIDTH",
        value_parser = Coord::parse,
        help = "Width of the crop in px or percent (e.g. 100 or 50%)"
    )]
    pub crop_width: Option<Coord>,

    #[arg(
        long = "crop-height",
        value_name = "HEIGHT",
        value_parser = Coord::parse,
        help = "Height of the crop in px or percent (e.g. 40 or 20%)"
    )]
    pub crop_height: Option<Coord>,

    #[arg(
        long = "crop-offset-x",
        value_name = "OFFSET_X",
        value_parser = Coord::parse,
        help = "Horizontal offset in px or percent (negative = from right edge)"
    )]
    pub offset_x: Option<Coord>,

    #[arg(
        long = "crop-offset-y",
        value_name = "OFFSET_Y",
        value_parser = Coord::parse,
        help = "Vertical offset in px or percent (negative = from bottom edge)"
    )]
    pub offset_y: Option<Coord>,
}

impl CropArgs {
    pub fn get(&self) -> Option<CropOptions> {
        Some(CropOptions {
            width: self.crop_width.clone(),
            height: self.crop_height.clone(),
            offset_x: self.offset_x.clone(),
            offset_y: self.offset_y.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiot_core::utils::coordinates::Coord;

    fn coord_px(val: i32) -> Coord {
        Coord::Px(val)
    }

    fn coord_percent(val: f32) -> Coord {
        Coord::Percent(val)
    }

    #[test]
    fn test_crop_args_get_all_none() {
        let args = CropArgs {
            crop_width: None,
            crop_height: None,
            offset_x: None,
            offset_y: None,
        };
        let crop_opts = args.get().unwrap();
        assert_eq!(crop_opts.width, None);
        assert_eq!(crop_opts.height, None);
        assert_eq!(crop_opts.offset_x, None);
        assert_eq!(crop_opts.offset_y, None);
    }

    #[test]
    fn test_crop_args_get_px_values() {
        let args = CropArgs {
            crop_width: Some(coord_px(100)),
            crop_height: Some(coord_px(50)),
            offset_x: Some(coord_px(10)),
            offset_y: Some(coord_px(20)),
        };
        let crop_opts = args.get().unwrap();
        assert_eq!(crop_opts.width, Some(coord_px(100)));
        assert_eq!(crop_opts.height, Some(coord_px(50)));
        assert_eq!(crop_opts.offset_x, Some(coord_px(10)));
        assert_eq!(crop_opts.offset_y, Some(coord_px(20)));
    }

    #[test]
    fn test_crop_args_get_percent_values() {
        let args = CropArgs {
            crop_width: Some(coord_percent(50.0)),
            crop_height: Some(coord_percent(25.0)),
            offset_x: Some(coord_percent(10.0)),
            offset_y: Some(coord_percent(5.0)),
        };
        let crop_opts = args.get().unwrap();
        assert_eq!(crop_opts.width, Some(coord_percent(50.0)));
        assert_eq!(crop_opts.height, Some(coord_percent(25.0)));
        assert_eq!(crop_opts.offset_x, Some(coord_percent(10.0)));
        assert_eq!(crop_opts.offset_y, Some(coord_percent(5.0)));
    }

    #[test]
    fn test_crop_args_get_mixed_values() {
        let args = CropArgs {
            crop_width: Some(coord_px(200)),
            crop_height: Some(coord_percent(30.0)),
            offset_x: None,
            offset_y: Some(coord_px(-15)),
        };
        let crop_opts = args.get().unwrap();
        assert_eq!(crop_opts.width, Some(coord_px(200)));
        assert_eq!(crop_opts.height, Some(coord_percent(30.0)));
        assert_eq!(crop_opts.offset_x, None);
        assert_eq!(crop_opts.offset_y, Some(coord_px(-15)));
    }
}
