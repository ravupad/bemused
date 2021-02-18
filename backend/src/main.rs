use crate::configuration::Configuration;
use crate::error::Error;
use crate::state::State;
use std::sync::Arc;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Error>;
type Request = hyper::Request<hyper::Body>;

// mod db;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new("Config.toml");
    let state = State::new(&configuration).await.expect("server could not be initialized");
    start(state, configuration.port).await;
}

mod logger {
    use crate::configuration::Configuration;
    use slog::o;
    use slog::Drain;
    use slog::Logger;
    use slog_async::Async;
    use slog_term::FullFormat;
    use slog_term::PlainDecorator;
    use std::fs::OpenOptions;
    
    pub fn get_logger(configuration: &Configuration) -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .append(true)
            .open(&configuration.log_file)
            .unwrap();
        let decorator = PlainDecorator::new(file);
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let async_drain = match configuration.terminal_log {
            false => Async::new(drain).build().fuse(),
            true => {
                let term_decorator = slog_term::TermDecorator::new().build();
                let term_drain = FullFormat::new(term_decorator).build();
                let drain = slog::Duplicate(drain, term_drain).fuse();
                Async::new(drain).build().fuse()
            }
        };
        Logger::root(async_drain, o!())
    }
}

async fn start(state: State, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let state = Arc::new(state);
    hyper::Server::bind(&addr).serve(make_service_fn(move |_| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |request| {
                let state = state.clone();
                async move {
                    let result = state::router(&state, request, &mut 0).await;
                    Ok::<_, Infallible>(response::result(result))
                }
            }))
        }
    })).await.unwrap();
}

mod request {
    use crate::error::Error;
    use crate::Result;
    use crate::Request;
    use futures::stream::StreamExt;

    pub fn path<'a>(request: &'a Request, path_offset: &mut usize) -> Result<&'a str> {
        let path = request.uri().path();
        let separator = '/' as u8;
        let mut start = *path_offset;
        while start < path.len() && path.as_bytes()[start] == separator {
            start += 1;
        }
        if start >= path.len() {
            return Err(Error::PageNotFound);
        }
        let mut end = start + 1;
        while end < path.len() && path.as_bytes()[end] != separator {
            end += 1;
        }
        *path_offset = end;
        unsafe {
            Ok(path.get_unchecked(start..end))
        }
    }

    
    pub async fn body(request: &mut Request) -> Result<Vec<u8>> {
        request.body_mut().fold(Ok(Vec::new()), |acc_res, chunk_res| async move {
            acc_res.and_then(move |mut acc| chunk_res.map(move |chunk| {
                acc.extend_from_slice(&chunk);
                acc
            }))
        }).await.map_err(Error::from)
    }
}

mod response {
    use hyper::Response;
    use hyper::Body;
    use hyper::StatusCode;
    use crate::error::Error;
    use crate::Result;
    
    pub fn error(err: Error) -> Response<Body> {
        let status_code = match err {
            Error::SessionIdHeaderMissing |
            Error::SessionDoesNotExist |
            Error::Authentication => StatusCode::UNAUTHORIZED,
            Error::PageNotFound => StatusCode::NOT_FOUND,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };
        let body = match err {
            Error::Internal(e) => format!("Internal Server Error: {}", e.to_string()),
            error => serde_json::to_string(&error).unwrap_or("Could not create message".to_owned())
        };
        Response::builder()
            .header("Content-Type", "application/json")
            .status(status_code)
            .body(Body::from(body))
            .unwrap()
    }

    pub fn void<T>(_: T) -> Response<Body> {
        Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap()
    }

    pub fn result(result: Result<Response<Body>>) -> Response<Body> {
        match result {
            Ok(body) => body,
            Err(err) => error(err)
        }
    }

    pub fn json<T: serde::Serialize>(body: T) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap()
    }
}

mod configuration {
    use std::fs::File;
    use std::io::Read;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Configuration {
        pub port: u16,
        pub sled_path: String,
        pub postgres: String,
        pub log_file: String,
        pub terminal_log: bool,
        pub static_path: String,
    }

    impl Configuration {
        pub fn new(configuration_path: &str) -> Self {
            let mut file = File::open(configuration_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        }
    }
}

mod utils {
    use crate::Result;
    use crate::Error;
    
    pub fn bc_de<'de, T>(t: &'de [u8]) -> Result<T>
    where T: serde::Deserialize<'de>
    {
        bincode::deserialize(t).map_err(Error::from)
    }

    pub fn bc_se<T>(t: &T) -> Result<Vec<u8>>
    where T: serde::Serialize
    {
        bincode::serialize(t).map_err(Error::from)
    }
    
    pub mod sled {
        pub mod in_transaction {
            use crate::Error;
            use sled::transaction::TransactionalTree;
            use sled::transaction::ConflictableTransactionResult;
            use sled::transaction::ConflictableTransactionError::Abort;
            
            pub fn generate_id(tree: &TransactionalTree) -> ConflictableTransactionResult<u64, Error> {
                tree.generate_id().map_err(|e| Abort(Error::from(e)))
            }

            pub fn serialize<T>(t: &T) -> ConflictableTransactionResult<Vec<u8>, Error>
            where T: serde::Serialize
            {
                bincode::serialize(t).map_err(|e| Abort(Error::from(e)))
            }
        }
    }
}

mod error {
    use std::error::Error as StdError;
    use serde::Serialize;
    use std::convert::From;

    #[derive(Serialize, Debug)]
    #[serde(tag = "error_code", content = "message")]
    pub enum Error {
        UsernameAlreadyExists,
        Authentication,
        PageNotFound,
        UsernameNotFound,
        EntityDoesNotExist,
        PasswordDoesNotMatch,
        SessionIdHeaderMissing,
        SessionDoesNotExist,
        SessionIdParse,
        #[serde(skip)]
        Internal(Box<dyn StdError + Send>),
    }

    impl<T: StdError + Send + 'static> From<T> for Error {
        fn from(error: T) -> Self {
            Error::Internal(Box::new(error))
        }
    }

    impl Error {
        pub fn sled(error: sled::transaction::TransactionError<Self>) -> Self {
            match error {
                sled::transaction::TransactionError::Storage(e) => Error::Internal(Box::new(e)),
                sled::transaction::TransactionError::Abort(e) => e,
            }
        }
    }
}

mod state {
    use crate::Error;
    use crate::Result;
    use crate::configuration::Configuration;
    use crate::logger;
    use slog::info;
    use crate::user;
    use crate::task;
    use crate::Request;
    use crate::request::path;
    use hyper::Body;
    use hyper::Response;
    use crate::user::Repository as UserRepository;
    use crate::task::Repository as TaskRepository;

    pub struct State {
        pub user: UserRepository,
        pub task: TaskRepository,
    }

    impl State {
        pub async fn new(config: &Configuration) -> Result<Self> {
            let logger = logger::get_logger(&config);
            let sled_db = sled::open(&config.sled_path)?;
            info!(logger, "opened sled database");            
            let user = UserRepository::new(&sled_db)?;
            let task = TaskRepository::new(&sled_db)?;
            Ok(State {
                user,
                task,
            })
        }
    }

    pub async fn router(state: &State, request: Request, path_offset: &mut usize) -> Result<Response<Body>> {
        match path(&request, path_offset)? {
            "api" => {
                match path(&request, path_offset)? {
                    "user" => user::router(state, request, path_offset).await,
                    "task" => task::router(state, request, path_offset).await,
                    _ => Err(Error::PageNotFound),
                }
            }
            _ => Err(Error::PageNotFound),
        }
    }
}

mod user {
    use crate::error::Error;
    use crate::Result;
    use crate::utils::sled::in_transaction as it;
    use crate::request::path;
    use crate::Request;
    use crate::response;
    use crate::state::State as MainState;
    use hyper::Method;
    use hyper::Body;
    use hyper::Response;
    use uuid::Uuid;
    use std::convert::TryInto;
    use sled::Tree;
    use sled::transaction::Transactional;
    use sled::transaction::abort;
    use serde::Serialize;
    use serde::Deserialize;
    
    #[derive(Serialize, Deserialize)]
    struct User {
        username: String,
        password: String,
    }
    
    pub struct Repository {
        main: Tree,
        username: Tree,
        session: Tree,
    }
    
    const USER_MAIN_TABLE: &'static str = "user_main";
    const USER_INDEX_USERNAME: &'static str = "user_idx_username";
    const USER_SESSION_TABLE: &'static str = "user_session";
    const SESSION_ID_HEADER: &'static str = "SessionId";

    impl Repository {
        pub fn new(db: &sled::Db) -> Result<Self> {
            Ok(Repository {
                main: db.open_tree(USER_MAIN_TABLE).map_err(Error::from)?,
                username: db.open_tree(USER_INDEX_USERNAME).map_err(Error::from)?,
                session: db.open_tree(USER_SESSION_TABLE).map_err(Error::from)?,
            })
        }

        fn save(&self, user: &User) -> Result<u64> {
            (&self.main, &self.username).transaction(|(main_tx, username_tx)| {
                let user_id = it::generate_id(main_tx)?;
                let key = it::serialize(&user_id)?;
                let value = it::serialize(user)?;
                if username_tx.insert(user.username.as_bytes(), &key[..])?.is_some() {
                    return abort(Error::UsernameAlreadyExists);
                }
                main_tx.insert(&key[..], value)?;
                Ok(user_id)
            }).map_err(Error::sled)
        }

        fn create_session(&self, username: &str, password: &str) -> Result<Uuid> {
            let user_id = self.username.get(username.as_bytes())
                .map_err(Error::from)?
                .ok_or(Error::UsernameNotFound)?;
            let user_raw = self.main.get(&user_id)
                .map_err(Error::from)?
                .ok_or(Error::UsernameNotFound)?;
            let user: User = bincode::deserialize(&user_raw)
                .map_err(Error::from)?;
            if user.password != password {
                return Err(Error::PasswordDoesNotMatch);
            }
            let session_id = Uuid::new_v4();
            self.session.insert(session_id.as_bytes(), user_id)
                .map_err(Error::from)?;
            Ok(session_id)
        }

        fn delete_session(&self, session_id: Uuid) -> Result<()> {
            self.session.remove(session_id.as_bytes())
                .map_err(Error::from)
                .map(|_| ())
        }

        fn username_available(&self, username: &str) -> Result<bool> {
            self.username.contains_key(username.as_bytes()).map_err(Error::from)
        }

        fn get_session_user_id(&self, session_id: Uuid) -> Result<u64> {
            let val = self.session.get(session_id.as_bytes())
                .map_err(Error::from)?
                .ok_or(Error::SessionDoesNotExist)?;
            Ok(u64::from_be_bytes(val.as_ref().try_into().unwrap()))
        }
    }

    pub fn id(repository: &Repository, request: &Request) -> Result<u64> {
        let session_id_str = request.headers().get(SESSION_ID_HEADER)
            .ok_or(Error::SessionIdHeaderMissing)?
            .to_str()
            .map_err(|_| Error::SessionIdParse)?;
        let session_id = Uuid::parse_str(session_id_str)
            .map_err(|_| Error::SessionIdParse)?;
        repository.get_session_user_id(session_id)
    }
    
    pub async fn router(state: &MainState, request: Request, path_offset: &mut usize) -> Result<Response<Body>> {
        match *request.method() {
            Method::PUT => {
                let mut user = User {
                    username: path(&request, path_offset)?.to_string(),
                    password: path(&request, path_offset)?.to_string(),
                };
                state.user.save(&mut user)
                    .map(response::void)
            }
            Method::POST => {
                let username = path(&request, path_offset)?;
                let password = path(&request, path_offset)?;
                state.user.create_session(username, password)
                    .map(|uuid| uuid.to_string())
                    .map(response::json)
            }
            Method::DELETE => {
                let session_id = path(&request, path_offset)?;
                let uuid = Uuid::parse_str(session_id).map_err(Error::from)?;
                state.user.delete_session(uuid)
                    .map(response::void)
            }
            Method::GET => {
                match path(&request, path_offset) {
                    Ok("available") => state.user.username_available(path(&request, path_offset)?).map(response::json),
                    _ => id(&state.user, &request).map(response::json),
                }
            }
            _ => Err(Error::PageNotFound),
        }
    }
}

mod task {
    use crate::error::Error;
    use crate::request::path;
    use crate::request::body;
    use crate::response;
    use crate::user;
    use crate::Result;
    use crate::state::State as MainState;
    use crate::Request;
    use crate::utils::sled::in_transaction as it;
    use crate::utils;
    use http::Method;
    use hyper::Body;
    use hyper::Response;
    use serde::Serialize;
    use serde::Deserialize;
    use chrono::{DateTime, Utc};
    use sled::Tree;
    use std::ops::Add;

    #[derive(Serialize, Deserialize)]
    pub enum RepeatUnit {
        Day,
        Week,
        Month,
        Year,
    }

    #[derive(Serialize, Deserialize)]
    pub enum RepeatBehavior {
        FromCompleted,
        FromScheduledInFuture,
        FromScheduled,
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct Task {
        pub text: String,
        pub note: String,
        pub category: String,
        pub at: DateTime<Utc>,
        pub repeat_value: u64,
        pub repeat_unit: RepeatUnit,
        pub repeat_behavior: RepeatBehavior,
        pub completed: bool,
    }

    const SLED_MAIN: &'static str = "task_main";

    pub struct Repository {
        main: Tree,
    }

    impl Repository {
        pub fn new(sled_db: &sled::Db) -> Result<Self> {
            Ok(Repository {
                main: sled_db.open_tree(SLED_MAIN)?,
            })
        }

        pub fn save(&self, user_id: u64, task: &Task) -> Result<u64> {
            self.main.transaction(|main| {
                let id = it::generate_id(main)?;
                let key = it::serialize(&(&user_id, &id))?;
                let value = it::serialize(task)?;
                main.insert(key, value)?;
                Ok(id)
            }).map_err(Error::sled)
        }

        pub fn find_by_id(&self, user_id: u64, id: u64) -> Result<Task> {
            let key = utils::bc_se(&(user_id, id))?;
            let value = self.main.get(key)?.ok_or(Error::EntityDoesNotExist)?;
            Ok(utils::bc_de(&value)?)
        }

        pub fn find_by_user_id(&self, user_id: u64) -> Result<Vec<(u64, Task)>> {
            let mut res = Vec::new();
            for item in self.main.scan_prefix(utils::bc_se(&user_id)?) {
                let (key, value) = item?;
                let key: (u64, u64) = utils::bc_de(&key)?;
                let value: Task = utils::bc_de(&value)?;
                res.push((key.1, value));
            }
            Ok(res)
        }

        pub fn update(&self, user_id: u64, id: u64, task: &Task) -> Result<()> {
            let key = utils::bc_se(&(user_id, id))?;
            let value = utils::bc_se(task)?;
            self.main.insert(key, value).map_err(Error::from).map(|_| ())
        }

        pub fn delete(&self, user_id: u64, id: u64) -> Result<()> {
            let key = utils::bc_se(&(user_id, id))?;
            self.main.remove(key)?.ok_or(Error::EntityDoesNotExist).map(|_| ())
        }
    }

    fn complete_task(repository: &Repository, user_id: u64, id: u64) -> Result<Option<DateTime<Utc>>> {
        let mut task = repository.find_by_id(user_id, id)?;
        match task.repeat_value {
            0 => {
                if task.completed != true {
                    task.completed = true;
                    repository.update(user_id, id, &task)?;
                }
                Ok(None)
            }
            value => {
                let duration = match task.repeat_unit {
                    RepeatUnit::Day => chrono::Duration::days(value as i64),
                    RepeatUnit::Week => chrono::Duration::weeks(value as i64),
                    RepeatUnit::Month => chrono::Duration::days((value*365/12) as i64),
                    RepeatUnit::Year => chrono::Duration::days((value*365) as i64),
                };
                task.at = match task.repeat_behavior {
                    RepeatBehavior::FromCompleted => Utc::now().add(duration),
                    RepeatBehavior::FromScheduled => task.at.add(duration),
                    RepeatBehavior::FromScheduledInFuture => {
                        let now = Utc::now();
                        let mut time = task.at + duration;
                        while (now - time).num_milliseconds() > 0 {
                            time = time + duration;
                        }
                        time
                    }
                };
                repository.update(user_id, id, &task)?;
                Ok(Some(task.at))
            }
        }
    }
    
    pub async fn router(state: &MainState, mut request: Request, path_offset: &mut usize) -> Result<Response<Body>> {
        match *request.method() {
            Method::PUT => {
                let user_id = user::id(&state.user, &request)?;
                let body = body(&mut request).await?;                
                let task = serde_json::from_slice(&body)?;
                state.task.save(user_id, &task).map(response::json)
            }
            Method::GET => {
                let user_id = user::id(&state.user, &request)?;
                state.task.find_by_user_id(user_id).map(response::json)
            }
            Method::POST => {
                let user_id = user::id(&state.user, &request)?;
                let id = path(&request, path_offset)?.parse()?;
                let body = body(&mut request).await?;
                let task = serde_json::from_slice(&body)?;
                state.task.update(user_id, id, &task).map(response::void)
            }
            Method::DELETE => {
                let task_id = path(&request, path_offset)?.parse()?;
                let user_id = user::id(&state.user, &request)?;
                state.task.delete(user_id, task_id).map(response::void)
            }
            Method::PATCH => {
                let id = path(&request, path_offset)?.parse()?;
                match path(&request, path_offset)? {
                    "complete" => {
                        let user_id = user::id(&state.user, &request)?;
                        complete_task(&state.task, user_id, id).map(response::json)
                    }
                    _ => Err(Error::PageNotFound),
                }
            }
            _ => Err(Error::PageNotFound),
        }
    }
}

