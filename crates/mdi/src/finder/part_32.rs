// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_32(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "music-note-outline" => Some(icons::MUSIC_NOTE_OUTLINE),
        "folder-alert" => Some(icons::FOLDER_ALERT),
        "upload-circle-outline" => Some(icons::UPLOAD_CIRCLE_OUTLINE),
        "api-off" => Some(icons::API_OFF),
        "panorama-wide-angle" => Some(icons::PANORAMA_WIDE_ANGLE),
        "table-eye-off" => Some(icons::TABLE_EYE_OFF),
        "bike-fast" => Some(icons::BIKE_FAST),
        "map-plus" => Some(icons::MAP_PLUS),
        "arrow-top-right-bold-box" => Some(icons::ARROW_TOP_RIGHT_BOLD_BOX),
        "hdr-off" => Some(icons::HDR_OFF),
        "caps-lock" => Some(icons::CAPS_LOCK),
        "hand-back-left" => Some(icons::HAND_BACK_LEFT),
        "file-image-marker" => Some(icons::FILE_IMAGE_MARKER),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "monitor-screenshot" => Some(icons::MONITOR_SCREENSHOT),
        "compost" => Some(icons::COMPOST),
        #[allow(deprecated)]
        "xmpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xmpp' is deprecated.").print(py);
            }
            Some(icons::XMPP)
        }
        "minus-network-outline" => Some(icons::MINUS_NETWORK_OUTLINE),
        "fingerprint-off" => Some(icons::FINGERPRINT_OFF),
        "motion-outline" => Some(icons::MOTION_OUTLINE),
        "picture-in-picture-top-right" => Some(icons::PICTURE_IN_PICTURE_TOP_RIGHT),
        "send-circle-outline" => Some(icons::SEND_CIRCLE_OUTLINE),
        "link-box" => Some(icons::LINK_BOX),
        "video-off-outline" => Some(icons::VIDEO_OFF_OUTLINE),
        "rivet" => Some(icons::RIVET),
        "content-save-cog" => Some(icons::CONTENT_SAVE_COG),
        "bed-king-outline" => Some(icons::BED_KING_OUTLINE),
        "creation-outline" => Some(icons::CREATION_OUTLINE),
        "bookmark-minus-outline" => Some(icons::BOOKMARK_MINUS_OUTLINE),
        "lambda" => Some(icons::LAMBDA),
        "home-minus-outline" => Some(icons::HOME_MINUS_OUTLINE),
        "usb-port" => Some(icons::USB_PORT),
        "file-cloud" => Some(icons::FILE_CLOUD),
        "file-percent-outline" => Some(icons::FILE_PERCENT_OUTLINE),
        "account-multiple-minus-outline" => Some(icons::ACCOUNT_MULTIPLE_MINUS_OUTLINE),
        "access-point-minus" => Some(icons::ACCESS_POINT_MINUS),
        "briefcase-account-outline" => Some(icons::BRIEFCASE_ACCOUNT_OUTLINE),
        "format-bold" => Some(icons::FORMAT_BOLD),
        "tram" => Some(icons::TRAM),
        "logout" => Some(icons::LOGOUT),
        "power-socket-uk" => Some(icons::POWER_SOCKET_UK),
        "access-point-plus" => Some(icons::ACCESS_POINT_PLUS),
        "face-woman-profile" => Some(icons::FACE_WOMAN_PROFILE),
        "file-star-four-points-outline" => Some(icons::FILE_STAR_FOUR_POINTS_OUTLINE),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "transfer" => Some(icons::TRANSFER),
        "truck-plus" => Some(icons::TRUCK_PLUS),
        #[allow(deprecated)]
        "apple-safari" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-safari' is deprecated.").print(py);
            }
            Some(icons::APPLE_SAFARI)
        }
        "human-walker" => Some(icons::HUMAN_WALKER),
        "clipboard-play-outline" => Some(icons::CLIPBOARD_PLAY_OUTLINE),
        "surround-sound-3-1" => Some(icons::SURROUND_SOUND_3_1),
        "gift-off" => Some(icons::GIFT_OFF),
        "network-strength-off-outline" => Some(icons::NETWORK_STRENGTH_OFF_OUTLINE),
        "seal-variant" => Some(icons::SEAL_VARIANT),
        "movie-settings" => Some(icons::MOVIE_SETTINGS),
        "pickaxe" => Some(icons::PICKAXE),
        "border-right-variant" => Some(icons::BORDER_RIGHT_VARIANT),
        "baseball-diamond-outline" => Some(icons::BASEBALL_DIAMOND_OUTLINE),
        "home-circle" => Some(icons::HOME_CIRCLE),
        "toggle-switch-variant-off" => Some(icons::TOGGLE_SWITCH_VARIANT_OFF),
        "view-split-horizontal" => Some(icons::VIEW_SPLIT_HORIZONTAL),
        "backspace-reverse" => Some(icons::BACKSPACE_REVERSE),
        "numeric-4-box-outline" => Some(icons::NUMERIC_4_BOX_OUTLINE),
        "gender-male-female" => Some(icons::GENDER_MALE_FEMALE),
        "bed-double-outline" => Some(icons::BED_DOUBLE_OUTLINE),
        "magnify-minus-cursor" => Some(icons::MAGNIFY_MINUS_CURSOR),
        "square-root-box" => Some(icons::SQUARE_ROOT_BOX),
        "car-cruise-control" => Some(icons::CAR_CRUISE_CONTROL),
        "tag" => Some(icons::TAG),
        "timer-minus-outline" => Some(icons::TIMER_MINUS_OUTLINE),
        "cards-spade" => Some(icons::CARDS_SPADE),
        "smart-card-reader-outline" => Some(icons::SMART_CARD_READER_OUTLINE),
        "circle-multiple" => Some(icons::CIRCLE_MULTIPLE),
        "file-key-outline" => Some(icons::FILE_KEY_OUTLINE),
        "storefront-plus" => Some(icons::STOREFRONT_PLUS),
        "emoticon-excited-outline" => Some(icons::EMOTICON_EXCITED_OUTLINE),
        #[allow(deprecated)]
        "digital-ocean" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'digital-ocean' is deprecated.").print(py);
            }
            Some(icons::DIGITAL_OCEAN)
        }
        "battery-charging-high" => Some(icons::BATTERY_CHARGING_HIGH),
        "apple-keyboard-caps" => Some(icons::APPLE_KEYBOARD_CAPS),
        "trademark" => Some(icons::TRADEMARK),
        "slope-uphill" => Some(icons::SLOPE_UPHILL),
        "amplifier-off" => Some(icons::AMPLIFIER_OFF),
        "chat-plus-outline" => Some(icons::CHAT_PLUS_OUTLINE),
        "domain-off" => Some(icons::DOMAIN_OFF),
        "file-settings-outline" => Some(icons::FILE_SETTINGS_OUTLINE),
        "home-plus" => Some(icons::HOME_PLUS),
        "cellphone-marker" => Some(icons::CELLPHONE_MARKER),
        "flag-triangle" => Some(icons::FLAG_TRIANGLE),
        "distribute-vertical-center" => Some(icons::DISTRIBUTE_VERTICAL_CENTER),
        "fleur-de-lis" => Some(icons::FLEUR_DE_LIS),
        "hvac" => Some(icons::HVAC),
        "home-city" => Some(icons::HOME_CITY),
        "chart-waterfall" => Some(icons::CHART_WATERFALL),
        "candelabra" => Some(icons::CANDELABRA),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "share-all-outline" => Some(icons::SHARE_ALL_OUTLINE),
        "hdr" => Some(icons::HDR),
        "smart-card-off-outline" => Some(icons::SMART_CARD_OFF_OUTLINE),
        "hololens" => Some(icons::HOLOLENS),
        "page-last" => Some(icons::PAGE_LAST),
        "pan-top-left" => Some(icons::PAN_TOP_LEFT),
        "surround-sound-7-1" => Some(icons::SURROUND_SOUND_7_1),
        "pumpkin" => Some(icons::PUMPKIN),
        "image-check-outline" => Some(icons::IMAGE_CHECK_OUTLINE),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        "email-plus-outline" => Some(icons::EMAIL_PLUS_OUTLINE),
        "gamepad-round" => Some(icons::GAMEPAD_ROUND),
        "file-certificate-outline" => Some(icons::FILE_CERTIFICATE_OUTLINE),
        "home-lightning-bolt-outline" => Some(icons::HOME_LIGHTNING_BOLT_OUTLINE),
        "drag-vertical-variant" => Some(icons::DRAG_VERTICAL_VARIANT),
        "checkerboard-remove" => Some(icons::CHECKERBOARD_REMOVE),
        "plus-minus-variant" => Some(icons::PLUS_MINUS_VARIANT),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "landslide" => Some(icons::LANDSLIDE),
        "hospital-box-outline" => Some(icons::HOSPITAL_BOX_OUTLINE),
        "relation-zero-or-one-to-one-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ONE_OR_MANY),
        "flask-minus-outline" => Some(icons::FLASK_MINUS_OUTLINE),
        "file-video" => Some(icons::FILE_VIDEO),
        "progress-pencil" => Some(icons::PROGRESS_PENCIL),
        "subtitles" => Some(icons::SUBTITLES),
        "head-minus-outline" => Some(icons::HEAD_MINUS_OUTLINE),
        "earbuds-outline" => Some(icons::EARBUDS_OUTLINE),
        "wallet-travel" => Some(icons::WALLET_TRAVEL),
        "movie-open-outline" => Some(icons::MOVIE_OPEN_OUTLINE),
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "archive-edit" => Some(icons::ARCHIVE_EDIT),
        "lock-smart" => Some(icons::LOCK_SMART),
        "wifi-sync" => Some(icons::WIFI_SYNC),
        "alpha-t-box" => Some(icons::ALPHA_T_BOX),
        "pretzel" => Some(icons::PRETZEL),
        "mushroom-off-outline" => Some(icons::MUSHROOM_OFF_OUTLINE),
        "selection-off" => Some(icons::SELECTION_OFF),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        "qrcode-remove" => Some(icons::QRCODE_REMOVE),
        "window-shutter-alert" => Some(icons::WINDOW_SHUTTER_ALERT),
        "bottle-wine" => Some(icons::BOTTLE_WINE),
        "bell-sleep" => Some(icons::BELL_SLEEP),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "hexagon-multiple" => Some(icons::HEXAGON_MULTIPLE),
        "folder-heart" => Some(icons::FOLDER_HEART),
        "calendar-check" => Some(icons::CALENDAR_CHECK),
        "hail" => Some(icons::HAIL),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "water-alert-outline" => Some(icons::WATER_ALERT_OUTLINE),
        "shield-search" => Some(icons::SHIELD_SEARCH),
        "comment-processing" => Some(icons::COMMENT_PROCESSING),
        "watch-variant" => Some(icons::WATCH_VARIANT),
        "music-clef-alto" => Some(icons::MUSIC_CLEF_ALTO),
        "alpha-v" => Some(icons::ALPHA_V),
        "code-equal" => Some(icons::CODE_EQUAL),
        "table-column-plus-before" => Some(icons::TABLE_COLUMN_PLUS_BEFORE),
        "antenna" => Some(icons::ANTENNA),
        "upload-multiple-outline" => Some(icons::UPLOAD_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "odnoklassniki" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'odnoklassniki' is deprecated.").print(py);
            }
            Some(icons::ODNOKLASSNIKI)
        }
        "blinds-horizontal" => Some(icons::BLINDS_HORIZONTAL),
        "roller-skate-off" => Some(icons::ROLLER_SKATE_OFF),
        "archive-off-outline" => Some(icons::ARCHIVE_OFF_OUTLINE),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "fridge-alert" => Some(icons::FRIDGE_ALERT),
        "folder-star" => Some(icons::FOLDER_STAR),
        "airplane-check" => Some(icons::AIRPLANE_CHECK),
        "pasta" => Some(icons::PASTA),
        "axis-y-rotate-counterclockwise" => Some(icons::AXIS_Y_ROTATE_COUNTERCLOCKWISE),
        "sitemap-outline" => Some(icons::SITEMAP_OUTLINE),
        "note-plus" => Some(icons::NOTE_PLUS),
        "database-arrow-left" => Some(icons::DATABASE_ARROW_LEFT),
        "comma" => Some(icons::COMMA),
        "arrow-left-drop-circle" => Some(icons::ARROW_LEFT_DROP_CIRCLE),
        "safety-goggles" => Some(icons::SAFETY_GOGGLES),
        "checkbook" => Some(icons::CHECKBOOK),
        "head-minus" => Some(icons::HEAD_MINUS),
        "heart-plus-outline" => Some(icons::HEART_PLUS_OUTLINE),
        #[allow(deprecated)]
        "deviantart" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'deviantart' is deprecated.").print(py);
            }
            Some(icons::DEVIANTART)
        }
        "anchor" => Some(icons::ANCHOR),
        "human-male-female" => Some(icons::HUMAN_MALE_FEMALE),
        "creation" => Some(icons::CREATION),
        "human-non-binary" => Some(icons::HUMAN_NON_BINARY),
        "unfold-less-horizontal" => Some(icons::UNFOLD_LESS_HORIZONTAL),
        "water" => Some(icons::WATER),
        "size-l" => Some(icons::SIZE_L),
        "printer-pos-star-outline" => Some(icons::PRINTER_POS_STAR_OUTLINE),
        "alpha-z-circle-outline" => Some(icons::ALPHA_Z_CIRCLE_OUTLINE),
        "broadcast" => Some(icons::BROADCAST),
        "ssh" => Some(icons::SSH),
        "arrow-decision-auto" => Some(icons::ARROW_DECISION_AUTO),
        "format-text-wrapping-overflow" => Some(icons::FORMAT_TEXT_WRAPPING_OVERFLOW),
        "chili-hot" => Some(icons::CHILI_HOT),
        "ufo" => Some(icons::UFO),
        "alpha-b-circle-outline" => Some(icons::ALPHA_B_CIRCLE_OUTLINE),
        "receipt" => Some(icons::RECEIPT),
        "file-table-box-multiple-outline" => Some(icons::FILE_TABLE_BOX_MULTIPLE_OUTLINE),
        "tooltip-check" => Some(icons::TOOLTIP_CHECK),
        "ev-plug-tesla" => Some(icons::EV_PLUG_TESLA),
        "fish" => Some(icons::FISH),
        "sign-direction" => Some(icons::SIGN_DIRECTION),
        "link" => Some(icons::LINK),
        #[allow(deprecated)]
        "gog" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gog' is deprecated.").print(py);
            }
            Some(icons::GOG)
        }
        "weather-night-partly-cloudy" => Some(icons::WEATHER_NIGHT_PARTLY_CLOUDY),
        "plus-circle-outline" => Some(icons::PLUS_CIRCLE_OUTLINE),
        "cards-playing-heart-multiple" => Some(icons::CARDS_PLAYING_HEART_MULTIPLE),
        _ => None,
    }
}
