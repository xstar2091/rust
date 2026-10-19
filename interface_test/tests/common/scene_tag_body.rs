use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SceneTagListResponseBody {
    pub scene_tag_list: Vec<SceneTagListResponseItem>,
}

#[derive(Deserialize, Debug)]
pub struct SceneTagListResponseItem {
    pub id: i64,
    pub name: String,
    pub description: String,
    // pub create_time: String,
}

#[derive(Deserialize, Debug)]
pub struct TagToSceneResponseBody {
    pub scene_list: Vec<TagToSceneResponseItem>,
}

#[derive(Deserialize, Debug)]
pub struct TagToSceneResponseItem {
    pub id: i64,
}
