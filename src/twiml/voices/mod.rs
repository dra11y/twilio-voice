// Auto-generated voice module
// Source: Twilio documentation
#![allow(non_local_definitions)]

#[cfg(feature = "af_za")]
pub mod af_za;
#[cfg(feature = "ar_ae")]
pub mod ar_ae;
#[cfg(feature = "ar_xa")]
pub mod ar_xa;
#[cfg(feature = "arb")]
pub mod arb;
#[cfg(feature = "bg_bg")]
pub mod bg_bg;
#[cfg(feature = "bn_in")]
pub mod bn_in;
#[cfg(feature = "ca_es")]
pub mod ca_es;
#[cfg(feature = "cmn_cn")]
pub mod cmn_cn;
#[cfg(feature = "cmn_tw")]
pub mod cmn_tw;
#[cfg(feature = "cs_cz")]
pub mod cs_cz;
#[cfg(feature = "cy_gb")]
pub mod cy_gb;
#[cfg(feature = "da_dk")]
pub mod da_dk;
#[cfg(feature = "de_at")]
pub mod de_at;
#[cfg(feature = "de_de")]
pub mod de_de;
#[cfg(feature = "el_gr")]
pub mod el_gr;
#[cfg(feature = "en_au")]
pub mod en_au;
#[cfg(feature = "en_gb")]
pub mod en_gb;
#[cfg(feature = "en_gb_wls")]
pub mod en_gb_wls;
#[cfg(feature = "en_ie")]
pub mod en_ie;
#[cfg(feature = "en_in")]
pub mod en_in;
#[cfg(feature = "en_nz")]
pub mod en_nz;
#[cfg(feature = "en_us")]
pub mod en_us;
#[cfg(feature = "en_za")]
pub mod en_za;
#[cfg(feature = "es_es")]
pub mod es_es;
#[cfg(feature = "es_mx")]
pub mod es_mx;
#[cfg(feature = "es_us")]
pub mod es_us;
#[cfg(feature = "eu_es")]
pub mod eu_es;
#[cfg(feature = "fi_fi")]
pub mod fi_fi;
#[cfg(feature = "fil_ph")]
pub mod fil_ph;
#[cfg(feature = "fr_be")]
pub mod fr_be;
#[cfg(feature = "fr_ca")]
pub mod fr_ca;
#[cfg(feature = "fr_fr")]
pub mod fr_fr;
#[cfg(feature = "gl_es")]
pub mod gl_es;
#[cfg(feature = "gu_in")]
pub mod gu_in;
#[cfg(feature = "he_il")]
pub mod he_il;
#[cfg(feature = "hi_in")]
pub mod hi_in;
#[cfg(feature = "hu_hu")]
pub mod hu_hu;
#[cfg(feature = "id_id")]
pub mod id_id;
#[cfg(feature = "is_is")]
pub mod is_is;
#[cfg(feature = "it_it")]
pub mod it_it;
#[cfg(feature = "ja_jp")]
pub mod ja_jp;
#[cfg(feature = "kn_in")]
pub mod kn_in;
#[cfg(feature = "ko_kr")]
pub mod ko_kr;
#[cfg(feature = "lt_lt")]
pub mod lt_lt;
#[cfg(feature = "lv_lv")]
pub mod lv_lv;
#[cfg(feature = "ml_in")]
pub mod ml_in;
#[cfg(feature = "mr_in")]
pub mod mr_in;
#[cfg(feature = "ms_my")]
pub mod ms_my;
#[cfg(feature = "nb_no")]
pub mod nb_no;
#[cfg(feature = "nl_be")]
pub mod nl_be;
#[cfg(feature = "nl_nl")]
pub mod nl_nl;
#[cfg(feature = "pa_in")]
pub mod pa_in;
#[cfg(feature = "pl_pl")]
pub mod pl_pl;
#[cfg(feature = "pt_br")]
pub mod pt_br;
#[cfg(feature = "pt_pt")]
pub mod pt_pt;
#[cfg(feature = "ro_ro")]
pub mod ro_ro;
#[cfg(feature = "ru_ru")]
pub mod ru_ru;
#[cfg(feature = "sk_sk")]
pub mod sk_sk;
#[cfg(feature = "sv_se")]
pub mod sv_se;
#[cfg(feature = "ta_in")]
pub mod ta_in;
#[cfg(feature = "te_in")]
pub mod te_in;
#[cfg(feature = "th_th")]
pub mod th_th;
#[cfg(feature = "tr_tr")]
pub mod tr_tr;
#[cfg(feature = "vi_vn")]
pub mod vi_vn;
#[cfg(feature = "yue_cn")]
pub mod yue_cn;
#[cfg(feature = "yue_hk")]
pub mod yue_hk;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Voice {
    #[cfg(feature = "af_za")]
    AfZa(af_za::Voice),
    #[cfg(feature = "ar_ae")]
    ArAe(ar_ae::Voice),
    #[cfg(feature = "ar_xa")]
    ArXa(ar_xa::Voice),
    #[cfg(feature = "arb")]
    Arb(arb::Voice),
    #[cfg(feature = "bg_bg")]
    BgBg(bg_bg::Voice),
    #[cfg(feature = "bn_in")]
    BnIn(bn_in::Voice),
    #[cfg(feature = "ca_es")]
    CaEs(ca_es::Voice),
    #[cfg(feature = "cmn_cn")]
    CmnCn(cmn_cn::Voice),
    #[cfg(feature = "cmn_tw")]
    CmnTw(cmn_tw::Voice),
    #[cfg(feature = "cs_cz")]
    CsCz(cs_cz::Voice),
    #[cfg(feature = "cy_gb")]
    CyGb(cy_gb::Voice),
    #[cfg(feature = "da_dk")]
    DaDk(da_dk::Voice),
    #[cfg(feature = "de_at")]
    DeAt(de_at::Voice),
    #[cfg(feature = "de_de")]
    DeDe(de_de::Voice),
    #[cfg(feature = "el_gr")]
    ElGr(el_gr::Voice),
    #[cfg(feature = "en_au")]
    EnAu(en_au::Voice),
    #[cfg(feature = "en_gb")]
    EnGb(en_gb::Voice),
    #[cfg(feature = "en_gb_wls")]
    EnGbWls(en_gb_wls::Voice),
    #[cfg(feature = "en_ie")]
    EnIe(en_ie::Voice),
    #[cfg(feature = "en_in")]
    EnIn(en_in::Voice),
    #[cfg(feature = "en_nz")]
    EnNz(en_nz::Voice),
    #[cfg(feature = "en_us")]
    EnUs(en_us::Voice),
    #[cfg(feature = "en_za")]
    EnZa(en_za::Voice),
    #[cfg(feature = "es_es")]
    EsEs(es_es::Voice),
    #[cfg(feature = "es_mx")]
    EsMx(es_mx::Voice),
    #[cfg(feature = "es_us")]
    EsUs(es_us::Voice),
    #[cfg(feature = "eu_es")]
    EuEs(eu_es::Voice),
    #[cfg(feature = "fi_fi")]
    FiFi(fi_fi::Voice),
    #[cfg(feature = "fil_ph")]
    FilPh(fil_ph::Voice),
    #[cfg(feature = "fr_be")]
    FrBe(fr_be::Voice),
    #[cfg(feature = "fr_ca")]
    FrCa(fr_ca::Voice),
    #[cfg(feature = "fr_fr")]
    FrFr(fr_fr::Voice),
    #[cfg(feature = "gl_es")]
    GlEs(gl_es::Voice),
    #[cfg(feature = "gu_in")]
    GuIn(gu_in::Voice),
    #[cfg(feature = "he_il")]
    HeIl(he_il::Voice),
    #[cfg(feature = "hi_in")]
    HiIn(hi_in::Voice),
    #[cfg(feature = "hu_hu")]
    HuHu(hu_hu::Voice),
    #[cfg(feature = "id_id")]
    IdId(id_id::Voice),
    #[cfg(feature = "is_is")]
    IsIs(is_is::Voice),
    #[cfg(feature = "it_it")]
    ItIt(it_it::Voice),
    #[cfg(feature = "ja_jp")]
    JaJp(ja_jp::Voice),
    #[cfg(feature = "kn_in")]
    KnIn(kn_in::Voice),
    #[cfg(feature = "ko_kr")]
    KoKr(ko_kr::Voice),
    #[cfg(feature = "lt_lt")]
    LtLt(lt_lt::Voice),
    #[cfg(feature = "lv_lv")]
    LvLv(lv_lv::Voice),
    #[cfg(feature = "ml_in")]
    MlIn(ml_in::Voice),
    #[cfg(feature = "mr_in")]
    MrIn(mr_in::Voice),
    #[cfg(feature = "ms_my")]
    MsMy(ms_my::Voice),
    #[cfg(feature = "nb_no")]
    NbNo(nb_no::Voice),
    #[cfg(feature = "nl_be")]
    NlBe(nl_be::Voice),
    #[cfg(feature = "nl_nl")]
    NlNl(nl_nl::Voice),
    #[cfg(feature = "pa_in")]
    PaIn(pa_in::Voice),
    #[cfg(feature = "pl_pl")]
    PlPl(pl_pl::Voice),
    #[cfg(feature = "pt_br")]
    PtBr(pt_br::Voice),
    #[cfg(feature = "pt_pt")]
    PtPt(pt_pt::Voice),
    #[cfg(feature = "ro_ro")]
    RoRo(ro_ro::Voice),
    #[cfg(feature = "ru_ru")]
    RuRu(ru_ru::Voice),
    #[cfg(feature = "sk_sk")]
    SkSk(sk_sk::Voice),
    #[cfg(feature = "sv_se")]
    SvSe(sv_se::Voice),
    #[cfg(feature = "ta_in")]
    TaIn(ta_in::Voice),
    #[cfg(feature = "te_in")]
    TeIn(te_in::Voice),
    #[cfg(feature = "th_th")]
    ThTh(th_th::Voice),
    #[cfg(feature = "tr_tr")]
    TrTr(tr_tr::Voice),
    #[cfg(feature = "vi_vn")]
    ViVn(vi_vn::Voice),
    #[cfg(feature = "yue_cn")]
    YueCn(yue_cn::Voice),
    #[cfg(feature = "yue_hk")]
    YueHk(yue_hk::Voice),
}
