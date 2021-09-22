use entity::BId;
use std::io;
use use_case::{HasViewUseCase, ViewUseCase};

pub fn view<T: HasViewUseCase>(
    app: &T,
    id: BId,
    writer: &mut impl io::Write,
) -> anyhow::Result<()> {
    let (_, content) = app.view_use_case().handle(id)?;
    write!(writer, "{}", content)?;
    Ok(())
}
