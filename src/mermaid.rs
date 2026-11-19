use std::sync::{Arc, OnceLock};

use mermaid_rs_renderer::{RenderOptions, Theme as DiagramTheme};
use resvg::{tiny_skia, usvg};

use crate::theme::Theme;

const MAX_SOURCE_BYTES: usize = 32 * 1024;
const MAX_IMAGE_SIDE: f32 = 2048.0;

static SYSTEM_FONTS: OnceLock<Arc<usvg::fontdb::Database>> = OnceLock::new();

pub fn render(source: &str, theme: Theme) -> Option<(Vec<u8>, u32, u32)> {
    if source.len() > MAX_SOURCE_BYTES {
        return None;
    }
    let theme = match theme {
        Theme::Dark => DiagramTheme::dark(),
        Theme::Light => DiagramTheme::mermaid_default(),
    };
    let svg = mermaid_rs_renderer::render_strict(
        source,
        RenderOptions {
            theme,
            ..RenderOptions::default()
        },
    )
    .ok()?;

    let options = usvg::Options {
        fontdb: Arc::clone(SYSTEM_FONTS.get_or_init(|| {
            let mut fonts = usvg::fontdb::Database::new();
            fonts.load_system_fonts();
            Arc::new(fonts)
        })),
        // The source document can be untrusted. Only render the diagram itself.
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: Box::new(|_, _, _| None),
            resolve_string: Box::new(|_, _| None),
        },
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(&svg, &options).ok()?;
    let size = tree.size();
    let scale = (MAX_IMAGE_SIDE / size.width().max(size.height())).min(1.0);
    let width = (size.width() * scale).ceil() as u32;
    let height = (size.height() * scale).ceil() as u32;
    let mut image = tiny_skia::Pixmap::new(width, height)?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut image.as_mut(),
    );
    Some((image.encode_png().ok()?, width, height))
}
