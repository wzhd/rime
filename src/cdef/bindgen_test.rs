#[allow(unused_imports)]
use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}

#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        // bindgen set it to 32usize but the test didn't pass
        24usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}

#[test]
fn bindgen_test_layout_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_traits() {
    assert_eq!(
        ::std::mem::size_of::<RimeTraits>(),
        56usize,
        concat!("Size of: ", stringify!(RimeTraits))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeTraits>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeTraits))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeTraits>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeTraits>())).shared_data_dir as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(shared_data_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeTraits>())).user_data_dir as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(user_data_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeTraits>())).distribution_name as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(distribution_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<RimeTraits>())).distribution_code_name as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(distribution_code_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeTraits>())).distribution_version as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(distribution_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeTraits>())).app_name as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeTraits),
            "::",
            stringify!(app_name)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_composition() {
    assert_eq!(
        ::std::mem::size_of::<RimeComposition>(),
        24usize,
        concat!("Size of: ", stringify!(RimeComposition))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeComposition>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeComposition))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeComposition>())).length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeComposition),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeComposition>())).cursor_pos as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeComposition),
            "::",
            stringify!(cursor_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeComposition>())).sel_start as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeComposition),
            "::",
            stringify!(sel_start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeComposition>())).sel_end as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeComposition),
            "::",
            stringify!(sel_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeComposition>())).preedit as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeComposition),
            "::",
            stringify!(preedit)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_candidate() {
    assert_eq!(
        ::std::mem::size_of::<RimeCandidate>(),
        24usize,
        concat!("Size of: ", stringify!(RimeCandidate))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeCandidate>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeCandidate))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeCandidate>())).text as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeCandidate),
            "::",
            stringify!(text)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeCandidate>())).comment as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeCandidate),
            "::",
            stringify!(comment)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeCandidate>())).reserved as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeCandidate),
            "::",
            stringify!(reserved)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_menu() {
    assert_eq!(
        ::std::mem::size_of::<RimeMenu>(),
        40usize,
        concat!("Size of: ", stringify!(RimeMenu))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeMenu>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeMenu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeMenu>())).page_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(page_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeMenu>())).page_no as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(page_no)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeMenu>())).is_last_page as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(is_last_page)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<RimeMenu>())).highlighted_candidate_index as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(highlighted_candidate_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeMenu>())).num_candidates as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(num_candidates)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeMenu>())).candidates as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(candidates)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeMenu>())).select_keys as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeMenu),
            "::",
            stringify!(select_keys)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_commit() {
    assert_eq!(
        ::std::mem::size_of::<RimeCommit>(),
        16usize,
        concat!("Size of: ", stringify!(RimeCommit))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeCommit>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeCommit))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeCommit>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeCommit),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeCommit>())).text as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeCommit),
            "::",
            stringify!(text)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_context() {
    assert_eq!(
        ::std::mem::size_of::<RimeContext>(),
        80usize,
        concat!("Size of: ", stringify!(RimeContext))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeContext>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeContext))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeContext>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeContext),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeContext>())).composition as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeContext),
            "::",
            stringify!(composition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeContext>())).menu as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeContext),
            "::",
            stringify!(menu)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeContext>())).commit_text_preview as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeContext),
            "::",
            stringify!(commit_text_preview)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_status_t() {
    assert_eq!(
        ::std::mem::size_of::<RimeStatus>(),
        56usize,
        concat!("Size of: ", stringify!(rime_status_t))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeStatus>(),
        8usize,
        concat!("Alignment of ", stringify!(rime_status_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).schema_id as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(schema_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).schema_name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(schema_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_disabled as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_disabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_composing as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_composing)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_ascii_mode as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_ascii_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_full_shape as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_full_shape)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_simplified as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_simplified)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_traditional as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_traditional)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeStatus>())).is_ascii_punct as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_status_t),
            "::",
            stringify!(is_ascii_punct)
        )
    );
}

#[test]
fn bindgen_test_layout_RimeConfig() {
    assert_eq!(
        ::std::mem::size_of::<RimeConfig>(),
        8usize,
        concat!("Size of: ", stringify!(RimeConfig))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeConfig>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeConfig))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeConfig>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeConfig),
            "::",
            stringify!(ptr)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_config_iterator() {
    assert_eq!(
        ::std::mem::size_of::<RimeConfigIterator>(),
        40usize,
        concat!("Size of: ", stringify!(RimeConfigIterator))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeConfigIterator>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeConfigIterator))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeConfigIterator>())).list as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeConfigIterator),
            "::",
            stringify!(list)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeConfigIterator>())).map as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeConfigIterator),
            "::",
            stringify!(map)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeConfigIterator>())).index as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeConfigIterator),
            "::",
            stringify!(index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeConfigIterator>())).key as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeConfigIterator),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeConfigIterator>())).path as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeConfigIterator),
            "::",
            stringify!(path)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_schema_list_item() {
    assert_eq!(
        ::std::mem::size_of::<RimeSchemaListItem>(),
        24usize,
        concat!("Size of: ", stringify!(RimeSchemaListItem))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeSchemaListItem>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeSchemaListItem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeSchemaListItem>())).schema_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeSchemaListItem),
            "::",
            stringify!(schema_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeSchemaListItem>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeSchemaListItem),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeSchemaListItem>())).reserved as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeSchemaListItem),
            "::",
            stringify!(reserved)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_schema_list() {
    assert_eq!(
        ::std::mem::size_of::<RimeSchemaList>(),
        16usize,
        concat!("Size of: ", stringify!(RimeSchemaList))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeSchemaList>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeSchemaList))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeSchemaList>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeSchemaList),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeSchemaList>())).list as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeSchemaList),
            "::",
            stringify!(list)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_custom_api() {
    assert_eq!(
        ::std::mem::size_of::<RimeCustomApi>(),
        4usize,
        concat!("Size of: ", stringify!(RimeCustomApi))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeCustomApi>(),
        4usize,
        concat!("Alignment of ", stringify!(RimeCustomApi))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeCustomApi>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeCustomApi),
            "::",
            stringify!(data_size)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_module() {
    assert_eq!(
        ::std::mem::size_of::<RimeModule>(),
        40usize,
        concat!("Size of: ", stringify!(RimeModule))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeModule>(),
        8usize,
        concat!("Alignment of ", stringify!(RimeModule))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeModule>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeModule),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeModule>())).module_name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeModule),
            "::",
            stringify!(module_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeModule>())).initialize as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeModule),
            "::",
            stringify!(initialize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeModule>())).finalize as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeModule),
            "::",
            stringify!(finalize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeModule>())).get_api as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RimeModule),
            "::",
            stringify!(get_api)
        )
    );
}

#[test]
fn bindgen_test_layout_rime_api_t() {
    assert_eq!(
        ::std::mem::size_of::<RimeApi>(),
        592usize,
        concat!("Size of: ", stringify!(rime_api_t))
    );
    assert_eq!(
        ::std::mem::align_of::<RimeApi>(),
        8usize,
        concat!("Alignment of ", stringify!(rime_api_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).data_size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).setup as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(setup)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<RimeApi>())).set_notification_handler as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(set_notification_handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).initialize as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(initialize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).finalize as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(finalize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).start_maintenance as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(start_maintenance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).is_maintenance_mode as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(is_maintenance_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).join_maintenance_thread as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(join_maintenance_thread)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).deployer_initialize as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(deployer_initialize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).prebuild as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(prebuild)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).deploy as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(deploy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).deploy_schema as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(deploy_schema)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).deploy_config_file as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(deploy_config_file)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).sync_user_data as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(sync_user_data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).create_session as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(create_session)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).find_session as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(find_session)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).destroy_session as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(destroy_session)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).cleanup_stale_sessions as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(cleanup_stale_sessions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).cleanup_all_sessions as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(cleanup_all_sessions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).process_key as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(process_key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).commit_composition as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(commit_composition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).clear_composition as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(clear_composition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_commit as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_commit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).free_commit as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(free_commit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_context as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).free_context as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(free_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_status as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).free_status as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(free_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).set_option as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(set_option)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_option as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_option)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).set_property as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(set_property)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_property as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_property)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_schema_list as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_schema_list)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).free_schema_list as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(free_schema_list)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_current_schema as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_current_schema)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).select_schema as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(select_schema)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).schema_open as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(schema_open)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_open as *const _ as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_open)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_close as *const _ as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_close)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_get_bool as *const _ as usize },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_get_bool)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_get_int as *const _ as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_get_int)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_get_double as *const _ as usize },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_get_double)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_get_string as *const _ as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_get_string)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_get_cstring as *const _ as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_get_cstring)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_update_signature as *const _ as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_update_signature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_begin_map as *const _ as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_begin_map)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_next as *const _ as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_end as *const _ as usize },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).simulate_key_sequence as *const _ as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(simulate_key_sequence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).register_module as *const _ as usize },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(register_module)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).find_module as *const _ as usize },
        400usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(find_module)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).run_task as *const _ as usize },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(run_task)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_shared_data_dir as *const _ as usize },
        416usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_shared_data_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_user_data_dir as *const _ as usize },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_user_data_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_sync_dir as *const _ as usize },
        432usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_sync_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_user_id as *const _ as usize },
        440usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_user_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_user_data_sync_dir as *const _ as usize },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_user_data_sync_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_init as *const _ as usize },
        456usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_load_string as *const _ as usize },
        464usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_load_string)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_set_bool as *const _ as usize },
        472usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_set_bool)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_set_int as *const _ as usize },
        480usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_set_int)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_set_double as *const _ as usize },
        488usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_set_double)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_set_string as *const _ as usize },
        496usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_set_string)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_get_item as *const _ as usize },
        504usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_get_item)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_set_item as *const _ as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_set_item)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_clear as *const _ as usize },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_clear)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_create_list as *const _ as usize },
        528usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_create_list)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_create_map as *const _ as usize },
        536usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_create_map)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_list_size as *const _ as usize },
        544usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_list_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).config_begin_list as *const _ as usize },
        552usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(config_begin_list)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_input as *const _ as usize },
        560usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_input)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_caret_pos as *const _ as usize },
        568usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_caret_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).select_candidate as *const _ as usize },
        576usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(select_candidate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RimeApi>())).get_version as *const _ as usize },
        584usize,
        concat!(
            "Offset of field: ",
            stringify!(rime_api_t),
            "::",
            stringify!(get_version)
        )
    );
}
