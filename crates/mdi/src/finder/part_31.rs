// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_31(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "battery-30" => Some(icons::BATTERY_30),
        "mini-sd" => Some(icons::MINI_SD),
        "content-save-settings-outline" => Some(icons::CONTENT_SAVE_SETTINGS_OUTLINE),
        "bed-single-outline" => Some(icons::BED_SINGLE_OUTLINE),
        "cellphone-message-off" => Some(icons::CELLPHONE_MESSAGE_OFF),
        "message-off-outline" => Some(icons::MESSAGE_OFF_OUTLINE),
        "account-cash-outline" => Some(icons::ACCOUNT_CASH_OUTLINE),
        "battery-alert-variant" => Some(icons::BATTERY_ALERT_VARIANT),
        "turnstile" => Some(icons::TURNSTILE),
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "archive-arrow-down-outline" => Some(icons::ARCHIVE_ARROW_DOWN_OUTLINE),
        "dns-outline" => Some(icons::DNS_OUTLINE),
        "vhs" => Some(icons::VHS),
        "spray-bottle" => Some(icons::SPRAY_BOTTLE),
        "balcony" => Some(icons::BALCONY),
        "database-arrow-left-outline" => Some(icons::DATABASE_ARROW_LEFT_OUTLINE),
        "blinds-vertical" => Some(icons::BLINDS_VERTICAL),
        "bulldozer" => Some(icons::BULLDOZER),
        "remote-tv" => Some(icons::REMOTE_TV),
        "wifi-strength-3-lock-open" => Some(icons::WIFI_STRENGTH_3_LOCK_OPEN),
        "folder-music" => Some(icons::FOLDER_MUSIC),
        "file-rotate-left-outline" => Some(icons::FILE_ROTATE_LEFT_OUTLINE),
        "home-group" => Some(icons::HOME_GROUP),
        "arrow-collapse-horizontal" => Some(icons::ARROW_COLLAPSE_HORIZONTAL),
        "syllabary-hiragana" => Some(icons::SYLLABARY_HIRAGANA),
        "flask-off-outline" => Some(icons::FLASK_OFF_OUTLINE),
        "vector-point-plus" => Some(icons::VECTOR_POINT_PLUS),
        "plus-minus-box" => Some(icons::PLUS_MINUS_BOX),
        "alpha-z-circle-outline" => Some(icons::ALPHA_Z_CIRCLE_OUTLINE),
        "coffee" => Some(icons::COFFEE),
        "file-word-outline" => Some(icons::FILE_WORD_OUTLINE),
        "circle-half-full" => Some(icons::CIRCLE_HALF_FULL),
        "delta" => Some(icons::DELTA),
        "fridge-industrial-alert" => Some(icons::FRIDGE_INDUSTRIAL_ALERT),
        "skateboard" => Some(icons::SKATEBOARD),
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "comment-flash" => Some(icons::COMMENT_FLASH),
        "atom-variant" => Some(icons::ATOM_VARIANT),
        "phone-sync-outline" => Some(icons::PHONE_SYNC_OUTLINE),
        "magnify-scan" => Some(icons::MAGNIFY_SCAN),
        "map-marker-star" => Some(icons::MAP_MARKER_STAR),
        "backspace-reverse-outline" => Some(icons::BACKSPACE_REVERSE_OUTLINE),
        "lightbulb-on-50" => Some(icons::LIGHTBULB_ON_50),
        "timer-lock-outline" => Some(icons::TIMER_LOCK_OUTLINE),
        "signal-variant" => Some(icons::SIGNAL_VARIANT),
        "battery-charging-40" => Some(icons::BATTERY_CHARGING_40),
        "outdoor-lamp" => Some(icons::OUTDOOR_LAMP),
        "message-cog-outline" => Some(icons::MESSAGE_COG_OUTLINE),
        "radio-handheld" => Some(icons::RADIO_HANDHELD),
        "roman-numeral-6" => Some(icons::ROMAN_NUMERAL_6),
        "bell-plus-outline" => Some(icons::BELL_PLUS_OUTLINE),
        "skull-scan-outline" => Some(icons::SKULL_SCAN_OUTLINE),
        "note-alert" => Some(icons::NOTE_ALERT),
        "movie-roll" => Some(icons::MOVIE_ROLL),
        "cloud-minus-outline" => Some(icons::CLOUD_MINUS_OUTLINE),
        "text-short" => Some(icons::TEXT_SHORT),
        "timer-cancel" => Some(icons::TIMER_CANCEL),
        "invoice-fast-outline" => Some(icons::INVOICE_FAST_OUTLINE),
        "toy-brick-plus-outline" => Some(icons::TOY_BRICK_PLUS_OUTLINE),
        "flower-pollen" => Some(icons::FLOWER_POLLEN),
        "share-outline" => Some(icons::SHARE_OUTLINE),
        "hub" => Some(icons::HUB),
        "zip-box" => Some(icons::ZIP_BOX),
        "battery-alert-bluetooth" => Some(icons::BATTERY_ALERT_BLUETOOTH),
        "fruit-watermelon" => Some(icons::FRUIT_WATERMELON),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "checkerboard" => Some(icons::CHECKERBOARD),
        "timer-sand-empty" => Some(icons::TIMER_SAND_EMPTY),
        "fireplace-off" => Some(icons::FIREPLACE_OFF),
        "go-kart" => Some(icons::GO_KART),
        "receipt-text-remove-outline" => Some(icons::RECEIPT_TEXT_REMOVE_OUTLINE),
        "octahedron" => Some(icons::OCTAHEDRON),
        "card-multiple-outline" => Some(icons::CARD_MULTIPLE_OUTLINE),
        "emoticon-minus" => Some(icons::EMOTICON_MINUS),
        "shield-remove-outline" => Some(icons::SHIELD_REMOVE_OUTLINE),
        "food-hot-dog" => Some(icons::FOOD_HOT_DOG),
        "format-vertical-align-center" => Some(icons::FORMAT_VERTICAL_ALIGN_CENTER),
        "beaker-remove-outline" => Some(icons::BEAKER_REMOVE_OUTLINE),
        #[allow(deprecated)]
        "debian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'debian' is deprecated.").print(py);
            }
            Some(icons::DEBIAN)
        }
        "lightbulb-multiple" => Some(icons::LIGHTBULB_MULTIPLE),
        "format-size" => Some(icons::FORMAT_SIZE),
        "pitchfork" => Some(icons::PITCHFORK),
        "bike-pedal-clipless" => Some(icons::BIKE_PEDAL_CLIPLESS),
        "heat-pump" => Some(icons::HEAT_PUMP),
        "battery-charging-10" => Some(icons::BATTERY_CHARGING_10),
        "alert-circle-check-outline" => Some(icons::ALERT_CIRCLE_CHECK_OUTLINE),
        "arrow-down-left" => Some(icons::ARROW_DOWN_LEFT),
        #[allow(deprecated)]
        "microsoft-outlook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-outlook' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_OUTLOOK)
        }
        "cloud-refresh-variant-outline" => Some(icons::CLOUD_REFRESH_VARIANT_OUTLINE),
        "alphabet-aurebesh" => Some(icons::ALPHABET_AUREBESH),
        "store-off" => Some(icons::STORE_OFF),
        "account" => Some(icons::ACCOUNT),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "flip-horizontal" => Some(icons::FLIP_HORIZONTAL),
        "lotion-plus" => Some(icons::LOTION_PLUS),
        "cellphone-key" => Some(icons::CELLPHONE_KEY),
        "script-text" => Some(icons::SCRIPT_TEXT),
        "axis-z-rotate-clockwise" => Some(icons::AXIS_Z_ROTATE_CLOCKWISE),
        "message-badge" => Some(icons::MESSAGE_BADGE),
        "alpha-a-box-outline" => Some(icons::ALPHA_A_BOX_OUTLINE),
        "flare" => Some(icons::FLARE),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "format-align-right" => Some(icons::FORMAT_ALIGN_RIGHT),
        "shark-fin-outline" => Some(icons::SHARK_FIN_OUTLINE),
        "diving-snorkel" => Some(icons::DIVING_SNORKEL),
        "subtitles" => Some(icons::SUBTITLES),
        "video-off" => Some(icons::VIDEO_OFF),
        "arrow-top-right-bold-box-outline" => Some(icons::ARROW_TOP_RIGHT_BOLD_BOX_OUTLINE),
        "notebook-minus-outline" => Some(icons::NOTEBOOK_MINUS_OUTLINE),
        "arrow-down-bold-circle-outline" => Some(icons::ARROW_DOWN_BOLD_CIRCLE_OUTLINE),
        "percent-outline" => Some(icons::PERCENT_OUTLINE),
        "vector-point-select" => Some(icons::VECTOR_POINT_SELECT),
        "numeric-7-box-outline" => Some(icons::NUMERIC_7_BOX_OUTLINE),
        "account-convert-outline" => Some(icons::ACCOUNT_CONVERT_OUTLINE),
        "shape-rectangle-plus" => Some(icons::SHAPE_RECTANGLE_PLUS),
        "printer-3d-nozzle-heat-outline" => Some(icons::PRINTER_3D_NOZZLE_HEAT_OUTLINE),
        "octagram-edit-outline" => Some(icons::OCTAGRAM_EDIT_OUTLINE),
        "book-information-variant" => Some(icons::BOOK_INFORMATION_VARIANT),
        "clock-outline" => Some(icons::CLOCK_OUTLINE),
        "airballoon-outline" => Some(icons::AIRBALLOON_OUTLINE),
        "printer-pos-play" => Some(icons::PRINTER_POS_PLAY),
        "hexagon-slice-2" => Some(icons::HEXAGON_SLICE_2),
        "window-open-variant" => Some(icons::WINDOW_OPEN_VARIANT),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "view-day" => Some(icons::VIEW_DAY),
        "size-s" => Some(icons::SIZE_S),
        "incognito-circle-off" => Some(icons::INCOGNITO_CIRCLE_OFF),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "battery-40" => Some(icons::BATTERY_40),
        "watch-export" => Some(icons::WATCH_EXPORT),
        "file-pdf-box" => Some(icons::FILE_PDF_BOX),
        "sticker-text-outline" => Some(icons::STICKER_TEXT_OUTLINE),
        "file-move-outline" => Some(icons::FILE_MOVE_OUTLINE),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "television-box" => Some(icons::TELEVISION_BOX),
        "phone-dial" => Some(icons::PHONE_DIAL),
        "car-windshield-outline" => Some(icons::CAR_WINDSHIELD_OUTLINE),
        "account-tie-hat" => Some(icons::ACCOUNT_TIE_HAT),
        "shape-circle-plus" => Some(icons::SHAPE_CIRCLE_PLUS),
        "text-box-edit-outline" => Some(icons::TEXT_BOX_EDIT_OUTLINE),
        "relation-zero-or-many-to-zero-or-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_ONE),
        "star-box-outline" => Some(icons::STAR_BOX_OUTLINE),
        "car-convertible" => Some(icons::CAR_CONVERTIBLE),
        "basket-off-outline" => Some(icons::BASKET_OFF_OUTLINE),
        "sawtooth-wave" => Some(icons::SAWTOOTH_WAVE),
        "pencil-box-outline" => Some(icons::PENCIL_BOX_OUTLINE),
        "handball" => Some(icons::HANDBALL),
        "call-missed" => Some(icons::CALL_MISSED),
        "wifi-sync" => Some(icons::WIFI_SYNC),
        "cards-outline" => Some(icons::CARDS_OUTLINE),
        "sofa" => Some(icons::SOFA),
        "arrow-top-right" => Some(icons::ARROW_TOP_RIGHT),
        #[allow(deprecated)]
        "unity" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unity' is deprecated.").print(py);
            }
            Some(icons::UNITY)
        }
        "tsunami" => Some(icons::TSUNAMI),
        #[allow(deprecated)]
        "webpack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webpack' is deprecated.").print(py);
            }
            Some(icons::WEBPACK)
        }
        "account-alert" => Some(icons::ACCOUNT_ALERT),
        "format-italic" => Some(icons::FORMAT_ITALIC),
        "contacts-outline" => Some(icons::CONTACTS_OUTLINE),
        "play-outline" => Some(icons::PLAY_OUTLINE),
        "arrow-projectile-multiple" => Some(icons::ARROW_PROJECTILE_MULTIPLE),
        "flask-outline" => Some(icons::FLASK_OUTLINE),
        "bank-off-outline" => Some(icons::BANK_OFF_OUTLINE),
        "gender-transgender" => Some(icons::GENDER_TRANSGENDER),
        "thermometer-minus" => Some(icons::THERMOMETER_MINUS),
        "credit-card-check-outline" => Some(icons::CREDIT_CARD_CHECK_OUTLINE),
        "radiobox-marked" => Some(icons::RADIOBOX_MARKED),
        "invoice-text-check-outline" => Some(icons::INVOICE_TEXT_CHECK_OUTLINE),
        #[allow(deprecated)]
        "kickstarter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kickstarter' is deprecated.").print(py);
            }
            Some(icons::KICKSTARTER)
        }
        "baseball-bat" => Some(icons::BASEBALL_BAT),
        "cards" => Some(icons::CARDS),
        "storage-tank-outline" => Some(icons::STORAGE_TANK_OUTLINE),
        "table-of-contents" => Some(icons::TABLE_OF_CONTENTS),
        "account-network" => Some(icons::ACCOUNT_NETWORK),
        "image-off-outline" => Some(icons::IMAGE_OFF_OUTLINE),
        "bug-pause" => Some(icons::BUG_PAUSE),
        "arrow-down-right-bold" => Some(icons::ARROW_DOWN_RIGHT_BOLD),
        "bullhorn-variant" => Some(icons::BULLHORN_VARIANT),
        "notebook-minus" => Some(icons::NOTEBOOK_MINUS),
        "select-place" => Some(icons::SELECT_PLACE),
        "gantry-crane" => Some(icons::GANTRY_CRANE),
        "ray-start" => Some(icons::RAY_START),
        "food-takeout-box" => Some(icons::FOOD_TAKEOUT_BOX),
        "emoticon-wink" => Some(icons::EMOTICON_WINK),
        "shield-check" => Some(icons::SHIELD_CHECK),
        "format-quote-open-outline" => Some(icons::FORMAT_QUOTE_OPEN_OUTLINE),
        "cookie-clock-outline" => Some(icons::COOKIE_CLOCK_OUTLINE),
        "archive-search-outline" => Some(icons::ARCHIVE_SEARCH_OUTLINE),
        "tram-side" => Some(icons::TRAM_SIDE),
        "ear-hearing-off" => Some(icons::EAR_HEARING_OFF),
        "message-question-outline" => Some(icons::MESSAGE_QUESTION_OUTLINE),
        "notebook-check-outline" => Some(icons::NOTEBOOK_CHECK_OUTLINE),
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "video-wireless" => Some(icons::VIDEO_WIRELESS),
        "file-jpg-box" => Some(icons::FILE_JPG_BOX),
        "chart-bell-curve" => Some(icons::CHART_BELL_CURVE),
        "alpha-b-circle-outline" => Some(icons::ALPHA_B_CIRCLE_OUTLINE),
        "wifi-marker" => Some(icons::WIFI_MARKER),
        "card-bulleted-outline" => Some(icons::CARD_BULLETED_OUTLINE),
        "timeline-text-outline" => Some(icons::TIMELINE_TEXT_OUTLINE),
        "flower-tulip-outline" => Some(icons::FLOWER_TULIP_OUTLINE),
        _ => None,
    }
}
