use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BiliConfig {
    pub config_version: u16,
    pub redis: String,
    pub worker_num: usize,
    pub http_port: u16,
    pub https_port: u16,
    pub https_support: bool,
    pub http2https_support: bool,
    pub limit_biliroaming_version_open: bool,
    pub limit_biliroaming_version_min: u16,
    pub limit_biliroaming_version_max: u16,
    pub rate_limit_per_second: u64,
    pub rate_limit_burst: u32,
    pub cn_app_playurl_api: String,
    pub tw_app_playurl_api: String,
    pub hk_app_playurl_api: String,
    pub th_app_playurl_api: String,
    pub cn_web_playurl_api: String,
    pub tw_web_playurl_api: String,
    pub hk_web_playurl_api: String,
    pub th_web_playurl_api: String,
    pub cn_app_search_api: String,
    pub tw_app_search_api: String,
    pub hk_app_search_api: String,
    pub th_app_search_api: String,
    pub cn_web_search_api: String,
    pub tw_web_search_api: String,
    pub hk_web_search_api: String,
    pub th_web_search_api: String,
    pub th_app_season_api: String,
    pub th_app_season_sub_api: String,
    pub th_app_season_sub_name: String,
    pub th_app_season_sub_open: bool,
    pub cn_proxy_playurl_url: String,
    pub hk_proxy_playurl_url: String,
    pub tw_proxy_playurl_url: String,
    pub th_proxy_playurl_url: String,
    pub cn_proxy_playurl_open: bool,
    pub hk_proxy_playurl_open: bool,
    pub tw_proxy_playurl_open: bool,
    pub th_proxy_playurl_open: bool,
    pub cn_proxy_search_url: String,
    pub hk_proxy_search_url: String,
    pub tw_proxy_search_url: String,
    pub th_proxy_search_url: String,
    pub cn_proxy_search_open: bool,
    pub hk_proxy_search_open: bool,
    pub tw_proxy_search_open: bool,
    pub th_proxy_search_open: bool,
    pub cn_proxy_token_url: String,
    pub th_proxy_token_url: String,
    pub cn_proxy_token_open: bool,
    pub th_proxy_token_open: bool,
    pub cn_proxy_accesskey_url: String,
    pub cn_proxy_accesskey_open: bool,
    pub th_proxy_subtitle_url: String,
    pub th_proxy_subtitle_open: bool,
    pub general_api_bilibili_com_proxy_api: String,
    pub general_app_bilibili_com_proxy_api: String,
    pub aid: u64,
    pub aid_replace_open: bool,
    pub resign_pub: HashMap<String, bool>,
    pub resign_open: HashMap<String, bool>,
    pub resign_from_existed_key: bool,
    pub resign_from_api_open: HashMap<String, bool>, //启用后assesskey从api获取
    pub resign_api: HashMap<String, String>,
    pub resign_api_sign: HashMap<String, String>,
    pub cache: HashMap<String, u64>,
    pub local_wblist: HashMap<String, (bool, bool)>,
    #[serde(default)]
    pub blacklist_config: BlackListType,
    pub appsearch_remake: HashMap<String, String>,
    pub websearch_remake: HashMap<String, String>,
    pub donate_url: String,
    pub api_assesskey_open: HashMap<String, bool>, //api是否暴露
    pub report_open: bool,
    #[serde(default)]
    pub report_config: ReportConfig,
    pub area_cache_open: bool,
    // 以下为不会序列化的配置
    #[serde(skip_serializing, default)]
    pub cn_resign_info: UserResignInfo,
    #[serde(skip_serializing, default)]
    pub th_resign_info: UserResignInfo,
}

impl BiliConfig {
    pub fn new() -> Self {
        Self {
            config_version: 4,
            redis: "".to_owned(),
            worker_num: 4,
            http_port: 0,
            https_port: 0,
            https_support: false,
            http2https_support: false,
            limit_biliroaming_version_open: false,
            limit_biliroaming_version_min: 0,
            limit_biliroaming_version_max: 32767,
            rate_limit_per_second: 3,
            rate_limit_burst: 20,
            cn_app_playurl_api: "".to_owned(),
            tw_app_playurl_api: "".to_owned(),
            hk_app_playurl_api: "".to_owned(),
            th_app_playurl_api: "".to_owned(),
            cn_web_playurl_api: "".to_owned(),
            tw_web_playurl_api: "".to_owned(),
            hk_web_playurl_api: "".to_owned(),
            th_web_playurl_api: "".to_owned(),
            cn_app_search_api: "".to_owned(),
            tw_app_search_api: "".to_owned(),
            hk_app_search_api: "".to_owned(),
            th_app_search_api: "".to_owned(),
            cn_web_search_api: "".to_owned(),
            tw_web_search_api: "".to_owned(),
            hk_web_search_api: "".to_owned(),
            th_web_search_api: "".to_owned(),
            th_app_season_api: "".to_owned(),
            th_app_season_sub_api: "".to_owned(),
            th_app_season_sub_name: "".to_owned(),
            th_app_season_sub_open: false,
            cn_proxy_playurl_url: "".to_owned(),
            hk_proxy_playurl_url: "".to_owned(),
            tw_proxy_playurl_url: "".to_owned(),
            th_proxy_playurl_url: "".to_owned(),
            cn_proxy_playurl_open: false,
            hk_proxy_playurl_open: false,
            tw_proxy_playurl_open: false,
            th_proxy_playurl_open: false,
            cn_proxy_search_url: "".to_owned(),
            hk_proxy_search_url: "".to_owned(),
            tw_proxy_search_url: "".to_owned(),
            th_proxy_search_url: "".to_owned(),
            cn_proxy_search_open: false,
            hk_proxy_search_open: false,
            tw_proxy_search_open: false,
            th_proxy_search_open: false,
            cn_proxy_token_url: "".to_owned(),
            th_proxy_token_url: "".to_owned(),
            cn_proxy_token_open: false,
            th_proxy_token_open: false,
            cn_proxy_accesskey_url: "".to_owned(),
            cn_proxy_accesskey_open: false,
            th_proxy_subtitle_url: "".to_owned(),
            th_proxy_subtitle_open: false,
            general_api_bilibili_com_proxy_api: "api.bilibili.com".to_owned(),
            general_app_bilibili_com_proxy_api: "app.bilibili.com".to_owned(),
            aid: 0,
            aid_replace_open: false,
            resign_pub: HashMap::from([
                ("1".to_owned(), false),
                ("2".to_owned(), false),
                ("3".to_owned(), false),
                ("4".to_owned(), false),
            ]),
            resign_open: HashMap::from([
                ("1".to_owned(), false),
                ("2".to_owned(), false),
                ("3".to_owned(), false),
                ("4".to_owned(), false),
            ]),
            resign_from_existed_key: false,
            resign_from_api_open: HashMap::from([
                ("1".to_owned(), false),
                ("2".to_owned(), false),
                ("3".to_owned(), false),
                ("4".to_owned(), false),
            ]),
            resign_api: HashMap::from([
                ("1".to_owned(), "".to_owned()),
                ("2".to_owned(), "".to_owned()),
                ("3".to_owned(), "".to_owned()),
                ("4".to_owned(), "".to_owned()),
            ]),
            resign_api_sign: HashMap::from([
                ("1".to_owned(), "".to_owned()),
                ("2".to_owned(), "".to_owned()),
                ("3".to_owned(), "".to_owned()),
                ("4".to_owned(), "".to_owned()),
            ]),
            cache: HashMap::from([
                ("0".to_owned(), 6480),
                ("-412".to_owned(), 1380),
                ("other".to_owned(), 1380),
                ("-404".to_owned(), 1380),
                ("-10403".to_owned(), 6480),
                ("thsub".to_owned(), 14400),
            ]),
            local_wblist: HashMap::new(),
            blacklist_config: BlackListType::MixedBlackList(OnlineBlackListConfig {
                api: "https://black.qimo.ink/api/users/".to_string(),
                api_version: 2,
            }),
            appsearch_remake: HashMap::new(),
            websearch_remake: HashMap::new(),
            donate_url: "".to_owned(),
            api_assesskey_open: HashMap::from([
                ("1".to_owned(), false),
                ("2".to_owned(), false),
                ("3".to_owned(), false),
                ("4".to_owned(), false),
            ]),
            report_open: false,
            report_config: ReportConfig::default(),
            area_cache_open: true,
            cn_resign_info: UserResignInfo::default(),
            th_resign_info: UserResignInfo::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BlackListType {
    OnlyLocalBlackList,
    OnlyOnlineBlackList(OnlineBlackListConfig),
    MixedBlackList(OnlineBlackListConfig),
    NoOnlineBlacklist,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OnlineBlackListConfig {
    pub api: String,
    pub api_version: u16, //暂时没用，以后向后兼容的时候会用到
}

impl std::default::Default for BlackListType {
    fn default() -> Self {
        Self::MixedBlackList(OnlineBlackListConfig {
            api: "https://black.qimo.ink/api/users/".to_string(),
            api_version: 2,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ReportConfig {
    TgBot(ReportConfigTgBot),
    PushPlus(ReportConfigPushplus),
    Custom(ReportConfigCustom),
}

impl std::default::Default for ReportConfig {
    fn default() -> Self {
        Self::TgBot(ReportConfigTgBot::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportConfigTgBot {
    pub tg_bot_token: String,
    pub tg_chat_id: String,
    pub tg_proxy_api_open: bool,
    pub tg_proxy_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportConfigPushplus {
    pub pushplus_push_title: String,
    pub pushplus_secret: String,
    pub pushplus_group_id: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportConfigCustom {
    pub method: ReportConfigCustomRequestMethod,
    pub url: String,
    pub content: String,
    pub proxy_open: bool,
    pub proxy_url: String,
}

impl std::default::Default for ReportConfigTgBot {
    fn default() -> Self {
        Self {
            tg_bot_token: String::new(),
            tg_chat_id: String::new(),
            tg_proxy_api_open: false,
            tg_proxy_url: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
enum ReportConfigCustomOrderName {
    CnPlayurl,
    HkPlayurl,
    TwPlayurl,
    ThPlayurl,
    CnSearch,
    HkSearch,
    TwSearch,
    ThSearch,
    ThSeason,
    ChangedAreaName,
    ChangedDataType,
    ChangedHealthType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ReportConfigCustomRequestMethod {
    Get,
    Post,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResignInfo {
    pub access_key: String,
    pub refresh_token: String,
    pub expire_time: u64,
}

impl Default for UserResignInfo {
    fn default() -> Self {
        Self {
            access_key: Default::default(),
            refresh_token: Default::default(),
            expire_time: Default::default(),
        }
    }
}

// lang 配置

pub enum Lang {
    ZhCn,
    EnUs,
}

pub enum LangSentence {
    ChooseRedisPort,
    ChooseHttpPort,
    ChooseWorkerNum,
    ChooseLimitBiliroamingVersionOpen,
    ChooseCnAppPlayurlApi,
    ChooseHkAppPlayurlApi,
    ChooseTwAppPlayurlApi,
    ChooseThAppPlayurlApi,
    ChooseCnAppSearchApi,
    ChooseHkAppSearchApi,
    ChooseTwAppSearchApi,
    ChooseThAppSearchApi,
    ChooseThAppSeasonApi,
    ChooseCnWebPlayurlApi,
    ChooseHkWebPlayurlApi,
    ChooseTwWebPlayurlApi,
    ChooseThWebPlayurlApi,
    ChooseCnWebSearchApi,
    ChooseHkWebSearchApi,
    ChooseTwWebSearchApi,
    ChooseThWebSearchApi,
    ChooseThAppSeasonSubApi,
    UseAutoProxy,
    EnterSubscriptionLink,
    SubscriptionLinkInvalid,
    EnterCnProxyUrl,
    EnterHkProxyUrl,
    EnterTwProxyUrl,
    EnterThProxyUrl,
    EnterTgBotToken,
    EnterTgChatId,
    EnterTgProxyUrl,
    ChooseAidReplace,
    EnterAid,
    EnterDonationLink,
    ChooseCurrentArea,
    Cn,
    Hk,
    Tw,
    Th,
    True,
    False,
    ReverseProxy,
    UseAutoHttps,
    EnterWebSiteName,
    ChooseHttpsPort,
    MassageAfterInstallHttpOnly,
    MassageAfterInstallHttps,
}

impl LangSentence {
    pub fn get_sentence(&self, lang: &Lang) -> &str {
        match lang {
            Lang::ZhCn => match self {
                Self::ChooseRedisPort => "请输入Redis端口号",
                Self::ChooseHttpPort => "请输入Http端口号",
                Self::ChooseWorkerNum => "请输入工作线程数",
                Self::ChooseLimitBiliroamingVersionOpen => "是否限制BiliRoaming版本",
                Self::ChooseCnAppPlayurlApi => "请输入国内App播放地址API",
                Self::ChooseHkAppPlayurlApi => "请输入香港App播放地址API",
                Self::ChooseTwAppPlayurlApi => "请输入台湾App播放地址API",
                Self::ChooseThAppPlayurlApi => "请输入泰国App播放地址API",
                Self::ChooseCnAppSearchApi => "请输入国内App搜索API",
                Self::ChooseHkAppSearchApi => "请输入香港App搜索API",
                Self::ChooseTwAppSearchApi => "请输入台湾App搜索API",
                Self::ChooseThAppSearchApi => "请输入泰国App搜索API",
                Self::ChooseThAppSeasonApi => "请输入泰国App番剧API",
                Self::ChooseCnWebPlayurlApi => "请输入国内Web播放地址API",
                Self::ChooseHkWebPlayurlApi => "请输入香港Web播放地址API",
                Self::ChooseTwWebPlayurlApi => "请输入台湾Web播放地址API",
                Self::ChooseThWebPlayurlApi => "请输入泰国Web播放地址API",
                Self::ChooseCnWebSearchApi => "请输入国内Web搜索API",
                Self::ChooseHkWebSearchApi => "请输入香港Web搜索API",
                Self::ChooseTwWebSearchApi => "请输入台湾Web搜索API",
                Self::ChooseThWebSearchApi => "请输入泰国Web搜索API",
                Self::ChooseThAppSeasonSubApi => "请输入泰国App番剧Season API",
                Self::UseAutoProxy => "是否使用自动代理",
                Self::EnterSubscriptionLink => "请输入订阅链接(按回车键结束)",
                Self::EnterCnProxyUrl => "请输入国内代理地址",
                Self::EnterHkProxyUrl => "请输入香港代理地址",
                Self::EnterTwProxyUrl => "请输入台湾代理地址",
                Self::EnterThProxyUrl => "请输入泰国代理地址",
                Self::EnterTgBotToken => "请输入Telegram Bot Token",
                Self::EnterTgChatId => "请输入Telegram Chat ID",
                Self::EnterTgProxyUrl => "请输入Telegram代理地址",
                Self::ChooseAidReplace => "是否替换Aid",
                Self::EnterAid => "请输入Aid",
                Self::EnterDonationLink => "请输入捐赠链接",
                Self::SubscriptionLinkInvalid => "订阅链接无效",
                Self::ChooseCurrentArea => "请选择当前区域",
                Self::Cn => "国内",
                Self::Hk => "香港",
                Self::Tw => "台湾",
                Self::Th => "东南亚",
                Self::True => "是",
                Self::False => "否",
                Self::ReverseProxy => "服务器已经开启, 请反代: ",
                Self::UseAutoHttps => "是否使用自动HTTPS",
                Self::EnterWebSiteName => "请输入网站域名",
                Self::ChooseHttpsPort => "请输入HTTPS端口号\n注意: 请确保80端口和443没有被占用\n此项为true时会覆写https_port和http_port\n此项为true时不用安装nginx",
                Self::MassageAfterInstallHttpOnly => "安装完成, 请反代: ",
                Self::MassageAfterInstallHttps => "安装完成, 您的解析服务器地址为: ",
            },
            Lang::EnUs => match self {
                Self::ChooseRedisPort => "Please enter the Redis port number",
                Self::ChooseHttpPort => "Please enter the Http port number",
                Self::ChooseWorkerNum => "Please enter the number of working threads",
                Self::ChooseLimitBiliroamingVersionOpen => "Limit BiliRoaming version",
                Self::ChooseCnAppPlayurlApi => "Please enter the Chinese App playback address API",
                Self::ChooseHkAppPlayurlApi => {
                    "Please enter the Hong Kong App playback address API"
                }
                Self::ChooseTwAppPlayurlApi => "Please enter the Taiwan App playback address API",
                Self::ChooseThAppPlayurlApi => "Please enter the Thai App playback address API",
                Self::ChooseCnAppSearchApi => "Please enter the Chinese App search API",
                Self::ChooseHkAppSearchApi => "Please enter the Hong Kong App search API",
                Self::ChooseTwAppSearchApi => "Please enter the Taiwan App search API",
                Self::ChooseThAppSearchApi => "Please enter the Thai App search API",
                Self::ChooseThAppSeasonApi => "Please enter the Thai App season API",
                Self::ChooseCnWebPlayurlApi => "Please enter the Chinese Web playback address API",
                Self::ChooseHkWebPlayurlApi => {
                    "Please enter the Hong Kong Web playback address API"
                }
                Self::ChooseTwWebPlayurlApi => "Please enter the Taiwan Web playback address API",
                Self::ChooseThWebPlayurlApi => "Please enter the Thai Web playback address API",
                Self::ChooseCnWebSearchApi => "Please enter the Chinese Web search API",
                Self::ChooseHkWebSearchApi => "Please enter the Hong Kong Web search API",
                Self::ChooseTwWebSearchApi => "Please enter the Taiwan Web search API",
                Self::ChooseThWebSearchApi => "Please enter the Thai Web search API",
                Self::ChooseThAppSeasonSubApi => "Please enter the Thai App season sub API",
                Self::UseAutoProxy => "Use automatic proxy",
                Self::EnterSubscriptionLink => "Please enter the subscription link(End with Enter)",
                Self::EnterCnProxyUrl => "Please enter the Chinese proxy address",
                Self::EnterHkProxyUrl => "Please enter the Hong Kong proxy address",
                Self::EnterTwProxyUrl => "Please enter the Taiwan proxy address",
                Self::EnterThProxyUrl => "Please enter the Thai proxy address",
                Self::EnterTgBotToken => "Please enter the Telegram Bot Token",
                Self::EnterTgChatId => "Please enter the Telegram Chat ID",
                Self::EnterTgProxyUrl => "Please enter the Telegram proxy address",
                Self::ChooseAidReplace => "Replace Aid",
                Self::EnterAid => "Please enter the Aid",
                Self::EnterDonationLink => "Please enter the donation link",
                Self::SubscriptionLinkInvalid => "Subscription link invalid",
                Self::ChooseCurrentArea => "Please choose the current area",
                Self::Cn => "China",
                Self::Hk => "Hong Kong",
                Self::Tw => "Taiwan",
                Self::Th => "Southeast Asia",
                Self::True => "Yes",
                Self::False => "No",
                Self::ReverseProxy => "The server has been started, please reverse proxy: ",
                Self::UseAutoHttps => "Use automatic HTTPS",
                Self::EnterWebSiteName => "Please enter the website domain name",
                Self::ChooseHttpsPort => "Please enter the HTTPS port number\nNote: Please make sure that port 80 and 443 are not occupied\nThis item will overwrite https_port and http_port when true\nThis item does not need to install nginx when true",
                Self::MassageAfterInstallHttpOnly => "Installation is complete, please reverse proxy: ",
                Self::MassageAfterInstallHttps => "Installation is complete, your server address is: ",
            },
        }
    }
}
