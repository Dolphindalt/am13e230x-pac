use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use sha2::{Digest, Sha256};
use svd_parser::svd::{
    Access, Cluster, Device, Field, MaybeArray, Register, RegisterCluster, ValidateLevel,
    WriteConstraint, WriteConstraintRange,
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

/// Exact versions the committed output was generated with. Installed into `target/tools`,
/// isolated from global installs.
const TOOLS: &[(&str, &str)] = &[
    ("svdtools", "0.5.0"),
    ("svd2rust", "0.37.1"),
    ("form", "0.13.0"),
];

/// Separate from `TOOLS` to keep target-gen out of `check`. Must match the probe-rs version
/// used to flash.
const TARGET_GEN: &[(&str, &str)] = &[("target-gen", "0.31.0")];

const GENERATED: &[&str] = &["svd/am13e230x.svd", "src", "build.rs", "device.x"];

/// Files copied by `vendor`, as (path in the pack or SDK, path in vendor/ti).
const PACK_FILES: &[(&str, &str)] = &[
    ("03_SVD/AM13E230x.svd", "AM13E230x.svd"),
    ("01_Pack/license.txt", "LICENSE-DFP.txt"),
];
const SDK_FILES: &[(&str, &str)] = &[("source/device/am13e230x/include/hw/hw_ints.h", "hw_ints.h")];

const USAGE: &str = "\
usage: cargo xtask <command>

  generate                  Regenerate svd/, src/, build.rs and device.x from vendor/ti and patches/.
  check                     Fail if the committed generated files differ from a fresh build.
  probe-rs-target [--check] Regenerate probe-rs/am13e230x.yaml from vendor/ti/dfp, or check it.
  compare-ccs [TARGETDB]    Compare the register layout with CCS targetdb XML.
                            Default: the newest ~/ti/ccs*/ccs/ccs_base/common/targetdb.
  vendor <PACK> <SDK>       Copy TI's files from a DFP pack and an AM13E2X SDK into vendor/ti.
                            Packs: https://www.keil.arm.com/packs/am13e230x_dfp-texasinstruments/";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("generate") => generate(),
        Some("check") => check(),
        Some("compare-ccs") => compare_ccs(args.get(1).map(PathBuf::from)),
        Some("probe-rs-target") => {
            probe_rs_target(args.get(1).map(String::as_str) == Some("--check"))
        }
        Some("vendor") => match (args.get(1), args.get(2)) {
            (Some(pack), Some(sdk)) => vendor(Path::new(pack), Path::new(sdk)),
            _ => Err(format!("vendor needs a pack and an SDK directory\n\n{USAGE}").into()),
        },
        None | Some("help" | "-h" | "--help") => {
            println!("{USAGE}");
            Ok(())
        }
        Some(other) => Err(format!("unknown command `{other}`\n\n{USAGE}").into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn generate() -> Result<()> {
    let root = root();
    let out = build(&root)?;
    for path in GENERATED {
        let dst = root.join(path);
        if dst.is_dir() {
            fs::remove_dir_all(&dst)?;
        } else if dst.exists() {
            fs::remove_file(&dst)?;
        }
        copy_tree(&out.join(path), &dst)?;
    }
    println!("generated {}", GENERATED.join(", "));
    Ok(())
}

fn check() -> Result<()> {
    let root = root();
    let out = build(&root)?;
    let mut diffs = Vec::new();
    for path in GENERATED {
        diff_tree(
            &out.join(path),
            &root.join(path),
            Path::new(path),
            &mut diffs,
        )?;
    }
    if diffs.is_empty() {
        println!("generated files are up to date");
        Ok(())
    } else {
        for d in &diffs {
            eprintln!("  {d}");
        }
        Err(format!(
            "{} generated file(s) out of date; run `cargo xtask generate`",
            diffs.len()
        )
        .into())
    }
}

fn build(root: &Path) -> Result<PathBuf> {
    let bin = install_tools(root, TOOLS)?;
    verify_vendor(&root.join("vendor/ti"))?;

    let work = root.join("target/xtask/work");
    let out = root.join("target/xtask/out");
    for dir in [&work, &out] {
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
    }
    fs::create_dir_all(out.join("svd"))?;
    fs::create_dir_all(work.join("gen"))?;

    let mut device = load_svd(&root.join("vendor/ti/AM13E230x.svd"))?;
    prepare(&mut device);
    save_svd(&device, &work.join("prepared.svd"))?;
    let patch = work.join("patch.yaml");
    fs::write(
        &patch,
        "_svd: prepared.svd\n_include:\n  - ../../../patches/am13e230x.yaml\n",
    )?;
    let patched = work.join("patched.svd");
    run(Command::new(bin.join("svdtools"))
        .arg("patch")
        .arg(&patch)
        .arg(&patched))?;
    let mut device = load_svd(&patched)?;
    tidy_enums(&mut device);
    declare_full_ranges(&mut device);
    let svd = out.join("svd/am13e230x.svd");
    save_svd(&device, &svd)?;
    check_interrupts(&bin, &root.join("vendor/ti/hw_ints.h"), &svd)?;

    let gen = work.join("gen");
    run(Command::new(bin.join("svd2rust"))
        .arg("-c")
        .arg(root.join("svd2rust.toml"))
        .arg("-i")
        .arg(&svd)
        .arg("-o")
        .arg(&gen))?;
    run(Command::new(bin.join("form"))
        .arg("-i")
        .arg(gen.join("lib.rs"))
        .arg("-o")
        .arg(out.join("src")))?;
    fs::copy(gen.join("build.rs"), out.join("build.rs"))?;
    fs::copy(gen.join("device.x"), out.join("device.x"))?;
    // The working directory selects the toolchain pinned in rust-toolchain.toml.
    run(Command::new("rustfmt")
        .current_dir(root)
        .args(["--edition", "2021"])
        .arg(out.join("src/lib.rs"))
        .arg(out.join("build.rs")))?;
    Ok(out)
}

fn load_svd(path: &Path) -> Result<Device> {
    let mut config = svd_parser::Config::default();
    config.validate_level = ValidateLevel::Disabled;
    Ok(svd_parser::parse_with_config(
        &fs::read_to_string(path)?,
        &config,
    )?)
}

fn save_svd(device: &Device, path: &Path) -> Result<()> {
    fs::write(path, svd_encoder::encode(device)?)?;
    Ok(())
}

/// Runs before the patches. Collapses one-element arrays such as `CPU_INT[%s]` into single
/// elements, which svdtools has no operation for. Replaces a cluster that only wraps one register
/// with that register, and drops empty clusters. Removes the peripheral or cluster name that TI
/// repeats at the start of register names (`GSC_LOCK`, `INT_EVENT0_IMASK`).
fn prepare(device: &mut Device) {
    for p in &mut device.peripherals {
        let prefix = format!("{}_", p.name);
        if let Some(children) = p.registers.as_mut() {
            prepare_children(children, &prefix);
        }
    }
}

fn prepare_children(children: &mut Vec<RegisterCluster>, prefix: &str) {
    for child in children.iter_mut() {
        match child {
            RegisterCluster::Register(r) => {
                if let Some(index) = take_single(r) {
                    r.name = single_name(&r.name, &index);
                }
            }
            RegisterCluster::Cluster(c) => {
                if let Some(index) = take_single(c) {
                    c.name = single_name(&c.name, &index);
                }
                let prefix = format!("{}_", c.name);
                prepare_children(&mut c.children, &prefix);
            }
        }
    }
    let mut kept = Vec::new();
    for child in std::mem::take(children) {
        match child {
            RegisterCluster::Cluster(MaybeArray::Single(c)) if c.children.is_empty() => {}
            RegisterCluster::Cluster(c) => match lift_wrapped_register(&c) {
                Some(r) => kept.push(RegisterCluster::Register(r)),
                None => kept.push(RegisterCluster::Cluster(c)),
            },
            r => kept.push(r),
        }
    }
    *children = kept;
    let names: Vec<String> = children.iter().map(|c| c.name().to_string()).collect();
    for child in children.iter_mut() {
        if let RegisterCluster::Register(r) = child {
            if let Some(rest) = strip_name_prefix(&r.name, prefix) {
                if !names.contains(&rest) {
                    r.name = rest;
                }
            }
        }
    }
}

/// `INT_EVENT0_IMASK` under `INT_EVENT0_` becomes `IMASK`. The result must start with a letter
/// and differ from the cluster's own name.
fn strip_name_prefix(name: &str, prefix: &str) -> Option<String> {
    let rest = name.strip_prefix(prefix)?;
    let base = rest.trim_end_matches("[%s]");
    (base.starts_with(|c: char| c.is_ascii_alphabetic()) && base != prefix.trim_end_matches('_'))
        .then(|| rest.to_string())
}

/// Turns a cluster holding a single register into that register, at the same address. An array
/// of such clusters becomes a register array.
fn lift_wrapped_register(c: &Cluster) -> Option<Register> {
    let [RegisterCluster::Register(wrapped)] = &c.children[..] else {
        return None;
    };
    let prefix = format!("{}_", c.name);
    let name = |r: &str| {
        r.strip_prefix(&prefix)
            .filter(|rest| rest.starts_with(|c: char| c.is_ascii_alphabetic()))
            .unwrap_or(r)
            .to_string()
    };
    match (c, wrapped.clone()) {
        (MaybeArray::Single(c), MaybeArray::Single(mut r)) => {
            r.address_offset += c.address_offset;
            r.name = name(&r.name);
            Some(MaybeArray::Single(r))
        }
        (MaybeArray::Single(c), MaybeArray::Array(mut r, dim)) => {
            r.address_offset += c.address_offset;
            r.name = name(&r.name);
            Some(MaybeArray::Array(r, dim))
        }
        (MaybeArray::Array(c, dim), MaybeArray::Single(mut r)) if dim.dim_index.is_none() => {
            r.address_offset += c.address_offset;
            r.name = format!("{}[%s]", name(&r.name));
            Some(MaybeArray::Array(r, dim.clone()))
        }
        _ => None,
    }
}

fn take_single<T: Clone>(item: &mut MaybeArray<T>) -> Option<String> {
    match item {
        MaybeArray::Array(info, dim) if dim.dim == 1 => {
            let index = dim
                .dim_index
                .as_ref()
                .and_then(|i| i.first().cloned())
                .unwrap_or_else(|| "0".into());
            *item = MaybeArray::Single(info.clone());
            Some(index)
        }
        _ => None,
    }
}

fn single_name(name: &str, index: &str) -> String {
    name.replace("[%s]", "").replace("%s", index)
}

/// Value names that only restate a single bit's state.
const BIT_STATES: &[&str] = &[
    "ASSERT",
    "CLEAR",
    "CLEARED",
    "CLR",
    "FALSE",
    "NO",
    "NOEFFECT",
    "NOP",
    "NORES",
    "NO_EFFECT",
    "ONE",
    "RESET",
    "SET",
    "TOGGLE",
    "TRUE",
    "YES",
    "ZERO",
];

/// Rust keywords and svd2rust reader and writer methods, which can't be value names.
const RESERVED_NAMES: &[&str] = &[
    "as",
    "async",
    "await",
    "bit",
    "bit_is_clear",
    "bit_is_set",
    "bits",
    "break",
    "clear_bit",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "set",
    "set_bit",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "try",
    "type",
    "unsafe",
    "use",
    "variant",
    "where",
    "while",
];

/// Runs after the patches. A single-bit enum is removed, and the field uses `set_bit` and
/// `bit_is_set`, when its values only restate the bit, repeat the field name (`ENABLE` =
/// `ENABLE`), define a single value, or name 0 "no effect" or "no event". Other enums lose the
/// word prefix or suffix that all their value names share (`Ecap_cap_event1_rise` becomes `rise`).
fn tidy_enums(device: &mut Device) {
    for p in &mut device.peripherals {
        for r in p.all_registers_mut() {
            for f in r.fields.iter_mut().flatten() {
                tidy_field(f);
            }
        }
    }
}

fn tidy_field(f: &mut Field) {
    let normal = |s: &str| {
        s.to_ascii_uppercase()
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
    };
    let names: Vec<String> = f
        .enumerated_values
        .iter()
        .flat_map(|e| e.values.iter().map(|v| normal(&v.name)))
        .collect();
    let field = normal(&f.name);
    let zero_is_noop = f
        .enumerated_values
        .iter()
        .flat_map(|e| &e.values)
        .filter(|v| v.value == Some(0))
        .any(|v| {
            ["NO_EFFECT", "NOEFFECT", "NO_EVENT"]
                .iter()
                .any(|w| normal(&v.name).contains(w))
        });
    if f.bit_range.width == 1
        && !names.is_empty()
        && (names.iter().all(|n| BIT_STATES.contains(&n.as_str()))
            || names.contains(&field)
            || names.len() == 1
            || zero_is_noop)
    {
        f.enumerated_values.clear();
        return;
    }
    for e in f
        .enumerated_values
        .iter_mut()
        .filter(|e| e.derived_from.is_none())
    {
        let old: Vec<&str> = e.values.iter().map(|v| v.name.as_str()).collect();
        if let Some(new) = strip_shared_words(&old) {
            for (v, name) in e.values.iter_mut().zip(new) {
                v.name = name;
            }
        }
    }
}

fn strip_shared_words(names: &[&str]) -> Option<Vec<String>> {
    if names.len() < 2 {
        return None;
    }
    let words: Vec<Vec<&str>> = names
        .iter()
        .map(|n| n.split('_').filter(|w| !w.is_empty()).collect())
        .collect();
    let same = |i: &dyn Fn(&Vec<&str>) -> usize| {
        let first = words[0][i(&words[0])].to_ascii_lowercase();
        words.iter().all(|w| w[i(w)].to_ascii_lowercase() == first)
    };
    let mut prefix = 0;
    while words.iter().all(|w| w.len() > prefix + 1) && same(&|_| prefix) {
        prefix += 1;
    }
    let mut suffix = 0;
    while words.iter().all(|w| w.len() > prefix + suffix + 1) && same(&|w| w.len() - 1 - suffix) {
        suffix += 1;
    }
    if prefix == 0 && suffix == 0 {
        return None;
    }
    let new: Vec<String> = words
        .iter()
        .map(|w| w[prefix..w.len() - suffix].join("_"))
        .collect();
    let mut lower: Vec<String> = new.iter().map(|n| n.to_ascii_lowercase()).collect();
    let valid = lower.iter().all(|n| {
        n.starts_with(|c: char| c.is_ascii_alphabetic()) && !RESERVED_NAMES.contains(&n.as_str())
    });
    lower.sort();
    lower.dedup();
    (valid && lower.len() == new.len()).then_some(new)
}

/// Peripherals where a written value can make hardware access arbitrary memory: DMA addresses and
/// sizes, flash controller commands, memory protection, flash wait states and the NPU's SRAM
/// pointers. Their multi-bit writes stay `unsafe`.
const UNSAFE_WRITES: &[&str] = &["DMA", "FRI", "GSC", "NVMNW", "TINIELITE"];

/// Registers of otherwise safe peripherals with the same property.
const UNSAFE_WRITE_REGISTERS: &[(&str, &str)] = &[("EPI0", "ADDRMAP")];

/// Runs after the patches. Declares that any value may be written to a writable register or
/// multi-bit field, which makes svd2rust's `bits` writers safe, except in `UNSAFE_WRITES`.
fn declare_full_ranges(device: &mut Device) {
    for p in &mut device.peripherals {
        if UNSAFE_WRITES.contains(&p.name.as_str()) {
            continue;
        }
        let peripheral = p.name.clone();
        for r in p.all_registers_mut() {
            if UNSAFE_WRITE_REGISTERS.contains(&(peripheral.as_str(), r.name.as_str())) {
                continue;
            }
            let access = r.properties.access.unwrap_or(Access::ReadWrite);
            if access == Access::ReadOnly {
                continue;
            }
            let size = r.properties.size.unwrap_or(32);
            r.write_constraint.get_or_insert(full_range(size));
            for f in r.fields.iter_mut().flatten() {
                let width = f.bit_range.width;
                if f.access.unwrap_or(access) != Access::ReadOnly && width > 1 {
                    f.write_constraint.get_or_insert(full_range(width));
                }
            }
        }
    }
}

fn full_range(width: u32) -> WriteConstraint {
    WriteConstraint::Range(WriteConstraintRange {
        min: 0,
        max: u64::MAX >> (64 - width),
    })
}

fn install_tools(root: &Path, tools: &[(&str, &str)]) -> Result<PathBuf> {
    let tools_root = root.join("target/tools");
    let bin = tools_root.join("bin");
    // `cargo install --list` prints `<crate> v<version>:` per installed crate.
    let list = Command::new("cargo")
        .current_dir(root)
        .args(["install", "--list", "--root"])
        .arg(&tools_root)
        .output()?;
    let installed = String::from_utf8_lossy(&list.stdout).into_owned();
    for (tool, want) in tools {
        let line = format!("{tool} v{want}:");
        if !installed.lines().any(|l| l == line) || !bin.join(tool).exists() {
            println!("installing {tool} {want} into target/tools");
            run(Command::new("cargo")
                .current_dir(root)
                .args(["install", "--quiet", "--locked", "--root"])
                .arg(&tools_root)
                .arg(format!("{tool}@{want}")))?;
        }
    }
    Ok(bin)
}

fn sha256_hex(path: &Path) -> Result<String> {
    Ok(Sha256::digest(fs::read(path)?)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

fn verify_vendor(dir: &Path) -> Result<()> {
    let sums = fs::read_to_string(dir.join("SHA256SUMS"))?;
    for line in sums.lines().filter(|l| !l.trim().is_empty()) {
        let (want, name) = line
            .split_once("  ")
            .ok_or_else(|| format!("bad SHA256SUMS line: {line}"))?;
        let got = sha256_hex(&dir.join(name))?;
        if got != want {
            return Err(format!("vendor/ti/{name} does not match SHA256SUMS (got {got})").into());
        }
    }
    Ok(())
}

fn vendor(pack: &Path, sdk: &Path) -> Result<()> {
    let root = root();
    let stage = root.join("target/xtask/vendor");
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    fs::create_dir_all(stage.join("dfp/02_Flash_Programming"))?;

    let mut files = Vec::new();
    let archive = fs::File::open(pack).map_err(|e| format!("{}: {e}", pack.display()))?;
    let mut archive =
        zip::ZipArchive::new(archive).map_err(|e| format!("{}: {e}", pack.display()))?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        let dst = if let Some((_, dst)) = PACK_FILES.iter().find(|(src, _)| *src == name) {
            dst.to_string()
        } else if (name.ends_with(".pdsc") && !name.contains('/'))
            || (name.starts_with("02_Flash_Programming/") && name.ends_with(".FLM"))
        {
            format!("dfp/{name}")
        } else {
            continue;
        };
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        fs::write(stage.join(&dst), bytes)?;
        files.push(dst);
    }
    let has = |pred: &dyn Fn(&str) -> bool| files.iter().any(|f| pred(f));
    if !PACK_FILES.iter().all(|(_, dst)| has(&|f| f == *dst))
        || !has(&|f| f.ends_with(".pdsc"))
        || !has(&|f| f.ends_with(".FLM"))
    {
        return Err(format!(
            "{}: expected 03_SVD/AM13E230x.svd, 01_Pack/license.txt, a .pdsc and 02_Flash_Programming/*.FLM",
            pack.display()
        )
        .into());
    }
    for (src, dst) in SDK_FILES {
        let src = sdk.join(src);
        fs::copy(&src, stage.join(dst)).map_err(|e| format!("{}: {e}", src.display()))?;
        files.push(dst.to_string());
    }
    files.sort();
    let mut sums = String::new();
    for f in &files {
        sums += &format!("{}  {f}\n", sha256_hex(&stage.join(f))?);
    }
    fs::write(stage.join("SHA256SUMS"), sums)?;

    let dir = root.join("vendor/ti");
    if dir.join("dfp").exists() {
        fs::remove_dir_all(dir.join("dfp"))?;
    }
    for f in files.iter().map(String::as_str).chain(["SHA256SUMS"]) {
        let dst = dir.join(f);
        fs::create_dir_all(dst.parent().unwrap())?;
        fs::copy(stage.join(f), dst)?;
    }

    let pdsc = files.iter().find(|f| f.ends_with(".pdsc")).unwrap();
    let pdsc = fs::read_to_string(stage.join(pdsc))?;
    let pdsc = roxmltree::Document::parse(&pdsc)?;
    let release = pdsc.descendants().find(|n| n.has_tag_name("release"));
    let svd = fs::read_to_string(stage.join("AM13E230x.svd"))?;
    let svd_version = svd
        .split_once("<version>")
        .and_then(|(_, rest)| rest.split_once("</version>"))
        .map_or("?", |(version, _)| version.trim());
    println!(
        "vendored pack {} ({}), SVD \"{svd_version}\", SDK {}",
        release.and_then(|r| r.attribute("version")).unwrap_or("?"),
        release.and_then(|r| r.attribute("date")).unwrap_or("?"),
        sdk.file_name().unwrap_or(sdk.as_os_str()).to_string_lossy()
    );
    println!(
        "next: run `cargo xtask generate` and `cargo xtask probe-rs-target`, check the values in \
         patches/ against the new SDK headers, and review the diff. With CCS installed, \
         `cargo xtask compare-ccs` cross-checks the layout."
    );
    Ok(())
}

fn check_interrupts(bin: &Path, header: &Path, svd: &Path) -> Result<()> {
    let mut want = BTreeMap::new();
    for line in fs::read_to_string(header)?.lines() {
        let Some((name, rest)) = line.trim().split_once("_IRQn") else {
            continue;
        };
        let Some(value) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let value: i32 = value.split(',').next().unwrap().trim().parse()?;
        if value >= 0 {
            want.insert(value, name.to_uppercase());
        }
    }

    let output = Command::new(bin.join("svdtools"))
        .arg("interrupts")
        .arg(svd)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "svdtools interrupts failed with {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let mut got = BTreeMap::new();
    for line in String::from_utf8(output.stdout)?.lines() {
        let mut words = line.split_whitespace();
        if let (Some(Ok(value)), Some(name)) = (words.next().map(str::parse::<i32>), words.next()) {
            got.insert(value, name.trim_end_matches(':').to_string());
        }
    }

    if got != want {
        let mut msg =
            String::from("interrupts in the patched SVD do not match vendor/ti/hw_ints.h:");
        for n in want.keys().chain(got.keys()).collect::<BTreeSet<_>>() {
            if want.get(n) != got.get(n) {
                msg += &format!(
                    "\n  IRQ {n}: hw_ints.h {:?}, SVD {:?}",
                    want.get(n),
                    got.get(n)
                );
            }
        }
        return Err(msg.into());
    }
    Ok(())
}

fn probe_rs_target(check: bool) -> Result<()> {
    let root = root();
    let bin = install_tools(&root, TARGET_GEN)?;
    verify_vendor(&root.join("vendor/ti"))?;

    let work = root.join("target/xtask/probe-rs");
    if work.exists() {
        fs::remove_dir_all(&work)?;
    }
    fs::create_dir_all(&work)?;
    run(Command::new(bin.join("target-gen"))
        .arg("pack")
        .arg(root.join("vendor/ti/dfp"))
        .arg(&work))?;
    let yaml = edit_probe_rs_target(&fs::read_to_string(work.join("AM13E230X_Series.yaml"))?)?;
    let committed = root.join("probe-rs/am13e230x.yaml");
    if check {
        if fs::read_to_string(&committed).ok().as_deref() != Some(yaml.as_str()) {
            return Err(
                "probe-rs/am13e230x.yaml is out of date; run `cargo xtask probe-rs-target`".into(),
            );
        }
        println!("probe-rs/am13e230x.yaml is up to date");
    } else {
        fs::create_dir_all(root.join("probe-rs"))?;
        fs::write(committed, yaml)?;
        println!("generated probe-rs/am13e230x.yaml");
    }
    Ok(())
}

/// Removes the NONMAIN flash algorithm. An invalid erase or write of the boot configuration
/// locks the device permanently.
///
/// Marks flash bank 1 as NVM. target-gen marks only the pack's default bank.
fn edit_probe_rs_target(yaml: &str) -> Result<String> {
    // target-gen names the NONMAIN algorithm shared by all three parts after the first one.
    const NONMAIN: &str = "am13e23017_non_main";
    let start = yaml
        .find(&format!("\n- name: {NONMAIN}\n"))
        .ok_or("NONMAIN flash algorithm not found")?
        + 1;
    if yaml[start..].contains("\n- name: ") {
        return Err("NONMAIN flash algorithm is no longer the last entry".into());
    }
    let out = yaml[..start].replace(&format!("  - {NONMAIN}\n"), "");

    let bank1 = "  - !Generic\n    name: Flash_Bank1\n";
    if out.matches(bank1).count() != 3 {
        return Err("expected a Generic Flash_Bank1 region in each of the 3 variants".into());
    }
    let out = out.replace(bank1, "  - !Nvm\n    name: Flash_Bank1\n");
    if out.contains("non_main") || out.contains("0x6010") {
        return Err("NONMAIN is still referenced".into());
    }
    Ok(out)
}

/// Compares register offsets and field bit ranges. Names are excluded because the naming rules
/// and `patches/` change them deliberately.
fn compare_ccs(targetdb: Option<PathBuf>) -> Result<()> {
    let targetdb = match targetdb {
        Some(dir) => dir,
        None => default_targetdb()?,
    };
    let device_xml = targetdb.join("devices/AM13E23019.xml");
    let device_text =
        fs::read_to_string(&device_xml).map_err(|e| format!("{}: {e}", device_xml.display()))?;
    let device = roxmltree::Document::parse(&device_text)?;

    let svd_text = fs::read_to_string(root().join("svd/am13e230x.svd"))?;
    let svd = roxmltree::Document::parse(&svd_text)?;
    let peripherals: BTreeMap<&str, roxmltree::Node> = svd
        .descendants()
        .filter(|n| n.has_tag_name("peripheral"))
        .map(|p| (child_text(p, "name").unwrap_or(""), p))
        .collect();

    let mut problems = Vec::new();
    let mut svd_only = Vec::new();
    let mut seen_modules = BTreeSet::new();
    let (mut registers, mut fields) = (0, 0);
    for inst in device.descendants().filter(|n| n.has_tag_name("instance")) {
        let (Some(href), Some(base)) = (inst.attribute("href"), inst.attribute("baseaddr")) else {
            continue;
        };
        if !href.contains("Modules/am13x/am13e230x/") {
            continue;
        }
        let id = inst.attribute("id").unwrap_or("?");
        let base = parse_int(base)?;
        let Some(periph) = peripherals.values().find(|p| {
            child_text(**p, "baseAddress")
                .map(parse_int)
                .transpose()
                .ok()
                .flatten()
                == Some(base)
        }) else {
            problems.push(format!("{id}: no SVD peripheral at {base:#x}"));
            continue;
        };
        if !seen_modules.insert(href.to_string()) {
            continue;
        }
        let module_path = targetdb.join("devices").join(href);
        let module_text = fs::read_to_string(&module_path)
            .map_err(|e| format!("{}: {e}", module_path.display()))?;
        let module = roxmltree::Document::parse(&module_text)?;
        let xml_regs = flatten_xml(module.root_element(), 0)?;

        let mut base_periph = *periph;
        while let Some(from) = base_periph.attribute("derivedFrom") {
            base_periph = *peripherals
                .get(from)
                .ok_or_else(|| format!("{id}: derivedFrom {from} is not in the SVD"))?;
        }
        let regs_node = base_periph
            .children()
            .find(|n| n.has_tag_name("registers"))
            .ok_or("peripheral without registers")?;
        let svd_regs = flatten_svd(regs_node, 0)?;

        for (offset, (name, xml_fields)) in &xml_regs {
            registers += 1;
            let Some((svd_name, svd_fields)) = svd_regs.get(offset) else {
                problems.push(format!("{id}+{offset:#x} {name}: missing from SVD"));
                continue;
            };
            for (field, bits) in xml_fields {
                fields += 1;
                if !svd_fields.values().any(|b| b == bits) {
                    problems.push(format!(
                        "{id}.{svd_name}.{field}: no SVD field at bits {}:{}",
                        bits.1, bits.0
                    ));
                }
            }
        }
        for (offset, (name, _)) in &svd_regs {
            if !xml_regs.contains_key(offset) {
                svd_only.push(format!("{id}+{offset:#x} {name}"));
            }
        }
    }

    if !svd_only.is_empty() {
        println!("registers in the SVD but not in targetdb:");
        for r in &svd_only {
            println!("  {r}");
        }
    }
    if problems.is_empty() {
        println!(
            "SVD matches CCS targetdb: {} modules, {registers} registers, {fields} fields",
            seen_modules.len()
        );
        Ok(())
    } else {
        for p in &problems {
            eprintln!("  {p}");
        }
        Err(format!(
            "{} layout difference(s) between SVD and CCS targetdb",
            problems.len()
        )
        .into())
    }
}

/// CCS installs to `~/ti/ccs<version>`; use the newest one that has a targetdb.
fn default_targetdb() -> Result<PathBuf> {
    let ti = PathBuf::from(env::var("HOME")?).join("ti");
    let mut newest: Option<(u32, PathBuf)> = None;
    for entry in fs::read_dir(&ti).map_err(|e| format!("{}: {e}", ti.display()))? {
        let entry = entry?;
        let Some(version) = entry
            .file_name()
            .to_str()
            .and_then(|n| n.strip_prefix("ccs"))
            .and_then(|v| v.parse::<u32>().ok())
        else {
            continue;
        };
        let targetdb = entry.path().join("ccs/ccs_base/common/targetdb");
        if targetdb.is_dir() && newest.as_ref().is_none_or(|(v, _)| version > *v) {
            newest = Some((version, targetdb));
        }
    }
    newest
        .map(|(_, targetdb)| targetdb)
        .ok_or_else(|| format!("no CCS targetdb under {}; pass its path", ti.display()).into())
}

type Layout = BTreeMap<u64, (String, BTreeMap<String, (u64, u64)>)>;

/// In targetdb, `<group>` offsets nest, and a repeated group lists only its
/// first instance, like the first element of an SVD `dim` array.
fn flatten_xml(node: roxmltree::Node, base: u64) -> Result<Layout> {
    let mut out = Layout::new();
    for child in node.children().filter(|n| n.is_element()) {
        let offset = base + parse_int(child.attribute("offset").unwrap_or("0"))?;
        if child.has_tag_name("group") {
            out.extend(flatten_xml(child, offset)?);
        } else if child.has_tag_name("register") {
            let mut fields = BTreeMap::new();
            for f in child.children().filter(|n| n.has_tag_name("bitfield")) {
                // targetdb uses begin = MSB, end = LSB.
                let (a, b) = (
                    parse_int(f.attribute("begin").unwrap_or("0"))?,
                    parse_int(f.attribute("end").unwrap_or("0"))?,
                );
                fields.insert(
                    f.attribute("id").unwrap_or("").to_uppercase(),
                    (a.min(b), a.max(b)),
                );
            }
            out.insert(
                offset,
                (child.attribute("id").unwrap_or("").to_uppercase(), fields),
            );
        }
    }
    Ok(out)
}

fn flatten_svd(node: roxmltree::Node, base: u64) -> Result<Layout> {
    let mut out = Layout::new();
    for child in node.children().filter(|n| n.is_element()) {
        let offset = || -> Result<u64> {
            Ok(base + parse_int(child_text(child, "addressOffset").unwrap_or("0"))?)
        };
        if child.has_tag_name("cluster") {
            out.extend(flatten_svd(child, offset()?)?);
        } else if child.has_tag_name("register") {
            let mut fields = BTreeMap::new();
            for f in child.descendants().filter(|n| n.has_tag_name("field")) {
                let bits =
                    if let (Some(lsb), Some(msb)) = (child_text(f, "lsb"), child_text(f, "msb")) {
                        (parse_int(lsb)?, parse_int(msb)?)
                    } else if let (Some(off), Some(width)) =
                        (child_text(f, "bitOffset"), child_text(f, "bitWidth"))
                    {
                        let off = parse_int(off)?;
                        (off, off + parse_int(width)? - 1)
                    } else {
                        let range = child_text(f, "bitRange").ok_or("field without bit range")?;
                        let (msb, lsb) = range
                            .trim_matches(|c| c == '[' || c == ']')
                            .split_once(':')
                            .ok_or("bad bitRange")?;
                        (parse_int(lsb)?, parse_int(msb)?)
                    };
                fields.insert(child_text(f, "name").unwrap_or("").to_uppercase(), bits);
            }
            out.insert(
                offset()?,
                (
                    child_text(child, "name").unwrap_or("").to_uppercase(),
                    fields,
                ),
            );
        }
    }
    Ok(out)
}

fn child_text<'a>(node: roxmltree::Node<'a, '_>, tag: &str) -> Option<&'a str> {
    node.children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|n| n.text())
        .map(str::trim)
}

fn parse_int(s: &str) -> Result<u64> {
    let s = s.trim();
    Ok(
        match s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            Some(hex) => u64::from_str_radix(hex, 16)?,
            None => s.parse()?,
        },
    )
}

fn run(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .map_err(|e| format!("cannot run {:?}: {e}", cmd.get_program()))?;
    if !status.success() {
        return Err(format!("{:?} failed with {status}", cmd.get_program()).into());
    }
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            copy_tree(&entry.path(), &dst.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dst)?;
    }
    Ok(())
}

fn diff_tree(new: &Path, old: &Path, rel: &Path, diffs: &mut Vec<String>) -> Result<()> {
    if new.is_dir() || old.is_dir() {
        let mut names = BTreeSet::new();
        for dir in [new, old] {
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    names.insert(entry?.file_name());
                }
            }
        }
        for name in names {
            diff_tree(&new.join(&name), &old.join(&name), &rel.join(&name), diffs)?;
        }
    } else if !old.exists() {
        diffs.push(format!("missing: {}", rel.display()));
    } else if !new.exists() {
        diffs.push(format!("stale: {}", rel.display()));
    } else if fs::read(new)? != fs::read(old)? {
        diffs.push(format!("changed: {}", rel.display()));
    }
    Ok(())
}
