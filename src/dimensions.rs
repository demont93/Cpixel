use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Dimensions {
    pub height: usize,
    pub width: usize,
}

impl Dimensions {
    pub fn fit_with_locked_ratio(
        image_dimensions: &Dimensions,
        screen_dimensions: &Dimensions,
    ) -> Self {
        assert_ne!(image_dimensions.width, 0, "Input image width can't be 0.");
        assert_ne!(image_dimensions.height, 0, "Input image height can't be 0.");
        assert_ne!(screen_dimensions.width, 0, "Screen width can't be 0.");
        assert_ne!(screen_dimensions.height, 0, "Screen height can't be 0.");
        let image_ratio = image_dimensions.ratio();
        let screen_ratio = screen_dimensions.ratio();
        let mut final_dim = match image_ratio.partial_cmp(&screen_ratio) {
            // Fit image height.
            Some(Ordering::Greater) => {
                Dimensions {
                    height: screen_dimensions.height,
                    width: (screen_dimensions.height as f64 / image_ratio) as usize,
                }
            }
            // Fit image width.
            Some(Ordering::Less) => {
                Dimensions {
                    height: (screen_dimensions.width as f64 * image_ratio) as usize,
                    width: screen_dimensions.width,
                }
            }
            // Return screen dimensions.
            Some(Ordering::Equal) => {
                *screen_dimensions
            }
            // Comparison failed.
            None => {
                panic!("Error comparing screen ratio with image ratio.")
            }
        };
        if final_dim.height <= 0 { final_dim.height = 1 }
        if final_dim.width <= 0 { final_dim.width = 1 }
        final_dim
    }

    pub fn ratio(&self) -> f64 {
        self.height as f64 / self.width as f64
    }

    pub fn total(&self) -> usize {
        self.height * self.width
    }

    pub fn locked_ratio_resize(&self, dim: &Dim) -> Self {
        match *dim {
            Dim::Width(w) => Dimensions {
                height: match (self.ratio() * w as f64) as usize {
                    h if h == 0 => 1,
                    h => h,
                },
                width: w,
            },
            Dim::Height(h) => Dimensions {
                height: h,
                width: match (h as f64 / self.ratio()) as usize {
                    w if w == 0 => 1,
                    w => w,
                },
            },
        }
    }
}

impl PartialOrd for Dimensions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let comp_1 = self.width.partial_cmp(&other.width);
        let comp_2 = self.height.partial_cmp(&other.height);
        comp_1.and_then(|ord_1| comp_2.and_then(|ord_2| {
            if ord_1 == ord_2 {
                Some(ord_1)
            } else if ord_1 == Ordering::Equal {
                Some(ord_2)
            } else if ord_2 == Ordering::Equal {
                Some(ord_1)
            } else {
                None
            }
        }))
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Dim {
    Width(usize),
    Height(usize),
}

#[cfg(test)]
mod tests {
    use super::{Dimensions};

    #[test]
    fn test_closest_best_size_same_size() {
        let screen_dimensions = Dimensions { height: 300, width: 1 };
        let image_dimensions = Dimensions { height: 300, width: 1 };
        let final_dim = Dimensions::fit_with_locked_ratio(&image_dimensions, &screen_dimensions);
        assert_eq!(final_dim, image_dimensions);
    }

    #[test]
    fn test_closest_best_size_same_ratio() {
        let screen_dimensions = Dimensions { height: 600, width: 2 };
        let image_dimensions = Dimensions { height: 300, width: 1 };
        assert_eq!(screen_dimensions.ratio(), image_dimensions.ratio());
        let final_dim = Dimensions::fit_with_locked_ratio(&image_dimensions,
                                                          &screen_dimensions);
        assert_eq!(final_dim, screen_dimensions);
    }

    #[test]
    fn test_fit_width() {
        let screen_dimensions = Dimensions { height: 100, width: 100 };
        let image_dimensions = Dimensions { height: 100, width: 200 };
        let final_dim = Dimensions::fit_with_locked_ratio(&image_dimensions,
                                                          &screen_dimensions);
        assert_eq!(final_dim, Dimensions { height: 50, width: 100 });
    }

    #[test]
    fn test_fit_height() {
        let screen_dimensions = Dimensions { height: 100, width: 100 };
        let image_dimensions = Dimensions { height: 200, width: 100 };
        let final_dim = Dimensions::fit_with_locked_ratio(&image_dimensions,
                                                          &screen_dimensions);
        assert_eq!(final_dim, Dimensions { height: 100, width: 50 });
    }

    #[test]
    fn test_minimum_height_1() {
        let screen_dimensions = Dimensions { height: 100, width: 100 };
        let image_dimensions = Dimensions { height: 1, width: 200 };
        let final_dim = Dimensions::fit_with_locked_ratio(&image_dimensions,
                                                          &screen_dimensions);
        assert_eq!(final_dim, Dimensions { height: 1, width: 100 });
    }

    #[test]
    fn test_minimum_width_1() {
        let screen_dimensions = Dimensions { height: 100, width: 100 };
        let image_dimensions = Dimensions { height: 200, width: 1 };
        let final_dim = Dimensions::fit_with_locked_ratio(&image_dimensions,
                                                          &screen_dimensions);
        assert_eq!(final_dim, Dimensions { height: 100, width: 1 });
    }

    #[test]
    #[should_panic]
    fn test_cant_pass_image_with_zero() {
        let screen_dimensions = Dimensions { height: 10, width: 100 };
        let image_dimensions = Dimensions { height: 0, width: 1 };
        Dimensions::fit_with_locked_ratio(&image_dimensions,
                                          &screen_dimensions);
    }

    #[test]
    #[should_panic]
    fn test_cant_pass_screen_with_zero() {
        let screen_dimensions = Dimensions { height: 10, width: 00 };
        let image_dimensions = Dimensions { height: 50, width: 1 };
        Dimensions::fit_with_locked_ratio(&image_dimensions,
                                          &screen_dimensions);
    }
}