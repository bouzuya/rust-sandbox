#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct Mm(f32);

impl Mm {
    pub(crate) fn to_px(&self) -> Px {
        Px(mm_to_px(self.0))
    }
}

impl std::ops::Sub<Mm> for Mm {
    type Output = Mm;

    fn sub(self, rhs: Mm) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct Px(f32);

impl Px {
    #[allow(dead_code)]
    pub(crate) fn to_mm(&self) -> Mm {
        Mm(px_to_mm(self.0))
    }

    pub(crate) fn to_f32(&self) -> f32 {
        self.0
    }
}

impl std::ops::Sub<Px> for Px {
    type Output = Px;

    fn sub(self, rhs: Px) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

pub(crate) trait F32Ext {
    fn mm(&self) -> Mm;
    fn px(&self) -> Px;
}

impl F32Ext for f32 {
    fn mm(&self) -> Mm {
        Mm(*self)
    }

    fn px(&self) -> Px {
        Px(*self)
    }
}

fn mm_to_px(mm: f32) -> f32 {
    let mmpi = 25.4; // mm per inch
    let dpi = 72.0; // dot (px) per inch
    let px = mm / mmpi * dpi;
    px
}

fn px_to_mm(px: f32) -> f32 {
    let mmpi = 25.4; // mm per inch
    let dpi = 72.0; // dot (px) per inch
    let mm = px / dpi * mmpi;
    mm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_to_px() {
        // A4 = 210mm x 297mm
        assert_eq!(mm_to_px(210.0), 595.2756);
        assert_eq!(mm_to_px(297.0), 841.88983);
    }

    #[test]
    fn test_px_to_mm() {
        // A4 = 210mm x 297mm
        assert_eq!(px_to_mm(595.2756), 210.0);
        assert_eq!(px_to_mm(841.88983), 297.0);
    }
}
