mod bits_per_component;
mod color_space;
mod image;
mod unit;

pub use self::image::Image;
pub(crate) use self::unit::NumExt;

use anyhow::Context;

use self::unit::Px;

pub(crate) struct Document(::lopdf::Document);

impl Document {
    pub(crate) fn load<P: AsRef<std::path::Path>>(input: P) -> anyhow::Result<Self> {
        let lopdf_document = ::lopdf::Document::load(input).context("Document::load")?;
        Ok(Self(lopdf_document))
    }

    pub(crate) fn insert_image(
        &mut self,
        page_no: u32,
        image: Image,
        (x, y): (Px, Px),
    ) -> anyhow::Result<()> {
        let page_object_id = *self.0.get_pages().get(&page_no).context("page not found")?;

        let page_height = {
            let media_box_rectangle = self
                .0
                .get_object(page_object_id)
                .expect("page with the specified page id to exist")
                .as_dict()
                .expect("page to be a dictionary")
                .get("MediaBox".as_bytes())
                .expect("page dictionary to contain MediaBox entry")
                .as_array()
                .expect("MediaBox value to be a rectangle")
                .iter()
                .map(|it| {
                    it.as_float()
                        .expect("MediaBox rectangle element to be float")
                })
                .collect::<Vec<f32>>();
            let lly = media_box_rectangle[1]; // lly: lower-left y
            let ury = media_box_rectangle[3]; // ury: upper-right y
            ury.px() - lly.px()
        };

        // top-left -> bottom-left
        let height = (image.height() as f32).px();
        let y = page_height - height - y;

        let position = (x.to_f32(), y.to_f32());
        let size = ((image.width() as f32), (image.height() as f32));
        self.0
            .insert_image(page_object_id, image.into_lopdf_stream(), position, size)
            .context("Document::insert_image")
    }

    pub(crate) fn save_to<W: std::io::Write>(&mut self, writer: &mut W) -> anyhow::Result<()> {
        self.0.save_to(writer).context("Document::save_to")
    }
}
