use std::sync::{Arc, OnceLock};

use mermaid_rs_renderer::{RenderOptions, Theme as DiagramTheme};
use resvg::usvg::{fontdb, roxmltree};
use resvg::{tiny_skia, usvg};

use crate::font::FALLBACK_FONT;
use crate::theme::Theme;

const MAX_SOURCE_BYTES: usize = 32 * 1024;
const MAX_IMAGE_SIDE: f32 = 2048.0;

static DIAGRAM_FONTS: OnceLock<Arc<fontdb::Database>> = OnceLock::new();

pub struct Diagram {
    pub png: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub labels: String,
}

pub fn render(source: &str, theme: Theme) -> Option<Diagram> {
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
        fontdb: Arc::clone(DIAGRAM_FONTS.get_or_init(|| {
            let mut fonts = fontdb::Database::new();
            fonts.load_system_fonts();
            // LastResort maps unsupported characters to placeholder glyphs.
            let placeholder_faces: Vec<_> = fonts
                .faces()
                .filter(|face| face.post_script_name == "LastResort")
                .map(|face| face.id)
                .collect();
            for id in placeholder_faces {
                fonts.remove_face(id);
            }
            // usvg uses the generic serif family as its last font choice.
            if fonts
                .query(&fontdb::Query {
                    families: &[fontdb::Family::Serif],
                    ..Default::default()
                })
                .is_none()
            {
                fonts.load_font_data(FALLBACK_FONT.to_vec());
                fonts.set_serif_family("Source Serif 4");
            }
            Arc::new(fonts)
        })),
        // The source document can be untrusted. Only render the diagram itself.
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: Box::new(|_, _, _| None),
            resolve_string: Box::new(|_, _| None),
        },
        ..Default::default()
    };
    rasterize_svg(&svg, &options)
}

fn rasterize_svg(svg: &str, options: &usvg::Options<'_>) -> Option<Diagram> {
    let xml = roxmltree::Document::parse(svg).ok()?;
    let labels: Vec<_> = xml
        .descendants()
        .filter(|node| node.has_tag_name(("http://www.w3.org/2000/svg", "text")))
        .map(|node| {
            node.descendants()
                .filter(roxmltree::Node::is_text)
                .filter_map(|part| part.text())
                .flat_map(str::split_whitespace)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|label| !label.is_empty())
        .collect();
    let tree = usvg::Tree::from_xmltree(&xml, options).ok()?;
    // usvg can keep the diagram geometry after dropping unrenderable labels.
    if rendered_label_count(tree.root())? != labels.len() {
        return None;
    }
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
    Some(Diagram {
        png: image.encode_png().ok()?,
        width,
        height,
        labels: labels.join("\n"),
    })
}

fn rendered_label_count(group: &usvg::Group) -> Option<usize> {
    let mut count = 0;
    for node in group.children() {
        match node {
            usvg::Node::Group(group) => count += rendered_label_count(group)?,
            usvg::Node::Text(text) => {
                if text.flattened().children().is_empty()
                    || text
                        .layouted()
                        .iter()
                        .flat_map(|span| &span.positioned_glyphs)
                        .any(|glyph| glyph.id.0 == 0)
                {
                    return None;
                }
                count += 1;
            }
            _ => {}
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_svg_whose_label_cannot_be_drawn() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="40">
            <rect width="100" height="40"/>
            <text x="10" y="20" fill="white">Label</text>
        </svg>"#;
        assert!(rasterize_svg(svg, &usvg::Options::default()).is_none());
    }

    #[test]
    fn rejects_diagram_with_a_missing_glyph() {
        assert!(render(
            "flowchart LR\nA[Ready] --> B[Missing\u{10ffff}]",
            Theme::Dark
        )
        .is_none());
    }
}
