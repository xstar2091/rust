use serde_json;

mod common;

#[tokio::test]
async fn test_scene_tag_create() {
    println!("测试：创建标签");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_create);
    println!("url:{}", url);

    let client = reqwest::Client::new();

    // 标签不存在
    for (index, item) in conf.tag_list.iter().enumerate() {
        let resp = client.put(&url).json(item).send().await.expect("Failed to send request");
        let resp_status = resp.status();
        let resp_text = resp.text().await.expect("Failed to read response body");
        let test_info = format!(
            r##"\t创建标签, index:{}, name:{}, description:{}
\t响应"
\t\tstatus:{}
\t\t  body:{}"##,
            index, item.name, item.description,
            resp_status.as_u16(),
            resp_text
        ).replace(r"\t", "\t");
        assert!(resp_status.is_success(), "{}", test_info);
    }

    // 请求体：name不存在，返回400，并返回错误信息
    let payload = serde_json::json!({"description": "a name missed test case"});
    let resp = client.put(&url).json(&payload).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info = format!(
        r##"\t创建标签: name字段缺失
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 400, "{}", test_info);
    assert!(resp_text.len() > 0, "{}", test_info);

    // 重复添加
    let resp = client.put(&url).json(&conf.tag_list[0]).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info: String = format!(
        r##"\t创建标签: 重复添加
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 400, "{}", test_info);
    assert!(resp_text.len() > 0, "{}", test_info);
}

#[tokio::test]
async fn test_scene_tag_relate() {
    println!("测试：场景打标签");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_relate);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    for item in &conf.scene_tag_relation_list {
        let resp = client.put(&url).json(item).send().await.expect("Failed to send request");
        let resp_status = resp.status();
        let resp_text = resp.text().await.expect("Failed to read response body");
        let test_info = format!(
            r##"\t请求数据:{:?},
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
            item,
            resp_status.as_u16(),
            resp_text
        ).replace(r"\t", "\t");
        assert!(resp_status.is_success(), "{}", test_info);
    }
}
