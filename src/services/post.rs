use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

use crate::{app_writer::AppResult, db::DB, dtos::post::*, entities::*};
use crate::db::get_db;
use crate::dtos::category::CategoryResponse;
use crate::entities::prelude::{Category, Post};

pub async fn add_post(req: PostAddRequest) -> AppResult<PostResponse> {
    let db = get_db();
    let model = post::ActiveModel {
        id: NotSet,
        title: Set(req.title.clone()),
        content: Set(req.content.clone()),
        category_id: Set(req.category_id.clone()),
    };
    let result = Post::insert(model).exec(db).await?;
    Ok(PostResponse {
        id: result.last_insert_id,
        title: req.title,
        content: req.content,
        category_id: req.category_id,
        category: None,
    })
}

pub async fn update_post(req: PostUpdateRequest) -> AppResult<PostResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = Post::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("Post does not exist.").into());
    }
    let mut model: post::ActiveModel = find.unwrap().into();

    model.title = Set(req.title);
    model.content = Set(req.content);
    model.category_id = Set(req.category_id);


    let result: post::Model = model.update(db).await?;

    Ok(PostResponse {
        id: result.id,
        title: result.title,
        content: result.content,
        category_id: result.category_id,
        category: None,
    })
}

pub async fn delete_post(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    Post::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn post_find_all() -> AppResult<Vec<PostResponse>> {
    let db = get_db();

    let test: Vec<(post::Model, Vec<category::Model>)> = Post::find().find_with_related(Category).all(db).await?;
    let map: Vec<PostResponse> = test.iter().map(|(p, cs)| {
        let mut post: PostResponse = p.into();
        post.category = Some(cs.get(0).unwrap().into());
        post
    }).collect();
    Ok(map)
}

///使用ID查询博客
pub async fn find_blog_by_id(id: i32) -> AppResult<PostResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let post = Post::find_by_id(id).find_also_related(category::Entity).one(db).await?;
    match post {
        None => {
            Err(anyhow::anyhow!("Post does not exist.").into())
        }
        Some(post) => {
            let cate = post.1.unwrap();
            let blog = post.0;
            Ok(PostResponse {
                id: blog.id,
                title: blog.title,
                content: blog.content,
                category_id: blog.category_id,
                category: Some(CategoryResponse {
                    id: cate.id,
                    name: cate.name,
                }),
            })
        }
    }
}


