use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ModuleArtifacts {
    inventory_md: String,
    scope_md: Option<String>,
    spec_mds: Vec<String>,
    bench_logs: Vec<String>,
    math_reviews: Vec<String>,
    tests_dir: Option<String>,
    benches: Vec<String>,
}

fn repo_root_from_crate_root(crate_root: &Path) -> Result<PathBuf, String> {
    // Standalone crate checkout: the crate root is the repo root.
    Ok(crate_root.to_path_buf())
}

fn rel_path(repo_root: &Path, path: &Path) -> Result<String, String> {
    let rel = path
        .canonicalize()
        .map_err(|e| format!("canonicalize failed for {path:?}: {e}"))?
        .strip_prefix(
            repo_root
                .canonicalize()
                .map_err(|e| format!("canonicalize failed for repo_root {repo_root:?}: {e}"))?,
        )
        .map_err(|e| format!("strip_prefix failed for {path:?}: {e}"))?
        .to_string_lossy()
        .to_string();
    Ok(rel.replace('\\', "/"))
}

fn discover_module_inventories(crate_root: &Path) -> Result<Vec<PathBuf>, String> {
    let src_dir = crate_root.join("src");
    let mut inventories: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(&src_dir).map_err(|e| format!("read_dir failed: {e}"))? {
        let entry = entry.map_err(|e| format!("read_dir entry failed: {e}"))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let inv = path.join("docs").join("inventory.md");
        if inv.is_file() {
            inventories.push(inv);
        }
    }
    inventories.sort_by(|a, b| {
        let ma = a
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .unwrap_or_else(|| OsStr::new(""))
            .to_string_lossy();
        let mb = b
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .unwrap_or_else(|| OsStr::new(""))
            .to_string_lossy();
        ma.cmp(&mb)
    });
    // Flat single-module crate: fall back to the crate-level inventory.
    if inventories.is_empty() {
        let inv = crate_root.join("docs").join("inventory.md");
        if inv.is_file() {
            inventories.push(inv);
        }
    }
    Ok(inventories)
}

fn collect_artifacts(
    repo_root: &Path,
    crate_root: &Path,
    module_inventory: &Path,
) -> Result<ModuleArtifacts, String> {
    let module_dir = module_inventory
        .parent()
        .and_then(|p| p.parent())
        .ok_or("module_inventory path is too short")?;
    let module_name = module_dir
        .file_name()
        .ok_or("module dir has no name")?
        .to_string_lossy()
        .to_string();
    let docs_dir = module_dir.join("docs");

    let inventory_md = rel_path(repo_root, module_inventory)?;
    let scope_path = docs_dir.join("scope.md");
    let scope_md = if scope_path.is_file() {
        Some(rel_path(repo_root, &scope_path)?)
    } else {
        None
    };

    let mut spec_mds: Vec<String> = vec![];
    let mut bench_logs: Vec<String> = vec![];
    if docs_dir.is_dir() {
        for entry in fs::read_dir(&docs_dir).map_err(|e| format!("read_dir docs failed: {e}"))? {
            let entry = entry.map_err(|e| format!("read_dir docs entry failed: {e}"))?;
            let p = entry.path();
            if !p.is_file() || p.extension() != Some(OsStr::new("md")) {
                continue;
            }
            let name = p
                .file_name()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy();
            if name == "inventory.md" || name == "scope.md" {
                continue;
            }
            if name.contains("SPEC") {
                spec_mds.push(rel_path(repo_root, &p)?);
            }
            let lower = name.to_lowercase();
            if lower.contains("bench") && lower.contains("result") {
                bench_logs.push(rel_path(repo_root, &p)?);
            }
        }
    }
    spec_mds.sort();
    bench_logs.sort();

    let reviews_dir = docs_dir.join("reviews");
    let mut math_reviews: Vec<String> = vec![];
    if reviews_dir.is_dir() {
        for entry in
            fs::read_dir(&reviews_dir).map_err(|e| format!("read_dir reviews failed: {e}"))?
        {
            let entry = entry.map_err(|e| format!("read_dir reviews entry failed: {e}"))?;
            let p = entry.path();
            if !p.is_file() || p.extension() != Some(OsStr::new("md")) {
                continue;
            }
            let name = p
                .file_name()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy();
            if name.contains("module_math_review") {
                math_reviews.push(rel_path(repo_root, &p)?);
            }
        }
    }
    math_reviews.sort();

    let tests_dir_path = module_dir.join("tests");
    let flat_tests_path = crate_root.join("src").join("tests");
    let tests_dir = if tests_dir_path.is_dir() {
        Some(rel_path(repo_root, &tests_dir_path)?)
    } else if flat_tests_path.is_dir() {
        Some(rel_path(repo_root, &flat_tests_path)?)
    } else {
        None
    };

    let benches_dir = crate_root.join("benches");
    let mut benches: Vec<String> = vec![];
    if benches_dir.is_dir() {
        for entry in
            fs::read_dir(&benches_dir).map_err(|e| format!("read_dir benches failed: {e}"))?
        {
            let entry = entry.map_err(|e| format!("read_dir benches entry failed: {e}"))?;
            let p = entry.path();
            if !p.is_file() || p.extension() != Some(OsStr::new("rs")) {
                continue;
            }
            let stem = p
                .file_stem()
                .unwrap_or_else(|| OsStr::new(""))
                .to_string_lossy()
                .to_string();
            if stem == module_name || stem.starts_with(&(module_name.clone() + "_")) {
                benches.push(rel_path(repo_root, &p)?);
            }
        }
    }
    benches.sort();

    Ok(ModuleArtifacts {
        inventory_md,
        scope_md,
        spec_mds,
        bench_logs,
        math_reviews,
        tests_dir,
        benches,
    })
}

fn parse_inventory_source_file_purposes(inventory_text: &str) -> HashMap<String, String> {
    // Accept only explicit file-purpose lines:
    // - `math/src/<module>/file.rs`: purpose...
    let mut map = HashMap::new();
    for raw_line in inventory_text.lines() {
        let line = raw_line.trim_start();
        if !line.starts_with("- `") {
            continue;
        }
        let rest = &line[3..];
        let Some(end_tick) = rest.find('`') else {
            continue;
        };
        let path = rest[..end_tick].trim();
        if !path.ends_with(".rs") {
            continue;
        }
        if !(path.starts_with("src/") || path.starts_with("bin/")) {
            continue;
        }
        let after = rest[end_tick + 1..].trim_start();
        if !after.starts_with(':') {
            continue;
        }
        let purpose = after[1..].trim();
        if purpose.is_empty() {
            continue;
        }
        map.insert(path.to_string(), purpose.to_string());
    }
    map
}

fn module_source_files(repo_root: &Path, module_dir: &Path) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = vec![];
    let mut stack: Vec<PathBuf> = vec![module_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).map_err(|e| format!("read_dir failed for {dir:?}: {e}"))? {
            let entry = entry.map_err(|e| format!("read_dir entry failed: {e}"))?;
            let p = entry.path();
            if p.is_dir() {
                // The flat-crate walk starts at the crate root; skip non-source trees.
                const SKIP: [&str; 10] = [
                    "tests",
                    "target",
                    "node_modules",
                    ".git",
                    ".pi",
                    ".dev",
                    ".vscode",
                    ".husky",
                    "docs",
                    "benches",
                ];
                let name = p.file_name().map(|n| n.to_string_lossy().to_string());
                if name.is_some_and(|n| SKIP.contains(&n.as_str())) {
                    continue;
                }
                stack.push(p);
                continue;
            }
            if p.is_file() && p.extension() == Some(OsStr::new("rs")) {
                out.push(rel_path(repo_root, &p)?);
            }
        }
    }
    out.sort();
    Ok(out)
}

fn validate_exists(repo_root: &Path, rel: &str) -> Result<(), String> {
    let p = repo_root.join(rel);
    if !p.exists() {
        return Err(format!("referenced path does not exist: {rel}"));
    }
    Ok(())
}

fn now_utc_iso_z() -> String {
    // Avoid extra deps; rely on RFC3339-ish output via chrono? not allowed.
    // We keep a stable placeholder with date only from system time.
    let now = std::time::SystemTime::now();
    let dt: dt::DateTime = now.into();
    dt.to_string()
}

mod dt {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct DateTime {
        y: i32,
        m: u32,
        d: u32,
        hh: u32,
        mm: u32,
        ss: u32,
    }

    impl From<SystemTime> for DateTime {
        fn from(t: SystemTime) -> Self {
            // Minimal UTC conversion without external crates.
            // This is used only for a generated timestamp; it does not affect correctness.
            let secs = t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
            unix_to_utc(secs)
        }
    }

    impl ToString for DateTime {
        fn to_string(&self) -> String {
            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                self.y, self.m, self.d, self.hh, self.mm, self.ss
            )
        }
    }

    fn unix_to_utc(mut secs: i64) -> DateTime {
        // Algorithm: convert seconds since epoch to UTC date/time.
        // Source: well-known civil-from-days approach (Howard Hinnant).
        let ss = (secs.rem_euclid(60)) as u32;
        secs = secs.div_euclid(60);
        let mm = (secs.rem_euclid(60)) as u32;
        secs = secs.div_euclid(60);
        let hh = (secs.rem_euclid(24)) as u32;
        let days = secs.div_euclid(24);

        let (y, m, d) = civil_from_days(days);
        DateTime {
            y,
            m,
            d,
            hh,
            mm,
            ss,
        }
    }

    fn civil_from_days(z: i64) -> (i32, u32, u32) {
        // Convert days since 1970-01-01 to Gregorian date.
        let z = z + 719468;
        let era = (if z >= 0 { z } else { z - 146096 }).div_euclid(146097);
        let doe = z - era * 146097;
        let yoe = (doe - doe.div_euclid(1460) + doe.div_euclid(36524) - doe.div_euclid(146096))
            .div_euclid(365);
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe.div_euclid(4) - yoe.div_euclid(100));
        let mp = (5 * doy + 2).div_euclid(153);
        let d = doy - (153 * mp + 2).div_euclid(5) + 1;
        let m = mp + if mp < 10 { 3 } else { -9 };
        let y = y + if m <= 2 { 1 } else { 0 };
        (y as i32, m as u32, d as u32)
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let strict = args.iter().any(|a| a == "--strict");

    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = repo_root_from_crate_root(&crate_root)?;
    let out_path = crate_root.join("inventory.md");

    let inventories = discover_module_inventories(&crate_root)?;
    let now = now_utc_iso_z();

    let mut all_file_paths: HashSet<String> = HashSet::new();
    let mut any_gap = false;

    let mut lines: Vec<String> = vec![];
    lines.push("".to_string());
    lines.push("# `rainflow` — Global Inventory (GENERATED; DO NOT EDIT)".to_string());
    lines.push("".to_string());
    lines.push(format!("Generated: {now}"));
    lines.push("Protocol: `.dev/protocols/global_inventory_generation_protocol.md`".to_string());
    lines.push("".to_string());
    lines.push(
        "This file is generated from the crate inventory at `docs/inventory.md`.".to_string(),
    );
    lines.push(
        "If a file purpose is missing in a module inventory, this file will mark it as `INVENTORY GAP`."
            .to_string(),
    );
    lines.push("".to_string());
    lines.push("## Modules".to_string());
    lines.push("".to_string());

    for inv in &inventories {
        let module_name = inv
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .ok_or("module name missing")?
            .to_string_lossy()
            .to_string();
        let inv_rel = rel_path(&repo_root, inv)?;
        validate_exists(&repo_root, &inv_rel)?;
        lines.push(format!("- `{module_name}`: `{inv_rel}`"));
    }

    lines.push("".to_string());
    lines.push("---".to_string());
    lines.push("".to_string());

    for inv in &inventories {
        let module_dir = inv
            .parent()
            .and_then(|p| p.parent())
            .ok_or("module dir missing")?;
        let module_name = module_dir
            .file_name()
            .ok_or("module name missing")?
            .to_string_lossy()
            .to_string();

        let artifacts = collect_artifacts(&repo_root, &crate_root, inv)?;
        validate_exists(&repo_root, &artifacts.inventory_md)?;
        if let Some(scope) = &artifacts.scope_md {
            validate_exists(&repo_root, scope)?;
        }
        for s in &artifacts.spec_mds {
            validate_exists(&repo_root, s)?;
        }
        for r in &artifacts.math_reviews {
            validate_exists(&repo_root, r)?;
        }
        for b in &artifacts.bench_logs {
            validate_exists(&repo_root, b)?;
        }
        if let Some(t) = &artifacts.tests_dir {
            validate_exists(&repo_root, t)?;
        }
        for b in &artifacts.benches {
            validate_exists(&repo_root, b)?;
        }

        let inv_text =
            fs::read_to_string(inv).map_err(|e| format!("read inventory failed: {inv:?}: {e}"))?;
        let purposes = parse_inventory_source_file_purposes(&inv_text);
        let module_files = module_source_files(&repo_root, module_dir)?;

        lines.push(format!("## `{module_name}`"));
        lines.push("".to_string());
        lines.push("### Artifacts".to_string());
        lines.push("".to_string());
        lines.push(format!("- Inventory: `{}`", artifacts.inventory_md));
        if let Some(scope) = &artifacts.scope_md {
            lines.push(format!("- Scope: `{scope}`"));
        }
        for s in &artifacts.spec_mds {
            lines.push(format!("- Spec: `{s}`"));
        }
        for r in &artifacts.math_reviews {
            lines.push(format!("- Math review: `{r}`"));
        }
        for b in &artifacts.bench_logs {
            lines.push(format!("- Bench log: `{b}`"));
        }
        if let Some(t) = &artifacts.tests_dir {
            lines.push(format!("- Tests: `{t}`"));
        }
        for b in &artifacts.benches {
            lines.push(format!("- Benches: `{b}`"));
        }

        lines.push("".to_string());
        lines.push("### Source Files".to_string());
        lines.push("".to_string());

        for rel in module_files {
            if all_file_paths.contains(&rel) {
                return Err(format!("duplicate file path across modules: {rel}"));
            }
            all_file_paths.insert(rel.clone());

            if let Some(purpose) = purposes.get(&rel) {
                lines.push(format!("- `{rel}`: {purpose}"));
            } else {
                any_gap = true;
                lines.push(format!(
                    "- `{rel}`: INVENTORY GAP (add 1-line purpose in `{}`)",
                    artifacts.inventory_md
                ));
            }
        }

        lines.push("".to_string());
        lines.push("---".to_string());
        lines.push("".to_string());
    }

    let mut content = lines.join("\n");
    if !content.ends_with('\n') {
        content.push('\n');
    }
    fs::write(&out_path, content).map_err(|e| format!("write failed for {out_path:?}: {e}"))?;

    if strict && any_gap {
        std::process::exit(2);
    }
    Ok(())
}
