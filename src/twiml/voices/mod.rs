// Auto-generated voice module
// Source: Twilio documentation
#![allow(non_local_definitions)]

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Voice {
    #[cfg(feature = "af-za")]
    AfZa(af_za::Voice),
    #[cfg(feature = "ar-ae")]
    ArAe(ar_ae::Voice),
    #[cfg(feature = "ar-xa")]
    ArXa(ar_xa::Voice),
    #[cfg(feature = "arb")]
    Arb(arb::Voice),
    #[cfg(feature = "bg-bg")]
    BgBg(bg_bg::Voice),
    #[cfg(feature = "bn-in")]
    BnIn(bn_in::Voice),
    #[cfg(feature = "ca-es")]
    CaEs(ca_es::Voice),
    #[cfg(feature = "cmn-cn")]
    CmnCn(cmn_cn::Voice),
    #[cfg(feature = "cmn-tw")]
    CmnTw(cmn_tw::Voice),
    #[cfg(feature = "cs-cz")]
    CsCz(cs_cz::Voice),
    #[cfg(feature = "cy-gb")]
    CyGb(cy_gb::Voice),
    #[cfg(feature = "da-dk")]
    DaDk(da_dk::Voice),
    #[cfg(feature = "de-at")]
    DeAt(de_at::Voice),
    #[cfg(feature = "de-de")]
    DeDe(de_de::Voice),
    #[cfg(feature = "el-gr")]
    ElGr(el_gr::Voice),
    #[cfg(feature = "en-au")]
    EnAu(en_au::Voice),
    #[cfg(feature = "en-gb")]
    EnGb(en_gb::Voice),
    #[cfg(feature = "en-gb-wls")]
    EnGbWls(en_gb_wls::Voice),
    #[cfg(feature = "en-ie")]
    EnIe(en_ie::Voice),
    #[cfg(feature = "en-in")]
    EnIn(en_in::Voice),
    #[cfg(feature = "en-nz")]
    EnNz(en_nz::Voice),
    #[cfg(feature = "en-us")]
    EnUs(en_us::Voice),
    #[cfg(feature = "en-za")]
    EnZa(en_za::Voice),
    #[cfg(feature = "es-es")]
    EsEs(es_es::Voice),
    #[cfg(feature = "es-mx")]
    EsMx(es_mx::Voice),
    #[cfg(feature = "es-us")]
    EsUs(es_us::Voice),
    #[cfg(feature = "eu-es")]
    EuEs(eu_es::Voice),
    #[cfg(feature = "fi-fi")]
    FiFi(fi_fi::Voice),
    #[cfg(feature = "fil-ph")]
    FilPh(fil_ph::Voice),
    #[cfg(feature = "fr-be")]
    FrBe(fr_be::Voice),
    #[cfg(feature = "fr-ca")]
    FrCa(fr_ca::Voice),
    #[cfg(feature = "fr-fr")]
    FrFr(fr_fr::Voice),
    #[cfg(feature = "gl-es")]
    GlEs(gl_es::Voice),
    #[cfg(feature = "gu-in")]
    GuIn(gu_in::Voice),
    #[cfg(feature = "he-il")]
    HeIl(he_il::Voice),
    #[cfg(feature = "hi-in")]
    HiIn(hi_in::Voice),
    #[cfg(feature = "hu-hu")]
    HuHu(hu_hu::Voice),
    #[cfg(feature = "id-id")]
    IdId(id_id::Voice),
    #[cfg(feature = "is-is")]
    IsIs(is_is::Voice),
    #[cfg(feature = "it-it")]
    ItIt(it_it::Voice),
    #[cfg(feature = "ja-jp")]
    JaJp(ja_jp::Voice),
    #[cfg(feature = "kn-in")]
    KnIn(kn_in::Voice),
    #[cfg(feature = "ko-kr")]
    KoKr(ko_kr::Voice),
    #[cfg(feature = "lt-lt")]
    LtLt(lt_lt::Voice),
    #[cfg(feature = "lv-lv")]
    LvLv(lv_lv::Voice),
    #[cfg(feature = "ml-in")]
    MlIn(ml_in::Voice),
    #[cfg(feature = "mr-in")]
    MrIn(mr_in::Voice),
    #[cfg(feature = "ms-my")]
    MsMy(ms_my::Voice),
    #[cfg(feature = "nb-no")]
    NbNo(nb_no::Voice),
    #[cfg(feature = "nl-be")]
    NlBe(nl_be::Voice),
    #[cfg(feature = "nl-nl")]
    NlNl(nl_nl::Voice),
    #[cfg(feature = "pa-in")]
    PaIn(pa_in::Voice),
    #[cfg(feature = "pl-pl")]
    PlPl(pl_pl::Voice),
    #[cfg(feature = "pt-br")]
    PtBr(pt_br::Voice),
    #[cfg(feature = "pt-pt")]
    PtPt(pt_pt::Voice),
    #[cfg(feature = "ro-ro")]
    RoRo(ro_ro::Voice),
    #[cfg(feature = "ru-ru")]
    RuRu(ru_ru::Voice),
    #[cfg(feature = "sk-sk")]
    SkSk(sk_sk::Voice),
    #[cfg(feature = "sv-se")]
    SvSe(sv_se::Voice),
    #[cfg(feature = "ta-in")]
    TaIn(ta_in::Voice),
    #[cfg(feature = "te-in")]
    TeIn(te_in::Voice),
    #[cfg(feature = "th-th")]
    ThTh(th_th::Voice),
    #[cfg(feature = "tr-tr")]
    TrTr(tr_tr::Voice),
    #[cfg(feature = "vi-vn")]
    ViVn(vi_vn::Voice),
    #[cfg(feature = "yue-cn")]
    YueCn(yue_cn::Voice),
    #[cfg(feature = "yue-hk")]
    YueHk(yue_hk::Voice),
}
