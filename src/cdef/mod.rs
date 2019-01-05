use std::os::raw::{c_char, c_int, c_void};

mod bindgen_test;
mod rime_struct;

pub use self::rime_struct::RimeSizedStruct;

pub type RimeSessionId = usize;
pub type CBool = c_int;

/// Rime doesn't provide documentation most of the time
/// Good luck guessing the usage
#[repr(C)]
pub struct RimeApi {
    pub data_size: c_int,
    #[doc = " setup"]
    #[doc = "  Call this function before accessing any other API functions."]
    pub setup: Option<unsafe extern "C" fn(traits: *mut RimeTraits)>,
    #[doc = " Set up the notification callbacks"]
    #[doc = "  Receive notifications"]
    #[doc = "  - on loading schema:"]
    #[doc = "    + message_type=\"schema\", message_value=\"luna_pinyin/Luna Pinyin\""]
    #[doc = "  - on changing mode:"]
    #[doc = "    + message_type=\"option\", message_value=\"ascii_mode\""]
    #[doc = "    + message_type=\"option\", message_value=\"!ascii_mode\""]
    #[doc = "  - on deployment:"]
    #[doc = "    + session_id = 0, message_type=\"deploy\", message_value=\"start\""]
    #[doc = "    + session_id = 0, message_type=\"deploy\", message_value=\"success\""]
    #[doc = "    + session_id = 0, message_type=\"deploy\", message_value=\"failure\""]
    #[doc = ""]
    #[doc = "  handler will be called with context_object as the first parameter"]
    #[doc = "  every time an event occurs in librime, until RimeFinalize() is called."]
    #[doc = "  when handler is NULL, notification is disabled."]
    pub set_notification_handler:
        Option<unsafe extern "C" fn(handler: RimeNotificationHandler, context_object: *mut c_void)>,
    pub initialize: Option<unsafe extern "C" fn(traits: *mut RimeTraits)>,
    /// Rime doesn't tell us what it does exactly but fcitx calls it when stopping Rime
    pub finalize: Option<unsafe extern "C" fn()>,
    pub start_maintenance: Option<unsafe extern "C" fn(full_check: CBool) -> CBool>,
    pub is_maintenance_mode: Option<unsafe extern "C" fn() -> CBool>,
    pub join_maintenance_thread: Option<unsafe extern "C" fn()>,
    pub deployer_initialize: Option<unsafe extern "C" fn(traits: *mut RimeTraits)>,
    pub prebuild: Option<unsafe extern "C" fn() -> CBool>,
    pub deploy: Option<unsafe extern "C" fn() -> CBool>,
    pub deploy_schema: Option<unsafe extern "C" fn(schema_file: *const c_char) -> CBool>,
    pub deploy_config_file:
        Option<unsafe extern "C" fn(file_name: *const c_char, version_key: *const c_char) -> CBool>,
    pub sync_user_data: Option<unsafe extern "C" fn() -> CBool>,
    pub create_session: Option<unsafe extern "C" fn() -> RimeSessionId>,
    pub find_session: Option<unsafe extern "C" fn(session_id: RimeSessionId) -> CBool>,
    pub destroy_session: Option<unsafe extern "C" fn(session_id: RimeSessionId) -> CBool>,
    pub cleanup_stale_sessions: Option<unsafe extern "C" fn()>,
    pub cleanup_all_sessions: Option<unsafe extern "C" fn()>,
    pub process_key: Option<
        unsafe extern "C" fn(session_id: RimeSessionId, keycode: c_int, mask: c_int) -> CBool,
    >,
    pub commit_composition: Option<unsafe extern "C" fn(session_id: RimeSessionId) -> CBool>,
    pub clear_composition: Option<unsafe extern "C" fn(session_id: RimeSessionId)>,
    pub get_commit:
        Option<unsafe extern "C" fn(session_id: RimeSessionId, commit: *mut RimeCommit) -> CBool>,
    pub free_commit: Option<unsafe extern "C" fn(commit: *mut RimeCommit) -> CBool>,
    pub get_context:
        Option<unsafe extern "C" fn(session_id: RimeSessionId, context: *mut RimeContext) -> CBool>,
    pub free_context: Option<unsafe extern "C" fn(ctx: *mut RimeContext) -> CBool>,
    pub get_status:
        Option<unsafe extern "C" fn(session_id: RimeSessionId, status: *mut RimeStatus) -> CBool>,
    pub free_status: Option<unsafe extern "C" fn(status: *mut RimeStatus) -> CBool>,
    pub set_option: Option<
        unsafe extern "C" fn(session_id: RimeSessionId, option: *const c_char, value: CBool),
    >,
    pub get_option:
        Option<unsafe extern "C" fn(session_id: RimeSessionId, option: *const c_char) -> CBool>,
    pub set_property: Option<
        unsafe extern "C" fn(session_id: RimeSessionId, prop: *const c_char, value: *const c_char),
    >,
    pub get_property: Option<
        unsafe extern "C" fn(
            session_id: RimeSessionId,
            prop: *const c_char,
            value: *mut c_char,
            buffer_size: usize,
        ) -> CBool,
    >,
    pub get_schema_list: Option<unsafe extern "C" fn(schema_list: *mut RimeSchemaList) -> CBool>,
    pub free_schema_list: Option<unsafe extern "C" fn(schema_list: *mut RimeSchemaList)>,
    pub get_current_schema: Option<
        unsafe extern "C" fn(
            session_id: RimeSessionId,
            schema_id: *mut c_char,
            buffer_size: usize,
        ) -> CBool,
    >,
    pub select_schema:
        Option<unsafe extern "C" fn(session_id: RimeSessionId, schema_id: *const c_char) -> CBool>,
    pub schema_open:
        Option<unsafe extern "C" fn(schema_id: *const c_char, config: *mut RimeConfig) -> CBool>,
    pub config_open:
        Option<unsafe extern "C" fn(config_id: *const c_char, config: *mut RimeConfig) -> CBool>,
    pub config_close: Option<unsafe extern "C" fn(config: *mut RimeConfig) -> CBool>,
    pub config_get_bool: Option<
        unsafe extern "C" fn(
            config: *mut RimeConfig,
            key: *const c_char,
            value: *mut CBool,
        ) -> CBool,
    >,
    pub config_get_int: Option<
        unsafe extern "C" fn(
            config: *mut RimeConfig,
            key: *const c_char,
            value: *mut c_int,
        ) -> CBool,
    >,
    pub config_get_double: Option<
        unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char, value: *mut f64) -> CBool,
    >,
    pub config_get_string: Option<
        unsafe extern "C" fn(
            config: *mut RimeConfig,
            key: *const c_char,
            value: *mut c_char,
            buffer_size: usize,
        ) -> CBool,
    >,
    pub config_get_cstring:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char) -> *const c_char>,
    pub config_update_signature:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, signer: *const c_char) -> CBool>,
    pub config_begin_map: Option<
        unsafe extern "C" fn(
            iterator: *mut RimeConfigIterator,
            config: *mut RimeConfig,
            key: *const c_char,
        ) -> CBool,
    >,
    pub config_next: Option<unsafe extern "C" fn(iterator: *mut RimeConfigIterator) -> CBool>,
    pub config_end: Option<unsafe extern "C" fn(iterator: *mut RimeConfigIterator)>,
    pub simulate_key_sequence: Option<
        unsafe extern "C" fn(session_id: RimeSessionId, key_sequence: *const c_char) -> CBool,
    >,
    pub register_module: Option<unsafe extern "C" fn(module: *mut RimeModule) -> CBool>,
    pub find_module: Option<unsafe extern "C" fn(module_name: *const c_char) -> *mut RimeModule>,
    pub run_task: Option<unsafe extern "C" fn(task_name: *const c_char) -> CBool>,
    pub get_shared_data_dir: Option<unsafe extern "C" fn() -> *const c_char>,
    pub get_user_data_dir: Option<unsafe extern "C" fn() -> *const c_char>,
    pub get_sync_dir: Option<unsafe extern "C" fn() -> *const c_char>,
    pub get_user_id: Option<unsafe extern "C" fn() -> *const c_char>,
    pub get_user_data_sync_dir: Option<unsafe extern "C" fn(dir: *mut c_char, buffer_size: usize)>,
    #[doc = "! initialize an empty config object"]
    #[doc = "*!"]
    #[doc = "* should call config_close() to free the object"]
    #[doc = "*/"]
    pub config_init: Option<unsafe extern "C" fn(config: *mut RimeConfig) -> CBool>,
    #[doc = "! deserialize config from a yaml string"]
    pub config_load_string:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, yaml: *const c_char) -> CBool>,
    pub config_set_bool: Option<
        unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char, value: CBool) -> CBool,
    >,
    pub config_set_int: Option<
        unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char, value: c_int) -> CBool,
    >,
    pub config_set_double: Option<
        unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char, value: f64) -> CBool,
    >,
    pub config_set_string: Option<
        unsafe extern "C" fn(
            config: *mut RimeConfig,
            key: *const c_char,
            value: *const c_char,
        ) -> CBool,
    >,
    pub config_get_item: Option<
        unsafe extern "C" fn(
            config: *mut RimeConfig,
            key: *const c_char,
            value: *mut RimeConfig,
        ) -> CBool,
    >,
    pub config_set_item: Option<
        unsafe extern "C" fn(
            config: *mut RimeConfig,
            key: *const c_char,
            value: *mut RimeConfig,
        ) -> CBool,
    >,
    pub config_clear:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char) -> CBool>,
    pub config_create_list:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char) -> CBool>,
    pub config_create_map:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char) -> CBool>,
    pub config_list_size:
        Option<unsafe extern "C" fn(config: *mut RimeConfig, key: *const c_char) -> usize>,
    pub config_begin_list: Option<
        unsafe extern "C" fn(
            iterator: *mut RimeConfigIterator,
            config: *mut RimeConfig,
            key: *const c_char,
        ) -> CBool,
    >,
    #[doc = "! get raw input"]
    #[doc = "*!"]
    #[doc = "*  NULL is returned if session does not exist."]
    #[doc = "*  the returned pointer to input string will become invalid upon editing."]
    #[doc = "*/"]
    pub get_input: Option<unsafe extern "C" fn(session_id: RimeSessionId) -> *const c_char>,
    #[doc = "! caret posistion in terms of raw input"]
    pub get_caret_pos: Option<unsafe extern "C" fn(session_id: RimeSessionId) -> usize>,
    #[doc = "! select a candidate from current page"]
    pub select_candidate:
        Option<unsafe extern "C" fn(session_id: RimeSessionId, index: usize) -> CBool>,
    #[doc = "! get the version of librime"]
    pub get_version: Option<unsafe extern "C" fn() -> *const c_char>,
}

#[link(name = "rime")]
#[allow(dead_code)]
extern "C" {
    #[doc = "! API entry"]
    #[doc = "*!"]
    #[doc = "*  Acquire the version controlled RimeApi structure."]
    #[doc = "*/"]
    pub fn rime_get_api() -> *mut RimeApi;
    #[doc = "  Call this function before accessing any other API."]
    pub fn RimeSetup(traits: *mut RimeTraits);
    #[doc = "! Receive notifications"]
    #[doc = "*!"]
    #[doc = "* - on loading schema:"]
    #[doc = "*   + message_type=\"schema\", message_value=\"luna_pinyin/Luna Pinyin\""]
    #[doc = "* - on changing mode:"]
    #[doc = "*   + message_type=\"option\", message_value=\"ascii_mode\""]
    #[doc = "*   + message_type=\"option\", message_value=\"!ascii_mode\""]
    #[doc = "* - on deployment:"]
    #[doc = "*   + session_id = 0, message_type=\"deploy\", message_value=\"start\""]
    #[doc = "*   + session_id = 0, message_type=\"deploy\", message_value=\"success\""]
    #[doc = "*   + session_id = 0, message_type=\"deploy\", message_value=\"failure\""]
    #[doc = "*"]
    #[doc = "*   handler will be called with context_object as the first parameter"]
    #[doc = "*   every time an event occurs in librime, until RimeFinalize() is called."]
    #[doc = "*   when handler is NULL, notification is disabled."]
    #[doc = "*/"]
    pub fn RimeSetNotificationHandler(
        handler: RimeNotificationHandler,
        context_object: *mut c_void,
    );
    pub fn RimeInitialize(traits: *mut RimeTraits);
    pub fn RimeFinalize();
    pub fn RimeStartMaintenance(full_check: CBool) -> CBool;
    pub fn RimeDeployerInitialize(traits: *mut RimeTraits);
    pub fn RimePrebuildAllSchemas() -> CBool;
    pub fn RimeDeployWorkspace() -> CBool;
    pub fn RimeDeploySchema(schema_file: *const c_char) -> CBool;
    pub fn RimeDeployConfigFile(file_name: *const c_char, version_key: *const c_char) -> CBool;
    pub fn RimeSyncUserData() -> CBool;
    pub fn RimeCreateSession() -> RimeSessionId;
    pub fn RimeFindSession(session_id: RimeSessionId) -> CBool;
    pub fn RimeDestroySession(session_id: RimeSessionId) -> CBool;
    pub fn RimeCleanupStaleSessions();
    pub fn RimeCleanupAllSessions();
    pub fn RimeProcessKey(session_id: RimeSessionId, keycode: c_int, mask: c_int) -> CBool;
    #[doc = " return True if there is unread commit text"]
    pub fn RimeCommitComposition(session_id: RimeSessionId) -> CBool;
    pub fn RimeClearComposition(session_id: RimeSessionId);
    pub fn RimeGetCommit(session_id: RimeSessionId, commit: *mut RimeCommit) -> CBool;
    pub fn RimeFreeCommit(commit: *mut RimeCommit) -> CBool;
    pub fn RimeGetContext(session_id: RimeSessionId, context: *mut RimeContext) -> CBool;
    pub fn RimeFreeContext(context: *mut RimeContext) -> CBool;
    pub fn RimeGetStatus(session_id: RimeSessionId, status: *mut RimeStatus) -> CBool;
    pub fn RimeFreeStatus(status: *mut RimeStatus) -> CBool;
    pub fn RimeSetOption(session_id: RimeSessionId, option: *const c_char, value: CBool);
    pub fn RimeGetOption(session_id: RimeSessionId, option: *const c_char) -> CBool;
    pub fn RimeSetProperty(session_id: RimeSessionId, prop: *const c_char, value: *const c_char);
    pub fn RimeGetProperty(
        session_id: RimeSessionId,
        prop: *const c_char,
        value: *mut c_char,
        buffer_size: usize,
    ) -> CBool;
    pub fn RimeGetSchemaList(schema_list: *mut RimeSchemaList) -> CBool;
    pub fn RimeFreeSchemaList(schema_list: *mut RimeSchemaList);
    pub fn RimeGetCurrentSchema(
        session_id: RimeSessionId,
        schema_id: *mut c_char,
        buffer_size: usize,
    ) -> CBool;
    pub fn RimeSelectSchema(session_id: RimeSessionId, schema_id: *const c_char) -> CBool;
    pub fn RimeSchemaOpen(schema_id: *const c_char, config: *mut RimeConfig) -> CBool;
    pub fn RimeConfigOpen(config_id: *const c_char, config: *mut RimeConfig) -> CBool;
    pub fn RimeConfigClose(config: *mut RimeConfig) -> CBool;
    pub fn RimeConfigInit(config: *mut RimeConfig) -> CBool;
    pub fn RimeConfigLoadString(config: *mut RimeConfig, yaml: *const c_char) -> CBool;
    pub fn RimeConfigGetBool(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *mut CBool,
    ) -> CBool;
    pub fn RimeConfigGetInt(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *mut c_int,
    ) -> CBool;
    pub fn RimeConfigGetDouble(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *mut f64,
    ) -> CBool;
    pub fn RimeConfigGetString(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *mut c_char,
        buffer_size: usize,
    ) -> CBool;
    pub fn RimeConfigGetCString(config: *mut RimeConfig, key: *const c_char) -> *const c_char;
    pub fn RimeConfigSetBool(config: *mut RimeConfig, key: *const c_char, value: CBool) -> CBool;
    pub fn RimeConfigSetInt(config: *mut RimeConfig, key: *const c_char, value: c_int) -> CBool;
    pub fn RimeConfigSetDouble(config: *mut RimeConfig, key: *const c_char, value: f64) -> CBool;
    pub fn RimeConfigSetString(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *const c_char,
    ) -> CBool;
    pub fn RimeConfigGetItem(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *mut RimeConfig,
    ) -> CBool;
    pub fn RimeConfigSetItem(
        config: *mut RimeConfig,
        key: *const c_char,
        value: *mut RimeConfig,
    ) -> CBool;
    pub fn RimeConfigClear(config: *mut RimeConfig, key: *const c_char) -> CBool;
    pub fn RimeConfigCreateList(config: *mut RimeConfig, key: *const c_char) -> CBool;
    pub fn RimeConfigCreateMap(config: *mut RimeConfig, key: *const c_char) -> CBool;
    pub fn RimeConfigListSize(config: *mut RimeConfig, key: *const c_char) -> usize;
    pub fn RimeConfigBeginList(
        iterator: *mut RimeConfigIterator,
        config: *mut RimeConfig,
        key: *const c_char,
    ) -> CBool;
    pub fn RimeConfigBeginMap(
        iterator: *mut RimeConfigIterator,
        config: *mut RimeConfig,
        key: *const c_char,
    ) -> CBool;
    pub fn RimeConfigNext(iterator: *mut RimeConfigIterator) -> CBool;
    pub fn RimeConfigEnd(iterator: *mut RimeConfigIterator);
    pub fn RimeConfigUpdateSignature(config: *mut RimeConfig, signer: *const c_char) -> CBool;
    pub fn RimeSimulateKeySequence(session_id: RimeSessionId, key_sequence: *const c_char)
        -> CBool;
    pub fn RimeRegisterModule(module: *mut RimeModule) -> CBool;
    pub fn RimeFindModule(module_name: *const c_char) -> *mut RimeModule;
    #[doc = "! Run a registered task"]
    pub fn RimeRunTask(task_name: *const c_char) -> CBool;
    pub fn RimeGetSharedDataDir() -> *const c_char;
    pub fn RimeGetUserDataDir() -> *const c_char;
    pub fn RimeGetSyncDir() -> *const c_char;
    pub fn RimeGetUserId() -> *const c_char;
}

#[repr(C)]
pub struct RimeTraits {
    /// total size of the struct except the field data_size itself
    pub data_size: c_int,
    pub shared_data_dir: *const c_char,
    pub user_data_dir: *const c_char,
    pub distribution_name: *const c_char,
    pub distribution_code_name: *const c_char,
    pub distribution_version: *const c_char,
    #[doc = " Pass a C-string constant in the format \"rime.x\""]
    #[doc = " where \'x\' is the name of your application."]
    #[doc = " Add prefix \"rime.\" to ensure old log files are automatically cleaned."]
    pub app_name: *const c_char,
}

#[repr(C)]
pub struct RimeComposition {
    pub length: c_int,
    pub cursor_pos: c_int,
    pub sel_start: c_int,
    pub sel_end: c_int,
    pub preedit: *mut c_char,
}

#[repr(C)]
pub struct RimeCandidate {
    pub text: *mut c_char,
    pub comment: *mut c_char,
    pub reserved: *mut c_void,
}

/// Used to select input candidates
#[repr(C)]
pub struct RimeMenu {
    pub page_size: c_int,
    pub page_no: c_int,
    pub is_last_page: CBool,
    pub highlighted_candidate_index: c_int,
    pub num_candidates: c_int,
    pub candidates: *mut RimeCandidate,
    pub select_keys: *mut c_char,
}

#[repr(C)]
pub struct RimeCommit {
    pub data_size: c_int,
    pub text: *mut c_char,
}

#[repr(C)]
pub struct RimeContext {
    pub data_size: c_int,
    pub composition: RimeComposition,
    pub menu: RimeMenu,
    pub commit_text_preview: *mut c_char,
}

#[repr(C)]
pub struct RimeStatus {
    pub data_size: c_int,
    pub schema_id: *mut c_char,
    pub schema_name: *mut c_char,
    pub is_disabled: CBool,
    pub is_composing: CBool,
    pub is_ascii_mode: CBool,
    pub is_full_shape: CBool,
    pub is_simplified: CBool,
    pub is_traditional: CBool,
    pub is_ascii_punct: CBool,
}

#[repr(C)]
pub struct RimeConfig {
    pub ptr: *mut c_void,
}

#[repr(C)]
pub struct RimeConfigIterator {
    pub list: *mut c_void,
    pub map: *mut c_void,
    pub index: c_int,
    pub key: *const c_char,
    pub path: *const c_char,
}

#[repr(C)]
pub struct RimeSchemaListItem {
    pub schema_id: *mut c_char,
    pub name: *mut c_char,
    pub reserved: *mut c_void,
}

#[repr(C)]
pub struct RimeSchemaList {
    pub size: usize,
    pub list: *mut RimeSchemaListItem,
}

pub type RimeNotificationHandler = Option<
    unsafe extern "C" fn(
        context_object: *mut c_void,
        session_id: RimeSessionId,
        message_type: *const c_char,
        message_value: *const c_char,
    ),
>;

#[doc = "  Extend the structure to publish custom data/functions in your specific module"]
#[repr(C)]
pub struct RimeCustomApi {
    pub data_size: c_int,
}

#[repr(C)]
pub struct RimeModule {
    pub data_size: c_int,
    pub module_name: *const c_char,
    pub initialize: Option<unsafe extern "C" fn()>,
    pub finalize: Option<unsafe extern "C" fn()>,
    pub get_api: Option<unsafe extern "C" fn() -> *mut RimeCustomApi>,
}

impl Drop for RimeApi {
    fn drop(&mut self) {
        unsafe {
            self.finalize.unwrap()();
        }
    }
}
