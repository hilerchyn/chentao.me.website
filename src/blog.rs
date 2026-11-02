use std::sync::OnceLock;

use pulldown_cmark::{html, Options, Parser};
use rust_embed::RustEmbed;
use serde::Deserialize;

use crate::i18n::Lang;

#[derive(RustEmbed)]
#[folder = "content/blog/"]
struct BlogFiles;

#[derive(Debug, Clone)]
pub struct Kind {
    pub id: String,
    pub order: u32,
    pub name_zh: String,
    pub name_en: String,
    pub desc_zh: String,
    pub desc_en: String,
}

impl Kind {
    pub fn name(&self, lang: Lang) -> &str {
        match lang {
            Lang::Zh => &self.name_zh,
            Lang::En => &self.name_en,
        }
    }

    pub fn desc(&self, lang: Lang) -> &str {
        match lang {
            Lang::Zh => &self.desc_zh,
            Lang::En => &self.desc_en,
        }
    }

    pub fn href(&self, lang: Lang) -> String {
        match lang {
            Lang::Zh => format!("/blog/{}", self.id),
            Lang::En => format!("/en/blog/{}", self.id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub kind: String,
    pub slug: String,
    pub lang: Lang,
    pub title: String,
    pub date: String,
    pub summary: String,
    pub markdown: String,
}

impl Post {
    pub fn href(&self, lang: Lang) -> String {
        match lang {
            Lang::Zh => format!("/blog/{}/{}", self.kind, self.slug),
            Lang::En => format!("/en/blog/{}/{}", self.kind, self.slug),
        }
    }

    pub fn html(&self) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        let parser = Parser::new_ext(&self.markdown, options);
        let mut out = String::new();
        html::push_html(&mut out, parser);
        out
    }
}

#[derive(Debug, Clone)]
pub struct Catalog {
    pub kinds: Vec<Kind>,
    pub posts: Vec<Post>,
}

impl Catalog {
    pub fn kind(&self, id: &str) -> Option<&Kind> {
        self.kinds.iter().find(|k| k.id == id)
    }

    pub fn posts_for(&self, lang: Lang, kind: Option<&str>) -> Vec<&Post> {
        let mut posts: Vec<&Post> = self
            .posts
            .iter()
            .filter(|p| p.lang == lang)
            .filter(|p| kind.map(|id| p.kind == id).unwrap_or(true))
            .collect();
        posts.sort_by(|a, b| b.date.cmp(&a.date).then(a.title.cmp(&b.title)));
        posts
    }

    pub fn post(&self, kind: &str, slug: &str, lang: Lang) -> Option<&Post> {
        self.posts
            .iter()
            .find(|p| p.kind == kind && p.slug == slug && p.lang == lang)
    }

    pub fn count(&self, lang: Lang, kind: &str) -> usize {
        self.posts
            .iter()
            .filter(|p| p.lang == lang && p.kind == kind)
            .count()
    }

    pub fn kind_views(&self, lang: Lang, active: Option<&str>) -> Vec<KindView> {
        self.kinds
            .iter()
            .map(|k| KindView {
                id: k.id.clone(),
                name: k.name(lang).to_string(),
                desc: k.desc(lang).to_string(),
                href: k.href(lang),
                count: self.count(lang, &k.id),
                active: active == Some(k.id.as_str()),
            })
            .collect()
    }

    pub fn post_views(&self, lang: Lang, kind: Option<&str>) -> Vec<PostView> {
        self.posts_for(lang, kind)
            .into_iter()
            .map(|p| {
                let kind_name = self
                    .kind(&p.kind)
                    .map(|k| k.name(lang).to_string())
                    .unwrap_or_else(|| p.kind.clone());
                PostView {
                    title: p.title.clone(),
                    summary: p.summary.clone(),
                    date: p.date.clone(),
                    kind_name,
                    kind_href: kind_href(lang, &p.kind),
                    href: p.href(lang),
                }
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct KindView {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub href: String,
    pub count: usize,
    pub active: bool,
}

#[derive(Clone)]
pub struct PostView {
    pub title: String,
    pub summary: String,
    pub date: String,
    pub kind_name: String,
    pub kind_href: String,
    pub href: String,
}

pub fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(load)
}

pub fn index_href(lang: Lang) -> &'static str {
    match lang {
        Lang::Zh => "/blog",
        Lang::En => "/en/blog",
    }
}

pub fn kind_href(lang: Lang, id: &str) -> String {
    match lang {
        Lang::Zh => format!("/blog/{id}"),
        Lang::En => format!("/en/blog/{id}"),
    }
}

pub fn post_href(lang: Lang, kind: &str, slug: &str) -> String {
    match lang {
        Lang::Zh => format!("/blog/{kind}/{slug}"),
        Lang::En => format!("/en/blog/{kind}/{slug}"),
    }
}

fn load() -> Catalog {
    let declared = load_types();
    let mut posts = Vec::new();
    let mut seen_kinds: Vec<String> = declared.iter().map(|k| k.id.clone()).collect();

    for path in BlogFiles::iter() {
        let path = path.as_ref();
        if path == "types.toml" || path.eq_ignore_ascii_case("readme.md") {
            continue;
        }
        let Some((kind_id, slug, lang)) = parse_path(path) else {
            tracing::warn!("skip blog file with unexpected name: {path}");
            continue;
        };
        let Some(file) = BlogFiles::get(path) else {
            continue;
        };
        let Ok(raw) = std::str::from_utf8(&file.data) else {
            tracing::warn!("blog file is not utf-8: {path}");
            continue;
        };
        match parse_article(raw) {
            Some((meta, markdown)) => {
                if !seen_kinds.iter().any(|id| id == &kind_id) {
                    seen_kinds.push(kind_id.clone());
                }
                posts.push(Post {
                    kind: kind_id,
                    slug,
                    lang,
                    title: meta.title,
                    date: meta.date,
                    summary: meta.summary,
                    markdown: markdown.to_string(),
                });
            }
            None => tracing::warn!("blog file is missing title/date front matter: {path}"),
        }
    }

    let mut kinds = declared;
    for id in seen_kinds {
        if kinds.iter().any(|k| k.id == id) {
            continue;
        }
        kinds.push(Kind {
            name_zh: id.clone(),
            name_en: id.clone(),
            desc_zh: String::new(),
            desc_en: String::new(),
            id,
            order: 1000,
        });
    }
    kinds.sort_by_key(|k| (k.order, k.id.clone()));

    Catalog { kinds, posts }
}

#[derive(Deserialize, Default)]
struct TypesFile {
    #[serde(rename = "type", default)]
    types: Vec<TypeToml>,
}

#[derive(Deserialize)]
struct TypeToml {
    id: String,
    #[serde(default)]
    order: u32,
    name_zh: String,
    name_en: String,
    #[serde(default)]
    desc_zh: String,
    #[serde(default)]
    desc_en: String,
}

fn load_types() -> Vec<Kind> {
    let Some(file) = BlogFiles::get("types.toml") else {
        return Vec::new();
    };
    let Ok(raw) = std::str::from_utf8(&file.data) else {
        return Vec::new();
    };
    match toml::from_str::<TypesFile>(raw) {
        Ok(parsed) => parsed
            .types
            .into_iter()
            .filter(|t| is_id(&t.id))
            .map(|t| Kind {
                id: t.id,
                order: t.order,
                name_zh: t.name_zh,
                name_en: t.name_en,
                desc_zh: t.desc_zh,
                desc_en: t.desc_en,
            })
            .collect(),
        Err(err) => {
            tracing::error!("types.toml: {err}");
            Vec::new()
        }
    }
}

#[derive(Deserialize)]
struct Meta {
    title: String,
    date: String,
    #[serde(default)]
    summary: String,
}

pub fn parse_path(path: &str) -> Option<(String, String, Lang)> {
    let path = path.replace('\\', "/");
    let mut parts = path.split('/');
    let kind = parts.next()?.to_string();
    let file = parts.next()?;
    if parts.next().is_some() || !is_id(&kind) {
        return None;
    }
    let (stem, lang) = if let Some(stem) = file.strip_suffix(".zh.md") {
        (stem, Lang::Zh)
    } else if let Some(stem) = file.strip_suffix(".en.md") {
        (stem, Lang::En)
    } else {
        return None;
    };
    if stem.is_empty() || !is_id(stem) {
        return None;
    }
    Some((kind, stem.to_string(), lang))
}

pub(crate) fn parse_article(raw: &str) -> Option<(Meta, &str)> {
    let raw = raw.trim_start_matches('\u{feff}').trim_start();
    let rest = raw.strip_prefix("+++")?.trim_start_matches(['\n', '\r']);
    let fence = rest.find("\n+++")?;
    let toml_src = rest[..fence].trim();
    let body = rest[fence + 4..].trim_start_matches(['+', '\n', '\r']).trim();
    let meta: Meta = toml::from_str(toml_src).ok()?;
    if meta.title.is_empty() || meta.date.is_empty() {
        return None;
    }
    Some((meta, body))
}

fn is_id(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_path_reads_kind_slug_lang() {
        let (kind, slug, lang) =
            parse_path("architecture/lay-the-path.zh.md").expect("path");
        assert_eq!(kind, "architecture");
        assert_eq!(slug, "lay-the-path");
        assert_eq!(lang, Lang::Zh);
    }

    #[test]
    fn parse_article_reads_toml_front_matter() {
        let raw = "+++\ntitle = \"Hello\"\ndate = \"2026-11-03\"\nsummary = \"S\"\n+++\n\nBody.\n";
        let (meta, body) = parse_article(raw).expect("article");
        assert_eq!(meta.title, "Hello");
        assert_eq!(meta.date, "2026-11-03");
        assert_eq!(meta.summary, "S");
        assert_eq!(body, "Body.");
    }

    #[test]
    fn catalog_includes_seed_types_and_posts() {
        let cat = load();
        assert!(cat.kind("architecture").is_some());
        assert!(cat.kind("ai").is_some());
        assert_eq!(cat.kinds.len(), 6);
        assert!(cat.post("architecture", "lay-the-path", Lang::Zh).is_some());
        assert!(cat.post("architecture", "lay-the-path", Lang::En).is_some());
        assert_eq!(cat.posts_for(Lang::Zh, None).len(), 6);
        assert_eq!(cat.posts_for(Lang::En, Some("ai")).len(), 1);
    }
}
