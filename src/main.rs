use biliroaming_rust_server_installer::mods::{
    build_box::{build_input_box, build_selected_option_box},
    request::{sync_download, sync_getwebpage},
    types::{BiliConfig, Lang, LangSentence},
};
use dotenv;
use std::{process::Command, io::Write};
use std::{env, fs::File};

fn main() {
    println!("Hello, welcome to Biliroaming-Rust-Server-Installer!");
    println!("This is a installer for Biliroaming-Rust-Server.");
    println!("This installer will help you to install Biliroaming-Rust-Server.");
    println!("This installer will help you to configure Biliroaming-Rust-Server.");
    let mut lang = Lang::EnUs;
    match build_selected_option_box(
        &Lang::EnUs,
        "Choose your language",
        &vec!["zh_cn".to_string(), "en_us".to_string()],
    ) {
        1 => {
            println!("You choose zh_cn");
            lang = Lang::ZhCn;
        }
        2 => println!("You choose en_us"),
        _ => panic!("choose error"),
    }

    let mut bili_config = BiliConfig::new();
    bili_config.redis = format!(
        "redis://127.0.0.1:{}",
        build_input_box(
            &lang,
            LangSentence::ChooseRedisPort.get_sentence(&lang),
            "6379"
        )
    );
    bili_config.port = build_input_box(
        &lang,
        LangSentence::ChooseHttpPort.get_sentence(&lang),
        "2662",
    )
    .parse::<u16>()
    .unwrap_or(2662);
    bili_config.worker_num = build_input_box(
        &lang,
        LangSentence::ChooseWorkerNum.get_sentence(&lang),
        "4",
    )
    .parse::<usize>()
    .unwrap_or(4);
    // ChooseCnAppPlayurlApi,
    bili_config.cn_app_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseCnAppPlayurlApi.get_sentence(&lang),
        "https://api.bilibili.com/pgc/player/api/playurl",
    );
    // ChooseHkAppPlayurlApi,
    bili_config.hk_app_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseHkAppPlayurlApi.get_sentence(&lang),
        "https://api.bilibili.com/pgc/player/api/playurl",
    );
    // ChooseTwAppPlayurlApi,
    bili_config.tw_app_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseTwAppPlayurlApi.get_sentence(&lang),
        "https://api.bilibili.com/pgc/player/api/playurl",
    );
    // ChooseThAppPlayurlApi,
    bili_config.th_app_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseThAppPlayurlApi.get_sentence(&lang),
        "https://app.biliintl.com/intl/gateway/v2/ogv/playurl",
    );
    // ChooseCnAppSearchApi,
    bili_config.cn_app_search_api = build_input_box(
        &lang,
        LangSentence::ChooseCnAppSearchApi.get_sentence(&lang),
        "https://app.bilibili.com/x/v2/search/type",
    );
    // ChooseHkAppSearchApi,
    bili_config.hk_app_search_api = build_input_box(
        &lang,
        LangSentence::ChooseHkAppSearchApi.get_sentence(&lang),
        "https://app.bilibili.com/x/v2/search/type",
    );
    // ChooseTwAppSearchApi,
    bili_config.tw_app_search_api = build_input_box(
        &lang,
        LangSentence::ChooseTwAppSearchApi.get_sentence(&lang),
        "https://app.bilibili.com/x/v2/search/type",
    );
    // ChooseThAppSearchApi,
    bili_config.th_app_search_api = build_input_box(
        &lang,
        LangSentence::ChooseThAppSearchApi.get_sentence(&lang),
        "https://app.biliintl.com/intl/gateway/v2/app/search/type",
    );
    // ChooseThAppSeasonApi,
    bili_config.th_app_season_api = build_input_box(
        &lang,
        LangSentence::ChooseThAppSeasonApi.get_sentence(&lang),
        "https://app.biliintl.com/intl/gateway/v2/ogv/view/app/season",
    );
    // ChooseCnWebPlayurlApi,
    bili_config.cn_web_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseCnWebPlayurlApi.get_sentence(&lang),
        "https://api.bilibili.com/pgc/player/web/playurl",
    );
    // ChooseHkWebPlayurlApi,
    bili_config.hk_web_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseHkWebPlayurlApi.get_sentence(&lang),
        "https://api.bilibili.com/pgc/player/web/playurl",
    );
    // ChooseTwWebPlayurlApi,
    bili_config.tw_web_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseTwWebPlayurlApi.get_sentence(&lang),
        "https://api.bilibili.com/pgc/player/web/playurl",
    );
    // ChooseThWebPlayurlApi,
    bili_config.th_web_playurl_api = build_input_box(
        &lang,
        LangSentence::ChooseThWebPlayurlApi.get_sentence(&lang),
        "https://app.biliintl.com/intl/gateway/v2/ogv/playurl",
    );
    // ChooseCnWebSearchApi,
    bili_config.cn_web_search_api = build_input_box(
        &lang,
        LangSentence::ChooseCnWebSearchApi.get_sentence(&lang),
        "https://api.bilibili.com/x/web-interface/search/type",
    );
    // ChooseHkWebSearchApi,
    bili_config.hk_web_search_api = build_input_box(
        &lang,
        LangSentence::ChooseHkWebSearchApi.get_sentence(&lang),
        "https://api.bilibili.com/x/web-interface/search/type",
    );
    // ChooseTwWebSearchApi,
    bili_config.tw_web_search_api = build_input_box(
        &lang,
        LangSentence::ChooseTwWebSearchApi.get_sentence(&lang),
        "https://api.bilibili.com/x/web-interface/search/type",
    );
    // ChooseThWebSearchApi,
    bili_config.th_web_search_api = build_input_box(
        &lang,
        LangSentence::ChooseThWebSearchApi.get_sentence(&lang),
        "https://app.biliintl.com/intl/gateway/v2/app/search/type",
    );
    // UseAutoProxy,
    let mut use_auto_proxy = build_input_box(
        &lang,
        LangSentence::UseAutoProxy.get_sentence(&lang),
        "true",
    )
    .parse::<bool>()
    .unwrap_or(true);
    let mut subscription_links = Vec::new();
    // EnterSubscriptionLink,
    if use_auto_proxy {
        loop {
            let subscription_link = build_input_box(
                &lang,
                LangSentence::EnterSubscriptionLink.get_sentence(&lang),
                "",
            );
            if !subscription_link.is_empty() {
                subscription_links.push(subscription_link);
            } else {
                break;
            }
        }
        subscription_links = subscription_links
            .into_iter()
            .filter(|link| link.starts_with("https://") || link.starts_with("http://"))
            .collect();
        if subscription_links.is_empty() {
            use_auto_proxy = false;
        }
    }

    if !use_auto_proxy {
        bili_config.cn_proxy_playurl_url =
            build_input_box(&lang, LangSentence::EnterCnProxyUrl.get_sentence(&lang), "");
        bili_config.cn_proxy_search_url = bili_config.cn_proxy_playurl_url.clone();
        if bili_config.cn_proxy_playurl_url.is_empty() {
            bili_config.cn_proxy_playurl_open = false;
            bili_config.cn_proxy_search_open = false;
        } else {
            bili_config.cn_proxy_playurl_open = true;
            bili_config.cn_proxy_search_open = true;
        }

        bili_config.hk_proxy_playurl_url =
            build_input_box(&lang, LangSentence::EnterHkProxyUrl.get_sentence(&lang), "");
        bili_config.hk_proxy_search_url = bili_config.hk_proxy_playurl_url.clone();
        if bili_config.hk_proxy_playurl_url.is_empty() {
            bili_config.hk_proxy_playurl_open = false;
            bili_config.hk_proxy_search_open = false;
        } else {
            bili_config.hk_proxy_playurl_open = true;
            bili_config.hk_proxy_search_open = true;
        }

        bili_config.tw_proxy_playurl_url =
            build_input_box(&lang, LangSentence::EnterTwProxyUrl.get_sentence(&lang), "");
        bili_config.tw_proxy_search_url = bili_config.tw_proxy_playurl_url.clone();
        if bili_config.tw_proxy_playurl_url.is_empty() {
            bili_config.tw_proxy_playurl_open = false;
            bili_config.tw_proxy_search_open = false;
        } else {
            bili_config.tw_proxy_playurl_open = true;
            bili_config.tw_proxy_search_open = true;
        }

        bili_config.th_proxy_playurl_url =
            build_input_box(&lang, LangSentence::EnterThProxyUrl.get_sentence(&lang), "");
        bili_config.th_proxy_search_url = bili_config.th_proxy_playurl_url.clone();
        bili_config.th_proxy_token_url = bili_config.th_proxy_playurl_url.clone();
        bili_config.th_proxy_subtitle_url = bili_config.th_proxy_playurl_url.clone();
        if bili_config.th_proxy_playurl_url.is_empty() {
            bili_config.th_proxy_playurl_open = false;
            bili_config.th_proxy_search_open = false;
            bili_config.th_proxy_token_open = false;
            bili_config.th_proxy_subtitle_open = false;
        } else {
            bili_config.th_proxy_playurl_open = true;
            bili_config.th_proxy_search_open = true;
            bili_config.th_proxy_token_open = true;
            bili_config.th_proxy_subtitle_open = true;
        }
    } else {
        let current_area = build_selected_option_box(
            &lang,
            LangSentence::ChooseCurrentArea.get_sentence(&lang),
            &vec![
                "CN".to_string(),
                "HK".to_string(),
                "TW".to_string(),
                "TH".to_string(),
            ],
        );
        bili_config.cn_proxy_playurl_open = true;
        bili_config.cn_proxy_search_open = true;
        bili_config.cn_proxy_playurl_url = "socks5h://127.0.0.1:2672".to_owned();
        bili_config.cn_proxy_search_url = "socks5h://127.0.0.1:2672".to_owned();
        bili_config.hk_proxy_playurl_open = true;
        bili_config.hk_proxy_search_open = true;
        bili_config.hk_proxy_playurl_url = "socks5h://127.0.0.1:2674".to_owned();
        bili_config.hk_proxy_search_url = "socks5h://127.0.0.1:2674".to_owned();
        bili_config.tw_proxy_playurl_open = true;
        bili_config.tw_proxy_search_open = true;
        bili_config.tw_proxy_playurl_url = "socks5h://127.0.0.1:2682".to_owned();
        bili_config.tw_proxy_search_url = "socks5h://127.0.0.1:2682".to_owned();
        bili_config.th_proxy_playurl_open = true;
        bili_config.th_proxy_search_open = true;
        bili_config.th_proxy_token_open = true;
        bili_config.th_proxy_subtitle_open = true;
        bili_config.th_proxy_playurl_url = "socks5h://127.0.0.1:2678".to_owned();
        bili_config.th_proxy_search_url = "socks5h://127.0.0.1:2678".to_owned();
        bili_config.th_proxy_token_url = "socks5h://127.0.0.1:2678".to_owned();
        bili_config.th_proxy_subtitle_url = "socks5h://127.0.0.1:2678".to_owned();
        match current_area {
            1 => {
                bili_config.cn_proxy_playurl_open = false;
                bili_config.cn_proxy_search_open = false;
            }
            2 => {
                bili_config.hk_proxy_playurl_open = false;
                bili_config.hk_proxy_search_open = false;
            }
            3 => {
                bili_config.tw_proxy_playurl_open = false;
                bili_config.tw_proxy_search_open = false;
            }
            4 => {
                bili_config.th_proxy_playurl_open = false;
                bili_config.th_proxy_search_open = false;
                bili_config.th_proxy_token_open = false;
                bili_config.th_proxy_subtitle_open = false;
            }
            _ => panic!("unknown area"),
        }
    };
    bili_config.aid_replace_open = build_selected_option_box(
        &lang,
        LangSentence::ChooseAidReplace.get_sentence(&lang),
        &vec![
            LangSentence::True.get_sentence(&lang).to_owned(),
            LangSentence::False.get_sentence(&lang).to_owned(),
        ],
    ) == 1;
    if bili_config.aid_replace_open {
        bili_config.aid = build_input_box(&lang, LangSentence::EnterAid.get_sentence(&lang), "")
            .parse::<u64>()
            .unwrap_or(0);
        if bili_config.aid == 0 {
            bili_config.aid_replace_open = false;
        }
    }
    bili_config.donate_url = build_input_box(
        &lang,
        LangSentence::EnterDonationLink.get_sentence(&lang),
        "",
    );

    dotenv::from_path("/etc/os-release").unwrap();
    if let Ok(id) = env::var("ID") {
        match &id[..] {
            "debian" | "ubuntu" | "devuan" => {
                Command::new("apt")
                    .args(["update"])
                    .status()
                    .expect("");
                Command::new("apt")
                    .args(["upgrade", "-y"])
                    .status()
                    .expect("");
                Command::new("apt-get")
                    .args(["install","redis","unzip","-y"])
                    .status()
                    .expect("");
            }
            "centos" | "fedora" | "rhel" => {
                Command::new("yum").args(["update","-y"]).status().expect("");
                Command::new("yum").args(["install","redis","unzip","-y"])
                    .status()
                    .expect("");
            }
            _ => {
                panic!("Unknown OS.");
            }
        }
    } else {
        panic!("Unknown OS.");
    }
    {
        let latest_server_info = if let Ok(value) = sync_getwebpage(
            "https://api.github.com/repos/pchpub/BiliRoaming-Rust-Server/releases/latest",
            "BiliRoaming-Rust-Server-Installer",
            "",
            None,
        ) {
            value.1
        } else {
            panic!("Failed to get latest server info.");
        };
        let latest_server_info_json: serde_json::Value =
            serde_json::from_str(latest_server_info.as_str()).unwrap();
        for asset in latest_server_info_json["assets"].as_array().unwrap() {
            let download_url = asset["browser_download_url"].as_str().unwrap();
            let download_file_name = asset["name"].as_str().unwrap();
            let download_file_path = format!("/tmp/{}", download_file_name);
            sync_download(download_url, download_file_path).unwrap();
        }
    }
    Command::new("mkdir")
        .arg("/opt/BiliRoaming-Rust-Server")
        .status()
        .expect("");
    Command::new("mv")
    .args(["/tmp/BiliRoaming-Rust-Server","/opt/BiliRoaming-Rust-Server/BiliRoaming-Rust-Server"])
    .status()
    .expect("");
    Command::new("sudo")
        .args(["chmod", "+x", "/opt/BiliRoaming-Rust-Server/BiliRoaming-Rust-Server"])
        .status()
        .expect("");
    serde_json::to_writer_pretty(
        File::create("/opt/BiliRoaming-Rust-Server/config.json").unwrap(),
        &bili_config,
    )
    .unwrap();
    if use_auto_proxy {
        let latest_server_info = if let Ok(value) = sync_getwebpage(
            "https://api.github.com/repos/pchpub/bili-sub-filter/releases/latest",
            "BiliRoaming-Rust-Server-Installer",
            "",
            None,
        ) {
            value.1
        } else {
            panic!("Failed to get latest bili-sub-filter info.");
        };
        let latest_server_info_json: serde_json::Value =
            serde_json::from_str(latest_server_info.as_str()).unwrap();
        for asset in latest_server_info_json["assets"].as_array().unwrap() {
            let download_url = asset["browser_download_url"].as_str().unwrap();
            let download_file_name = asset["name"].as_str().unwrap();
            let download_file_path = format!("/tmp/{}", download_file_name);
            sync_download(download_url, download_file_path).unwrap();
        }
        Command::new("mkdir")
            .args(["-p", "/opt/bili-sub-filter"])
            .status()
            .expect("");
        Command::new("unzip").args(["-C","/tmp/bili-sub-filter.zip","-d","/opt/bili-sub-filter/"]).status().expect("");
        let mut bili_sub_filter_config: serde_json::Value = serde_json::from_reader(File::open("/opt/bili-sub-filter/config.json").unwrap()).unwrap();
        for sub in subscription_links {
            bili_sub_filter_config["subs"].as_array_mut().unwrap().push(serde_json::Value::String(sub));
        }
        serde_json::to_writer_pretty(File::create("/opt/bili-sub-filter/config.json").unwrap(), &bili_sub_filter_config).unwrap();
        let mut file = std::fs::File::create("/etc/systemd/system/bili-sub-filter.service").expect("create failed");
        file.write_all(r#"[Unit]
Description=Bili Sub Filter
After=network.target
[Install]
WantedBy=multi-user.target
[Service]
Type=simple
WorkingDirectory=/opt/bili-sub-filter
ExecStart=/opt/bili-sub-filter/bili-sub-filter
Restart=always
ExecStop=/usr/bin/kill -2 $MAINPID
StandardOutput=file:/opt/bili-sub-filter/bili-sub-filter.log"#.as_bytes()).unwrap();
    }
    let mut file = std::fs::File::create("/etc/systemd/system/biliroaming_rust_server.service").expect("create failed");
        file.write_all(r#"[Unit]
Description=Biliroaming Rust Server
After=network.target
[Install]
WantedBy=multi-user.target
[Service]
Type=simple
WorkingDirectory=/opt/BiliRoaming-Rust-Server
ExecStart=/opt/BiliRoaming-Rust-Server/biliroaming_rust_server
Restart=always
ExecStop=/usr/bin/kill -2 $MAINPID
StandardOutput=file:/opt/BiliRoaming-Rust-Server/biliroaming_rust_server.log"#.as_bytes()).unwrap();
    Command::new("systemctl").arg("daemon-reload").status().expect("");
    Command::new("systemctl").arg("enable biliroaming_rust_server").status().expect("");
    Command::new("systemctl").args(["start","biliroaming_rust_server"]).status().expect("");
    if use_auto_proxy {
        Command::new("systemctl").arg("enable bili-sub-filter").status().expect("");
        Command::new("systemctl").args(["start","bili-sub-filter"]).status().expect("");
    }
    println!("BiliRoaming Rust Server installed successfully.");
    println!("{}http://127.0.0.1:{}",LangSentence::ReverseProxy.get_sentence(&lang),bili_config.port);
}
