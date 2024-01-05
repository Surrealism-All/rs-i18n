use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

/// ## i18n name enum
pub enum I18ns {
    /// 简体中文(中国)
    ZHCN,
    /// 繁体中文(台湾地区)
    ZHTW,
    /// 繁体中文(香港)
    ZHHK,
    /// 英语(香港)
    ENHK,
    /// 英语(美国)
    ENUS,
    /// 英语(英国)
    ENGB,
    /// 英语(全球)
    ENWW,
    /// 英语(加拿大)
    ENCA,
    /// 英语(澳大利亚)
    ENAU,
    /// 英语(爱尔兰)
    ENIE,
    /// 英语(芬兰)
    ENFI,
    /// 芬兰语(芬兰)
    FIFI,
    /// 英语(丹麦)
    ENDK,
    /// 丹麦语(丹麦)
    DADK,
    /// 英语(以色列)
    ENIL,
    /// 希伯来语(以色列)
    HEIL,
    /// 英语(南非)
    ENZA,
    /// 英语(印度)
    ENIN,
    /// 英语(挪威)
    ENNO,
    /// 英语(新加坡)
    ENSG,
    /// 英语(新西兰)
    ENNZ,
    /// 英语(印度尼西亚)
    ENID,
    /// 英语(菲律宾)
    ENPH,
    /// 英语(泰国)
    ENTH,
    /// 英语(马来西亚)
    ENMY,
    /// 英语(阿拉伯)
    ENXA,
    /// 韩文(韩国)
    KOKR,
    /// 日语(日本)
    JAJP,
    /// 荷兰语(荷兰)
    NLNL,
    /// 荷兰语(比利时)
    NLBE,
    /// 葡萄牙语(葡萄牙)
    PTPT,
    /// 葡萄牙语(巴西)
    PTBR,
    /// 法语(法国)
    FRFR,
    /// 法语(卢森堡)
    FRLU,
    /// 法语(瑞士)
    FRCH,
    /// 法语(比利时)
    FRBE,
    /// 法语(加拿大)
    FRCA,
    /// 西班牙语(拉丁美洲)
    ESLA,
    /// 西班牙语(西班牙)
    ESES,
    /// 西班牙语(阿根廷)
    ESAR,
    /// 西班牙语(美国)
    ESUS,
    /// 西班牙语(墨西哥)
    ESMX,
    /// 西班牙语(哥伦比亚)
    ESCO,
    /// 西班牙语(波多黎各)
    ESPR,
    /// 德语(德国)
    DEDE,
    /// 德语(奥地利)
    DEAT,
    /// 德语(瑞士)
    DECH,
    /// 俄语(俄罗斯)
    RURU,
    /// 意大利语(意大利)
    ITIT,
    /// 希腊语(希腊)
    ELGR,
    /// 挪威语(挪威)
    NONO,
    /// 匈牙利语(匈牙利)
    HUHU,
    /// 土耳其语(土耳其)
    TRTR,
    /// 捷克语(捷克共和国)
    CSCZ,
    /// 斯洛文尼亚语
    SLSL,
    /// 波兰语(波兰)
    PLPL,
    /// 瑞典语(瑞典)
    SVSE,
    /// 西班牙语 (智利)
    ESCL,
}

impl Default for I18ns {
    fn default() -> Self {
        I18ns::ENUS
    }
}

impl ToString for I18ns {
    fn to_string(&self) -> String {
        let result = match self {
            I18ns::ZHCN => "zh_CN",
            I18ns::ZHTW => "zh_TW",
            I18ns::ZHHK => "zh_HK",
            I18ns::ENHK => "en_HK",
            I18ns::ENUS => "en_US",
            I18ns::ENGB => "en_GB",
            I18ns::ENWW => "en_WW",
            I18ns::ENCA => "en_CA",
            I18ns::ENAU => "en_AU",
            I18ns::ENIE => "en_IE",
            I18ns::ENFI => "en_FI",
            I18ns::FIFI => "fi_FI",
            I18ns::ENDK => "en_DK",
            I18ns::DADK => "da_DK",
            I18ns::ENIL => "en_IL",
            I18ns::HEIL => "he_IL",
            I18ns::ENZA => "en_ZA",
            I18ns::ENIN => "en_IN",
            I18ns::ENNO => "en_NO",
            I18ns::ENSG => "en_SG",
            I18ns::ENNZ => "en_NZ",
            I18ns::ENID => "en_ID",
            I18ns::ENPH => "en_PH",
            I18ns::ENTH => "en_TH",
            I18ns::ENMY => "en_MY",
            I18ns::ENXA => "en_XA",
            I18ns::KOKR => "ko_KR",
            I18ns::JAJP => "ja_JP",
            I18ns::NLNL => "nl_NL",
            I18ns::NLBE => "nl_BE",
            I18ns::PTPT => "pt_PT",
            I18ns::PTBR => "pt_BR",
            I18ns::FRFR => "fr_FR",
            I18ns::FRLU => "fr_LU",
            I18ns::FRCH => "fr_CN",
            I18ns::FRBE => "fr_BE",
            I18ns::FRCA => "fr_CA",
            I18ns::ESLA => "es_LA",
            I18ns::ESES => "es_ES",
            I18ns::ESAR => "es_AR",
            I18ns::ESUS => "es_US",
            I18ns::ESMX => "es_MX",
            I18ns::ESCO => "es_CO",
            I18ns::ESPR => "es_PR",
            I18ns::DEDE => "de_DE",
            I18ns::DEAT => "de_DE",
            I18ns::DECH => "de_CH",
            I18ns::RURU => "ru_RU",
            I18ns::ITIT => "it_IT",
            I18ns::ELGR => "el_GR",
            I18ns::NONO => "no_NO",
            I18ns::HUHU => "hu_HU",
            I18ns::TRTR => "tr_TR",
            I18ns::CSCZ => "cs_CZ",
            I18ns::SLSL => "sl_SL",
            I18ns::PLPL => "pl_PL",
            I18ns::SVSE => "sv_SE",
            I18ns::ESCL => "es_CL",
        };
        String::from(result)
    }
}

impl FromStr for I18ns {
    type Err = &'static str;
    /// ## Return a I18ns instance from a str
    /// the str is from the dir name
    ///
    /// you will never see it return an Error , when it cannot match
    /// this function will return `I18ns::default()` (en_US)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s.to_lowercase().as_str() {
            "zh" | "zh_cn" => I18ns::ZHCN,
            "tw" | "zh_tw" => I18ns::ZHTW,
            "hk" | "zh_hk" => I18ns::ZHHK,
            "en_hk" => I18ns::ENHK,
            "us" | "en_us" => I18ns::ENUS,
            "gb" | "en_gb" => I18ns::ENGB,
            "ww" | "en_ww" => I18ns::ENWW,
            "ca" | "en_ca" => I18ns::ENCA,
            "au" | "en_au" => I18ns::ENAU,
            "ie" | "en_ie" => I18ns::ENIE,
            "fi" | "en_fi" => I18ns::ENFI,
            "fi_fi" => I18ns::FIFI,
            "dk" | "en_dk" => I18ns::ENDK,
            "da_dk" => I18ns::DADK,
            "il" | "en_il" => I18ns::ENIL,
            "he_il" => I18ns::HEIL,
            "za" | "en_za" => I18ns::ENZA,
            "in" | "en_in" => I18ns::ENIN,
            "no" | "en_no" => I18ns::ENNO,
            "sg" | "en_sg" => I18ns::ENSG,
            "nz" | "en_nz" => I18ns::ENNZ,
            "id" | "en_id" => I18ns::ENID,
            "ph" | "en_ph" => I18ns::ENPH,
            "th" | "en_th" => I18ns::ENTH,
            "my" | "en_my" => I18ns::ENMY,
            "xa" | "en_xa" => I18ns::ENXA,
            "kr" | "ko_kr" => I18ns::KOKR,
            "jp" | "ja_jp" => I18ns::JAJP,
            "nl" | "nl_nl" => I18ns::NLNL,
            "be" | "nl_be" => I18ns::NLBE,
            "pt" | "pt_pt" => I18ns::PTPT,
            "br" | "pt_br" => I18ns::PTBR,
            "fr" | "fr_fr" => I18ns::FRFR,
            "lu" | "fr_lu" => I18ns::FRLU,
            "fr_cn" => I18ns::FRCH,
            "fr_be" => I18ns::FRBE,
            "fr_ca" => I18ns::FRCA,
            "la" | "es_la" => I18ns::ESLA,
            "es" | "es_es" => I18ns::ESES,
            "ar" | "es_ar" => I18ns::ESAR,
            "es_us" => I18ns::ESUS,
            "mx" | "es_mx" => I18ns::ESMX,
            "co" | "es_co" => I18ns::ESCO,
            "pr" | "es_pr" => I18ns::ESPR,
            "de" | "de_de" => I18ns::DEDE,
            "at" | "de_at" => I18ns::DEAT,
            "ch" | "de_ch" => I18ns::DECH,
            "ru" | "ru_ru" => I18ns::RURU,
            "it" | "it_it" => I18ns::ITIT,
            "gr" | "el_GR" => I18ns::ELGR,
            "no_NO" => I18ns::NONO,
            "hu" | "hu_HU" => I18ns::HUHU,
            "tr" | "tr_TR" => I18ns::TRTR,
            "cz" | "cs_CZ" => I18ns::CSCZ,
            "sl" | "sl_SL" => I18ns::SLSL,
            "pl" | "pl_PL" => I18ns::PLPL,
            "se" | "sv_SE" => I18ns::SVSE,
            "cl" | "es_CL" => I18ns::ESCL,
            _ => I18ns::default(),
        };
        Ok(result)
    }
}

impl From<String> for I18ns {
    fn from(value: String) -> Self {
        I18ns::from_str(value.as_str()).unwrap()
    }
}

impl From<PathBuf> for I18ns {
    fn from(value: PathBuf) -> Self {
        return if let Some(path_str) = value.to_str() {
            I18ns::from_str(path_str).unwrap()
        } else {
            I18ns::default()
        };
    }
}
