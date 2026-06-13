// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_3(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "table-row-plus-before" => Some(icons::TABLE_ROW_PLUS_BEFORE),
        "folder-check-outline" => Some(icons::FOLDER_CHECK_OUTLINE),
        "television-play" => Some(icons::TELEVISION_PLAY),
        "lock-open-remove-outline" => Some(icons::LOCK_OPEN_REMOVE_OUTLINE),
        "calendar-export" => Some(icons::CALENDAR_EXPORT),
        "crosshairs" => Some(icons::CROSSHAIRS),
        "timeline-clock" => Some(icons::TIMELINE_CLOCK),
        "link-variant-plus" => Some(icons::LINK_VARIANT_PLUS),
        "account-hard-hat-outline" => Some(icons::ACCOUNT_HARD_HAT_OUTLINE),
        #[allow(deprecated)]
        "atlassian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'atlassian' is deprecated.").print(py);
            }
            Some(icons::ATLASSIAN)
        }
        "gate-xnor" => Some(icons::GATE_XNOR),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "table-minus" => Some(icons::TABLE_MINUS),
        "content-save-move" => Some(icons::CONTENT_SAVE_MOVE),
        "hand-okay" => Some(icons::HAND_OKAY),
        "table-search" => Some(icons::TABLE_SEARCH),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "application-array" => Some(icons::APPLICATION_ARRAY),
        "timeline-question" => Some(icons::TIMELINE_QUESTION),
        "bookmark-plus" => Some(icons::BOOKMARK_PLUS),
        "mixed-martial-arts" => Some(icons::MIXED_MARTIAL_ARTS),
        "home-flood" => Some(icons::HOME_FLOOD),
        "clipboard" => Some(icons::CLIPBOARD),
        "cards-club-outline" => Some(icons::CARDS_CLUB_OUTLINE),
        "cards-spade" => Some(icons::CARDS_SPADE),
        "rename-outline" => Some(icons::RENAME_OUTLINE),
        "water-alert" => Some(icons::WATER_ALERT),
        "receipt-text-arrow-left" => Some(icons::RECEIPT_TEXT_ARROW_LEFT),
        "application-settings" => Some(icons::APPLICATION_SETTINGS),
        "arrow-top-right" => Some(icons::ARROW_TOP_RIGHT),
        "rhombus-medium-outline" => Some(icons::RHOMBUS_MEDIUM_OUTLINE),
        "message-bookmark-outline" => Some(icons::MESSAGE_BOOKMARK_OUTLINE),
        "human-male-male" => Some(icons::HUMAN_MALE_MALE),
        "trophy" => Some(icons::TROPHY),
        "smoke-detector-off-outline" => Some(icons::SMOKE_DETECTOR_OFF_OUTLINE),
        "cake-layered" => Some(icons::CAKE_LAYERED),
        "microscope" => Some(icons::MICROSCOPE),
        "math-log" => Some(icons::MATH_LOG),
        "battery-20-bluetooth" => Some(icons::BATTERY_20_BLUETOOTH),
        "projector-screen-variant" => Some(icons::PROJECTOR_SCREEN_VARIANT),
        "connection" => Some(icons::CONNECTION),
        "sort-bool-ascending-variant" => Some(icons::SORT_BOOL_ASCENDING_VARIANT),
        "home-automation" => Some(icons::HOME_AUTOMATION),
        "purse-outline" => Some(icons::PURSE_OUTLINE),
        "abugida-devanagari" => Some(icons::ABUGIDA_DEVANAGARI),
        "receipt-text-minus" => Some(icons::RECEIPT_TEXT_MINUS),
        "relation-zero-or-one-to-zero-or-many" => Some(icons::RELATION_ZERO_OR_ONE_TO_ZERO_OR_MANY),
        "firewire" => Some(icons::FIREWIRE),
        "lan-connect" => Some(icons::LAN_CONNECT),
        "water-pump-off" => Some(icons::WATER_PUMP_OFF),
        "power-socket-de" => Some(icons::POWER_SOCKET_DE),
        "bug-check" => Some(icons::BUG_CHECK),
        "checkerboard-remove" => Some(icons::CHECKERBOARD_REMOVE),
        "weather-cloudy-clock" => Some(icons::WEATHER_CLOUDY_CLOCK),
        "text-box-minus" => Some(icons::TEXT_BOX_MINUS),
        "skew-more" => Some(icons::SKEW_MORE),
        "dice-5-outline" => Some(icons::DICE_5_OUTLINE),
        "clipboard-check-outline" => Some(icons::CLIPBOARD_CHECK_OUTLINE),
        "bulletin-board" => Some(icons::BULLETIN_BOARD),
        "flower-tulip-outline" => Some(icons::FLOWER_TULIP_OUTLINE),
        "vector-ellipse" => Some(icons::VECTOR_ELLIPSE),
        "email-seal-outline" => Some(icons::EMAIL_SEAL_OUTLINE),
        "triangle-wave" => Some(icons::TRIANGLE_WAVE),
        "marker-check" => Some(icons::MARKER_CHECK),
        #[allow(deprecated)]
        "google-cardboard" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-cardboard' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_CARDBOARD)
        }
        "card-text-outline" => Some(icons::CARD_TEXT_OUTLINE),
        "sausage" => Some(icons::SAUSAGE),
        "sledding" => Some(icons::SLEDDING),
        "credit-card-clock" => Some(icons::CREDIT_CARD_CLOCK),
        "folder-file" => Some(icons::FOLDER_FILE),
        "battery-charging-80" => Some(icons::BATTERY_CHARGING_80),
        "golf-cart" => Some(icons::GOLF_CART),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-alert" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-alert' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_ALERT)
        }
        "alpha-l-box-outline" => Some(icons::ALPHA_L_BOX_OUTLINE),
        "arrow-down-right-bold" => Some(icons::ARROW_DOWN_RIGHT_BOLD),
        "table-refresh" => Some(icons::TABLE_REFRESH),
        "checkbox-multiple-marked-circle-outline" => {
            Some(icons::CHECKBOX_MULTIPLE_MARKED_CIRCLE_OUTLINE)
        }
        "sun-clock-outline" => Some(icons::SUN_CLOCK_OUTLINE),
        "border-horizontal" => Some(icons::BORDER_HORIZONTAL),
        "door-sliding-open" => Some(icons::DOOR_SLIDING_OPEN),
        #[allow(deprecated)]
        "apple-finder" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-finder' is deprecated.").print(py);
            }
            Some(icons::APPLE_FINDER)
        }
        "kettle-alert" => Some(icons::KETTLE_ALERT),
        "calendar-check-outline" => Some(icons::CALENDAR_CHECK_OUTLINE),
        "view-grid-compact" => Some(icons::VIEW_GRID_COMPACT),
        "arrow-u-right-bottom" => Some(icons::ARROW_U_RIGHT_BOTTOM),
        "video-switch" => Some(icons::VIDEO_SWITCH),
        "stethoscope" => Some(icons::STETHOSCOPE),
        "format-subscript" => Some(icons::FORMAT_SUBSCRIPT),
        #[allow(deprecated)]
        "manjaro" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'manjaro' is deprecated.").print(py);
            }
            Some(icons::MANJARO)
        }
        "file-phone" => Some(icons::FILE_PHONE),
        #[allow(deprecated)]
        "language-ruby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUBY)
        }
        "controller" => Some(icons::CONTROLLER),
        "align-horizontal-right" => Some(icons::ALIGN_HORIZONTAL_RIGHT),
        "cloud-cancel" => Some(icons::CLOUD_CANCEL),
        "cigar-off" => Some(icons::CIGAR_OFF),
        "flower-pollen-outline" => Some(icons::FLOWER_POLLEN_OUTLINE),
        "account-question-outline" => Some(icons::ACCOUNT_QUESTION_OUTLINE),
        "water-remove-outline" => Some(icons::WATER_REMOVE_OUTLINE),
        "format-letter-spacing-variant" => Some(icons::FORMAT_LETTER_SPACING_VARIANT),
        "ski" => Some(icons::SKI),
        "text-box-remove-outline" => Some(icons::TEXT_BOX_REMOVE_OUTLINE),
        "all-inclusive-box-outline" => Some(icons::ALL_INCLUSIVE_BOX_OUTLINE),
        "briefcase-variant-off-outline" => Some(icons::BRIEFCASE_VARIANT_OFF_OUTLINE),
        "notebook-check" => Some(icons::NOTEBOOK_CHECK),
        "fast-forward-45" => Some(icons::FAST_FORWARD_45),
        "skull" => Some(icons::SKULL),
        "flask-empty-off" => Some(icons::FLASK_EMPTY_OFF),
        "thermometer-lines" => Some(icons::THERMOMETER_LINES),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "magazine-rifle" => Some(icons::MAGAZINE_RIFLE),
        "account-arrow-right" => Some(icons::ACCOUNT_ARROW_RIGHT),
        "rolodex-outline" => Some(icons::ROLODEX_OUTLINE),
        "hospital-marker" => Some(icons::HOSPITAL_MARKER),
        "tag-search" => Some(icons::TAG_SEARCH),
        "flag-remove-outline" => Some(icons::FLAG_REMOVE_OUTLINE),
        "map-marker-right" => Some(icons::MAP_MARKER_RIGHT),
        "angle-right" => Some(icons::ANGLE_RIGHT),
        "account-credit-card" => Some(icons::ACCOUNT_CREDIT_CARD),
        "food-variant-off" => Some(icons::FOOD_VARIANT_OFF),
        "keyboard-outline" => Some(icons::KEYBOARD_OUTLINE),
        "duck" => Some(icons::DUCK),
        "invoice-text-fast" => Some(icons::INVOICE_TEXT_FAST),
        "tent" => Some(icons::TENT),
        "watch-export-variant" => Some(icons::WATCH_EXPORT_VARIANT),
        "hand-back-right-off-outline" => Some(icons::HAND_BACK_RIGHT_OFF_OUTLINE),
        "alert" => Some(icons::ALERT),
        "progress-star" => Some(icons::PROGRESS_STAR),
        "gas-burner" => Some(icons::GAS_BURNER),
        "play-protected-content" => Some(icons::PLAY_PROTECTED_CONTENT),
        "led-off" => Some(icons::LED_OFF),
        "gamepad" => Some(icons::GAMEPAD),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "printer-pos-sync" => Some(icons::PRINTER_POS_SYNC),
        "numeric-2-box-multiple-outline" => Some(icons::NUMERIC_2_BOX_MULTIPLE_OUTLINE),
        "timer-cog-outline" => Some(icons::TIMER_COG_OUTLINE),
        "qrcode" => Some(icons::QRCODE),
        "numeric-3-box" => Some(icons::NUMERIC_3_BOX),
        "format-float-right" => Some(icons::FORMAT_FLOAT_RIGHT),
        "rolodex" => Some(icons::ROLODEX),
        "home-off" => Some(icons::HOME_OFF),
        "export-variant" => Some(icons::EXPORT_VARIANT),
        "folder-download" => Some(icons::FOLDER_DOWNLOAD),
        "cloud-arrow-left" => Some(icons::CLOUD_ARROW_LEFT),
        "head" => Some(icons::HEAD),
        "heart-outline" => Some(icons::HEART_OUTLINE),
        "food-steak" => Some(icons::FOOD_STEAK),
        "projector-screen-variant-outline" => Some(icons::PROJECTOR_SCREEN_VARIANT_OUTLINE),
        "arrow-down-bold" => Some(icons::ARROW_DOWN_BOLD),
        "selection-remove" => Some(icons::SELECTION_REMOVE),
        "pause-octagon" => Some(icons::PAUSE_OCTAGON),
        "map-marker-multiple" => Some(icons::MAP_MARKER_MULTIPLE),
        "alpha-s-box" => Some(icons::ALPHA_S_BOX),
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "signal-variant" => Some(icons::SIGNAL_VARIANT),
        "shape-square-rounded-plus" => Some(icons::SHAPE_SQUARE_ROUNDED_PLUS),
        "database-remove" => Some(icons::DATABASE_REMOVE),
        "fast-forward-15" => Some(icons::FAST_FORWARD_15),
        #[allow(deprecated)]
        "microsoft-azure-devops" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-azure-devops' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_AZURE_DEVOPS)
        }
        "head-plus-outline" => Some(icons::HEAD_PLUS_OUTLINE),
        "movie-remove" => Some(icons::MOVIE_REMOVE),
        "weather-snowy" => Some(icons::WEATHER_SNOWY),
        "epsilon" => Some(icons::EPSILON),
        "file-star-four-points" => Some(icons::FILE_STAR_FOUR_POINTS),
        "ocr" => Some(icons::OCR),
        "flash-outline" => Some(icons::FLASH_OUTLINE),
        "text-box-plus-outline" => Some(icons::TEXT_BOX_PLUS_OUTLINE),
        "vector-difference" => Some(icons::VECTOR_DIFFERENCE),
        "gift-open-outline" => Some(icons::GIFT_OPEN_OUTLINE),
        "lock-minus" => Some(icons::LOCK_MINUS),
        "folder-upload-outline" => Some(icons::FOLDER_UPLOAD_OUTLINE),
        "calendar-week-begin-outline" => Some(icons::CALENDAR_WEEK_BEGIN_OUTLINE),
        "book-clock" => Some(icons::BOOK_CLOCK),
        "star-remove" => Some(icons::STAR_REMOVE),
        "card-remove-outline" => Some(icons::CARD_REMOVE_OUTLINE),
        "battery-80" => Some(icons::BATTERY_80),
        "chart-tree" => Some(icons::CHART_TREE),
        "filter-variant-remove" => Some(icons::FILTER_VARIANT_REMOVE),
        "briefcase-off" => Some(icons::BRIEFCASE_OFF),
        "cloud-arrow-up" => Some(icons::CLOUD_ARROW_UP),
        "head-cog" => Some(icons::HEAD_COG),
        "circle-opacity" => Some(icons::CIRCLE_OPACITY),
        "food-drumstick-off-outline" => Some(icons::FOOD_DRUMSTICK_OFF_OUTLINE),
        "relation-one-to-only-one" => Some(icons::RELATION_ONE_TO_ONLY_ONE),
        "calendar-question-outline" => Some(icons::CALENDAR_QUESTION_OUTLINE),
        "sign-real-estate" => Some(icons::SIGN_REAL_ESTATE),
        "lingerie" => Some(icons::LINGERIE),
        "ice-cream" => Some(icons::ICE_CREAM),
        "ghost" => Some(icons::GHOST),
        "tag-plus-outline" => Some(icons::TAG_PLUS_OUTLINE),
        "folder-arrow-down" => Some(icons::FOLDER_ARROW_DOWN),
        "saw-blade" => Some(icons::SAW_BLADE),
        "printer-pos-plus" => Some(icons::PRINTER_POS_PLUS),
        "web-minus" => Some(icons::WEB_MINUS),
        "note-multiple-outline" => Some(icons::NOTE_MULTIPLE_OUTLINE),
        "hospital-box" => Some(icons::HOSPITAL_BOX),
        "label-off-outline" => Some(icons::LABEL_OFF_OUTLINE),
        "sort-calendar-ascending" => Some(icons::SORT_CALENDAR_ASCENDING),
        "ray-end" => Some(icons::RAY_END),
        #[allow(deprecated)]
        "google-analytics" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-analytics' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ANALYTICS)
        }
        "alpha-e" => Some(icons::ALPHA_E),
        _ => None,
    }
}
