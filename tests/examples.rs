extern crate content_security_policy;
use content_security_policy::*;

macro_rules! test_should_request_be_blocked {
    ($((name: $name:ident, url: $url:expr, origin: $origin:expr, policy: $policy:expr, dest: $destination:tt, result: $result:tt)),*$(,)*) => {
        $(
            #[test]
            fn $name() {
                let csp_list = CspList::parse($policy, PolicySource::Header, PolicyDisposition::Enforce);
                let (check_result, _) = csp_list.should_request_be_blocked(&Request {
                    url: Url::parse($url).unwrap(),
                    origin: Url::parse($origin).unwrap().origin(),
                    redirect_count: 0,
                    destination: Destination::$destination,
                    initiator: Initiator::None,
                    nonce: String::new(),
                    integrity_metadata: String::new(),
                    parser_metadata: ParserMetadata::None,
                });
                assert_eq!(check_result, CheckResult::$result);
            }
        )*
    }
}

// all tests should have a name starting with pre_request_
test_should_request_be_blocked!{
    (   name: pre_request_wild_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_wild_script_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_upgrade_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_upgrade_script_block,
        url: "https://www.evil.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src *.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_exact_upgrade_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src www.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_exact_upgrade_script_block,
        url: "https://www.evil.com/script.js",
        origin: "http://www.notriddle.com",
        policy: "script-src www.notriddle.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_wild_style_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "style-src *.notriddle.com",
        dest: Style,
        result: Allowed),
    (   name: pre_request_wild_style_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "style-src *.notriddle.com",
        dest: Style,
        result: Blocked),
    // Check every possible "dest" value against default-src
    (   name: pre_request_default_audio_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Audio,
        result: Allowed),
    (   name: pre_request_default_audio_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Audio,
        result: Blocked),
    (   name: pre_request_default_document_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Document,
        result: Allowed),
    (   name: pre_request_default_document_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Document,
        result: Blocked),
    (   name: pre_request_default_embed_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Embed,
        result: Allowed),
    (   name: pre_request_default_embed_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Embed,
        result: Blocked),
    (   name: pre_request_default_font_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Font,
        result: Allowed),
    (   name: pre_request_default_font_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Font,
        result: Blocked),
    (   name: pre_request_default_image_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Image,
        result: Allowed),
    (   name: pre_request_default_image_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Image,
        result: Blocked),
    (   name: pre_request_default_manifest_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Manifest,
        result: Allowed),
    (   name: pre_request_default_manifest_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Manifest,
        result: Blocked),
    (   name: pre_request_default_object_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Object,
        result: Allowed),
    (   name: pre_request_default_object_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Object,
        result: Blocked),
    (   name: pre_request_default_script_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle.com",
        dest: Script,
        result: Allowed),
    (   name: pre_request_default_script_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Script,
        result: Blocked),
    (   name: pre_request_default_service_worker_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle.com",
        dest: ServiceWorker,
        result: Allowed),
    (   name: pre_request_default_service_worker_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: ServiceWorker,
        result: Blocked),
    (   name: pre_request_default_shared_worker_allow,
        url: "https://www.notriddle.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle.com",
        dest: SharedWorker,
        result: Allowed),
    (   name: pre_request_default_shared_worker_block,
        url: "https://www.evil.com/script.js",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: SharedWorker,
        result: Blocked),
    (   name: pre_request_default_style_allow,
        url: "https://www.notriddle2.com/style.css",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Style,
        result: Allowed),
    (   name: pre_request_default_style_block,
        url: "https://www.evil.com/style.css",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Style,
        result: Blocked),
    (   name: pre_request_default_track_allow,
        url: "https://www.notriddle2.com/track",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Track,
        result: Allowed),
    (   name: pre_request_default_track_block,
        url: "https://www.evil.com/track",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Track,
        result: Blocked),
    (   name: pre_request_default_video_allow,
        url: "https://www.notriddle2.com/style.mp4",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Video,
        result: Allowed),
    (   name: pre_request_default_video_block,
        url: "https://www.evil.com/style.mp4",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Video,
        result: Blocked),
    (   name: pre_request_default_worker_allow,
        url: "https://www.notriddle2.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Worker,
        result: Allowed),
    (   name: pre_request_default_worker_block,
        url: "https://www.evil.com/child/",
        origin: "https://www.notriddle.com",
        policy: "default-src www.notriddle2.com",
        dest: Worker,
        result: Blocked),
}

macro_rules! test_should_elements_inline_type_behavior_be_blocked {
    ($((name: $name:ident, policy: $policy:expr, nonce: $nonce:expr, inline_check_type: $type_:tt, source: $source:expr, result: $result:tt)),*$(,)*) => {
        $(
            #[test]
            fn $name() {
                let csp_list = CspList::parse($policy, PolicySource::Header, PolicyDisposition::Enforce);
                let (check_result, _) = csp_list.should_elements_inline_type_behavior_be_blocked(
                    &Element { nonce: $nonce },
                    InlineCheckType::$type_,
                    $source,
                );
                assert_eq!(check_result, CheckResult::$result);
            }
        )*
    }
}

// all tests should have a name starting with inline_
test_should_elements_inline_type_behavior_be_blocked!{
    (   name: inline_blocked_script,
        policy: "script-src 'none'",
        nonce: None,
        inline_check_type: Script,
        source: "",
        result: Blocked),
    (   name: inline_allowed_script_check_style,
        policy: "script-src 'none'",
        nonce: None,
        inline_check_type: Style,
        source: "",
        result: Allowed),
}