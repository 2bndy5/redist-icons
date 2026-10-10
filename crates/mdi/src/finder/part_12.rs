// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "step-forward-2" => Some(icons::STEP_FORWARD_2),
        "lightbulb-auto" => Some(icons::LIGHTBULB_AUTO),
        "comma-box-outline" => Some(icons::COMMA_BOX_OUTLINE),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "sheep" => Some(icons::SHEEP),
        "format-align-justify" => Some(icons::FORMAT_ALIGN_JUSTIFY),
        "lightning-bolt-outline" => Some(icons::LIGHTNING_BOLT_OUTLINE),
        "account-supervisor" => Some(icons::ACCOUNT_SUPERVISOR),
        #[allow(deprecated)]
        "plex" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'plex' is deprecated.").print(py);
            }
            Some(icons::PLEX)
        }
        "book-multiple-outline" => Some(icons::BOOK_MULTIPLE_OUTLINE),
        "numeric-6" => Some(icons::NUMERIC_6),
        "sprinkler-variant" => Some(icons::SPRINKLER_VARIANT),
        "mouse-bluetooth" => Some(icons::MOUSE_BLUETOOTH),
        "cookie-settings-outline" => Some(icons::COOKIE_SETTINGS_OUTLINE),
        "credit-card-refresh" => Some(icons::CREDIT_CARD_REFRESH),
        "coffee-outline" => Some(icons::COFFEE_OUTLINE),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "nature-people-outline" => Some(icons::NATURE_PEOPLE_OUTLINE),
        "numeric-6-circle-outline" => Some(icons::NUMERIC_6_CIRCLE_OUTLINE),
        "weather-night-partly-cloudy" => Some(icons::WEATHER_NIGHT_PARTLY_CLOUDY),
        "arrow-up-thin" => Some(icons::ARROW_UP_THIN),
        "alpha-q-circle-outline" => Some(icons::ALPHA_Q_CIRCLE_OUTLINE),
        "printer-pos-minus-outline" => Some(icons::PRINTER_POS_MINUS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-empty" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-empty' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_EMPTY)
        }
        "folder-search" => Some(icons::FOLDER_SEARCH),
        "credit-card-remove" => Some(icons::CREDIT_CARD_REMOVE),
        "cup-outline" => Some(icons::CUP_OUTLINE),
        "basketball-hoop-outline" => Some(icons::BASKETBALL_HOOP_OUTLINE),
        "pencil" => Some(icons::PENCIL),
        "fridge" => Some(icons::FRIDGE),
        "phone-minus-outline" => Some(icons::PHONE_MINUS_OUTLINE),
        "timer-sand-complete" => Some(icons::TIMER_SAND_COMPLETE),
        "radioactive-off" => Some(icons::RADIOACTIVE_OFF),
        "division" => Some(icons::DIVISION),
        "card-bulleted-settings" => Some(icons::CARD_BULLETED_SETTINGS),
        "wall-sconce-outline" => Some(icons::WALL_SCONCE_OUTLINE),
        "source-repository-multiple" => Some(icons::SOURCE_REPOSITORY_MULTIPLE),
        "water-alert-outline" => Some(icons::WATER_ALERT_OUTLINE),
        "math-norm" => Some(icons::MATH_NORM),
        "smog" => Some(icons::SMOG),
        "police-station" => Some(icons::POLICE_STATION),
        #[allow(deprecated)]
        "wechat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wechat' is deprecated.").print(py);
            }
            Some(icons::WECHAT)
        }
        "fan-alert" => Some(icons::FAN_ALERT),
        "upload-box" => Some(icons::UPLOAD_BOX),
        "alert-octagram-outline" => Some(icons::ALERT_OCTAGRAM_OUTLINE),
        "barn" => Some(icons::BARN),
        "home-silo-outline" => Some(icons::HOME_SILO_OUTLINE),
        "view-column" => Some(icons::VIEW_COLUMN),
        "web-plus" => Some(icons::WEB_PLUS),
        "camera-flip-outline" => Some(icons::CAMERA_FLIP_OUTLINE),
        "network-off-outline" => Some(icons::NETWORK_OFF_OUTLINE),
        "paperclip-remove" => Some(icons::PAPERCLIP_REMOVE),
        "earth-arrow-down" => Some(icons::EARTH_ARROW_DOWN),
        "qrcode" => Some(icons::QRCODE),
        "waterfall" => Some(icons::WATERFALL),
        "percent-box-outline" => Some(icons::PERCENT_BOX_OUTLINE),
        "wifi-strength-3" => Some(icons::WIFI_STRENGTH_3),
        "cursor-default" => Some(icons::CURSOR_DEFAULT),
        "phone-cancel-outline" => Some(icons::PHONE_CANCEL_OUTLINE),
        "garage-alert-variant" => Some(icons::GARAGE_ALERT_VARIANT),
        "battery-check-outline" => Some(icons::BATTERY_CHECK_OUTLINE),
        "puzzle-minus" => Some(icons::PUZZLE_MINUS),
        "timeline" => Some(icons::TIMELINE),
        "numeric-10-box-outline" => Some(icons::NUMERIC_10_BOX_OUTLINE),
        "alpha-q-circle" => Some(icons::ALPHA_Q_CIRCLE),
        "hand-wash-outline" => Some(icons::HAND_WASH_OUTLINE),
        "alpha-j-box" => Some(icons::ALPHA_J_BOX),
        "arrow-left-bold-box-outline" => Some(icons::ARROW_LEFT_BOLD_BOX_OUTLINE),
        "selection" => Some(icons::SELECTION),
        "church-outline" => Some(icons::CHURCH_OUTLINE),
        "hdmi-port" => Some(icons::HDMI_PORT),
        "shopping-outline" => Some(icons::SHOPPING_OUTLINE),
        "bug-stop" => Some(icons::BUG_STOP),
        "numeric-10-circle-outline" => Some(icons::NUMERIC_10_CIRCLE_OUTLINE),
        "wifi-arrow-left" => Some(icons::WIFI_ARROW_LEFT),
        "shuffle" => Some(icons::SHUFFLE),
        "format-page-split" => Some(icons::FORMAT_PAGE_SPLIT),
        "home-minus-outline" => Some(icons::HOME_MINUS_OUTLINE),
        "car-2-plus" => Some(icons::CAR_2_PLUS),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        "video-plus-outline" => Some(icons::VIDEO_PLUS_OUTLINE),
        "human-edit" => Some(icons::HUMAN_EDIT),
        "karate" => Some(icons::KARATE),
        "backspace-outline" => Some(icons::BACKSPACE_OUTLINE),
        "amplifier-off" => Some(icons::AMPLIFIER_OFF),
        "archive-sync-outline" => Some(icons::ARCHIVE_SYNC_OUTLINE),
        "bus-multiple" => Some(icons::BUS_MULTIPLE),
        "file-phone" => Some(icons::FILE_PHONE),
        "select-arrow-up" => Some(icons::SELECT_ARROW_UP),
        "bookmark-remove" => Some(icons::BOOKMARK_REMOVE),
        "train-car-passenger-door" => Some(icons::TRAIN_CAR_PASSENGER_DOOR),
        "credit-card-search-outline" => Some(icons::CREDIT_CARD_SEARCH_OUTLINE),
        "clipboard-text-play" => Some(icons::CLIPBOARD_TEXT_PLAY),
        "home-analytics" => Some(icons::HOME_ANALYTICS),
        "book-check" => Some(icons::BOOK_CHECK),
        "bottle-tonic" => Some(icons::BOTTLE_TONIC),
        "comment-text-multiple-outline" => Some(icons::COMMENT_TEXT_MULTIPLE_OUTLINE),
        "store-plus" => Some(icons::STORE_PLUS),
        "card-account-details-star" => Some(icons::CARD_ACCOUNT_DETAILS_STAR),
        #[allow(deprecated)]
        "nodejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nodejs' is deprecated.").print(py);
            }
            Some(icons::NODEJS)
        }
        "led-on" => Some(icons::LED_ON),
        "close" => Some(icons::CLOSE),
        "timer-alert" => Some(icons::TIMER_ALERT),
        "arrow-right-bold-box-outline" => Some(icons::ARROW_RIGHT_BOLD_BOX_OUTLINE),
        "sync" => Some(icons::SYNC),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        "folder-search-outline" => Some(icons::FOLDER_SEARCH_OUTLINE),
        "radiator-disabled" => Some(icons::RADIATOR_DISABLED),
        "thermometer-check" => Some(icons::THERMOMETER_CHECK),
        "arrow-collapse-down" => Some(icons::ARROW_COLLAPSE_DOWN),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "hand-wave-outline" => Some(icons::HAND_WAVE_OUTLINE),
        "relation-one-or-many-to-zero-or-many" => Some(icons::RELATION_ONE_OR_MANY_TO_ZERO_OR_MANY),
        "panorama-wide-angle" => Some(icons::PANORAMA_WIDE_ANGLE),
        "movie-edit" => Some(icons::MOVIE_EDIT),
        "movie-open-check" => Some(icons::MOVIE_OPEN_CHECK),
        "battery-lock" => Some(icons::BATTERY_LOCK),
        "braille" => Some(icons::BRAILLE),
        "domain-off" => Some(icons::DOMAIN_OFF),
        "comment-edit-outline" => Some(icons::COMMENT_EDIT_OUTLINE),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "key-outline" => Some(icons::KEY_OUTLINE),
        "reorder-horizontal" => Some(icons::REORDER_HORIZONTAL),
        "stamper" => Some(icons::STAMPER),
        "expand-all-outline" => Some(icons::EXPAND_ALL_OUTLINE),
        "stethoscope" => Some(icons::STETHOSCOPE),
        "weather-cloudy-arrow-right" => Some(icons::WEATHER_CLOUDY_ARROW_RIGHT),
        "code-parentheses-box" => Some(icons::CODE_PARENTHESES_BOX),
        "search-web" => Some(icons::SEARCH_WEB),
        "image-filter-black-white" => Some(icons::IMAGE_FILTER_BLACK_WHITE),
        "arrow-down-box" => Some(icons::ARROW_DOWN_BOX),
        "credit-card-multiple-outline" => Some(icons::CREDIT_CARD_MULTIPLE_OUTLINE),
        "snowshoeing" => Some(icons::SNOWSHOEING),
        "cloud-alert-outline" => Some(icons::CLOUD_ALERT_OUTLINE),
        "ray-start-end" => Some(icons::RAY_START_END),
        "file-cloud" => Some(icons::FILE_CLOUD),
        "server" => Some(icons::SERVER),
        "golf" => Some(icons::GOLF),
        "sort-bool-descending" => Some(icons::SORT_BOOL_DESCENDING),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "format-color-text" => Some(icons::FORMAT_COLOR_TEXT),
        "run-fast" => Some(icons::RUN_FAST),
        "set-all" => Some(icons::SET_ALL),
        "arrow-down-thick" => Some(icons::ARROW_DOWN_THICK),
        "printer-3d-nozzle-alert-outline" => Some(icons::PRINTER_3D_NOZZLE_ALERT_OUTLINE),
        "store-search" => Some(icons::STORE_SEARCH),
        "emoticon-plus-outline" => Some(icons::EMOTICON_PLUS_OUTLINE),
        "message-text-fast" => Some(icons::MESSAGE_TEXT_FAST),
        "resistor-nodes" => Some(icons::RESISTOR_NODES),
        "transcribe-close" => Some(icons::TRANSCRIBE_CLOSE),
        "receipt-text-plus-outline" => Some(icons::RECEIPT_TEXT_PLUS_OUTLINE),
        "play-box-lock" => Some(icons::PLAY_BOX_LOCK),
        "music-note-quarter" => Some(icons::MUSIC_NOTE_QUARTER),
        "home-edit" => Some(icons::HOME_EDIT),
        "pin-off" => Some(icons::PIN_OFF),
        "eye-arrow-left" => Some(icons::EYE_ARROW_LEFT),
        "tie" => Some(icons::TIE),
        "garage" => Some(icons::GARAGE),
        "comment-account-outline" => Some(icons::COMMENT_ACCOUNT_OUTLINE),
        "microphone-variant-off" => Some(icons::MICROPHONE_VARIANT_OFF),
        "television-play" => Some(icons::TELEVISION_PLAY),
        "camera-retake-outline" => Some(icons::CAMERA_RETAKE_OUTLINE),
        "furigana-horizontal" => Some(icons::FURIGANA_HORIZONTAL),
        #[allow(deprecated)]
        "apple" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple' is deprecated.").print(py);
            }
            Some(icons::APPLE)
        }
        "image-sync-outline" => Some(icons::IMAGE_SYNC_OUTLINE),
        "snowmobile" => Some(icons::SNOWMOBILE),
        #[allow(deprecated)]
        "mapbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mapbox' is deprecated.").print(py);
            }
            Some(icons::MAPBOX)
        }
        "basket-check" => Some(icons::BASKET_CHECK),
        "cellphone-link-off" => Some(icons::CELLPHONE_LINK_OFF),
        "map-marker-account-outline" => Some(icons::MAP_MARKER_ACCOUNT_OUTLINE),
        "widgets-outline" => Some(icons::WIDGETS_OUTLINE),
        "robot-mower" => Some(icons::ROBOT_MOWER),
        "radio-off" => Some(icons::RADIO_OFF),
        "projector" => Some(icons::PROJECTOR),
        "globe-light" => Some(icons::GLOBE_LIGHT),
        "align-vertical-center" => Some(icons::ALIGN_VERTICAL_CENTER),
        "altimeter" => Some(icons::ALTIMETER),
        "notebook-multiple" => Some(icons::NOTEBOOK_MULTIPLE),
        "signal-4g" => Some(icons::SIGNAL_4G),
        "chevron-triple-right" => Some(icons::CHEVRON_TRIPLE_RIGHT),
        "clipboard-list-outline" => Some(icons::CLIPBOARD_LIST_OUTLINE),
        "zodiac-scorpio" => Some(icons::ZODIAC_SCORPIO),
        "cart-outline" => Some(icons::CART_OUTLINE),
        "battery-charging" => Some(icons::BATTERY_CHARGING),
        #[allow(deprecated)]
        "language-swift" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-swift' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_SWIFT)
        }
        "message-text-clock" => Some(icons::MESSAGE_TEXT_CLOCK),
        "beaker-question" => Some(icons::BEAKER_QUESTION),
        "ceiling-fan-light" => Some(icons::CEILING_FAN_LIGHT),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "silverware" => Some(icons::SILVERWARE),
        "bucket-outline" => Some(icons::BUCKET_OUTLINE),
        "lightning-bolt" => Some(icons::LIGHTNING_BOLT),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "circle-outline" => Some(icons::CIRCLE_OUTLINE),
        "table-merge-cells" => Some(icons::TABLE_MERGE_CELLS),
        "pocket" => Some(icons::POCKET),
        "lambda" => Some(icons::LAMBDA),
        "food-outline" => Some(icons::FOOD_OUTLINE),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "barcode-off" => Some(icons::BARCODE_OFF),
        _ => None,
    }
}
