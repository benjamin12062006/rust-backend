use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use backend_application::user_application::dto::input::create_user_input::CreateUserInput;
use backend_application::user_application::dto::output::user_output::UserOutput;
use backend_application::user_ports::inbound::user_use_case::IUserUseCase;
use backend_application::user_ports::out::user::records::input::create_user_record::CreateUserRecord;
use backend_application::user_ports::out::user::records::output::stored_user_record::StoredUserRecord;
use backend_application::user_ports::out::user::user_repository::IUserRepository;
use backend_common::common::error::{CustomError, Result};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct FakeUserRepository {
    state: Arc<Mutex<FakeUserRepositoryState>>,
}

#[derive(Default)]
struct FakeUserRepositoryState {
    create_result: Option<Result<StoredUserRecord>>,
    list_result: Option<Result<Vec<StoredUserRecord>>>,
    get_result: Option<Result<StoredUserRecord>>,
    created_records: Vec<CreateUserRecord>,
    last_get_id: Option<Uuid>,
}

impl FakeUserRepository {
    pub fn with_create_result(self, result: Result<StoredUserRecord>) -> Self {
        self.state
            .lock()
            .expect("fake repository lock")
            .create_result = Some(result);
        self
    }

    pub fn with_list_result(self, result: Result<Vec<StoredUserRecord>>) -> Self {
        self.state.lock().expect("fake repository lock").list_result = Some(result);
        self
    }

    pub fn with_get_result(self, result: Result<StoredUserRecord>) -> Self {
        self.state.lock().expect("fake repository lock").get_result = Some(result);
        self
    }

    pub fn created_records(&self) -> Vec<CreateUserRecord> {
        self.state
            .lock()
            .expect("fake repository lock")
            .created_records
            .clone()
    }

    pub fn last_get_id(&self) -> Option<Uuid> {
        self.state.lock().expect("fake repository lock").last_get_id
    }
}

#[async_trait]
impl IUserRepository for FakeUserRepository {
    async fn create_user(&self, record: &CreateUserRecord) -> Result<StoredUserRecord> {
        let mut state = self.state.lock().expect("fake repository lock");
        state.created_records.push(record.clone());

        match state.create_result.clone() {
            Some(result) => result,
            None => Ok(StoredUserRecord {
                user_id: Uuid::from_u128(1),
                first_name: record.first_name.clone(),
                last_name: record.last_name.clone(),
                email: record.email.clone(),
                username: record.username.clone(),
                password_hash: record.password_hash.clone(),
            }),
        }
    }

    async fn list_users(&self) -> Result<Vec<StoredUserRecord>> {
        self.state
            .lock()
            .expect("fake repository lock")
            .list_result
            .clone()
            .unwrap_or_else(|| Ok(vec![]))
    }

    async fn get_user(&self, id: &Uuid) -> Result<StoredUserRecord> {
        let mut state = self.state.lock().expect("fake repository lock");
        state.last_get_id = Some(*id);
        state
            .get_result
            .clone()
            .unwrap_or_else(|| Err(CustomError::HttpError("user not found".to_string())))
    }
}

#[derive(Clone)]
pub struct FakeUserUseCase {
    state: Arc<Mutex<FakeUserUseCaseState>>,
}

struct FakeUserUseCaseState {
    create_result: Result<UserOutput>,
    list_result: Result<Vec<UserOutput>>,
    get_result: Result<UserOutput>,
    created_inputs: Vec<CreateUserInput>,
    last_get_id: Option<Uuid>,
}

impl FakeUserUseCase {
    pub fn new(output: UserOutput) -> Self {
        Self {
            state: Arc::new(Mutex::new(FakeUserUseCaseState {
                create_result: Ok(output.clone()),
                list_result: Ok(vec![output.clone()]),
                get_result: Ok(output),
                created_inputs: vec![],
                last_get_id: None,
            })),
        }
    }

    pub fn with_create_result(self, result: Result<UserOutput>) -> Self {
        self.state.lock().expect("fake use case lock").create_result = result;
        self
    }

    pub fn with_list_result(self, result: Result<Vec<UserOutput>>) -> Self {
        self.state.lock().expect("fake use case lock").list_result = result;
        self
    }

    pub fn with_get_result(self, result: Result<UserOutput>) -> Self {
        self.state.lock().expect("fake use case lock").get_result = result;
        self
    }

    pub fn created_inputs(&self) -> Vec<CreateUserInput> {
        self.state
            .lock()
            .expect("fake use case lock")
            .created_inputs
            .clone()
    }

    pub fn last_get_id(&self) -> Option<Uuid> {
        self.state.lock().expect("fake use case lock").last_get_id
    }

    pub fn into_arc(self) -> Arc<dyn IUserUseCase> {
        Arc::new(self)
    }
}

#[async_trait]
impl IUserUseCase for FakeUserUseCase {
    async fn create_user(&self, input: CreateUserInput) -> Result<UserOutput> {
        let mut state = self.state.lock().expect("fake use case lock");
        state.created_inputs.push(input);
        state.create_result.clone()
    }

    async fn list_users(&self) -> Result<Vec<UserOutput>> {
        self.state
            .lock()
            .expect("fake use case lock")
            .list_result
            .clone()
    }

    async fn get_user(&self, id: &Uuid) -> Result<UserOutput> {
        let mut state = self.state.lock().expect("fake use case lock");
        state.last_get_id = Some(*id);
        state.get_result.clone()
    }
}
