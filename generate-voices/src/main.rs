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

fn main() {
    let result = fetch_and_generate_voice_modules();
    match result {
        Ok(_) => println!("Successfully generated voice modules!"),
        Err(e) => eprintln!("Error generating voice modules: {e:?}"),
    }
}

const DIR_PATH: &str = "../src/twiml/voices";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VoiceData {
    language_code: String,
    voice_type: String,
    gender: String,
    provider: String,
    voice_name: String,
}

fn fetch_and_generate_voice_modules() -> Result<(), Box<dyn Error>> {
    let html = fetch_html()?;
    let all_voices = parse_html_into_voices(html);
    println!("Found {} unique voices", all_voices.len());
    generate_voice_module_structure(&all_voices)?;
    Command::new("cargo")
        .arg("fmt")
        .current_dir("../")
        .status()?;
    Ok(())
}

fn parse_html_into_voices(html: String) -> HashSet<VoiceData> {
    let document = Html::parse_document(&html);
    let row_selector = Selector::parse("table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

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
        if gender.contains("child") {
            println!("GENDER: {gender}");
        }
        if gender.is_empty() {
            continue;
        }

        let provider = cells[4].text().collect::<String>().trim().to_string();
        if provider.is_empty() {
            continue;
        }

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

    all_voices
}

fn fetch_html() -> Result<String, Box<dyn Error>> {
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
        let url = "https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages";
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
    Ok(html)
}

fn generate_voice_module_structure(voices: &HashSet<VoiceData>) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(DIR_PATH)?;

    let lang_groups = generate_main_file(voices)?;

    for (lang_code, voices_in_lang) in &lang_groups {
        generate_lang_file(lang_code, voices_in_lang)?;
    }

    Ok(())
}

fn generate_lang_file(
    lang_code: &str,
    voices_in_lang: &[&VoiceData],
) -> Result<(), Box<dyn Error>> {
    println!("generate_lang_file: {lang_code}");
    // println!("voices_in_lang: {voices_in_lang:#?}");
    let module_name = lang_code.to_case(Case::Snake);
    let lang_variant = lang_code.to_case(Case::Pascal);

    let mut lang_file = String::new();
    writeln!(lang_file, "use serde::{{Serialize, Deserialize}};\n")?;

    let mut type_groups: HashMap<String, Vec<&VoiceData>> = HashMap::new();

    for voice in voices_in_lang {
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

            println!("gender_maps provider={provider}: {gender_maps:#?}");

            writeln!(
                lang_file,
                "    pub mod {provider_module} {{\n        use super::*;\n"
            )?;

            for (gender, voice_map) in &gender_maps {
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

            writeln!(
                lang_file,
                "        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
            )?;
            writeln!(lang_file, "        #[serde(untagged)]")?;
            writeln!(lang_file, "        pub enum Voice {{")?;

            for gender in gender_maps.keys() {
                writeln!(lang_file, "            {gender}({gender}),")?;
            }
            writeln!(lang_file, "        }}\n    }}\n")?;
        }

        writeln!(
            lang_file,
            "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
        )?;
        writeln!(lang_file, "    #[serde(untagged)]")?;
        writeln!(lang_file, "    pub enum Voice {{")?;
        for (provider, voices_by_provider) in &provider_groups {
            if provider.is_empty() || voices_by_provider.is_empty() {
                continue;
            }

            let provider_module = provider.to_lowercase();
            let variant_name = provider_module.to_case(Case::Pascal);
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
    for (voice_type, groups) in &type_groups {
        if groups.is_empty() {
            continue;
        }

        let variant_name = voice_type.to_case(Case::Pascal);
        writeln!(
            lang_file,
            "    {variant_name}({}::Voice),",
            voice_type.to_lowercase()
        )?;
    }
    writeln!(lang_file, "}}")?;
    File::create(Path::new(DIR_PATH).join(format!("{module_name}.rs")))?
        .write_all(lang_file.as_bytes())?;
    Ok(())
}

fn generate_main_file(
    voices: &HashSet<VoiceData>,
) -> Result<HashMap<String, Vec<&VoiceData>>, Box<dyn Error>> {
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
        let variant_name = module_name.to_case(Case::Pascal);
        writeln!(main_file, "    #[cfg(feature = \"{module_name}\")]")?;
        writeln!(main_file, "    {variant_name}({module_name}::Voice),")?;
    }
    writeln!(main_file, "}}")?;
    File::create(Path::new(DIR_PATH).join("mod.rs"))?.write_all(main_file.as_bytes())?;
    Ok(lang_groups)
}

fn extract_short_name(voice_name: &str) -> String {
    let parts = voice_name.split('-');
    if parts.clone().count() < 3 {
        return voice_name.to_case(Case::Pascal);
    }

    parts
        .skip(2)
        .collect::<Vec<&str>>()
        .join("")
        .to_case(Case::Pascal)
}
