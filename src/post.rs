pub type Posts = Response<Post>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
}

impl Post {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostRequest {
    pub message: Option<String>,
}

impl PostRequest {
    pub fn to_post(&self) -> Option<Post> {
        match &self.message {
            Some(message) => Some(Post::new(message.to_string())),
            None => None,
        }
    }
}

#[get("/posts")]
pub async fn list() -> HttpResponse {
    let posts = Posts { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(posts)
}

#[post("/posts")]
pub async fn create(post_req: Json<PostRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(post_req.to_post())
}

#[get("/posts/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    let found_post: Option<Post> = None;

    match found_post {
        Some(post) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(post),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

#[delete("/posts/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}