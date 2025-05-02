use std::collections::{HashMap, HashSet};
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use convert_case::{Case, Casing};
use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};

fn main() {
    let result = fetch_and_generate_voice_modules();
    match result {
        Ok(_) => println!("Successfully generated voice modules!"),
        Err(e) => eprintln!("Error generating voice modules: {e:?}"),
    }
}

fn fetch_and_generate_voice_modules() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages";
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    let cache_path = format!("/tmp/voices_{today}.html");

    let html = if Path::new(&cache_path).exists() {
        println!("Using cached HTML file: {cache_path}");
        let mut file = File::open(&cache_path)?;
        let mut html_content = String::new();
        file.read_to_string(&mut html_content)?;
        html_content
    } else {
        println!("Cache not found, fetching from web...");
        let options = LaunchOptions {
            headless: true,
            ..Default::default()
        };
        let browser = Browser::new(options)?;
        let tab = browser.new_tab()?;

        tab.navigate_to(url)?;
        println!("Waiting for page to load and JavaScript to execute...");
        tab.wait_for_element("table tbody tr")?;
        std::thread::sleep(std::time::Duration::from_secs(5));

        let html_content = tab.get_content()?;
        let mut cache_file = File::create(&cache_path)?;
        cache_file.write_all(html_content.as_bytes())?;
        println!("Cached HTML to: {cache_path}");

        html_content
    };

    let document = Html::parse_document(&html);
    let row_selector = Selector::parse("table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut all_voices = Vec::new();
    for row in document.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() < 6 {
            continue;
        }

        let language_code = cells[1].text().collect::<String>().trim().to_string();
        let voice_type = cells[2].text().collect::<String>().trim().to_string();
        let gender = cells[3].text().collect::<String>().trim().to_string();
        let provider = cells[4].text().collect::<String>().trim().to_string();
        let voice_name = cells[5]
            .text()
            .collect::<String>()
            .replace('*', "")
            .trim()
            .to_string();

        all_voices.push(VoiceData {
            language_code,
            voice_type,
            gender,
            provider,
            voice_name,
        });
    }

    println!("Found {} voices", all_voices.len());

    let mut unique_voices = Vec::new();
    let mut seen = HashSet::new();
    for voice in all_voices {
        let key = format!("{}.{}", voice.provider, voice.voice_name);
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);
        unique_voices.push(voice);
    }

    println!("Found {} unique voices", unique_voices.len());
    generate_voice_module_structure(&unique_voices)
}

#[derive(Debug, Clone)]
struct VoiceData {
    language_code: String,
    voice_type: String,
    gender: String,
    provider: String,
    voice_name: String,
}

fn generate_voice_module_structure(voices: &[VoiceData]) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = Path::new("../src/twiml/voices");
    std::fs::create_dir_all(dir_path)?;

    let mut lang_groups: HashMap<String, Vec<&VoiceData>> = HashMap::new();
    for voice in voices {
        lang_groups
            .entry(voice.language_code.clone())
            .or_default()
            .push(voice);
    }

    let mut main_file = String::new();
    writeln!(
        main_file,
        "// Auto-generated voice module\n// Source: Twilio documentation"
    )?;
    writeln!(main_file, "#![allow(non_local_definitions)]\n")?;

    let mut lang_codes: Vec<_> = lang_groups.keys().collect();
    lang_codes.sort();

    for lang_code in &lang_codes {
        let module_name = lang_code.replace('-', "_").to_lowercase();
        writeln!(main_file, "#[cfg(feature = \"{module_name}\")]")?;
        writeln!(main_file, "pub mod {module_name};")?;
    }

    writeln!(main_file, "\nuse serde::{{Serialize, Deserialize}};\n")?;
    writeln!(
        main_file,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
    )?;
    writeln!(main_file, "#[serde(untagged)]\n#[non_exhaustive]")?;
    writeln!(main_file, "pub enum Voice {{")?;
    for lang_code in &lang_codes {
        let module_name = lang_code.replace('-', "_").to_lowercase();
        let variant_name = to_pascal_case(&module_name);
        writeln!(main_file, "    #[cfg(feature = \"{module_name}\")]")?;
        writeln!(main_file, "    {variant_name}({module_name}::Voice),")?;
    }
    writeln!(main_file, "}}")?;

    File::create(dir_path.join("mod.rs"))?.write_all(main_file.as_bytes())?;

    for (lang_code, voices_in_lang) in lang_groups {
        let module_name = lang_code.to_case(Case::Snake);
        let lang_variant = lang_code.to_case(Case::Pascal);
        let mut lang_file = String::new();

        writeln!(lang_file, "use serde::{{Serialize, Deserialize}};\n")?;
        let mut type_groups: HashMap<String, Vec<&VoiceData>> = HashMap::new();
        for voice in voices_in_lang {
            if voice.voice_type.is_empty() {
                continue;
            }
            type_groups
                .entry(voice.voice_type.clone())
                .or_default()
                .push(voice);
        }

        for (voice_type, voices_of_type) in &type_groups {
            let type_module = voice_type.to_lowercase();
            writeln!(lang_file, "pub mod {type_module} {{\n    use super::*;\n")?;

            let mut provider_groups: HashMap<String, Vec<&&VoiceData>> = HashMap::new();
            for voice in voices_of_type {
                provider_groups
                    .entry(voice.provider.clone())
                    .or_default()
                    .push(voice);
            }

            for (provider, voices_by_provider) in &provider_groups {
                let provider_module = provider.to_lowercase();
                if provider_module.is_empty() {
                    continue;
                }

                writeln!(
                    lang_file,
                    "    pub mod {provider_module} {{\n        use super::*;\n"
                )?;
                let mut voice_maps: HashMap<&str, HashMap<String, String>> = HashMap::new();

                for voice in voices_by_provider.iter() {
                    let variant_name = extract_short_name(&voice.voice_name);
                    if variant_name.is_empty() {
                        continue;
                    }

                    let gender = if voice.gender.starts_with("Female") {
                        "Female"
                    } else if voice.gender.starts_with("Male") {
                        "Male"
                    } else {
                        continue;
                    };

                    let gender_map = voice_maps.entry(gender).or_default();
                    if !gender_map.contains_key(&variant_name) {
                        gender_map.insert(
                            variant_name.clone(),
                            format!("{}.{}", voice.provider, voice.voice_name),
                        );
                    }
                }

                for gender in &["Female", "Male"] {
                    if let Some(voice_map) = voice_maps.get(*gender) {
                        if voice_map.is_empty() {
                            continue;
                        }

                        writeln!(
                            lang_file,
                            "        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
                        )?;
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

                        writeln!(
                            lang_file,
                            r#"
                            impl From<{gender}> for crate::Voice {{
                                fn from(value: {gender}) -> Self {{
                                    Self::{lang_variant}(super::super::Voice::{voice_type}(super::Voice::{provider}(
                                        Voice::{gender}(value),
                                    )))
                                }}
                            }}
                        "#
                        )?;
                    }
                }

                writeln!(
                    lang_file,
                    "        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
                )?;
                writeln!(lang_file, "        #[serde(untagged)]")?;
                writeln!(lang_file, "        pub enum Voice {{")?;
                for gender in &["Female", "Male"] {
                    if voice_maps.get(*gender).is_some_and(|m| !m.is_empty()) {
                        writeln!(lang_file, "            {gender}({gender}),")?;
                    }
                }
                writeln!(lang_file, "        }}\n    }}\n")?;
            }

            writeln!(
                lang_file,
                "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
            )?;
            writeln!(lang_file, "    #[serde(untagged)]")?;
            writeln!(lang_file, "    pub enum Voice {{")?;
            for provider in provider_groups.keys() {
                let provider_module = provider.to_lowercase();
                let variant_name = to_pascal_case(&provider_module);
                writeln!(
                    lang_file,
                    "        {variant_name}({provider_module}::Voice),"
                )?;
            }
            writeln!(lang_file, "    }}\n}}\n")?;
        }

        writeln!(
            lang_file,
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
        )?;
        writeln!(lang_file, "#[serde(untagged)]")?;
        writeln!(lang_file, "pub enum Voice {{")?;
        for voice_type in type_groups.keys() {
            let variant_name = to_pascal_case(&voice_type.to_lowercase());
            writeln!(
                lang_file,
                "    {variant_name}({}::Voice),",
                voice_type.to_lowercase()
            )?;
        }
        writeln!(lang_file, "}}")?;

        File::create(dir_path.join(format!("{module_name}.rs")))?
            .write_all(lang_file.as_bytes())?;
    }

    Ok(())
}

fn extract_short_name(voice_name: &str) -> String {
    voice_name
        .split('-')
        .skip(2)
        .collect::<Vec<&str>>()
        .join("")
        .replace(['.', ' '], "")
}

fn to_pascal_case(s: &str) -> String {
    s.split(&['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect()
}
