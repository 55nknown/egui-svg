#[derive(Copy, Clone)]
pub enum SvgFit {
    Width(u32),
    Height(u32),
    Size(u32, u32),
    Scale(f32),
}

impl Into<usvg::FitTo> for SvgFit {
    fn into(self) -> usvg::FitTo {
        match self {
            Self::Width(w) => usvg::FitTo::Width(w),
            Self::Height(h) => usvg::FitTo::Height(h),
            Self::Size(w, h) => usvg::FitTo::Size(w, h),
            Self::Scale(s) => usvg::FitTo::Zoom(s),
        }
    }
}
