use std::collections::{HashMap, HashSet};
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
    // URL of the Twilio page containing voice data
    let url =
        "https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages";

    // Check for cached HTML file
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
        // Launch headless Chrome browser
        let options = LaunchOptions {
            headless: true,
            ..Default::default()
        };
        let browser = Browser::new(options)?;
        let tab = browser.new_tab()?;

        // Navigate to the URL
        tab.navigate_to(url)?;

        // Wait for the page to load and the table to be visible
        println!("Waiting for page to load and JavaScript to execute...");
        tab.wait_for_element("table tbody tr")?;

        // Wait a bit more to ensure all rows have loaded
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Get the rendered HTML
        let html_content = tab.get_content()?;

        // Cache the HTML content
        let mut cache_file = File::create(&cache_path)?;
        cache_file.write_all(html_content.as_bytes())?;
        println!("Cached HTML to: {cache_path}");

        html_content
    };

    // Parse the HTML and extract voice data
    let document = Html::parse_document(&html);

    // Find table rows
    let row_selector = Selector::parse("table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    // Store all voice data
    let mut all_voices = Vec::new();

    // Parse each row
    for row in document.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() >= 6 {
            let locale = cells[0].text().collect::<String>().trim().to_string();
            let language_code = cells[1].text().collect::<String>().trim().to_string();
            let voice_type = cells[2].text().collect::<String>().trim().to_string();
            let gender = cells[3].text().collect::<String>().trim().to_string();
            let provider = cells[4].text().collect::<String>().trim().to_string();

            // Clean up voice name by removing asterisks and trimming once
            let voice_name = cells[5]
                .text()
                .collect::<String>()
                .replace("*", "")
                .trim()
                .to_string();

            all_voices.push(VoiceData {
                locale,
                language_code,
                voice_type,
                gender,
                provider,
                voice_name,
            });
        }
    }

    println!("Found {} voices", all_voices.len());

    // Filter out duplicate voice names
    let mut unique_voices = Vec::new();
    let mut seen = HashSet::new();

    for voice in all_voices {
        let key = format!("{}.{}", voice.provider, voice.voice_name);
        if !seen.contains(&key) {
            seen.insert(key);
            unique_voices.push(voice);
        }
    }

    println!(
        "Found {} unique voices (after removing duplicates)",
        unique_voices.len()
    );

    // Generate module structure and voice enum
    generate_voice_module_structure(&unique_voices)?;

    println!("Successfully generated voice module structure and enum.");
    Ok(())
}

#[derive(Debug, Clone)]
struct VoiceData {
    #[allow(unused)]
    locale: String,
    language_code: String,
    voice_type: String,
    gender: String,
    provider: String,
    voice_name: String,
}

fn generate_voice_module_structure(voices: &[VoiceData]) -> Result<(), Box<dyn std::error::Error>> {
    // Create the directory structure
    let dir_path = Path::new("../src/twiml/voices");
    std::fs::create_dir_all(dir_path)?;

    // Group by language code
    let mut lang_groups: HashMap<String, Vec<&VoiceData>> = HashMap::new();
    for voice in voices {
        let lang_code = voice.language_code.clone();
        lang_groups.entry(lang_code).or_default().push(voice);
    }

    // Create the main mod.rs file
    let mut main_file = String::new();
    writeln!(
        main_file,
        "// Auto-generated voice module from Twilio documentation"
    )?;
    writeln!(
        main_file,
        "// Source: https://www.twilio.com/docs/voice/twiml/say/text-speech"
    )?;
    writeln!(main_file, "// Generated on: {}", chrono::Local::now())?;
    writeln!(main_file)?;
    writeln!(main_file, "#![allow(non_local_definitions)]")?;
    writeln!(main_file)?;

    // Create module declarations
    for lang_code in lang_groups.keys() {
        let module_name = lang_code.replace("-", "_").to_lowercase();
        writeln!(main_file, "pub mod {module_name};")?;
    }

    writeln!(main_file)?;

    // Create the main Voice enum with serde import
    writeln!(main_file, "use serde::{{Serialize, Deserialize}};")?;
    writeln!(main_file)?;

    // Fix for the root module path - using just the module path without 'root'
    writeln!(main_file, "#[amass::amass_telety(crate::twiml::voices)]")?;
    writeln!(
        main_file,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
    )?;
    writeln!(main_file, "#[serde(untagged)]")?;
    writeln!(main_file, "pub enum Voice {{")?;

    let mut lang_codes: Vec<_> = lang_groups.keys().collect();
    lang_codes.sort();

    for lang_code in lang_codes {
        let module_name = lang_code.replace("-", "_").to_lowercase();

        // Use PascalCase for enum variants
        let variant_name = to_pascal_case(&module_name);
        writeln!(main_file, "    {variant_name}({module_name}::Voice),")?;
    }

    writeln!(main_file, "}}")?;

    // Write the mod.rs file
    let mod_path = dir_path.join("mod.rs");
    let mut file = File::create(mod_path)?;
    file.write_all(main_file.as_bytes())?;

    // Create a single file per language code
    for (lang_code, voices_in_lang) in lang_groups {
        let module_name = lang_code.replace("-", "_").to_lowercase();
        let mut lang_file = String::new();

        writeln!(lang_file, "// Voice module for {lang_code} language")?;
        writeln!(lang_file, "use serde::{{Serialize, Deserialize}};")?;
        writeln!(lang_file)?;

        // Group by voice type within this language
        let mut type_groups: HashMap<String, Vec<&VoiceData>> = HashMap::new();
        for voice in voices_in_lang {
            if !voice.voice_type.is_empty() {
                // Ensure voice_type is not empty
                type_groups
                    .entry(voice.voice_type.clone())
                    .or_default()
                    .push(voice);
            }
        }

        // Add voice type modules
        for (voice_type, voices_of_type) in &type_groups {
            let type_module = voice_type.to_lowercase();

            // Start type module
            writeln!(lang_file, "pub mod {type_module} {{")?;

            // Always include serde imports at each module level
            writeln!(lang_file, "    use serde::{{Serialize, Deserialize}};")?;
            writeln!(lang_file)?;

            // Group by provider
            let mut provider_groups: HashMap<String, Vec<&&VoiceData>> = HashMap::new();
            for voice in voices_of_type {
                if !voice.provider.is_empty() {
                    // Ensure provider is not empty
                    provider_groups
                        .entry(voice.provider.clone())
                        .or_default()
                        .push(voice);
                }
            }

            // Add provider modules
            for (provider, voices_by_provider) in &provider_groups {
                let provider_module = provider.to_lowercase();
                if !provider_module.is_empty() {
                    // Check for empty module names
                    writeln!(lang_file, "    pub mod {provider_module} {{")?;

                    // Make sure to include the serde import at provider level
                    writeln!(lang_file, "        use serde::{{Serialize, Deserialize}};")?;
                    writeln!(lang_file)?;

                    // Create mappings to deduplicate voices by name
                    let mut female_voice_map = HashMap::new();
                    let mut male_voice_map = HashMap::new();

                    // Process voices into maps to deduplicate by variant name
                    for voice in voices_by_provider.iter() {
                        let variant_name = extract_short_name(&voice.voice_name);
                        if !variant_name.is_empty() {
                            // The variant_name should now already include the technology type (Neural2A, WavenetA, etc.)
                            if voice.gender.starts_with("Female") {
                                // Only insert if not present yet
                                if !female_voice_map.contains_key(&variant_name) {
                                    female_voice_map.insert(
                                        variant_name.clone(),
                                        format!("{}.{}", voice.provider, voice.voice_name),
                                    );
                                }
                            } else if voice.gender.starts_with("Male") {
                                // Only insert if not present yet
                                if !male_voice_map.contains_key(&variant_name) {
                                    male_voice_map.insert(
                                        variant_name.clone(),
                                        format!("{}.{}", voice.provider, voice.voice_name),
                                    );
                                }
                            }
                        }
                    }

                    // Female enum if needed
                    if !female_voice_map.is_empty() {
                        writeln!(
                            lang_file,
                            "        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
                        )?;
                        writeln!(lang_file, "        pub enum Female {{")?;

                        // Sort keys for consistent output
                        let mut keys: Vec<_> = female_voice_map.keys().collect();
                        keys.sort();

                        for key in keys {
                            let full_name = &female_voice_map[key];
                            writeln!(lang_file, "            #[serde(rename = \"{full_name}\")]")?;
                            writeln!(lang_file, "            {key},")?;
                        }

                        writeln!(lang_file, "        }}")?;
                        writeln!(lang_file)?;
                    }

                    // Male enum if needed
                    if !male_voice_map.is_empty() {
                        writeln!(
                            lang_file,
                            "        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
                        )?;
                        writeln!(lang_file, "        pub enum Male {{")?;

                        // Sort keys for consistent output
                        let mut keys: Vec<_> = male_voice_map.keys().collect();
                        keys.sort();

                        for key in keys {
                            let full_name = &male_voice_map[key];
                            writeln!(lang_file, "            #[serde(rename = \"{full_name}\")]")?;
                            writeln!(lang_file, "            {key},")?;
                        }

                        writeln!(lang_file, "        }}")?;
                        writeln!(lang_file)?;
                    }

                    // Provider Voice enum - use unique path for provider level
                    let provider_path = format!(
                        "crate::twiml::voices::{module_name}::{type_module}::{provider_module}"
                    );
                    writeln!(lang_file, "        #[amass::amass_telety({provider_path})]")?;
                    writeln!(
                        lang_file,
                        "        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
                    )?;
                    writeln!(lang_file, "        #[serde(untagged)]")?;
                    writeln!(lang_file, "        pub enum Voice {{")?;

                    if !female_voice_map.is_empty() {
                        writeln!(lang_file, "            Female(Female),")?;
                    }

                    if !male_voice_map.is_empty() {
                        writeln!(lang_file, "            Male(Male),")?;
                    }

                    writeln!(lang_file, "        }}")?;

                    writeln!(lang_file, "    }}")?; // End provider module
                }
            }

            // Type Voice enum - use unique path for type level
            let type_path = format!("crate::twiml::voices::{module_name}::{type_module}");
            writeln!(lang_file, "    #[amass::amass_telety({type_path})]")?;
            writeln!(
                lang_file,
                "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
            )?;
            writeln!(lang_file, "    #[serde(untagged)]")?;
            writeln!(lang_file, "    pub enum Voice {{")?;

            for provider in provider_groups.keys() {
                let provider_module = provider.to_lowercase();
                if !provider_module.is_empty() {
                    // Check for empty module names
                    // Use PascalCase for enum variants
                    let variant_name = to_pascal_case(&provider_module);
                    writeln!(
                        lang_file,
                        "        {variant_name}({provider_module}::Voice),"
                    )?;
                }
            }

            writeln!(lang_file, "    }}")?;

            writeln!(lang_file, "}}")?; // End type module
        }

        // Language Voice enum - use unique path for language level
        let lang_path = format!("crate::twiml::voices::{module_name}");
        writeln!(lang_file, "#[amass::amass_telety({lang_path})]")?;
        writeln!(
            lang_file,
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]"
        )?;
        writeln!(lang_file, "#[serde(untagged)]")?;
        writeln!(lang_file, "pub enum Voice {{")?;

        for voice_type in type_groups.keys() {
            let type_module = voice_type.to_lowercase();

            // Use PascalCase for enum variants
            let variant_name = to_pascal_case(&type_module);
            writeln!(lang_file, "    {variant_name}({type_module}::Voice),")?;
        }

        writeln!(lang_file, "}}")?;

        // Write the language file - using ../src path
        let lang_path = dir_path.join(format!("{module_name}.rs"));
        let mut file = File::create(lang_path)?;
        file.write_all(lang_file.as_bytes())?;
    }

    Ok(())
}

fn extract_short_name(voice_name: &str) -> String {
    if voice_name.contains('-') {
        // For names with format like "en-US-Neural2-A" or "en-US-Wavenet-A",
        // extract both the technology (Neural2/Wavenet) and the voice identifier (A)
        let parts: Vec<&str> = voice_name.split('-').collect();

        if parts.len() > 2 {
            // Format: en-US-Neural2-A should become Neural2A
            //         en-US-Chirp3-HD-Orus should become Chirp3HDOrus
            return parts[2..].join("").replace(['.', ' '], "");
        }
    }

    // For simpler names (e.g., "Joanna"), use as is
    voice_name.replace(['-', '.', ' '], "")
}

// Helper function to convert snake_case or kebab-case to PascalCase
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
