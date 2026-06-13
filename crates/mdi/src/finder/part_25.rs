// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_25(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "tooltip-minus-outline" => Some(icons::TOOLTIP_MINUS_OUTLINE),
        "download" => Some(icons::DOWNLOAD),
        "boxing-glove" => Some(icons::BOXING_GLOVE),
        "star-off-outline" => Some(icons::STAR_OFF_OUTLINE),
        "checkbox-multiple-marked-outline" => Some(icons::CHECKBOX_MULTIPLE_MARKED_OUTLINE),
        "alpha-b-circle" => Some(icons::ALPHA_B_CIRCLE),
        "head-sync-outline" => Some(icons::HEAD_SYNC_OUTLINE),
        "sign-direction-remove" => Some(icons::SIGN_DIRECTION_REMOVE),
        #[allow(deprecated)]
        "sina-weibo" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'sina-weibo' is deprecated.").print(py);
            }
            Some(icons::SINA_WEIBO)
        }
        "rotate-360" => Some(icons::ROTATE_360),
        "source-commit-end-local" => Some(icons::SOURCE_COMMIT_END_LOCAL),
        "timer-pause" => Some(icons::TIMER_PAUSE),
        "face-woman" => Some(icons::FACE_WOMAN),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "archive" => Some(icons::ARCHIVE),
        "download-lock" => Some(icons::DOWNLOAD_LOCK),
        "movie-open-minus" => Some(icons::MOVIE_OPEN_MINUS),
        "pan-bottom-right" => Some(icons::PAN_BOTTOM_RIGHT),
        #[allow(deprecated)]
        "webrtc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webrtc' is deprecated.").print(py);
            }
            Some(icons::WEBRTC)
        }
        "video-4k-box" => Some(icons::VIDEO_4K_BOX),
        "coffee-to-go-outline" => Some(icons::COFFEE_TO_GO_OUTLINE),
        "rename" => Some(icons::RENAME),
        "key-star" => Some(icons::KEY_STAR),
        "folder-cancel" => Some(icons::FOLDER_CANCEL),
        "radio" => Some(icons::RADIO),
        "account-clock-outline" => Some(icons::ACCOUNT_CLOCK_OUTLINE),
        "puzzle-minus-outline" => Some(icons::PUZZLE_MINUS_OUTLINE),
        "camera-metering-partial" => Some(icons::CAMERA_METERING_PARTIAL),
        "hammer-sickle" => Some(icons::HAMMER_SICKLE),
        "face-man" => Some(icons::FACE_MAN),
        "chart-arc" => Some(icons::CHART_ARC),
        "video" => Some(icons::VIDEO),
        #[allow(deprecated)]
        "language-python" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-python' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_PYTHON)
        }
        "border-top" => Some(icons::BORDER_TOP),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "filter-menu-outline" => Some(icons::FILTER_MENU_OUTLINE),
        "file-marker" => Some(icons::FILE_MARKER),
        "chair-rolling" => Some(icons::CHAIR_ROLLING),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "candy-off-outline" => Some(icons::CANDY_OFF_OUTLINE),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "arrow-decision" => Some(icons::ARROW_DECISION),
        "pitchfork" => Some(icons::PITCHFORK),
        "image-check-outline" => Some(icons::IMAGE_CHECK_OUTLINE),
        "map-search" => Some(icons::MAP_SEARCH),
        "food-turkey" => Some(icons::FOOD_TURKEY),
        "file-document-check" => Some(icons::FILE_DOCUMENT_CHECK),
        "fridge" => Some(icons::FRIDGE),
        "folder-plus" => Some(icons::FOLDER_PLUS),
        "clock-time-four" => Some(icons::CLOCK_TIME_FOUR),
        "magnet" => Some(icons::MAGNET),
        "ufo" => Some(icons::UFO),
        "text-search-variant" => Some(icons::TEXT_SEARCH_VARIANT),
        "wifi-plus" => Some(icons::WIFI_PLUS),
        "alpha-l-circle-outline" => Some(icons::ALPHA_L_CIRCLE_OUTLINE),
        "reload" => Some(icons::RELOAD),
        "firework-off" => Some(icons::FIREWORK_OFF),
        "octahedron-off" => Some(icons::OCTAHEDRON_OFF),
        "store-check" => Some(icons::STORE_CHECK),
        "kettle-pour-over" => Some(icons::KETTLE_POUR_OVER),
        "cloud-percent" => Some(icons::CLOUD_PERCENT),
        "format-header-5" => Some(icons::FORMAT_HEADER_5),
        "numeric-3-box-multiple" => Some(icons::NUMERIC_3_BOX_MULTIPLE),
        "chart-donut" => Some(icons::CHART_DONUT),
        "minus-box-outline" => Some(icons::MINUS_BOX_OUTLINE),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "television-classic-off" => Some(icons::TELEVISION_CLASSIC_OFF),
        "battery-sync" => Some(icons::BATTERY_SYNC),
        "hand-pointing-left" => Some(icons::HAND_POINTING_LEFT),
        "turnstile-outline" => Some(icons::TURNSTILE_OUTLINE),
        "pail-plus-outline" => Some(icons::PAIL_PLUS_OUTLINE),
        "pier-crane" => Some(icons::PIER_CRANE),
        "flare" => Some(icons::FLARE),
        "beehive-outline" => Some(icons::BEEHIVE_OUTLINE),
        "flower-outline" => Some(icons::FLOWER_OUTLINE),
        "toy-brick-search-outline" => Some(icons::TOY_BRICK_SEARCH_OUTLINE),
        #[allow(deprecated)]
        "microsoft-windows" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-windows' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS)
        }
        "human-female-girl" => Some(icons::HUMAN_FEMALE_GIRL),
        "eye-check" => Some(icons::EYE_CHECK),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "alpha-o-circle-outline" => Some(icons::ALPHA_O_CIRCLE_OUTLINE),
        "chevron-double-right" => Some(icons::CHEVRON_DOUBLE_RIGHT),
        "domain" => Some(icons::DOMAIN),
        "lighthouse-on" => Some(icons::LIGHTHOUSE_ON),
        "arrow-collapse-all" => Some(icons::ARROW_COLLAPSE_ALL),
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "format-underline" => Some(icons::FORMAT_UNDERLINE),
        "projector-off" => Some(icons::PROJECTOR_OFF),
        "download-network" => Some(icons::DOWNLOAD_NETWORK),
        "sort-variant-off" => Some(icons::SORT_VARIANT_OFF),
        "bag-personal-tag-outline" => Some(icons::BAG_PERSONAL_TAG_OUTLINE),
        "code-tags-check" => Some(icons::CODE_TAGS_CHECK),
        "heating-coil" => Some(icons::HEATING_COIL),
        "bash" => Some(icons::BASH),
        "signal" => Some(icons::SIGNAL),
        "arrow-left-top" => Some(icons::ARROW_LEFT_TOP),
        "progress-helper" => Some(icons::PROGRESS_HELPER),
        "tram-side" => Some(icons::TRAM_SIDE),
        "signal-hspa-plus" => Some(icons::SIGNAL_HSPA_PLUS),
        "fire-hydrant-alert" => Some(icons::FIRE_HYDRANT_ALERT),
        "movie-off" => Some(icons::MOVIE_OFF),
        "storefront-minus-outline" => Some(icons::STOREFRONT_MINUS_OUTLINE),
        "eight-track" => Some(icons::EIGHT_TRACK),
        "dog-service" => Some(icons::DOG_SERVICE),
        "wifi-star" => Some(icons::WIFI_STAR),
        "fridge-variant-alert" => Some(icons::FRIDGE_VARIANT_ALERT),
        "card-account-details" => Some(icons::CARD_ACCOUNT_DETAILS),
        "square-edit-outline" => Some(icons::SQUARE_EDIT_OUTLINE),
        "cctv" => Some(icons::CCTV),
        "view-gallery" => Some(icons::VIEW_GALLERY),
        "lightbulb-on-80" => Some(icons::LIGHTBULB_ON_80),
        "bus-marker" => Some(icons::BUS_MARKER),
        "iron-outline" => Some(icons::IRON_OUTLINE),
        "land-rows-horizontal" => Some(icons::LAND_ROWS_HORIZONTAL),
        "auto-download" => Some(icons::AUTO_DOWNLOAD),
        "axis-lock" => Some(icons::AXIS_LOCK),
        "contactless-payment" => Some(icons::CONTACTLESS_PAYMENT),
        "medal-outline" => Some(icons::MEDAL_OUTLINE),
        "arm-flex" => Some(icons::ARM_FLEX),
        "storage-tank" => Some(icons::STORAGE_TANK),
        "code-braces" => Some(icons::CODE_BRACES),
        "bank-circle" => Some(icons::BANK_CIRCLE),
        "home-alert-outline" => Some(icons::HOME_ALERT_OUTLINE),
        "map-marker-check-outline" => Some(icons::MAP_MARKER_CHECK_OUTLINE),
        "page-layout-header-footer" => Some(icons::PAGE_LAYOUT_HEADER_FOOTER),
        "fire-station" => Some(icons::FIRE_STATION),
        "rotate-left" => Some(icons::ROTATE_LEFT),
        "archive-minus" => Some(icons::ARCHIVE_MINUS),
        "briefcase-search-outline" => Some(icons::BRIEFCASE_SEARCH_OUTLINE),
        #[allow(deprecated)]
        "android" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android' is deprecated.").print(py);
            }
            Some(icons::ANDROID)
        }
        "dna" => Some(icons::DNA),
        "ruler-square-compass" => Some(icons::RULER_SQUARE_COMPASS),
        "presentation" => Some(icons::PRESENTATION),
        "relation-one-or-many-to-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE),
        "view-headline" => Some(icons::VIEW_HEADLINE),
        "account-arrow-left" => Some(icons::ACCOUNT_ARROW_LEFT),
        "fan" => Some(icons::FAN),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        "label-variant-outline" => Some(icons::LABEL_VARIANT_OUTLINE),
        "security" => Some(icons::SECURITY),
        "transmission-tower-off" => Some(icons::TRANSMISSION_TOWER_OFF),
        "alpha-r-circle-outline" => Some(icons::ALPHA_R_CIRCLE_OUTLINE),
        "arrow-bottom-right-thin" => Some(icons::ARROW_BOTTOM_RIGHT_THIN),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        "current-ac" => Some(icons::CURRENT_AC),
        "tower-fire" => Some(icons::TOWER_FIRE),
        "shield-lock-outline" => Some(icons::SHIELD_LOCK_OUTLINE),
        "close-circle-outline" => Some(icons::CLOSE_CIRCLE_OUTLINE),
        "arrow-down-thin-circle-outline" => Some(icons::ARROW_DOWN_THIN_CIRCLE_OUTLINE),
        "image-area-close" => Some(icons::IMAGE_AREA_CLOSE),
        "cellphone" => Some(icons::CELLPHONE),
        "projector-screen-variant-off" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF),
        "smoke-detector-alert" => Some(icons::SMOKE_DETECTOR_ALERT),
        "tag-arrow-up-outline" => Some(icons::TAG_ARROW_UP_OUTLINE),
        "wifi-strength-2" => Some(icons::WIFI_STRENGTH_2),
        "umbrella-outline" => Some(icons::UMBRELLA_OUTLINE),
        "vector-difference-ab" => Some(icons::VECTOR_DIFFERENCE_AB),
        "storefront-edit-outline" => Some(icons::STOREFRONT_EDIT_OUTLINE),
        "garage-alert" => Some(icons::GARAGE_ALERT),
        "desktop-tower" => Some(icons::DESKTOP_TOWER),
        "music-accidental-flat" => Some(icons::MUSIC_ACCIDENTAL_FLAT),
        "account-badge-outline" => Some(icons::ACCOUNT_BADGE_OUTLINE),
        "horse-human" => Some(icons::HORSE_HUMAN),
        "cellphone-play" => Some(icons::CELLPHONE_PLAY),
        "attachment-off" => Some(icons::ATTACHMENT_OFF),
        "parking" => Some(icons::PARKING),
        "eye" => Some(icons::EYE),
        "cloud-refresh" => Some(icons::CLOUD_REFRESH),
        "briefcase-download" => Some(icons::BRIEFCASE_DOWNLOAD),
        "map-marker-distance" => Some(icons::MAP_MARKER_DISTANCE),
        "clock-time-two-outline" => Some(icons::CLOCK_TIME_TWO_OUTLINE),
        "laptop" => Some(icons::LAPTOP),
        "archive-music-outline" => Some(icons::ARCHIVE_MUSIC_OUTLINE),
        "music-box-outline" => Some(icons::MUSIC_BOX_OUTLINE),
        "message-processing-outline" => Some(icons::MESSAGE_PROCESSING_OUTLINE),
        "alarm-panel" => Some(icons::ALARM_PANEL),
        "alpha-g-box" => Some(icons::ALPHA_G_BOX),
        "diaper-outline" => Some(icons::DIAPER_OUTLINE),
        "account-filter-outline" => Some(icons::ACCOUNT_FILTER_OUTLINE),
        "delete-restore" => Some(icons::DELETE_RESTORE),
        "numeric-8-circle" => Some(icons::NUMERIC_8_CIRCLE),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "earth-off" => Some(icons::EARTH_OFF),
        "camera" => Some(icons::CAMERA),
        "graph" => Some(icons::GRAPH),
        "bucket-outline" => Some(icons::BUCKET_OUTLINE),
        "receipt-text-arrow-right-outline" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT_OUTLINE),
        "amplifier" => Some(icons::AMPLIFIER),
        "flashlight-off" => Some(icons::FLASHLIGHT_OFF),
        "gate-xor" => Some(icons::GATE_XOR),
        "content-paste" => Some(icons::CONTENT_PASTE),
        "application-cog-outline" => Some(icons::APPLICATION_COG_OUTLINE),
        #[allow(deprecated)]
        "whatsapp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'whatsapp' is deprecated.").print(py);
            }
            Some(icons::WHATSAPP)
        }
        "credit-card-search" => Some(icons::CREDIT_CARD_SEARCH),
        #[allow(deprecated)]
        "codepen" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'codepen' is deprecated.").print(py);
            }
            Some(icons::CODEPEN)
        }
        "invoice-check" => Some(icons::INVOICE_CHECK),
        "timer-sand-paused" => Some(icons::TIMER_SAND_PAUSED),
        "file-document-multiple" => Some(icons::FILE_DOCUMENT_MULTIPLE),
        #[allow(deprecated)]
        "language-xaml" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-xaml' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_XAML)
        }
        _ => None,
    }
}
