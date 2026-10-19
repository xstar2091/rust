use common::scene_tag_body::{SceneTagListResponseBody, TagToSceneResponseBody};
use serde_json;
use std::collections::HashSet;

mod common;

#[tokio::test]
async fn test_scene_tag() {
    test_all_before().await;
    test_scene_tag_create().await;
    test_scene_tag_list_all().await;
    test_scene_tag_list_like().await;
    insert_scene_list().await;
    test_scene_tag_relate().await;
    test_from_scene_to_tag().await;
    test_from_tag_to_scene_tag_id_list().await;
    test_from_tag_to_scene_tag_name_head().await;
    test_from_tag_to_scene_tag_name_middle().await;
    test_from_tag_to_scene_tag_name_tail().await;
    test_scene_tag_update_name().await;
    test_scene_tag_update_description().await;
    test_scene_tag_update_name_description().await;
    test_scene_tag_delete().await;
    test_scene_tag_list_after_delete_all().await;
    test_scene_tag_relation_list_after_delete_all().await;
    test_all_after().await;
}

async fn test_all_before() {
    println!("测试开始前：初始化数据库");
    let conf = common::config::get_global_config();

    let (client, connection) = tokio_postgres::connect(
        &conf.database_connection_string,
        tokio_postgres::NoTls
    ).await.expect("Failed to connect to Postgres database connection");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("Database connection error: {}", e);
        }
    });

    client.execute(r##"CALL "public"."scene_tag_interface_test_before"();"##, &[])
        .await.expect("Failed to call \"scene_tag_interface_test_before\".");
}

async fn test_all_after() {
    println!("测试结束后");
    let conf = common::config::get_global_config();

    let (client, connection) = tokio_postgres::connect(
        &conf.database_connection_string,
        tokio_postgres::NoTls
    ).await.expect("Failed to connect to Postgres database connection");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("Database connection error: {}", e);
        }
    });

    client.execute(r##"CALL "public"."scene_tag_interface_test_after"();"##, &[])
        .await.expect("Failed to call \"scene_tag_interface_test_after\".");
}

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

async fn test_scene_tag_list_all() {
    println!("测试：读取整个标签列表");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_list);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info: String = format!(
        r##"\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 200, "{}", test_info);

    let body: SceneTagListResponseBody = serde_json::from_str(&resp_text).expect("Failed to parse json");
    assert_eq!(body.scene_tag_list.len(), conf.tag_list.len(), "读取到的标签数量和插入的标签数量不匹配");
    for (index, item) in conf.tag_list.iter().enumerate() {
        assert_eq!(item.name, body.scene_tag_list[index].name);
        assert_eq!(item.description, body.scene_tag_list[index].description);
    }
}

async fn test_scene_tag_list_like() {
    println!("测试：标签列表模糊查询");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_list);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let resp = client.get(&url).query(&[("name", "雪")]).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info: String = format!(
        r##"\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 200, "{}", test_info);

    let body: SceneTagListResponseBody = serde_json::from_str(&resp_text).expect("Failed to parse json");

    let mut map: HashSet<String> = HashSet::new();
    for item in &conf.tag_list {
        if item.name.contains("雪") {
            map.insert(item.name.clone());
        }
    }
    assert_eq!(map.len(), body.scene_tag_list.len());

    for item in &body.scene_tag_list {
        assert!(map.contains(&item.name));
    }
}

async fn insert_scene_list() {
    println!("测试数据准备：插入场景");
    let conf = common::config::get_global_config();

    let (client, connection) = tokio_postgres::connect(
        &conf.database_connection_string,
        tokio_postgres::NoTls
    ).await.expect("Failed to connect to Postgres database connection");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("Database connection error: {}", e);
        }
    });

    for item in &conf.scene_list {
        client.execute(
            "INSERT INTO scene_list (name) VALUES ($1)",
            &[&item.name]
        ).await.expect("Failed to insert scene list");
    }

    let rows = client.query(
        "SELECT id, name FROM scene_list WHERE id <= 100 ORDER BY id ASC",
        &[]
    ).await.expect("Failed to select rows");
    assert_eq!(rows.len(), conf.scene_list.len());

    for (index, row) in rows.iter().enumerate() {
        let id: i32 = row.get("id");
        let name: &str = row.get("name");
        assert_eq!(id, conf.scene_list[index].id);
        assert_eq!(name, &conf.scene_list[index].name);
    }
}

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

async fn test_from_scene_to_tag() {
    println!("测试：已知场景查询有哪些标签");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.from_scene_to_tag);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    for item in &conf.scene_tag_relation_list {
        let resp = client.get(&url).query(&[("scene_id", item.scene_id)]).send().await.expect("Failed to send request");
        let resp_status = resp.status();
        let resp_text = resp.text().await.expect("Failed to read response body");
        let test_info = format!(
            r##"\t场景id:{},
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
            item.scene_id,
            resp_status.as_u16(),
            resp_text
        ).replace(r"\t", "\t");
        assert_eq!(resp_status.as_u16(), 200, "{}", test_info);
        let body = serde_json::from_str::<SceneTagListResponseBody>(resp_text.as_str()).expect("Failed to parse json");
        for relation in &body.scene_tag_list {
            assert!(item.tag_id_list.contains(&relation.id));
        }
    }
}

async fn test_from_tag_to_scene_tag_id_list() {
    println!("测试：根据标签筛选场景(根据标签id列表)");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.from_tag_to_scene);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let query_id_list_str = "[6, 7]";
    let resp = client.get(&url).query(&[("tag_id_list", query_id_list_str)]).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info = format!(
        r##"\t标签id列表:{},
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        query_id_list_str,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 200, "{}", test_info);
    let body = serde_json::from_str::<TagToSceneResponseBody>(&resp_text).expect("Failed to parse json");
    let map: HashSet<i64> = vec![1, 2, 3, 4].into_iter().collect();
    assert_eq!(map.len(), body.scene_list.len());
    for item in body.scene_list {
        assert!(map.contains(&item.id));
    }
}

async fn test_from_tag_to_scene_tag_name_head() {
    println!("测试：根据标签筛选场景(根据标签名称，关键在开头)");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.from_tag_to_scene);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let query_id_list_str = "降";
    let resp = client.get(&url).query(&[("tag_name", query_id_list_str)]).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info = format!(
        r##"\t标签id:{},
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        query_id_list_str,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 200, "{}", test_info);
    let body = serde_json::from_str::<TagToSceneResponseBody>(&resp_text).expect("Failed to parse json");
    let map: HashSet<i64> = vec![1, 2, 3, 4].into_iter().collect();
    assert_eq!(map.len(), body.scene_list.len());
    for item in body.scene_list {
        assert!(map.contains(&item.id));
    }
}

async fn test_from_tag_to_scene_tag_name_middle() {
    println!("测试：根据标签筛选场景(根据标签名称，关键在中间)");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.from_tag_to_scene);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let query_id_list_str = "雪";
    let resp = client.get(&url).query(&[("tag_name", query_id_list_str)]).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info = format!(
        r##"\t标签id:{},
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        query_id_list_str,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 200, "{}", test_info);
    let body = serde_json::from_str::<TagToSceneResponseBody>(&resp_text).expect("Failed to parse json");
    let map: HashSet<i64> = vec![1, 2, 3, 4, 5, 6, 9].into_iter().collect();
    assert_eq!(map.len(), body.scene_list.len());
    for item in body.scene_list {
        assert!(map.contains(&item.id));
    }
}

async fn test_from_tag_to_scene_tag_name_tail() {
    println!("测试：根据标签筛选场景(根据标签名称，关键在末尾)");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.from_tag_to_scene);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let query_id_list_str = "量";
    let resp = client.get(&url).query(&[("tag_name", query_id_list_str)]).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    let test_info = format!(
        r##"\t标签id:{},
\t响应
\t\tstatus:{},
\t\t  body:{}"##,
        query_id_list_str,
        resp_status.as_u16(),
        resp_text
    ).replace(r"\t", "\t");
    assert_eq!(resp_status.as_u16(), 200, "{}", test_info);
    let body = serde_json::from_str::<TagToSceneResponseBody>(&resp_text).expect("Failed to parse json");
    let map: HashSet<i64> = vec![1, 2, 3, 4, 6, 8].into_iter().collect();
    assert_eq!(map.len(), body.scene_list.len());
    for item in body.scene_list {
        assert!(map.contains(&item.id));
    }
}

async fn test_scene_tag_update_name() {
    println!("测试：更新标签名称");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_update);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let tag_id: i64 = 1;
    let new_tag_name = "test-update-tag-name";
    let payload = serde_json::json!({"id": tag_id, "name": new_tag_name});
    let resp = client.put(&url).json(&payload).send().await.expect("Failed to send request");
    assert!(resp.status().is_success());

    let scene_tag_list_get_url = format!("{}{}", conf.host, conf.url_path.scene_tag_list);
    let resp = client.get(&scene_tag_list_get_url).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    assert!(resp_status.is_success());
    let body = serde_json::from_str::<SceneTagListResponseBody>(&resp_text).expect("Failed to parse json");
    let mut count = 0;
    for item in body.scene_tag_list {
        if item.id == tag_id {
            count += 1;
            assert_eq!(item.name, new_tag_name);
        }
    }
    assert_eq!(count, 1);
}

async fn test_scene_tag_update_description() {
    println!("测试：更新标签名称");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_update);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let tag_id: i64 = 1;
    let new_tag_description = "test-update-tag-description";
    let payload = serde_json::json!({"id": tag_id, "description": new_tag_description});
    let resp = client.put(&url).json(&payload).send().await.expect("Failed to send request");
    assert!(resp.status().is_success());

    let scene_tag_list_get_url = format!("{}{}", conf.host, conf.url_path.scene_tag_list);
    let resp = client.get(&scene_tag_list_get_url).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    assert!(resp_status.is_success());
    let body = serde_json::from_str::<SceneTagListResponseBody>(&resp_text).expect("Failed to parse json");
    let mut count = 0;
    for item in body.scene_tag_list {
        if item.id == tag_id {
            count += 1;
            assert_eq!(item.description, new_tag_description);
        }
    }
    assert_eq!(count, 1);
}

async fn test_scene_tag_update_name_description() {
    println!("测试：更新标签名称");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_update);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let tag_id: i64 = 1;
    let new_tag_name = "test-update-tag-name-xxx";
    let new_tag_description = "test-update-tag-description-xxx";
    let payload = serde_json::json!({"id": tag_id, "name": new_tag_name, "description": new_tag_description});
    let resp = client.put(&url).json(&payload).send().await.expect("Failed to send request");
    assert!(resp.status().is_success());

    let scene_tag_list_get_url = format!("{}{}", conf.host, conf.url_path.scene_tag_list);
    let resp = client.get(&scene_tag_list_get_url).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    assert!(resp_status.is_success());
    let body = serde_json::from_str::<SceneTagListResponseBody>(&resp_text).expect("Failed to parse json");
    let mut count = 0;
    for item in body.scene_tag_list {
        if item.id == tag_id {
            count += 1;
            assert_eq!(item.name, new_tag_name);
            assert_eq!(item.description, new_tag_description);
        }
    }
    assert_eq!(count, 1);
}

async fn test_scene_tag_delete() {
    println!("测试：删除");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_with_id);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    for item in &conf.tag_list {
        let url = format!("{}{}/{}", conf.host, conf.url_path.scene_tag_with_id, item.id);
        let resp = client.delete(&url).send().await.expect("Failed to send request");
        assert!(resp.status().is_success());
    }
}

async fn test_scene_tag_list_after_delete_all() {
    println!("测试：删除后标签列表应为空");
    let conf = common::config::get_global_config();
    let url = format!("{}{}", conf.host, conf.url_path.scene_tag_list);
    println!("url:{}", url);

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await.expect("Failed to send request");
    let resp_status = resp.status();
    let resp_text = resp.text().await.expect("Failed to read response body");
    assert!(resp_status.is_success());
    let body = serde_json::from_str::<SceneTagListResponseBody>(&resp_text).expect("Failed to parse json");
    assert!(body.scene_tag_list.is_empty());
}

async fn test_scene_tag_relation_list_after_delete_all() {
    println!("测试：删除后场景-标签关系表应为空");
    let conf = common::config::get_global_config();

    let (client, connection) = tokio_postgres::connect(
        &conf.database_connection_string,
        tokio_postgres::NoTls
    ).await.expect("Failed to connect to Postgres database connection");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("Database connection error: {}", e);
        }
    });

    let rows = client.query_one("SELECT COUNT(*) FROM scene_tag_relation", &[]).await.expect("Failed to execute query");
    let count: i64 = rows.get(0);
    assert_eq!(count, 0);
}
