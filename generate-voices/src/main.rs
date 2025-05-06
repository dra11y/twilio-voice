use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use convert_case::{Case, Casing};
use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};

/// Entry point: fetches Twilio voice data and generates voice module files
fn main() {
    let result = fetch_and_generate_voice_modules();
    match result {
        Ok(_) => println!("Successfully generated voice modules!"),
        Err(e) => eprintln!("Error generating voice modules: {e:?}"),
    }
}

/// Output directory for generated voice modules
const DIR_PATH: &str = "../src/twiml/voices";
/// Twilio documentation page containing voice data
const TWILIO_DOC_URL: &str =
    "https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages";
/// Common Rust derive macros for voice enums
const ENUM_DERIVE: &str =
    "#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]";

/// Represents a single voice option with its metadata
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VoiceData {
    language_code: String,
    voice_type: String,
    gender: String,
    provider: String,
    voice_name: String,
}

/// Orchestrates the entire voice module generation process
fn fetch_and_generate_voice_modules() -> Result<(), Box<dyn Error>> {
    let html = fetch_html()?;
    let (pricing, all_voices) = parse_html_into_voices(html);
    println!("Found {} unique voices", all_voices.len());
    println!("Pricing:\n{pricing:#?}");
    generate_voice_module_structure(pricing, &all_voices)?;
    Command::new("cargo")
        .arg("fmt")
        .current_dir("../")
        .status()?;
    Ok(())
}

/// Extracts voice data and pricing information from Twilio's documentation HTML
fn parse_html_into_voices(html: String) -> (HashMap<String, f32>, HashSet<VoiceData>) {
    let document = Html::parse_document(&html);
    let row_selector = Selector::parse("table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    // Extract pricing information from the HTML
    let mut pricing = HashMap::new();

    // Process pricing tables
    let table_selector = Selector::parse("table").unwrap();
    let header_selector = Selector::parse("th").unwrap();

    // First extract pricing information from tables containing price data
    for table in document.select(&table_selector) {
        // Check if this is a pricing table by examining headers
        let headers: Vec<_> = table.select(&header_selector).collect();
        if headers.len() < 3 {
            continue;
        }
        let header_texts: Vec<String> = headers
            .iter()
            .map(|h| h.text().collect::<String>().trim().to_string())
            .collect();

        if !header_texts.contains(&"Price per 100 characters".to_string()) {
            // not a pricing table
            continue;
        }

        // extract the base pricing tier (starting from 0 characters)
        for row in table.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            if cells.len() < 3 {
                continue;
            }

            let min_chars = cells[0].text().collect::<String>().trim().to_string();

            if min_chars == "0" {
                let price_text = cells[2].text().collect::<String>().trim().to_string();
                let price = price_text
                    .strip_prefix('$')
                    .expect("no text in expected price column")
                    .parse::<f32>()
                    .expect("failed to parse expected price");

                // Navigate up to find the section ID to determine voice type (Standard/Neural/Generative)
                let mut parent = table.parent();
                while let Some(node) = parent {
                    if let Some(element) = node.value().as_element() {
                        if element.name() == "section" {
                            let section_id = element.id();
                            if let Some(id) = section_id {
                                if id.contains("standard-voices") {
                                    pricing.insert("Standard".to_string(), price);
                                } else if id.contains("neural-voices") {
                                    pricing.insert("Neural".to_string(), price);
                                } else if id.contains("generative-voices") {
                                    pricing.insert("Generative".to_string(), price);
                                }
                            }
                            break;
                        }
                    }
                    parent = node.parent();
                }
            }
        }
    }

    // Then extract voice data from voice tables
    let mut all_voices = HashSet::new();
    for row in document.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() < 6 {
            continue;
        }

        let language_code = cells[1].text().collect::<String>().trim().to_string();
        if language_code.is_empty() {
            continue;
        }

        let voice_type = cells[2].text().collect::<String>().trim().to_string();
        if voice_type == "Basic" || voice_type.is_empty() {
            continue;
        }

        let gender = cells[3]
            .text()
            .collect::<String>()
            .trim()
            .replace(['(', ')'], " ")
            .to_case(Case::Pascal);

        let provider = cells[4].text().collect::<String>().trim().to_string();

        let voice_name = cells[5]
            .text()
            .collect::<String>()
            .replace('*', "")
            .trim()
            .to_string();
        if voice_name.is_empty() {
            continue;
        }

        all_voices.insert(VoiceData {
            language_code,
            voice_type,
            gender,
            provider,
            voice_name,
        });
    }

    (pricing, all_voices)
}

/// Fetches HTML from Twilio docs using headless Chrome
/// Uses a local cache to avoid refetching the same page multiple times per day
fn fetch_html() -> Result<String, Box<dyn Error>> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let cache_path = format!("/tmp/voices_{today}.html");

    // Check for cached version first
    if Path::new(&cache_path).exists() {
        println!("Using cached HTML file: {cache_path}");
        let mut file = File::open(&cache_path)?;
        let mut html_content = String::new();
        file.read_to_string(&mut html_content)?;
        return Ok(html_content);
    }

    println!("Cache not found, fetching: {TWILIO_DOC_URL} ...");
    let options = LaunchOptions {
        headless: true,
        ..Default::default()
    };
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(TWILIO_DOC_URL)?;
    println!("Waiting for navigation...");
    tab.wait_until_navigated()?;

    let html_content = tab.get_content()?;
    let mut cache_file = File::create(&cache_path)?;
    cache_file.write_all(html_content.as_bytes())?;
    println!("Cached HTML to: {cache_path}");

    Ok(html_content)
}

/// Creates the directory structure and generates all voice module files
fn generate_voice_module_structure(
    pricing: HashMap<String, f32>,
    voices: &HashSet<VoiceData>,
) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(DIR_PATH)?;

    // Generate main mod.rs first, which returns language groups for further processing
    let lang_groups = generate_main_file(pricing, voices)?;

    // Then generate a module file for each language
    for (lang_code, voices_in_lang) in &lang_groups {
        generate_lang_file(lang_code, voices_in_lang)?;
    }

    Ok(())
}

/// Generates gender alias convenience modules
fn generate_gender_aliases(
    lang_file: &mut String,
    type_groups: &HashMap<String, Vec<&VoiceData>>,
) -> Result<(), Box<dyn Error>> {
    // Collect all genders first
    let mut all_genders = HashSet::new();
    for voices in type_groups.values() {
        for voice in voices {
            all_genders.insert(voice.gender.clone());
        }
    }

    let mut all_genders: Vec<String> = all_genders.into_iter().collect();
    all_genders.sort();

    // For each gender, generate alias modules
    for gender in all_genders {
        writeln!(lang_file, "\npub mod {} {{", gender.to_case(Case::Snake))?;

        // For each voice type that has this gender
        for (voice_type, voices) in type_groups {
            // Check if this voice type has voices of the current gender
            let voices_with_gender: Vec<_> = voices.iter().filter(|v| v.gender == gender).collect();

            if voices_with_gender.is_empty() {
                continue;
            }

            let voice_type_module = voice_type.to_case(Case::Snake);

            writeln!(lang_file, "    pub mod {voice_type_module} {{",)?;

            // Group by provider
            let provider_map = group_voices_by(&voices_with_gender, |v| v.provider.clone());

            // For each provider
            for (provider, provider_voices) in provider_map {
                let provider_module = provider.to_case(Case::Snake);
                writeln!(lang_file, "        pub mod {provider_module} {{",)?;
                writeln!(
                    lang_file,
                    "            use super::super::super::{voice_type_module}::{provider_module}::*;",
                )?;

                // For each voice in this provider/gender combo
                let mut voices: Vec<_> = provider_voices
                    .iter()
                    .map(|v| extract_short_name(&v.voice_name))
                    .collect();
                voices.sort();

                for voice in &voices {
                    writeln!(
                        lang_file,
                        "            pub const {voice}: {gender} = {gender}::{voice};",
                    )?;
                }

                writeln!(lang_file, "        }}")?;
            }

            writeln!(lang_file, "    }}")?;
        }

        writeln!(lang_file, "}}")?;
    }

    Ok(())
}

/// Generates a language-specific module file with voice types, providers, and genders
fn generate_lang_file(
    lang_code: &str,
    voices_in_lang: &[&VoiceData],
) -> Result<(), Box<dyn Error>> {
    let module_name = lang_code.to_case(Case::Snake);
    let lang_variant = lang_code.to_case(Case::Pascal);

    let mut lang_file = String::new();

    // Group voices by type (Standard, Neural, Generative)
    let type_groups = group_voices_by(voices_in_lang, |v| v.voice_type.clone());

    writeln!(lang_file)?;
    writeln!(lang_file, "#![allow(non_upper_case_globals)]\n")?;
    writeln!(
        lang_file,
        "use crate::twiml::{{Gender, VoiceGender, VoicePrice, voices::{{"
    )?;
    for voice_type in type_groups.keys() {
        writeln!(
            lang_file,
            "{}_VOICE_PRICE,",
            voice_type.to_case(Case::Constant)
        )?;
    }
    writeln!(lang_file, "}},}};\n")?;
    writeln!(lang_file, "use serde::{{Serialize, Deserialize}};\n")?;

    // Generate modules for each voice type (Standard, Neural, Generative)
    for (voice_type, voices_of_type) in &type_groups {
        let type_module = voice_type.to_case(Case::Snake);
        writeln!(lang_file, "pub mod {type_module} {{\n    use super::*;\n")?;

        // Group voices by provider (Amazon, Google, etc.)
        let provider_groups = group_voices_by(voices_of_type, |v| v.provider.clone());

        // Generate modules for each provider
        for (provider, voices_by_provider) in &provider_groups {
            let provider_module = provider.to_case(Case::Snake);

            // Group voices by gender for this provider
            let mut gender_maps: HashMap<&str, HashMap<String, String>> = HashMap::new();

            for voice in voices_by_provider {
                let variant_name = extract_short_name(&voice.voice_name);
                if variant_name.is_empty() {
                    panic!(
                        "variant_name is empty! voice.voice_name: {}",
                        voice.voice_name
                    );
                }

                let gender_map = gender_maps.entry(&voice.gender).or_default();
                if !gender_map.contains_key(&variant_name) {
                    gender_map.insert(
                        variant_name.clone(),
                        format!("{}.{}", voice.provider, voice.voice_name),
                    );
                }
            }

            writeln!(
                lang_file,
                "    pub mod {provider_module} {{\n        use super::*;\n"
            )?;

            // Generate enums for each gender
            for (gender, voice_map) in &gender_maps {
                if voice_map.is_empty() {
                    continue;
                }
                writeln!(lang_file, "        {ENUM_DERIVE}")?;
                writeln!(lang_file, "        #[non_exhaustive]")?;
                writeln!(lang_file, "        pub enum {gender} {{")?;
                let mut keys: Vec<_> = voice_map.keys().collect();
                keys.sort();
                for key in keys {
                    let full_name = &voice_map[key];
                    writeln!(lang_file, "            #[serde(rename = \"{full_name}\")]")?;
                    writeln!(lang_file, "            {key},")?;
                }
                writeln!(lang_file, "        }}\n")?;

                write_voice_price_impl(&mut lang_file, gender, Some(voice_type), None)?;
                write_voice_gender_impl(
                    &mut lang_file,
                    gender,
                    Some(&format!("Gender::{gender}")),
                    None,
                )?;

                writeln!(
                    lang_file,
                    r#"
                        impl From<{gender}> for crate::twiml::Voice {{
                            fn from(value: {gender}) -> Self {{
                                Self::{lang_variant}(super::super::Voice::{voice_type}(super::Voice::{provider}(
                                    Voice::{gender}(value),
                                )))
                            }}
                        }}
                    "#
                )?;
            }

            // Create a Voice enum that contains all gender variants
            writeln!(lang_file, "        {ENUM_DERIVE}")?;
            writeln!(lang_file, "        #[serde(untagged)]")?;
            writeln!(lang_file, "        pub enum Voice {{")?;

            let mut gender_arms = Vec::new();
            for gender in gender_maps.keys() {
                writeln!(lang_file, "            {gender}({gender}),")?;

                gender_arms.push((
                    None,
                    format!("Voice::{gender}(_)"),
                    format!("Gender::{gender}"),
                ));
            }
            writeln!(lang_file, "        }}\n")?;

            write_voice_price_impl(&mut lang_file, "Voice", Some(voice_type), None)?;

            write_voice_gender_impl(&mut lang_file, "Voice", None, Some(&gender_arms))?;

            writeln!(lang_file, "    }}\n")?;
        }

        // Create a Voice enum that contains all provider variants
        writeln!(lang_file, "     {ENUM_DERIVE}")?;
        writeln!(lang_file, "    #[serde(untagged)]")?;
        writeln!(lang_file, "    pub enum Voice {{")?;

        for (provider, voices_by_provider) in &provider_groups {
            if voices_by_provider.is_empty() {
                continue;
            }

            let provider_module = provider.to_case(Case::Snake);
            let variant_name = provider_module.to_case(Case::Pascal);
            writeln!(
                lang_file,
                "        {variant_name}({provider_module}::Voice),"
            )?;
        }
        writeln!(lang_file, "    }}\n")?;

        write_voice_price_impl(&mut lang_file, "Voice", Some(voice_type), None)?;

        let mut gender_arms = Vec::new();
        for provider in provider_groups.keys() {
            gender_arms.push((
                None,
                format!("Voice::{provider}(voice)"),
                "voice.gender()".to_string(),
            ));
        }

        write_voice_gender_impl(&mut lang_file, "Voice", None, Some(&gender_arms))?;

        writeln!(lang_file, "}}\n")?;
    }

    // Create a top-level Voice enum for this language
    writeln!(lang_file, "{ENUM_DERIVE}")?;
    writeln!(lang_file, "#[serde(untagged)]")?;
    writeln!(lang_file, "pub enum Voice {{")?;
    for (voice_type, groups) in &type_groups {
        if groups.is_empty() {
            continue;
        }

        let variant_name = voice_type.to_case(Case::Pascal);
        writeln!(
            lang_file,
            "    {variant_name}({}::Voice),",
            voice_type.to_case(Case::Snake)
        )?;
    }
    writeln!(lang_file, "}}")?;

    // Implement price calculation for the language's Voice enum
    let mut price_arms = Vec::new();
    let mut gender_arms = Vec::new();

    for voice_type in type_groups.keys() {
        let voice_type_const = voice_type.to_case(Case::Constant);
        price_arms.push((
            None,
            format!("Voice::{voice_type}(_)"),
            format!("Some({voice_type_const}_VOICE_PRICE)"),
        ));
        gender_arms.push((
            None,
            format!("Voice::{voice_type}(voice)"),
            "voice.gender()".to_string(),
        ));
    }

    write_voice_price_impl(&mut lang_file, "Voice", None, Some(&price_arms))?;
    write_voice_gender_impl(&mut lang_file, "Voice", None, Some(&gender_arms))?;

    // Generate gender-based alias modules for easier access
    generate_gender_aliases(&mut lang_file, &type_groups)?;

    // Write the file to disk
    File::create(Path::new(DIR_PATH).join(format!("{module_name}.rs")))?
        .write_all(lang_file.as_bytes())?;
    Ok(())
}

/// Generates the main mod.rs file with language-specific modules and price constants
fn generate_main_file(
    pricing: HashMap<String, f32>,
    voices: &HashSet<VoiceData>,
) -> Result<HashMap<String, Vec<&VoiceData>>, Box<dyn Error>> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%I").to_string();

    // Group voices by language code
    let lang_groups: HashMap<String, Vec<&VoiceData>> =
        voices.iter().fold(HashMap::new(), |mut map, voice| {
            map.entry(voice.language_code.clone())
                .or_default()
                .push(voice);
            map
        });

    let mut main_file = String::new();
    writeln!(
        main_file,
        "// Auto-generated at: {now}\n// Source: {TWILIO_DOC_URL}"
    )?;
    writeln!(main_file, "#![allow(non_local_definitions)]\n")?;

    // Generate price constants for each voice type (Standard, Neural, Generative)
    for (voice_type, price_per_100_chars) in pricing {
        writeln!(
            main_file,
            "/// Current price of {voice_type} voices per 100 chars as of {now} UTC"
        )?;
        writeln!(
            main_file,
            "pub const {}_VOICE_PRICE: f32 = {price_per_100_chars};",
            voice_type.to_case(Case::Constant)
        )?;
    }

    writeln!(main_file)?;

    // Generate module imports for each language with feature flags
    let mut lang_codes: Vec<_> = lang_groups.keys().collect();
    lang_codes.sort();

    for lang_code in &lang_codes {
        let module_name = lang_code.to_case(Case::Snake);
        let feature_name = lang_code.to_case(Case::Kebab);
        writeln!(main_file, "#[cfg(feature = \"{feature_name}\")]")?;
        writeln!(main_file, "pub mod {module_name};")?;
    }
    writeln!(main_file, "\nuse serde::{{Serialize, Deserialize}};\n")?;

    // Define the VoicePrice trait for pricing calculations
    writeln!(
        main_file,
        r#"
        pub trait VoicePrice {{
            /// Cost of the voice per 100 characters (rounded down per call)
            fn price(&self) -> Option<f32>;
        }}
    "#
    )?;

    let mut genders = voices
        .iter()
        .map(|v| v.gender.clone())
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    genders.sort();
    let genders = genders.join(", ");

    // Define the VoiceGender trait to provide the gender of the voice
    writeln!(
        main_file,
        r#"
        {ENUM_DERIVE}
        #[non_exhaustive]
        #[serde(rename = "kebab-case")]
        pub enum Gender {{
            {genders}
        }}

        pub trait VoiceGender {{
            /// Gender of the voice
            fn gender(&self) -> Gender;
        }}
    "#
    )?;

    // Create the top-level Language enum
    writeln!(main_file, "{ENUM_DERIVE}")?;
    writeln!(main_file, "#[non_exhaustive]")?;
    writeln!(main_file, "pub enum Language {{")?;
    for lang_code in &lang_codes {
        let module_name = lang_code.to_case(Case::Snake);
        let variant_name = module_name.to_case(Case::Pascal);
        let feature_name = lang_code.to_case(Case::Kebab);
        writeln!(main_file, "    #[cfg(feature = \"{feature_name}\")]")?;
        writeln!(main_file, "    #[serde(rename = \"{lang_code}\")]")?;
        writeln!(main_file, "    {variant_name},")?;
    }
    writeln!(main_file, "}}\n")?;

    // Create the top-level Voice enum with variants for each language
    writeln!(main_file, "{ENUM_DERIVE}")?;
    writeln!(main_file, "#[serde(untagged)]\n#[non_exhaustive]")?;
    writeln!(main_file, "pub enum Voice {{")?;
    writeln!(
        main_file,
        r#"#[serde(rename = "man")] Man, #[serde(rename = "woman")] Woman,"#
    )?;
    for lang_code in &lang_codes {
        let module_name = lang_code.to_case(Case::Snake);
        let variant_name = module_name.to_case(Case::Pascal);
        let feature_name = lang_code.to_case(Case::Kebab);
        writeln!(main_file, "    #[cfg(feature = \"{feature_name}\")]")?;
        writeln!(main_file, "    {variant_name}({module_name}::Voice),")?;
    }
    writeln!(main_file, "}}\n")?;

    // Implement the VoicePrice trait for the Voice enum
    let mut price_arms = Vec::new();
    let mut gender_arms = Vec::new();

    for gender in ["Man", "Woman"] {
        price_arms.push((None, format!("Voice::{gender}"), "Some(0.)".to_string()));
        gender_arms.push((
            None,
            format!("Voice::{gender}"),
            format!(
                "Gender::{}",
                if gender == "Woman" { "Female" } else { "Male" }
            ),
        ));
    }

    for lang_code in &lang_codes {
        let variant_name = lang_code.to_case(Case::Pascal);
        let lang_code_snake = lang_code.to_case(Case::Snake);
        let feature_name = lang_code.to_case(Case::Kebab);
        price_arms.push((
            Some(feature_name.clone()),
            format!("Voice::{variant_name}({lang_code_snake})"),
            format!("{lang_code_snake}.price()"),
        ));
        gender_arms.push((
            Some(feature_name),
            format!("Voice::{variant_name}({lang_code_snake})"),
            format!("{lang_code_snake}.gender()"),
        ));
    }

    write_voice_price_impl(&mut main_file, "Voice", None, Some(&price_arms))?;
    write_voice_gender_impl(&mut main_file, "Voice", None, Some(&gender_arms))?;

    // Write the file to disk
    File::create(Path::new(DIR_PATH).join("mod.rs"))?.write_all(main_file.as_bytes())?;
    Ok(lang_groups)
}

/// Creates a canonical shortened name from a full voice name (e.g., "Chirp3HdZephyr" from "en-US-Chirp3-HD-Zephyr")
fn extract_short_name(voice_name: &str) -> String {
    let parts = voice_name.split('-');
    if parts.clone().count() < 3 {
        return voice_name.to_case(Case::Pascal);
    }

    // Skip provider locale name parts, keep only the actual voice name
    parts
        .skip(2)
        .collect::<Vec<&str>>()
        .join("")
        .to_case(Case::Pascal)
}

/// Groups voices by a key function and returns a HashMap of groups
fn group_voices_by<F, K, V>(voices: &[V], key_fn: F) -> HashMap<K, Vec<V>>
where
    F: Fn(&V) -> K,
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    let mut groups: HashMap<K, Vec<V>> = HashMap::new();
    for voice in voices {
        groups.entry(key_fn(voice)).or_default().push(voice.clone());
    }
    groups
}

/// Implements the VoicePrice trait for various voice types
/// Handles both direct voice type pricing and complex match-based pricing
fn write_voice_price_impl(
    output: &mut String,
    type_name: &str,
    voice_type: Option<&str>,
    match_arms: Option<&[(Option<String>, String, String)]>,
) -> Result<(), Box<dyn Error>> {
    writeln!(output, "    impl VoicePrice for {type_name} {{")?;
    writeln!(output, "        fn price(&self) -> Option<f32> {{")?;

    if let Some(arms) = match_arms {
        writeln!(output, "            match self {{")?;
        for (cfg_feature, pattern, result) in arms {
            if let Some(feature_name) = cfg_feature {
                writeln!(
                    output,
                    r#"                #[cfg(feature = "{feature_name}")]"#
                )?;
            }
            writeln!(output, "                {pattern} => {result},")?;
        }
        writeln!(output, "            }}")?;
    } else {
        writeln!(
            output,
            "            Some({}_VOICE_PRICE)",
            voice_type.unwrap().to_case(Case::Constant)
        )?;
    }

    writeln!(output, "        }}")?;
    writeln!(output, "    }}\n")?;

    Ok(())
}

/// Implements the VoiceGender trait for various voice types
fn write_voice_gender_impl(
    output: &mut String,
    type_name: &str,
    gender: Option<&str>,
    match_arms: Option<&[(Option<String>, String, String)]>,
) -> Result<(), Box<dyn Error>> {
    writeln!(output, "    impl VoiceGender for {type_name} {{")?;
    writeln!(output, "        fn gender(&self) -> Gender {{")?;

    if let Some(arms) = match_arms {
        writeln!(output, "            match self {{")?;
        for (cfg_feature, pattern, result) in arms {
            if let Some(feature_name) = cfg_feature {
                writeln!(
                    output,
                    r#"                #[cfg(feature = "{feature_name}")]"#
                )?;
            }
            writeln!(output, "                {pattern} => {result},")?;
        }
        writeln!(output, "            }}")?;
    } else {
        writeln!(output, "            {}", gender.unwrap())?;
    }

    writeln!(output, "        }}")?;
    writeln!(output, "    }}\n")?;

    Ok(())
}
