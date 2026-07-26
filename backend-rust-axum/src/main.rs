use axum::{
    Router,
    routing::{
        get,
        post,
        put,
        delete,
    },
    extract::{
        Path,
        State,
    },
    Json,
    http::StatusCode,
};

use serde::{
    Serialize,
    Deserialize,
};

use sqlx::{
    PgPool,
    postgres::PgPoolOptions,
};

use dotenvy::dotenv;

use std::{
    env,
    sync::Arc,
};

use validator::{
    Validate,
};

use utoipa::{
    OpenApi,
};

use utoipa_swagger_ui::{
    SwaggerUi,
};


// ============================
// DATABASE MODEL
// ============================


#[derive(
    Serialize,
    Deserialize,
    sqlx::FromRow,
    utoipa::ToSchema
)]
struct Todo {


    id:i32,


    title:String,


    description:Option<String>,


    completed:bool,

}


// ============================
// DTOs
// ============================


#[derive(
    Deserialize,
    Validate,
    ToSchema
)]
struct CreateTodo {


    #[validate(length(min=1))]
    title:String,


    description:Option<String>,


}


#[derive(
    Deserialize,
    Validate,
    ToSchema
)]
struct UpdateTodo {


    title:Option<String>,


    description:Option<String>,


    completed:Option<bool>,


}


// ============================
// APPLICATION STATE
// ============================


#[derive(Clone)]
struct AppState {


    db:PgPool


}


// ============================
// CRUD HANDLERS
// ============================


#[utoipa::path(
    post,
    path="/todos",
    request_body=CreateTodo,
    responses(
        (status=200,body=Todo)
    )
)]
async fn create_todo(

    State(state):State<Arc<AppState>>,

    Json(payload):Json<CreateTodo>

)
-> Result<Json<Todo>,StatusCode>


{


    let todo = sqlx::query_as::<_,Todo>(
        "
        INSERT INTO todos
        (title,description)
        VALUES ($1,$2)
        RETURNING *
        "
    )


    .bind(payload.title)
    .bind(payload.description)
    .fetch_one(
        &state.db
    )
    .await
    .map_err(
        |_|StatusCode::INTERNAL_SERVER_ERROR
    )?;


    Ok(
        Json(todo)
    )

}


#[utoipa::path(
    get,
    path="/todos",
    responses(
        (status=200,body=Vec<Todo>)
    )
)]
async fn get_todos(

    State(state):State<Arc<AppState>>

)
-> Result<Json<Vec<Todo>>,StatusCode>


{


let todos =
sqlx::query_as::<_,Todo>(
    "
    SELECT *
    FROM todos
    ORDER BY id
    "
)


.fetch_all(
    &state.db
)
.await
.map_err(
    |_|StatusCode::INTERNAL_SERVER_ERROR
)?;


Ok(
    Json(todos)
)


}


#[utoipa::path(
    get,
    path="/todos/{id}",
    params(
        ("id"=i32,Path)
    ),
    responses(
        (status=200,body=Todo)
    )
)]
async fn get_todo(

    State(state):State<Arc<AppState>>,

    Path(id):Path<i32>

)
-> Result<Json<Todo>,StatusCode>


{


let todo =
sqlx::query_as::<_,Todo>(
    "
    SELECT *
    FROM todos
    WHERE id=$1
    "
)


.bind(id)
.fetch_optional(
    &state.db
)
.await
.map_err(
    |_|StatusCode::INTERNAL_SERVER_ERROR
)?
.ok_or(
    StatusCode::NOT_FOUND
)?;


Ok(
    Json(todo)
)


}


#[utoipa::path(
    put,
    path="/todos/{id}",
    request_body=UpdateTodo,
    params(
        ("id"=i32,Path)
    ),
    responses(
        (status=200,body=Todo)
    )
)]
async fn update_todo(

State(state):State<Arc<AppState>>,

Path(id):Path<i32>,

Json(payload):Json<UpdateTodo>

)
-> Result<Json<Todo>,StatusCode>


{


let todo =
sqlx::query_as::<_,Todo>(
"
UPDATE todos
SET
title =
COALESCE($1,title),
description =
COALESCE($2,description),
completed =
COALESCE($3,completed)
WHERE id=$4
RETURNING *
"


)


.bind(payload.title)
.bind(payload.description)
.bind(payload.completed)
.bind(id)
.fetch_one(
    &state.db
)
.await
.map_err(
    |_|StatusCode::NOT_FOUND
)?;


Ok(
    Json(todo)
)


}


#[utoipa::path(
    delete,
    path="/todos/{id}",
    params(
        ("id"=i32,Path)
    )
)]
async fn delete_todo(

State(state):State<Arc<AppState>>,

Path(id):Path<i32>

)
-> StatusCode


{


sqlx::query(
"
DELETE FROM todos
WHERE id=$1
"


)


.bind(id)
.execute(
    &state.db
)
.await
.unwrap();


StatusCode::NO_CONTENT


}


// ============================
// SWAGGER
// ============================


#[derive(OpenApi)]
#[openapi(

paths(

    create_todo,

    get_todos,

    get_todo,

    update_todo,

    delete_todo

),


components(

schemas(

    Todo,
    CreateTodo,
    UpdateTodo

)

)

)]
struct ApiDoc;


// ============================
// MAIN
// ============================


#[tokio::main]

async fn main()
{


dotenv().ok();


let database_url =
env::var(
    "DATABASE_URL"
)
.unwrap();


let pool =
PgPoolOptions::new()


.max_connections(10)
.connect(
    &database_url
)
.await
.unwrap();


sqlx::query(
"
CREATE TABLE IF NOT EXISTS todos
(
id SERIAL PRIMARY KEY,
title TEXT NOT NULL,
description TEXT,
completed BOOLEAN DEFAULT FALSE
)


"
)


.execute(
    &pool
)
.await
.unwrap();


let state =
Arc::new(
    AppState{
        db:pool
    }
);


let app = Router::new()
.route(
    "/todos",
    post(create_todo)
    .get(get_todos)
)


.route(
    "/todos/:id",
    get(get_todo)
    .put(update_todo)
    .delete(delete_todo)
)


.merge(

SwaggerUi::new(
    "/docs"
)


.url(
    "/api-docs/openapi.json",
    ApiDoc::openapi()
)


)


.with_state(
    state
);


let port =
env::var("PORT")
.unwrap_or(
    "8080".into()
);


let listener =
tokio::net::TcpListener::bind(
format!(
    "0.0.0.0:{}",
    port
)
)
.await
.unwrap();


println!(
    "API running on http://localhost:{}",
    port
);


println!(
    "Swagger: http://localhost:{}/docs",
    port
);


axum::serve(
    listener,
    app
)
.await
.unwrap();


}