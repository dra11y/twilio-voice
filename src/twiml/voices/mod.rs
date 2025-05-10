// Auto-generated at: 2025-05-10 21:09
// Source: https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages
#![allow(non_local_definitions)]

/// Current price of Neural voices per 100 chars as of 2025-05-10 21:09 UTC
pub const NEURAL_VOICE_PRICE: f32 = 0.0032;
/// Current price of Standard voices per 100 chars as of 2025-05-10 21:09 UTC
pub const STANDARD_VOICE_PRICE: f32 = 0.0008;
/// Current price of Generative voices per 100 chars as of 2025-05-10 21:09 UTC
pub const GENERATIVE_VOICE_PRICE: f32 = 0.013;

#[cfg(feature = "af-za")]
pub mod af_za;
#[cfg(feature = "ar-ae")]
pub mod ar_ae;
#[cfg(feature = "ar-xa")]
pub mod ar_xa;
#[cfg(feature = "arb")]
pub mod arb;
#[cfg(feature = "bg-bg")]
pub mod bg_bg;
#[cfg(feature = "bn-in")]
pub mod bn_in;
#[cfg(feature = "ca-es")]
pub mod ca_es;
#[cfg(feature = "cmn-cn")]
pub mod cmn_cn;
#[cfg(feature = "cmn-tw")]
pub mod cmn_tw;
#[cfg(feature = "cs-cz")]
pub mod cs_cz;
#[cfg(feature = "cy-gb")]
pub mod cy_gb;
#[cfg(feature = "da-dk")]
pub mod da_dk;
#[cfg(feature = "de-at")]
pub mod de_at;
#[cfg(feature = "de-de")]
pub mod de_de;
#[cfg(feature = "el-gr")]
pub mod el_gr;
#[cfg(feature = "en-au")]
pub mod en_au;
#[cfg(feature = "en-gb")]
pub mod en_gb;
#[cfg(feature = "en-gb-wls")]
pub mod en_gb_wls;
#[cfg(feature = "en-ie")]
pub mod en_ie;
#[cfg(feature = "en-in")]
pub mod en_in;
#[cfg(feature = "en-nz")]
pub mod en_nz;
#[cfg(feature = "en-us")]
pub mod en_us;
#[cfg(feature = "en-za")]
pub mod en_za;
#[cfg(feature = "es-es")]
pub mod es_es;
#[cfg(feature = "es-mx")]
pub mod es_mx;
#[cfg(feature = "es-us")]
pub mod es_us;
#[cfg(feature = "eu-es")]
pub mod eu_es;
#[cfg(feature = "fi-fi")]
pub mod fi_fi;
#[cfg(feature = "fil-ph")]
pub mod fil_ph;
#[cfg(feature = "fr-be")]
pub mod fr_be;
#[cfg(feature = "fr-ca")]
pub mod fr_ca;
#[cfg(feature = "fr-fr")]
pub mod fr_fr;
#[cfg(feature = "gl-es")]
pub mod gl_es;
#[cfg(feature = "gu-in")]
pub mod gu_in;
#[cfg(feature = "he-il")]
pub mod he_il;
#[cfg(feature = "hi-in")]
pub mod hi_in;
#[cfg(feature = "hu-hu")]
pub mod hu_hu;
#[cfg(feature = "id-id")]
pub mod id_id;
#[cfg(feature = "is-is")]
pub mod is_is;
#[cfg(feature = "it-it")]
pub mod it_it;
#[cfg(feature = "ja-jp")]
pub mod ja_jp;
#[cfg(feature = "kn-in")]
pub mod kn_in;
#[cfg(feature = "ko-kr")]
pub mod ko_kr;
#[cfg(feature = "lt-lt")]
pub mod lt_lt;
#[cfg(feature = "lv-lv")]
pub mod lv_lv;
#[cfg(feature = "ml-in")]
pub mod ml_in;
#[cfg(feature = "mr-in")]
pub mod mr_in;
#[cfg(feature = "ms-my")]
pub mod ms_my;
#[cfg(feature = "nb-no")]
pub mod nb_no;
#[cfg(feature = "nl-be")]
pub mod nl_be;
#[cfg(feature = "nl-nl")]
pub mod nl_nl;
#[cfg(feature = "pa-in")]
pub mod pa_in;
#[cfg(feature = "pl-pl")]
pub mod pl_pl;
#[cfg(feature = "pt-br")]
pub mod pt_br;
#[cfg(feature = "pt-pt")]
pub mod pt_pt;
#[cfg(feature = "ro-ro")]
pub mod ro_ro;
#[cfg(feature = "ru-ru")]
pub mod ru_ru;
#[cfg(feature = "sk-sk")]
pub mod sk_sk;
#[cfg(feature = "sv-se")]
pub mod sv_se;
#[cfg(feature = "ta-in")]
pub mod ta_in;
#[cfg(feature = "te-in")]
pub mod te_in;
#[cfg(feature = "th-th")]
pub mod th_th;
#[cfg(feature = "tr-tr")]
pub mod tr_tr;
#[cfg(feature = "vi-vn")]
pub mod vi_vn;
#[cfg(feature = "yue-cn")]
pub mod yue_cn;
#[cfg(feature = "yue-hk")]
pub mod yue_hk;

use serde::{Deserialize, Serialize};

pub trait VoicePrice {
    /// Cost of the voice per 100 characters (rounded down per call)
    fn price(&self) -> Option<f32>;
}

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[strum(serialize_all = "kebab-case")]
#[serde(rename = "kebab-case")]
pub enum Gender {
    Female,
    FemaleChild,
    Male,
    MaleChild,
}

pub trait VoiceGender {
    /// Gender of the voice
    fn gender(&self) -> Gender;
}

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Language {
    #[cfg(feature = "af-za")]
    #[strum(to_string = "af-ZA")]
    #[serde(rename = "af-ZA")]
    AfZa,
    #[cfg(feature = "ar-ae")]
    #[strum(to_string = "ar-AE")]
    #[serde(rename = "ar-AE")]
    ArAe,
    #[cfg(feature = "ar-xa")]
    #[strum(to_string = "ar-XA")]
    #[serde(rename = "ar-XA")]
    ArXa,
    #[cfg(feature = "arb")]
    #[strum(to_string = "arb")]
    #[serde(rename = "arb")]
    Arb,
    #[cfg(feature = "bg-bg")]
    #[strum(to_string = "bg-BG")]
    #[serde(rename = "bg-BG")]
    BgBg,
    #[cfg(feature = "bn-in")]
    #[strum(to_string = "bn-IN")]
    #[serde(rename = "bn-IN")]
    BnIn,
    #[cfg(feature = "ca-es")]
    #[strum(to_string = "ca-ES")]
    #[serde(rename = "ca-ES")]
    CaEs,
    #[cfg(feature = "cmn-cn")]
    #[strum(to_string = "cmn-CN")]
    #[serde(rename = "cmn-CN")]
    CmnCn,
    #[cfg(feature = "cmn-tw")]
    #[strum(to_string = "cmn-TW")]
    #[serde(rename = "cmn-TW")]
    CmnTw,
    #[cfg(feature = "cs-cz")]
    #[strum(to_string = "cs-CZ")]
    #[serde(rename = "cs-CZ")]
    CsCz,
    #[cfg(feature = "cy-gb")]
    #[strum(to_string = "cy-GB")]
    #[serde(rename = "cy-GB")]
    CyGb,
    #[cfg(feature = "da-dk")]
    #[strum(to_string = "da-DK")]
    #[serde(rename = "da-DK")]
    DaDk,
    #[cfg(feature = "de-at")]
    #[strum(to_string = "de-AT")]
    #[serde(rename = "de-AT")]
    DeAt,
    #[cfg(feature = "de-de")]
    #[strum(to_string = "de-DE")]
    #[serde(rename = "de-DE")]
    DeDe,
    #[cfg(feature = "el-gr")]
    #[strum(to_string = "el-GR")]
    #[serde(rename = "el-GR")]
    ElGr,
    #[cfg(feature = "en-au")]
    #[strum(to_string = "en-AU")]
    #[serde(rename = "en-AU")]
    EnAu,
    #[cfg(feature = "en-gb")]
    #[strum(to_string = "en-GB")]
    #[serde(rename = "en-GB")]
    EnGb,
    #[cfg(feature = "en-gb-wls")]
    #[strum(to_string = "en-GB-WLS")]
    #[serde(rename = "en-GB-WLS")]
    EnGbWls,
    #[cfg(feature = "en-ie")]
    #[strum(to_string = "en-IE")]
    #[serde(rename = "en-IE")]
    EnIe,
    #[cfg(feature = "en-in")]
    #[strum(to_string = "en-IN")]
    #[serde(rename = "en-IN")]
    EnIn,
    #[cfg(feature = "en-nz")]
    #[strum(to_string = "en-NZ")]
    #[serde(rename = "en-NZ")]
    EnNz,
    #[cfg(feature = "en-us")]
    #[strum(to_string = "en-US")]
    #[serde(rename = "en-US")]
    EnUs,
    #[cfg(feature = "en-za")]
    #[strum(to_string = "en-ZA")]
    #[serde(rename = "en-ZA")]
    EnZa,
    #[cfg(feature = "es-es")]
    #[strum(to_string = "es-ES")]
    #[serde(rename = "es-ES")]
    EsEs,
    #[cfg(feature = "es-mx")]
    #[strum(to_string = "es-MX")]
    #[serde(rename = "es-MX")]
    EsMx,
    #[cfg(feature = "es-us")]
    #[strum(to_string = "es-US")]
    #[serde(rename = "es-US")]
    EsUs,
    #[cfg(feature = "eu-es")]
    #[strum(to_string = "eu-ES")]
    #[serde(rename = "eu-ES")]
    EuEs,
    #[cfg(feature = "fi-fi")]
    #[strum(to_string = "fi-FI")]
    #[serde(rename = "fi-FI")]
    FiFi,
    #[cfg(feature = "fil-ph")]
    #[strum(to_string = "fil-PH")]
    #[serde(rename = "fil-PH")]
    FilPh,
    #[cfg(feature = "fr-be")]
    #[strum(to_string = "fr-BE")]
    #[serde(rename = "fr-BE")]
    FrBe,
    #[cfg(feature = "fr-ca")]
    #[strum(to_string = "fr-CA")]
    #[serde(rename = "fr-CA")]
    FrCa,
    #[cfg(feature = "fr-fr")]
    #[strum(to_string = "fr-FR")]
    #[serde(rename = "fr-FR")]
    FrFr,
    #[cfg(feature = "gl-es")]
    #[strum(to_string = "gl-ES")]
    #[serde(rename = "gl-ES")]
    GlEs,
    #[cfg(feature = "gu-in")]
    #[strum(to_string = "gu-IN")]
    #[serde(rename = "gu-IN")]
    GuIn,
    #[cfg(feature = "he-il")]
    #[strum(to_string = "he-IL")]
    #[serde(rename = "he-IL")]
    HeIl,
    #[cfg(feature = "hi-in")]
    #[strum(to_string = "hi-IN")]
    #[serde(rename = "hi-IN")]
    HiIn,
    #[cfg(feature = "hu-hu")]
    #[strum(to_string = "hu-HU")]
    #[serde(rename = "hu-HU")]
    HuHu,
    #[cfg(feature = "id-id")]
    #[strum(to_string = "id-ID")]
    #[serde(rename = "id-ID")]
    IdId,
    #[cfg(feature = "is-is")]
    #[strum(to_string = "is-IS")]
    #[serde(rename = "is-IS")]
    IsIs,
    #[cfg(feature = "it-it")]
    #[strum(to_string = "it-IT")]
    #[serde(rename = "it-IT")]
    ItIt,
    #[cfg(feature = "ja-jp")]
    #[strum(to_string = "ja-JP")]
    #[serde(rename = "ja-JP")]
    JaJp,
    #[cfg(feature = "kn-in")]
    #[strum(to_string = "kn-IN")]
    #[serde(rename = "kn-IN")]
    KnIn,
    #[cfg(feature = "ko-kr")]
    #[strum(to_string = "ko-KR")]
    #[serde(rename = "ko-KR")]
    KoKr,
    #[cfg(feature = "lt-lt")]
    #[strum(to_string = "lt-LT")]
    #[serde(rename = "lt-LT")]
    LtLt,
    #[cfg(feature = "lv-lv")]
    #[strum(to_string = "lv-LV")]
    #[serde(rename = "lv-LV")]
    LvLv,
    #[cfg(feature = "ml-in")]
    #[strum(to_string = "ml-IN")]
    #[serde(rename = "ml-IN")]
    MlIn,
    #[cfg(feature = "mr-in")]
    #[strum(to_string = "mr-IN")]
    #[serde(rename = "mr-IN")]
    MrIn,
    #[cfg(feature = "ms-my")]
    #[strum(to_string = "ms-MY")]
    #[serde(rename = "ms-MY")]
    MsMy,
    #[cfg(feature = "nb-no")]
    #[strum(to_string = "nb-NO")]
    #[serde(rename = "nb-NO")]
    NbNo,
    #[cfg(feature = "nl-be")]
    #[strum(to_string = "nl-BE")]
    #[serde(rename = "nl-BE")]
    NlBe,
    #[cfg(feature = "nl-nl")]
    #[strum(to_string = "nl-NL")]
    #[serde(rename = "nl-NL")]
    NlNl,
    #[cfg(feature = "pa-in")]
    #[strum(to_string = "pa-IN")]
    #[serde(rename = "pa-IN")]
    PaIn,
    #[cfg(feature = "pl-pl")]
    #[strum(to_string = "pl-PL")]
    #[serde(rename = "pl-PL")]
    PlPl,
    #[cfg(feature = "pt-br")]
    #[strum(to_string = "pt-BR")]
    #[serde(rename = "pt-BR")]
    PtBr,
    #[cfg(feature = "pt-pt")]
    #[strum(to_string = "pt-PT")]
    #[serde(rename = "pt-PT")]
    PtPt,
    #[cfg(feature = "ro-ro")]
    #[strum(to_string = "ro-RO")]
    #[serde(rename = "ro-RO")]
    RoRo,
    #[cfg(feature = "ru-ru")]
    #[strum(to_string = "ru-RU")]
    #[serde(rename = "ru-RU")]
    RuRu,
    #[cfg(feature = "sk-sk")]
    #[strum(to_string = "sk-SK")]
    #[serde(rename = "sk-SK")]
    SkSk,
    #[cfg(feature = "sv-se")]
    #[strum(to_string = "sv-SE")]
    #[serde(rename = "sv-SE")]
    SvSe,
    #[cfg(feature = "ta-in")]
    #[strum(to_string = "ta-IN")]
    #[serde(rename = "ta-IN")]
    TaIn,
    #[cfg(feature = "te-in")]
    #[strum(to_string = "te-IN")]
    #[serde(rename = "te-IN")]
    TeIn,
    #[cfg(feature = "th-th")]
    #[strum(to_string = "th-TH")]
    #[serde(rename = "th-TH")]
    ThTh,
    #[cfg(feature = "tr-tr")]
    #[strum(to_string = "tr-TR")]
    #[serde(rename = "tr-TR")]
    TrTr,
    #[cfg(feature = "vi-vn")]
    #[strum(to_string = "vi-VN")]
    #[serde(rename = "vi-VN")]
    ViVn,
    #[cfg(feature = "yue-cn")]
    #[strum(to_string = "yue-CN")]
    #[serde(rename = "yue-CN")]
    YueCn,
    #[cfg(feature = "yue-hk")]
    #[strum(to_string = "yue-HK")]
    #[serde(rename = "yue-HK")]
    YueHk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Voice {
    #[serde(alias = "man")]
    Man,
    #[serde(alias = "woman")]
    Woman,
    #[cfg(feature = "af-za")]
    #[serde(untagged)]
    AfZa(af_za::Voice),
    #[cfg(feature = "ar-ae")]
    #[serde(untagged)]
    ArAe(ar_ae::Voice),
    #[cfg(feature = "ar-xa")]
    #[serde(untagged)]
    ArXa(ar_xa::Voice),
    #[cfg(feature = "arb")]
    #[serde(untagged)]
    Arb(arb::Voice),
    #[cfg(feature = "bg-bg")]
    #[serde(untagged)]
    BgBg(bg_bg::Voice),
    #[cfg(feature = "bn-in")]
    #[serde(untagged)]
    BnIn(bn_in::Voice),
    #[cfg(feature = "ca-es")]
    #[serde(untagged)]
    CaEs(ca_es::Voice),
    #[cfg(feature = "cmn-cn")]
    #[serde(untagged)]
    CmnCn(cmn_cn::Voice),
    #[cfg(feature = "cmn-tw")]
    #[serde(untagged)]
    CmnTw(cmn_tw::Voice),
    #[cfg(feature = "cs-cz")]
    #[serde(untagged)]
    CsCz(cs_cz::Voice),
    #[cfg(feature = "cy-gb")]
    #[serde(untagged)]
    CyGb(cy_gb::Voice),
    #[cfg(feature = "da-dk")]
    #[serde(untagged)]
    DaDk(da_dk::Voice),
    #[cfg(feature = "de-at")]
    #[serde(untagged)]
    DeAt(de_at::Voice),
    #[cfg(feature = "de-de")]
    #[serde(untagged)]
    DeDe(de_de::Voice),
    #[cfg(feature = "el-gr")]
    #[serde(untagged)]
    ElGr(el_gr::Voice),
    #[cfg(feature = "en-au")]
    #[serde(untagged)]
    EnAu(en_au::Voice),
    #[cfg(feature = "en-gb")]
    #[serde(untagged)]
    EnGb(en_gb::Voice),
    #[cfg(feature = "en-gb-wls")]
    #[serde(untagged)]
    EnGbWls(en_gb_wls::Voice),
    #[cfg(feature = "en-ie")]
    #[serde(untagged)]
    EnIe(en_ie::Voice),
    #[cfg(feature = "en-in")]
    #[serde(untagged)]
    EnIn(en_in::Voice),
    #[cfg(feature = "en-nz")]
    #[serde(untagged)]
    EnNz(en_nz::Voice),
    #[cfg(feature = "en-us")]
    #[serde(untagged)]
    EnUs(en_us::Voice),
    #[cfg(feature = "en-za")]
    #[serde(untagged)]
    EnZa(en_za::Voice),
    #[cfg(feature = "es-es")]
    #[serde(untagged)]
    EsEs(es_es::Voice),
    #[cfg(feature = "es-mx")]
    #[serde(untagged)]
    EsMx(es_mx::Voice),
    #[cfg(feature = "es-us")]
    #[serde(untagged)]
    EsUs(es_us::Voice),
    #[cfg(feature = "eu-es")]
    #[serde(untagged)]
    EuEs(eu_es::Voice),
    #[cfg(feature = "fi-fi")]
    #[serde(untagged)]
    FiFi(fi_fi::Voice),
    #[cfg(feature = "fil-ph")]
    #[serde(untagged)]
    FilPh(fil_ph::Voice),
    #[cfg(feature = "fr-be")]
    #[serde(untagged)]
    FrBe(fr_be::Voice),
    #[cfg(feature = "fr-ca")]
    #[serde(untagged)]
    FrCa(fr_ca::Voice),
    #[cfg(feature = "fr-fr")]
    #[serde(untagged)]
    FrFr(fr_fr::Voice),
    #[cfg(feature = "gl-es")]
    #[serde(untagged)]
    GlEs(gl_es::Voice),
    #[cfg(feature = "gu-in")]
    #[serde(untagged)]
    GuIn(gu_in::Voice),
    #[cfg(feature = "he-il")]
    #[serde(untagged)]
    HeIl(he_il::Voice),
    #[cfg(feature = "hi-in")]
    #[serde(untagged)]
    HiIn(hi_in::Voice),
    #[cfg(feature = "hu-hu")]
    #[serde(untagged)]
    HuHu(hu_hu::Voice),
    #[cfg(feature = "id-id")]
    #[serde(untagged)]
    IdId(id_id::Voice),
    #[cfg(feature = "is-is")]
    #[serde(untagged)]
    IsIs(is_is::Voice),
    #[cfg(feature = "it-it")]
    #[serde(untagged)]
    ItIt(it_it::Voice),
    #[cfg(feature = "ja-jp")]
    #[serde(untagged)]
    JaJp(ja_jp::Voice),
    #[cfg(feature = "kn-in")]
    #[serde(untagged)]
    KnIn(kn_in::Voice),
    #[cfg(feature = "ko-kr")]
    #[serde(untagged)]
    KoKr(ko_kr::Voice),
    #[cfg(feature = "lt-lt")]
    #[serde(untagged)]
    LtLt(lt_lt::Voice),
    #[cfg(feature = "lv-lv")]
    #[serde(untagged)]
    LvLv(lv_lv::Voice),
    #[cfg(feature = "ml-in")]
    #[serde(untagged)]
    MlIn(ml_in::Voice),
    #[cfg(feature = "mr-in")]
    #[serde(untagged)]
    MrIn(mr_in::Voice),
    #[cfg(feature = "ms-my")]
    #[serde(untagged)]
    MsMy(ms_my::Voice),
    #[cfg(feature = "nb-no")]
    #[serde(untagged)]
    NbNo(nb_no::Voice),
    #[cfg(feature = "nl-be")]
    #[serde(untagged)]
    NlBe(nl_be::Voice),
    #[cfg(feature = "nl-nl")]
    #[serde(untagged)]
    NlNl(nl_nl::Voice),
    #[cfg(feature = "pa-in")]
    #[serde(untagged)]
    PaIn(pa_in::Voice),
    #[cfg(feature = "pl-pl")]
    #[serde(untagged)]
    PlPl(pl_pl::Voice),
    #[cfg(feature = "pt-br")]
    #[serde(untagged)]
    PtBr(pt_br::Voice),
    #[cfg(feature = "pt-pt")]
    #[serde(untagged)]
    PtPt(pt_pt::Voice),
    #[cfg(feature = "ro-ro")]
    #[serde(untagged)]
    RoRo(ro_ro::Voice),
    #[cfg(feature = "ru-ru")]
    #[serde(untagged)]
    RuRu(ru_ru::Voice),
    #[cfg(feature = "sk-sk")]
    #[serde(untagged)]
    SkSk(sk_sk::Voice),
    #[cfg(feature = "sv-se")]
    #[serde(untagged)]
    SvSe(sv_se::Voice),
    #[cfg(feature = "ta-in")]
    #[serde(untagged)]
    TaIn(ta_in::Voice),
    #[cfg(feature = "te-in")]
    #[serde(untagged)]
    TeIn(te_in::Voice),
    #[cfg(feature = "th-th")]
    #[serde(untagged)]
    ThTh(th_th::Voice),
    #[cfg(feature = "tr-tr")]
    #[serde(untagged)]
    TrTr(tr_tr::Voice),
    #[cfg(feature = "vi-vn")]
    #[serde(untagged)]
    ViVn(vi_vn::Voice),
    #[cfg(feature = "yue-cn")]
    #[serde(untagged)]
    YueCn(yue_cn::Voice),
    #[cfg(feature = "yue-hk")]
    #[serde(untagged)]
    YueHk(yue_hk::Voice),
}

impl std::str::FromStr for Voice {
    type Err = serde_plain::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s)
    }
}

impl std::fmt::Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_plain::to_string(self)
            .map_err(|_| std::fmt::Error)
            .and_then(|s| write!(f, "{s}"))
    }
}

impl VoicePrice for Voice {
    fn price(&self) -> Option<f32> {
        match self {
            Voice::Man => Some(0.),
            Voice::Woman => Some(0.),
            #[cfg(feature = "af-za")]
            Voice::AfZa(af_za) => af_za.price(),
            #[cfg(feature = "ar-ae")]
            Voice::ArAe(ar_ae) => ar_ae.price(),
            #[cfg(feature = "ar-xa")]
            Voice::ArXa(ar_xa) => ar_xa.price(),
            #[cfg(feature = "arb")]
            Voice::Arb(arb) => arb.price(),
            #[cfg(feature = "bg-bg")]
            Voice::BgBg(bg_bg) => bg_bg.price(),
            #[cfg(feature = "bn-in")]
            Voice::BnIn(bn_in) => bn_in.price(),
            #[cfg(feature = "ca-es")]
            Voice::CaEs(ca_es) => ca_es.price(),
            #[cfg(feature = "cmn-cn")]
            Voice::CmnCn(cmn_cn) => cmn_cn.price(),
            #[cfg(feature = "cmn-tw")]
            Voice::CmnTw(cmn_tw) => cmn_tw.price(),
            #[cfg(feature = "cs-cz")]
            Voice::CsCz(cs_cz) => cs_cz.price(),
            #[cfg(feature = "cy-gb")]
            Voice::CyGb(cy_gb) => cy_gb.price(),
            #[cfg(feature = "da-dk")]
            Voice::DaDk(da_dk) => da_dk.price(),
            #[cfg(feature = "de-at")]
            Voice::DeAt(de_at) => de_at.price(),
            #[cfg(feature = "de-de")]
            Voice::DeDe(de_de) => de_de.price(),
            #[cfg(feature = "el-gr")]
            Voice::ElGr(el_gr) => el_gr.price(),
            #[cfg(feature = "en-au")]
            Voice::EnAu(en_au) => en_au.price(),
            #[cfg(feature = "en-gb")]
            Voice::EnGb(en_gb) => en_gb.price(),
            #[cfg(feature = "en-gb-wls")]
            Voice::EnGbWls(en_gb_wls) => en_gb_wls.price(),
            #[cfg(feature = "en-ie")]
            Voice::EnIe(en_ie) => en_ie.price(),
            #[cfg(feature = "en-in")]
            Voice::EnIn(en_in) => en_in.price(),
            #[cfg(feature = "en-nz")]
            Voice::EnNz(en_nz) => en_nz.price(),
            #[cfg(feature = "en-us")]
            Voice::EnUs(en_us) => en_us.price(),
            #[cfg(feature = "en-za")]
            Voice::EnZa(en_za) => en_za.price(),
            #[cfg(feature = "es-es")]
            Voice::EsEs(es_es) => es_es.price(),
            #[cfg(feature = "es-mx")]
            Voice::EsMx(es_mx) => es_mx.price(),
            #[cfg(feature = "es-us")]
            Voice::EsUs(es_us) => es_us.price(),
            #[cfg(feature = "eu-es")]
            Voice::EuEs(eu_es) => eu_es.price(),
            #[cfg(feature = "fi-fi")]
            Voice::FiFi(fi_fi) => fi_fi.price(),
            #[cfg(feature = "fil-ph")]
            Voice::FilPh(fil_ph) => fil_ph.price(),
            #[cfg(feature = "fr-be")]
            Voice::FrBe(fr_be) => fr_be.price(),
            #[cfg(feature = "fr-ca")]
            Voice::FrCa(fr_ca) => fr_ca.price(),
            #[cfg(feature = "fr-fr")]
            Voice::FrFr(fr_fr) => fr_fr.price(),
            #[cfg(feature = "gl-es")]
            Voice::GlEs(gl_es) => gl_es.price(),
            #[cfg(feature = "gu-in")]
            Voice::GuIn(gu_in) => gu_in.price(),
            #[cfg(feature = "he-il")]
            Voice::HeIl(he_il) => he_il.price(),
            #[cfg(feature = "hi-in")]
            Voice::HiIn(hi_in) => hi_in.price(),
            #[cfg(feature = "hu-hu")]
            Voice::HuHu(hu_hu) => hu_hu.price(),
            #[cfg(feature = "id-id")]
            Voice::IdId(id_id) => id_id.price(),
            #[cfg(feature = "is-is")]
            Voice::IsIs(is_is) => is_is.price(),
            #[cfg(feature = "it-it")]
            Voice::ItIt(it_it) => it_it.price(),
            #[cfg(feature = "ja-jp")]
            Voice::JaJp(ja_jp) => ja_jp.price(),
            #[cfg(feature = "kn-in")]
            Voice::KnIn(kn_in) => kn_in.price(),
            #[cfg(feature = "ko-kr")]
            Voice::KoKr(ko_kr) => ko_kr.price(),
            #[cfg(feature = "lt-lt")]
            Voice::LtLt(lt_lt) => lt_lt.price(),
            #[cfg(feature = "lv-lv")]
            Voice::LvLv(lv_lv) => lv_lv.price(),
            #[cfg(feature = "ml-in")]
            Voice::MlIn(ml_in) => ml_in.price(),
            #[cfg(feature = "mr-in")]
            Voice::MrIn(mr_in) => mr_in.price(),
            #[cfg(feature = "ms-my")]
            Voice::MsMy(ms_my) => ms_my.price(),
            #[cfg(feature = "nb-no")]
            Voice::NbNo(nb_no) => nb_no.price(),
            #[cfg(feature = "nl-be")]
            Voice::NlBe(nl_be) => nl_be.price(),
            #[cfg(feature = "nl-nl")]
            Voice::NlNl(nl_nl) => nl_nl.price(),
            #[cfg(feature = "pa-in")]
            Voice::PaIn(pa_in) => pa_in.price(),
            #[cfg(feature = "pl-pl")]
            Voice::PlPl(pl_pl) => pl_pl.price(),
            #[cfg(feature = "pt-br")]
            Voice::PtBr(pt_br) => pt_br.price(),
            #[cfg(feature = "pt-pt")]
            Voice::PtPt(pt_pt) => pt_pt.price(),
            #[cfg(feature = "ro-ro")]
            Voice::RoRo(ro_ro) => ro_ro.price(),
            #[cfg(feature = "ru-ru")]
            Voice::RuRu(ru_ru) => ru_ru.price(),
            #[cfg(feature = "sk-sk")]
            Voice::SkSk(sk_sk) => sk_sk.price(),
            #[cfg(feature = "sv-se")]
            Voice::SvSe(sv_se) => sv_se.price(),
            #[cfg(feature = "ta-in")]
            Voice::TaIn(ta_in) => ta_in.price(),
            #[cfg(feature = "te-in")]
            Voice::TeIn(te_in) => te_in.price(),
            #[cfg(feature = "th-th")]
            Voice::ThTh(th_th) => th_th.price(),
            #[cfg(feature = "tr-tr")]
            Voice::TrTr(tr_tr) => tr_tr.price(),
            #[cfg(feature = "vi-vn")]
            Voice::ViVn(vi_vn) => vi_vn.price(),
            #[cfg(feature = "yue-cn")]
            Voice::YueCn(yue_cn) => yue_cn.price(),
            #[cfg(feature = "yue-hk")]
            Voice::YueHk(yue_hk) => yue_hk.price(),
        }
    }
}

impl VoiceGender for Voice {
    fn gender(&self) -> Gender {
        match self {
            Voice::Man => Gender::Male,
            Voice::Woman => Gender::Female,
            #[cfg(feature = "af-za")]
            Voice::AfZa(af_za) => af_za.gender(),
            #[cfg(feature = "ar-ae")]
            Voice::ArAe(ar_ae) => ar_ae.gender(),
            #[cfg(feature = "ar-xa")]
            Voice::ArXa(ar_xa) => ar_xa.gender(),
            #[cfg(feature = "arb")]
            Voice::Arb(arb) => arb.gender(),
            #[cfg(feature = "bg-bg")]
            Voice::BgBg(bg_bg) => bg_bg.gender(),
            #[cfg(feature = "bn-in")]
            Voice::BnIn(bn_in) => bn_in.gender(),
            #[cfg(feature = "ca-es")]
            Voice::CaEs(ca_es) => ca_es.gender(),
            #[cfg(feature = "cmn-cn")]
            Voice::CmnCn(cmn_cn) => cmn_cn.gender(),
            #[cfg(feature = "cmn-tw")]
            Voice::CmnTw(cmn_tw) => cmn_tw.gender(),
            #[cfg(feature = "cs-cz")]
            Voice::CsCz(cs_cz) => cs_cz.gender(),
            #[cfg(feature = "cy-gb")]
            Voice::CyGb(cy_gb) => cy_gb.gender(),
            #[cfg(feature = "da-dk")]
            Voice::DaDk(da_dk) => da_dk.gender(),
            #[cfg(feature = "de-at")]
            Voice::DeAt(de_at) => de_at.gender(),
            #[cfg(feature = "de-de")]
            Voice::DeDe(de_de) => de_de.gender(),
            #[cfg(feature = "el-gr")]
            Voice::ElGr(el_gr) => el_gr.gender(),
            #[cfg(feature = "en-au")]
            Voice::EnAu(en_au) => en_au.gender(),
            #[cfg(feature = "en-gb")]
            Voice::EnGb(en_gb) => en_gb.gender(),
            #[cfg(feature = "en-gb-wls")]
            Voice::EnGbWls(en_gb_wls) => en_gb_wls.gender(),
            #[cfg(feature = "en-ie")]
            Voice::EnIe(en_ie) => en_ie.gender(),
            #[cfg(feature = "en-in")]
            Voice::EnIn(en_in) => en_in.gender(),
            #[cfg(feature = "en-nz")]
            Voice::EnNz(en_nz) => en_nz.gender(),
            #[cfg(feature = "en-us")]
            Voice::EnUs(en_us) => en_us.gender(),
            #[cfg(feature = "en-za")]
            Voice::EnZa(en_za) => en_za.gender(),
            #[cfg(feature = "es-es")]
            Voice::EsEs(es_es) => es_es.gender(),
            #[cfg(feature = "es-mx")]
            Voice::EsMx(es_mx) => es_mx.gender(),
            #[cfg(feature = "es-us")]
            Voice::EsUs(es_us) => es_us.gender(),
            #[cfg(feature = "eu-es")]
            Voice::EuEs(eu_es) => eu_es.gender(),
            #[cfg(feature = "fi-fi")]
            Voice::FiFi(fi_fi) => fi_fi.gender(),
            #[cfg(feature = "fil-ph")]
            Voice::FilPh(fil_ph) => fil_ph.gender(),
            #[cfg(feature = "fr-be")]
            Voice::FrBe(fr_be) => fr_be.gender(),
            #[cfg(feature = "fr-ca")]
            Voice::FrCa(fr_ca) => fr_ca.gender(),
            #[cfg(feature = "fr-fr")]
            Voice::FrFr(fr_fr) => fr_fr.gender(),
            #[cfg(feature = "gl-es")]
            Voice::GlEs(gl_es) => gl_es.gender(),
            #[cfg(feature = "gu-in")]
            Voice::GuIn(gu_in) => gu_in.gender(),
            #[cfg(feature = "he-il")]
            Voice::HeIl(he_il) => he_il.gender(),
            #[cfg(feature = "hi-in")]
            Voice::HiIn(hi_in) => hi_in.gender(),
            #[cfg(feature = "hu-hu")]
            Voice::HuHu(hu_hu) => hu_hu.gender(),
            #[cfg(feature = "id-id")]
            Voice::IdId(id_id) => id_id.gender(),
            #[cfg(feature = "is-is")]
            Voice::IsIs(is_is) => is_is.gender(),
            #[cfg(feature = "it-it")]
            Voice::ItIt(it_it) => it_it.gender(),
            #[cfg(feature = "ja-jp")]
            Voice::JaJp(ja_jp) => ja_jp.gender(),
            #[cfg(feature = "kn-in")]
            Voice::KnIn(kn_in) => kn_in.gender(),
            #[cfg(feature = "ko-kr")]
            Voice::KoKr(ko_kr) => ko_kr.gender(),
            #[cfg(feature = "lt-lt")]
            Voice::LtLt(lt_lt) => lt_lt.gender(),
            #[cfg(feature = "lv-lv")]
            Voice::LvLv(lv_lv) => lv_lv.gender(),
            #[cfg(feature = "ml-in")]
            Voice::MlIn(ml_in) => ml_in.gender(),
            #[cfg(feature = "mr-in")]
            Voice::MrIn(mr_in) => mr_in.gender(),
            #[cfg(feature = "ms-my")]
            Voice::MsMy(ms_my) => ms_my.gender(),
            #[cfg(feature = "nb-no")]
            Voice::NbNo(nb_no) => nb_no.gender(),
            #[cfg(feature = "nl-be")]
            Voice::NlBe(nl_be) => nl_be.gender(),
            #[cfg(feature = "nl-nl")]
            Voice::NlNl(nl_nl) => nl_nl.gender(),
            #[cfg(feature = "pa-in")]
            Voice::PaIn(pa_in) => pa_in.gender(),
            #[cfg(feature = "pl-pl")]
            Voice::PlPl(pl_pl) => pl_pl.gender(),
            #[cfg(feature = "pt-br")]
            Voice::PtBr(pt_br) => pt_br.gender(),
            #[cfg(feature = "pt-pt")]
            Voice::PtPt(pt_pt) => pt_pt.gender(),
            #[cfg(feature = "ro-ro")]
            Voice::RoRo(ro_ro) => ro_ro.gender(),
            #[cfg(feature = "ru-ru")]
            Voice::RuRu(ru_ru) => ru_ru.gender(),
            #[cfg(feature = "sk-sk")]
            Voice::SkSk(sk_sk) => sk_sk.gender(),
            #[cfg(feature = "sv-se")]
            Voice::SvSe(sv_se) => sv_se.gender(),
            #[cfg(feature = "ta-in")]
            Voice::TaIn(ta_in) => ta_in.gender(),
            #[cfg(feature = "te-in")]
            Voice::TeIn(te_in) => te_in.gender(),
            #[cfg(feature = "th-th")]
            Voice::ThTh(th_th) => th_th.gender(),
            #[cfg(feature = "tr-tr")]
            Voice::TrTr(tr_tr) => tr_tr.gender(),
            #[cfg(feature = "vi-vn")]
            Voice::ViVn(vi_vn) => vi_vn.gender(),
            #[cfg(feature = "yue-cn")]
            Voice::YueCn(yue_cn) => yue_cn.gender(),
            #[cfg(feature = "yue-hk")]
            Voice::YueHk(yue_hk) => yue_hk.gender(),
        }
    }
}
