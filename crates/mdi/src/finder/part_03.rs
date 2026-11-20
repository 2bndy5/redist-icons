// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_3(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "table-column-width" => Some(icons::TABLE_COLUMN_WIDTH),
        "calendar-import-outline" => Some(icons::CALENDAR_IMPORT_OUTLINE),
        "sunglasses" => Some(icons::SUNGLASSES),
        "bag-personal-off" => Some(icons::BAG_PERSONAL_OFF),
        "assistant" => Some(icons::ASSISTANT),
        "book-multiple" => Some(icons::BOOK_MULTIPLE),
        "cup-water" => Some(icons::CUP_WATER),
        "stop" => Some(icons::STOP),
        "flask-empty-remove" => Some(icons::FLASK_EMPTY_REMOVE),
        "home-roof" => Some(icons::HOME_ROOF),
        "magnify" => Some(icons::MAGNIFY),
        "align-horizontal-right" => Some(icons::ALIGN_HORIZONTAL_RIGHT),
        "content-save-move" => Some(icons::CONTENT_SAVE_MOVE),
        "silverware-fork" => Some(icons::SILVERWARE_FORK),
        "image-minus" => Some(icons::IMAGE_MINUS),
        "web-check" => Some(icons::WEB_CHECK),
        "pan-left" => Some(icons::PAN_LEFT),
        "spotlight-beam" => Some(icons::SPOTLIGHT_BEAM),
        "light-flood-down" => Some(icons::LIGHT_FLOOD_DOWN),
        #[allow(deprecated)]
        "goodreads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'goodreads' is deprecated.").print(py);
            }
            Some(icons::GOODREADS)
        }
        "mailbox" => Some(icons::MAILBOX),
        "sort-variant" => Some(icons::SORT_VARIANT),
        "sun-thermometer" => Some(icons::SUN_THERMOMETER),
        "download" => Some(icons::DOWNLOAD),
        "magnify-expand" => Some(icons::MAGNIFY_EXPAND),
        "battery-heart-outline" => Some(icons::BATTERY_HEART_OUTLINE),
        "movie-play" => Some(icons::MOVIE_PLAY),
        "message-text-fast-outline" => Some(icons::MESSAGE_TEXT_FAST_OUTLINE),
        "reply" => Some(icons::REPLY),
        "monitor-cellphone" => Some(icons::MONITOR_CELLPHONE),
        "anvil" => Some(icons::ANVIL),
        "home-heart" => Some(icons::HOME_HEART),
        "radio-fm" => Some(icons::RADIO_FM),
        "invoice-text-remove-outline" => Some(icons::INVOICE_TEXT_REMOVE_OUTLINE),
        "webcam" => Some(icons::WEBCAM),
        "archive-cancel" => Some(icons::ARCHIVE_CANCEL),
        "comment-quote-outline" => Some(icons::COMMENT_QUOTE_OUTLINE),
        "transfer-up" => Some(icons::TRANSFER_UP),
        "land-fields" => Some(icons::LAND_FIELDS),
        "slash-forward-box" => Some(icons::SLASH_FORWARD_BOX),
        "tag-arrow-left" => Some(icons::TAG_ARROW_LEFT),
        "pill" => Some(icons::PILL),
        "emoticon-wink" => Some(icons::EMOTICON_WINK),
        "human-pregnant" => Some(icons::HUMAN_PREGNANT),
        "tab-plus" => Some(icons::TAB_PLUS),
        "seed-plus-outline" => Some(icons::SEED_PLUS_OUTLINE),
        "map-clock" => Some(icons::MAP_CLOCK),
        "bucket-outline" => Some(icons::BUCKET_OUTLINE),
        "unicorn-variant" => Some(icons::UNICORN_VARIANT),
        "fullscreen" => Some(icons::FULLSCREEN),
        "close-box-multiple-outline" => Some(icons::CLOSE_BOX_MULTIPLE_OUTLINE),
        "pitchfork" => Some(icons::PITCHFORK),
        "book-plus-multiple-outline" => Some(icons::BOOK_PLUS_MULTIPLE_OUTLINE),
        "bottle-tonic-plus" => Some(icons::BOTTLE_TONIC_PLUS),
        "earth-arrow-up" => Some(icons::EARTH_ARROW_UP),
        "alpha-q-box-outline" => Some(icons::ALPHA_Q_BOX_OUTLINE),
        "qrcode-minus" => Some(icons::QRCODE_MINUS),
        "airplane-off" => Some(icons::AIRPLANE_OFF),
        "cash-off" => Some(icons::CASH_OFF),
        "engine-off" => Some(icons::ENGINE_OFF),
        "arrow-right-bold-box-outline" => Some(icons::ARROW_RIGHT_BOLD_BOX_OUTLINE),
        "select-inverse" => Some(icons::SELECT_INVERSE),
        #[allow(deprecated)]
        "dev-to" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dev-to' is deprecated.").print(py);
            }
            Some(icons::DEV_TO)
        }
        "ticket-confirmation-outline" => Some(icons::TICKET_CONFIRMATION_OUTLINE),
        "archive-plus-outline" => Some(icons::ARCHIVE_PLUS_OUTLINE),
        "trending-up" => Some(icons::TRENDING_UP),
        "battery-charging-wireless-50" => Some(icons::BATTERY_CHARGING_WIRELESS_50),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "calendar-today-outline" => Some(icons::CALENDAR_TODAY_OUTLINE),
        "highway" => Some(icons::HIGHWAY),
        "soy-sauce" => Some(icons::SOY_SAUCE),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "package-variant-closed-check" => Some(icons::PACKAGE_VARIANT_CLOSED_CHECK),
        "water-thermometer-outline" => Some(icons::WATER_THERMOMETER_OUTLINE),
        "text-box-check-outline" => Some(icons::TEXT_BOX_CHECK_OUTLINE),
        "phone-incoming-outgoing-outline" => Some(icons::PHONE_INCOMING_OUTGOING_OUTLINE),
        "lighthouse" => Some(icons::LIGHTHOUSE),
        "home-remove-outline" => Some(icons::HOME_REMOVE_OUTLINE),
        "decimal-comma-increase" => Some(icons::DECIMAL_COMMA_INCREASE),
        "book-remove" => Some(icons::BOOK_REMOVE),
        "network-strength-outline" => Some(icons::NETWORK_STRENGTH_OUTLINE),
        "post-outline" => Some(icons::POST_OUTLINE),
        "nature-people" => Some(icons::NATURE_PEOPLE),
        "skew-more" => Some(icons::SKEW_MORE),
        "registered-trademark" => Some(icons::REGISTERED_TRADEMARK),
        "sine-wave" => Some(icons::SINE_WAVE),
        "account-multiple-plus-outline" => Some(icons::ACCOUNT_MULTIPLE_PLUS_OUTLINE),
        "table-column-remove" => Some(icons::TABLE_COLUMN_REMOVE),
        "arrow-up-bold-outline" => Some(icons::ARROW_UP_BOLD_OUTLINE),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        #[allow(deprecated)]
        "blender-software" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'blender-software' is deprecated.")
                    .print(py);
            }
            Some(icons::BLENDER_SOFTWARE)
        }
        "mailbox-open-up-outline" => Some(icons::MAILBOX_OPEN_UP_OUTLINE),
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        "calendar-week-begin-outline" => Some(icons::CALENDAR_WEEK_BEGIN_OUTLINE),
        "call-made" => Some(icons::CALL_MADE),
        "account-school-outline" => Some(icons::ACCOUNT_SCHOOL_OUTLINE),
        "gymnastics" => Some(icons::GYMNASTICS),
        "ticket-percent" => Some(icons::TICKET_PERCENT),
        "dice-6" => Some(icons::DICE_6),
        "key-variant" => Some(icons::KEY_VARIANT),
        "folder-heart-outline" => Some(icons::FOLDER_HEART_OUTLINE),
        "weight" => Some(icons::WEIGHT),
        "format-vertical-align-top" => Some(icons::FORMAT_VERTICAL_ALIGN_TOP),
        "briefcase-off-outline" => Some(icons::BRIEFCASE_OFF_OUTLINE),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "signal-hspa-plus" => Some(icons::SIGNAL_HSPA_PLUS),
        "fire-hydrant-alert" => Some(icons::FIRE_HYDRANT_ALERT),
        "credit-card-plus-outline" => Some(icons::CREDIT_CARD_PLUS_OUTLINE),
        "eight-track" => Some(icons::EIGHT_TRACK),
        "human-male" => Some(icons::HUMAN_MALE),
        "alpha-y-box" => Some(icons::ALPHA_Y_BOX),
        "chevron-right-circle" => Some(icons::CHEVRON_RIGHT_CIRCLE),
        "camera-plus-outline" => Some(icons::CAMERA_PLUS_OUTLINE),
        "camera-burst" => Some(icons::CAMERA_BURST),
        "alpha-z" => Some(icons::ALPHA_Z),
        "key-link" => Some(icons::KEY_LINK),
        "thermometer-high" => Some(icons::THERMOMETER_HIGH),
        "heart-outline" => Some(icons::HEART_OUTLINE),
        "transmission-tower-off" => Some(icons::TRANSMISSION_TOWER_OFF),
        "coat-rack" => Some(icons::COAT_RACK),
        "account-off" => Some(icons::ACCOUNT_OFF),
        "ip-outline" => Some(icons::IP_OUTLINE),
        "pencil-box-multiple-outline" => Some(icons::PENCIL_BOX_MULTIPLE_OUTLINE),
        "airplane-alert" => Some(icons::AIRPLANE_ALERT),
        "variable" => Some(icons::VARIABLE),
        "airplane" => Some(icons::AIRPLANE),
        "upload-lock" => Some(icons::UPLOAD_LOCK),
        "information-outline" => Some(icons::INFORMATION_OUTLINE),
        "alert-minus" => Some(icons::ALERT_MINUS),
        "screw-machine-flat-top" => Some(icons::SCREW_MACHINE_FLAT_TOP),
        "filter-off-outline" => Some(icons::FILTER_OFF_OUTLINE),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "wifi-lock" => Some(icons::WIFI_LOCK),
        "paperclip-lock" => Some(icons::PAPERCLIP_LOCK),
        "email-arrow-left" => Some(icons::EMAIL_ARROW_LEFT),
        "rodent" => Some(icons::RODENT),
        "table-eye" => Some(icons::TABLE_EYE),
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "code-not-equal" => Some(icons::CODE_NOT_EQUAL),
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        "content-save-off" => Some(icons::CONTENT_SAVE_OFF),
        "alien" => Some(icons::ALIEN),
        "file-star-outline" => Some(icons::FILE_STAR_OUTLINE),
        "folder-arrow-left-right" => Some(icons::FOLDER_ARROW_LEFT_RIGHT),
        "cloud-outline" => Some(icons::CLOUD_OUTLINE),
        "phone-classic-off" => Some(icons::PHONE_CLASSIC_OFF),
        "chart-arc" => Some(icons::CHART_ARC),
        "flask-remove-outline" => Some(icons::FLASK_REMOVE_OUTLINE),
        "fast-forward-outline" => Some(icons::FAST_FORWARD_OUTLINE),
        "bullhorn-variant" => Some(icons::BULLHORN_VARIANT),
        "diaper-outline" => Some(icons::DIAPER_OUTLINE),
        "restart-off" => Some(icons::RESTART_OFF),
        "emoticon" => Some(icons::EMOTICON),
        "water-minus" => Some(icons::WATER_MINUS),
        "battery-heart-variant" => Some(icons::BATTERY_HEART_VARIANT),
        "hanger" => Some(icons::HANGER),
        "inbox-multiple-outline" => Some(icons::INBOX_MULTIPLE_OUTLINE),
        "tooltip-plus" => Some(icons::TOOLTIP_PLUS),
        "watch-export" => Some(icons::WATCH_EXPORT),
        "radio" => Some(icons::RADIO),
        "comment-question" => Some(icons::COMMENT_QUESTION),
        #[allow(deprecated)]
        "pinterest" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pinterest' is deprecated.").print(py);
            }
            Some(icons::PINTEREST)
        }
        "clock-time-two-outline" => Some(icons::CLOCK_TIME_TWO_OUTLINE),
        "sign-real-estate" => Some(icons::SIGN_REAL_ESTATE),
        "fan-chevron-up" => Some(icons::FAN_CHEVRON_UP),
        "card-account-details-star" => Some(icons::CARD_ACCOUNT_DETAILS_STAR),
        "image-filter-frames" => Some(icons::IMAGE_FILTER_FRAMES),
        "truck-fast-outline" => Some(icons::TRUCK_FAST_OUTLINE),
        "plus-minus-box" => Some(icons::PLUS_MINUS_BOX),
        "phone-voip" => Some(icons::PHONE_VOIP),
        "beaker" => Some(icons::BEAKER),
        "view-headline" => Some(icons::VIEW_HEADLINE),
        "queue-first-in-last-out" => Some(icons::QUEUE_FIRST_IN_LAST_OUT),
        "music-off" => Some(icons::MUSIC_OFF),
        #[allow(deprecated)]
        "polymer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'polymer' is deprecated.").print(py);
            }
            Some(icons::POLYMER)
        }
        "folder-network-outline" => Some(icons::FOLDER_NETWORK_OUTLINE),
        "syllabary-hiragana" => Some(icons::SYLLABARY_HIRAGANA),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "arrow-right-bottom" => Some(icons::ARROW_RIGHT_BOTTOM),
        "cradle-outline" => Some(icons::CRADLE_OUTLINE),
        "compare-vertical" => Some(icons::COMPARE_VERTICAL),
        #[allow(deprecated)]
        "cryengine" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cryengine' is deprecated.").print(py);
            }
            Some(icons::CRYENGINE)
        }
        "account-music" => Some(icons::ACCOUNT_MUSIC),
        "text-box-search" => Some(icons::TEXT_BOX_SEARCH),
        "database-arrow-left-outline" => Some(icons::DATABASE_ARROW_LEFT_OUTLINE),
        "movie-open-minus" => Some(icons::MOVIE_OPEN_MINUS),
        "umbrella-closed-variant" => Some(icons::UMBRELLA_CLOSED_VARIANT),
        "music-note-quarter" => Some(icons::MUSIC_NOTE_QUARTER),
        "frequently-asked-questions" => Some(icons::FREQUENTLY_ASKED_QUESTIONS),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "home-edit-outline" => Some(icons::HOME_EDIT_OUTLINE),
        "package-variant-closed-minus" => Some(icons::PACKAGE_VARIANT_CLOSED_MINUS),
        "battery-charging-70" => Some(icons::BATTERY_CHARGING_70),
        "phone-log" => Some(icons::PHONE_LOG),
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "home-floor-l" => Some(icons::HOME_FLOOR_L),
        "table-heart" => Some(icons::TABLE_HEART),
        "pokeball" => Some(icons::POKEBALL),
        "fuse-off" => Some(icons::FUSE_OFF),
        "camera-enhance" => Some(icons::CAMERA_ENHANCE),
        _ => None,
    }
}
