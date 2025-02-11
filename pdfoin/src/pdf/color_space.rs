// Table 62 - Colour Space Families
// Table 90 - Default Decode Arrays
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorSpace {
    CIEBased(CIEBasedColorSpace),
    Device(DeviceColorSpace),
    Special(SpecialColorSpace),
}

impl ColorSpace {
    pub(crate) fn to_lopdf_object_name(&self) -> ::lopdf::Object {
        match self {
            ColorSpace::CIEBased(cs) => cs.to_lopdf_object_name(),
            ColorSpace::Device(cs) => cs.to_lopdf_object_name(),
            ColorSpace::Special(cs) => cs.to_lopdf_object_name(),
        }
    }
}

impl From<CIEBasedColorSpace> for ColorSpace {
    fn from(value: CIEBasedColorSpace) -> Self {
        Self::CIEBased(value)
    }
}

impl From<DeviceColorSpace> for ColorSpace {
    fn from(value: DeviceColorSpace) -> Self {
        Self::Device(value)
    }
}

impl From<SpecialColorSpace> for ColorSpace {
    fn from(value: SpecialColorSpace) -> Self {
        Self::Special(value)
    }
}

// Figure 20 - Colour Specification
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CIEBasedColorSpace {
    CalGray,
    CalRGB,
    Lab,
    ICCBased,
}

impl CIEBasedColorSpace {
    pub(crate) fn to_lopdf_object_name(&self) -> ::lopdf::Object {
        let s = match self {
            CIEBasedColorSpace::CalGray => "CalGray",
            CIEBasedColorSpace::CalRGB => "CalRGB",
            CIEBasedColorSpace::Lab => "Lab",
            CIEBasedColorSpace::ICCBased => "ICCBased",
        };
        ::lopdf::Object::Name(s.into())
    }
}

// Figure 20 - Colour Specification
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeviceColorSpace {
    DeviceGray,
    DeviceRGB,
    DeviceCMYK,
}

impl DeviceColorSpace {
    pub(crate) fn to_lopdf_object_name(&self) -> ::lopdf::Object {
        let s = match self {
            DeviceColorSpace::DeviceGray => "DeviceGray",
            DeviceColorSpace::DeviceRGB => "DeviceRGB",
            DeviceColorSpace::DeviceCMYK => "DeviceCMYK",
        };
        ::lopdf::Object::Name(s.into())
    }
}

// Figure 20 - Colour Specification
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpecialColorSpace {
    Indexed,
    Pattern,
    Separation,
    DeviceN,
}

impl SpecialColorSpace {
    pub(crate) fn to_lopdf_object_name(&self) -> ::lopdf::Object {
        let s = match self {
            SpecialColorSpace::Indexed => "Indexed",
            SpecialColorSpace::Pattern => "Pattern",
            SpecialColorSpace::Separation => "Separation",
            SpecialColorSpace::DeviceN => "DeviceN",
        };
        ::lopdf::Object::Name(s.into())
    }
}
