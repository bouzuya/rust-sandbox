use anyhow::Context as _;

use crate::pdf::{
    bits_per_component::BitsPerComponent,
    color_space::{ColorSpace, DeviceColorSpace},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    width: i64,
    height: i64,
    color_space: ColorSpace,
    bits_per_component: BitsPerComponent,
    samples: Vec<u8>,
    alphas: Option<Vec<u8>>,
}

impl Image {
    pub(crate) fn from_png_file_path<P: AsRef<std::path::Path>>(p: P) -> anyhow::Result<Self> {
        let decoder = png::Decoder::new(std::fs::File::open(p)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        let bytes = &buf[..info.buffer_size()];
        let color_space = ColorSpace::from(match info.color_type {
            png::ColorType::Grayscale => DeviceColorSpace::DeviceGray,
            png::ColorType::Rgb => DeviceColorSpace::DeviceRGB,
            png::ColorType::Indexed => todo!(),
            png::ColorType::GrayscaleAlpha => DeviceColorSpace::DeviceGray,
            png::ColorType::Rgba => DeviceColorSpace::DeviceRGB,
        });
        let bits_per_component = BitsPerComponent::from_u8(info.bit_depth as u8)
            .context("png crate BitDepth -> BitsPerComponent")?;
        let (samples, alphas) = {
            let step = info.color_type.samples();
            let grayscale = match info.color_type {
                png::ColorType::Grayscale | png::ColorType::GrayscaleAlpha => true,
                png::ColorType::Rgb | png::ColorType::Rgba => false,
                png::ColorType::Indexed => todo!(),
            };
            let alpha = match info.color_type {
                png::ColorType::Grayscale | png::ColorType::Rgb => false,
                png::ColorType::Indexed => todo!(),
                png::ColorType::GrayscaleAlpha | png::ColorType::Rgba => true,
            };
            let mut s = Vec::with_capacity(bytes.len() / step * if grayscale { 1 } else { 3 });
            let mut a = Vec::with_capacity(bytes.len() / step);
            for i in (0..bytes.len()).step_by(step) {
                for j in 0..step - if alpha { 1 } else { 0 } {
                    s.push(bytes[i + j]);
                }
                if alpha {
                    a.push(bytes[i + step - 1]);
                }
            }
            (s, alpha.then_some(a))
        };
        let width = i64::from(info.width);
        let height = i64::from(info.height);
        Ok(Self {
            color_space,
            width,
            height,
            bits_per_component,
            samples,
            alphas,
        })
    }

    #[allow(dead_code)]
    pub fn components_per_sample(&self) -> usize {
        match self.color_space {
            ColorSpace::CIEBased(_cs) => todo!(),
            ColorSpace::Device(cs) => match cs {
                DeviceColorSpace::DeviceGray => 1,
                DeviceColorSpace::DeviceRGB => 3,
                DeviceColorSpace::DeviceCMYK => 4,
            },
            ColorSpace::Special(_cs) => todo!(),
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub(crate) fn into_lopdf_stream(self) -> ::lopdf::Stream {
        let mut stream = lopdf::Stream::new(
            // Table 89 - Additional Entries Specific to an Image Dictionary
            lopdf::Dictionary::from_iter(
                vec![
                    ("Type", ::lopdf::Object::Name("XObject".into())),
                    ("Subtype", ::lopdf::Object::Name("Image".into())),
                    ("Width", ::lopdf::Object::Integer(self.width)),
                    ("Height", ::lopdf::Object::Integer(self.height)),
                    ("ColorSpace", self.color_space.to_lopdf_object_name()),
                    (
                        "BitsPerComponent",
                        self.bits_per_component.to_lopdf_object_integer(),
                    ),
                    // Intent
                    // ImageMask
                    // Mask
                    // Decode
                    // Interpolate
                    // Alternates
                    // SMaskInData
                    // Name
                    // StructParent
                    // ID
                    // OPI
                    // Metadata
                    // OC
                ]
                .into_iter()
                .chain(
                    self.alphas
                        .map(|samples| {
                            vec![(
                                "SMask",
                                ::lopdf::Object::Stream(
                                    Self {
                                        color_space: ColorSpace::Device(
                                            DeviceColorSpace::DeviceGray,
                                        ),
                                        width: self.width,
                                        height: self.height,
                                        bits_per_component: BitsPerComponent::Eight,
                                        samples,
                                        alphas: None,
                                    }
                                    .into_lopdf_stream(),
                                ),
                            )]
                            .into_iter()
                        })
                        .unwrap_or_default(),
                ),
            ),
            self.samples,
        );
        stream.compress().unwrap();
        stream
    }
}
