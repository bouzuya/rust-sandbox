use anyhow::bail;
use entity::BId;
use std::io;
use use_case::{HasViewUseCase, ViewUseCase};

pub fn view<T: HasViewUseCase>(
    app: &T,
    content: bool,
    id: BId,
    meta: bool,
    writer: &mut impl io::Write,
) -> anyhow::Result<()> {
    let (m, c) = app.view_use_case().handle(id)?;
    match (content, meta) {
        (true, true) => bail!("meta or content"),
        (true, false) => write!(writer, "{}", c)?,
        (false, true) => {
            #[derive(serde::Serialize)]
            struct Json {
                id: String,
                tags: Vec<String>,
            }
            write!(
                writer,
                "{}",
                serde_json::to_string(&Json {
                    id: m.id.to_string(),
                    tags: m.tags,
                })?
            )?
        }
        (false, false) => write!(writer, "{}", c)?,
    }
    Ok(())
}
