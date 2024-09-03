use super::fileserv::Assets;
use leptos::prelude::*;
use leptos::{
    attr::global::GlobalAttributes, component, prelude::LeptosOptions, tachys::html::element::link,
    IntoView,
};
use leptos_meta::*;

/// Custom version of Leptos's HashedStylesheet that attempts to load the hash.txt
/// file from embedded assets before looking on the filesystem.
#[component]
pub fn HashedStylesheet(
    /// Leptos options
    options: LeptosOptions,
    /// An ID for the stylesheet.
    #[prop(optional, into)]
    id: Option<String>,
) -> impl IntoView {
    let mut css_file_name = options.output_name.to_string();
    if options.hash_files {
        let hash_path = std::env::current_exe()
            .map(|path| path.parent().map(|p| p.to_path_buf()).unwrap_or_default())
            .unwrap_or_default()
            .join(&options.hash_file);
        let hashes = match Assets::get(&options.hash_file) {
            Some(hashes) => String::from_utf8(hashes.data.to_vec())
                .expect("failed to parse embedded hash file as text"),
            None => std::fs::read_to_string(&hash_path).expect("failed to read hash file"),
        };
        for line in hashes.lines() {
            let line = line.trim();
            if !line.is_empty() {
                if let Some((file, hash)) = line.split_once(':') {
                    if file == "css" {
                        css_file_name.push_str(&format!(".{}", hash.trim()));
                    }
                }
            }
        }
    }
    css_file_name.push_str(".css");
    let pkg_path = &options.site_pkg_dir;
    view! {
        <Stylesheet id={id.unwrap_or_default()} href=format!("/{pkg_path}/{css_file_name}") />
    }
}
