use std::fs;
use std::path::Path;

pub fn generate_pwa_manifest(site_title: &str) {
    let dist_dir = Path::new("frontend/dist");
    if !dist_dir.exists() {
        return;
    }

    let pwa_manifest = serde_json::json!({
        "name": site_title,
        "short_name": site_title,
        "description": "Minimalist server status and visor telemetry dashboard",
        "start_url": "/",
        "display": "standalone",
        "background_color": "#ffffff",
        "theme_color": "#000000",
        "icons": [
            { "src": "favicon.svg", "type": "image/svg+xml", "sizes": "any" },
            { "src": "favicon.png", "type": "image/png", "sizes": "192x192" },
            { "src": "favicon.png", "type": "image/png", "sizes": "512x512" }
        ],
        "orientation": "any"
    });

    let _ = fs::write(
        dist_dir.join("manifest.json"),
        serde_json::to_string_pretty(&pwa_manifest).unwrap_or_default(),
    );

    // Also generate asset-manifest.json for service worker registration
    let mut assets = Vec::new();
    fn walk_dir(dir: &Path, prefix: &str, assets: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let rel = if prefix.is_empty() {
                    entry.file_name().to_string_lossy().to_string()
                } else {
                    format!("{}/{}", prefix, entry.file_name().to_string_lossy())
                };
                if path.is_dir() {
                    walk_dir(&path, &rel, assets);
                } else {
                    assets.push(format!("/{}", rel));
                }
            }
        }
    }
    walk_dir(dist_dir, "", &mut assets);
    let _ = fs::write(
        dist_dir.join("asset-manifest.json"),
        serde_json::to_string_pretty(&assets).unwrap_or_default(),
    );
}
