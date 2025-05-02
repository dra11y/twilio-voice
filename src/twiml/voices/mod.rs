// Auto-generated voice module from Twilio documentation
// Source: https://www.twilio.com/docs/voice/twiml/say/text-speech
// Generated on: 2025-05-02 04:33:50.476515 -06:00

#![allow(non_local_definitions)]

pub mod it_it;
pub mod bn_in;
pub mod es_es;
pub mod yue_hk;
pub mod he_il;
pub mod lv_lv;
pub mod ar_ae;
pub mod en_gb;
pub mod en_za;
pub mod id_id;
pub mod te_in;
pub mod pl_pl;
pub mod cs_cz;
pub mod sv_se;
pub mod fil_ph;
pub mod hi_in;
pub mod ja_jp;
pub mod nb_no;
pub mod en_in;
pub mod hu_hu;
pub mod pa_in;
pub mod ar_xa;
pub mod sk_sk;
pub mod nl_be;
pub mod en_us;
pub mod en_nz;
pub mod pt_br;
pub mod gu_in;
pub mod fr_ca;
pub mod kn_in;
pub mod yue_cn;
pub mod de_at;
pub mod eu_es;
pub mod el_gr;
pub mod pt_pt;
pub mod ms_my;
pub mod es_us;
pub mod fr_fr;
pub mod th_th;
pub mod tr_tr;
pub mod ca_es;
pub mod ko_kr;
pub mod cy_gb;
pub mod vi_vn;
pub mod ta_in;
pub mod de_de;
pub mod da_dk;
pub mod fr_be;
pub mod gl_es;
pub mod is_is;
pub mod ml_in;
pub mod ro_ro;
pub mod es_mx;
pub mod nl_nl;
pub mod en_au;
pub mod af_za;
pub mod ru_ru;
pub mod bg_bg;
pub mod cmn_tw;
pub mod en_gb_wls;
pub mod en_ie;
pub mod cmn_cn;
pub mod arb;
pub mod fi_fi;
pub mod mr_in;
pub mod lt_lt;

use serde::{Serialize, Deserialize};

#[amass::amass_telety(crate::twiml::voices)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    AfZa(af_za::Voice),
    ArAe(ar_ae::Voice),
    ArXa(ar_xa::Voice),
    Arb(arb::Voice),
    BgBg(bg_bg::Voice),
    BnIn(bn_in::Voice),
    CaEs(ca_es::Voice),
    CmnCn(cmn_cn::Voice),
    CmnTw(cmn_tw::Voice),
    CsCz(cs_cz::Voice),
    CyGb(cy_gb::Voice),
    DaDk(da_dk::Voice),
    DeAt(de_at::Voice),
    DeDe(de_de::Voice),
    ElGr(el_gr::Voice),
    EnAu(en_au::Voice),
    EnGb(en_gb::Voice),
    EnGbWls(en_gb_wls::Voice),
    EnIe(en_ie::Voice),
    EnIn(en_in::Voice),
    EnNz(en_nz::Voice),
    EnUs(en_us::Voice),
    EnZa(en_za::Voice),
    EsEs(es_es::Voice),
    EsMx(es_mx::Voice),
    EsUs(es_us::Voice),
    EuEs(eu_es::Voice),
    FiFi(fi_fi::Voice),
    FilPh(fil_ph::Voice),
    FrBe(fr_be::Voice),
    FrCa(fr_ca::Voice),
    FrFr(fr_fr::Voice),
    GlEs(gl_es::Voice),
    GuIn(gu_in::Voice),
    HeIl(he_il::Voice),
    HiIn(hi_in::Voice),
    HuHu(hu_hu::Voice),
    IdId(id_id::Voice),
    IsIs(is_is::Voice),
    ItIt(it_it::Voice),
    JaJp(ja_jp::Voice),
    KnIn(kn_in::Voice),
    KoKr(ko_kr::Voice),
    LtLt(lt_lt::Voice),
    LvLv(lv_lv::Voice),
    MlIn(ml_in::Voice),
    MrIn(mr_in::Voice),
    MsMy(ms_my::Voice),
    NbNo(nb_no::Voice),
    NlBe(nl_be::Voice),
    NlNl(nl_nl::Voice),
    PaIn(pa_in::Voice),
    PlPl(pl_pl::Voice),
    PtBr(pt_br::Voice),
    PtPt(pt_pt::Voice),
    RoRo(ro_ro::Voice),
    RuRu(ru_ru::Voice),
    SkSk(sk_sk::Voice),
    SvSe(sv_se::Voice),
    TaIn(ta_in::Voice),
    TeIn(te_in::Voice),
    ThTh(th_th::Voice),
    TrTr(tr_tr::Voice),
    ViVn(vi_vn::Voice),
    YueCn(yue_cn::Voice),
    YueHk(yue_hk::Voice),
}
