// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_33(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "silverware" => Some(icons::SILVERWARE),
        "arrow-right-circle-outline" => Some(icons::ARROW_RIGHT_CIRCLE_OUTLINE),
        "alert-plus" => Some(icons::ALERT_PLUS),
        "comment-text-outline" => Some(icons::COMMENT_TEXT_OUTLINE),
        #[allow(deprecated)]
        "material-ui" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-ui' is deprecated.").print(py);
            }
            Some(icons::MATERIAL_UI)
        }
        #[allow(deprecated)]
        "facebook-messenger" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-messenger' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_MESSENGER)
        }
        "printer-alert" => Some(icons::PRINTER_ALERT),
        "angle-obtuse" => Some(icons::ANGLE_OBTUSE),
        "source-branch-sync" => Some(icons::SOURCE_BRANCH_SYNC),
        "arrow-left-bold-box-outline" => Some(icons::ARROW_LEFT_BOLD_BOX_OUTLINE),
        "lightning-bolt" => Some(icons::LIGHTNING_BOLT),
        "border-style" => Some(icons::BORDER_STYLE),
        "cash-plus" => Some(icons::CASH_PLUS),
        "cards-variant" => Some(icons::CARDS_VARIANT),
        "order-bool-ascending-variant" => Some(icons::ORDER_BOOL_ASCENDING_VARIANT),
        "ammunition" => Some(icons::AMMUNITION),
        "phone-remove" => Some(icons::PHONE_REMOVE),
        "micro-sd" => Some(icons::MICRO_SD),
        "battery-90-bluetooth" => Some(icons::BATTERY_90_BLUETOOTH),
        "monitor-dashboard" => Some(icons::MONITOR_DASHBOARD),
        "account-arrow-right-outline" => Some(icons::ACCOUNT_ARROW_RIGHT_OUTLINE),
        "filter-multiple-outline" => Some(icons::FILTER_MULTIPLE_OUTLINE),
        "store-outline" => Some(icons::STORE_OUTLINE),
        "format-paragraph-spacing" => Some(icons::FORMAT_PARAGRAPH_SPACING),
        "relation-only-one-to-one-or-many" => Some(icons::RELATION_ONLY_ONE_TO_ONE_OR_MANY),
        "ultra-high-definition" => Some(icons::ULTRA_HIGH_DEFINITION),
        #[allow(deprecated)]
        "arch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'arch' is deprecated.").print(py);
            }
            Some(icons::ARCH)
        }
        "numeric-4" => Some(icons::NUMERIC_4),
        "router-network" => Some(icons::ROUTER_NETWORK),
        "car-lifted-pickup" => Some(icons::CAR_LIFTED_PICKUP),
        "store-check" => Some(icons::STORE_CHECK),
        "head-lightbulb-outline" => Some(icons::HEAD_LIGHTBULB_OUTLINE),
        "submarine" => Some(icons::SUBMARINE),
        "calendar-lock-open" => Some(icons::CALENDAR_LOCK_OPEN),
        "bell-sleep-outline" => Some(icons::BELL_SLEEP_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-charging" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-charging' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_CHARGING)
        }
        "battery-charging-10" => Some(icons::BATTERY_CHARGING_10),
        "puzzle-minus-outline" => Some(icons::PUZZLE_MINUS_OUTLINE),
        "puzzle-star-outline" => Some(icons::PUZZLE_STAR_OUTLINE),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "football-australian" => Some(icons::FOOTBALL_AUSTRALIAN),
        "plus-network" => Some(icons::PLUS_NETWORK),
        #[allow(deprecated)]
        "jquery" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jquery' is deprecated.").print(py);
            }
            Some(icons::JQUERY)
        }
        "lightbulb-on" => Some(icons::LIGHTBULB_ON),
        "circle-box-outline" => Some(icons::CIRCLE_BOX_OUTLINE),
        "undo-variant" => Some(icons::UNDO_VARIANT),
        "scoreboard" => Some(icons::SCOREBOARD),
        "close-circle" => Some(icons::CLOSE_CIRCLE),
        "shield-refresh" => Some(icons::SHIELD_REFRESH),
        "page-layout-body" => Some(icons::PAGE_LAYOUT_BODY),
        "hand-back-left-off" => Some(icons::HAND_BACK_LEFT_OFF),
        "reflect-horizontal" => Some(icons::REFLECT_HORIZONTAL),
        "candy-off-outline" => Some(icons::CANDY_OFF_OUTLINE),
        "currency-cny" => Some(icons::CURRENCY_CNY),
        "file-rotate-left-outline" => Some(icons::FILE_ROTATE_LEFT_OUTLINE),
        "timer-sync" => Some(icons::TIMER_SYNC),
        "wallpaper" => Some(icons::WALLPAPER),
        "pot" => Some(icons::POT),
        "fountain-pen" => Some(icons::FOUNTAIN_PEN),
        "video-minus-outline" => Some(icons::VIDEO_MINUS_OUTLINE),
        #[allow(deprecated)]
        "kodi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kodi' is deprecated.").print(py);
            }
            Some(icons::KODI)
        }
        "speaker-bluetooth" => Some(icons::SPEAKER_BLUETOOTH),
        "archive-eye-outline" => Some(icons::ARCHIVE_EYE_OUTLINE),
        "water-opacity" => Some(icons::WATER_OPACITY),
        "account-injury-outline" => Some(icons::ACCOUNT_INJURY_OUTLINE),
        #[allow(deprecated)]
        "firefox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firefox' is deprecated.").print(py);
            }
            Some(icons::FIREFOX)
        }
        "ungroup" => Some(icons::UNGROUP),
        "tag-heart-outline" => Some(icons::TAG_HEART_OUTLINE),
        "strategy" => Some(icons::STRATEGY),
        "steering" => Some(icons::STEERING),
        "application-outline" => Some(icons::APPLICATION_OUTLINE),
        "keyboard-f11" => Some(icons::KEYBOARD_F11),
        "air-purifier" => Some(icons::AIR_PURIFIER),
        "printer-pos-pause-outline" => Some(icons::PRINTER_POS_PAUSE_OUTLINE),
        "pizza" => Some(icons::PIZZA),
        "fencing" => Some(icons::FENCING),
        "bird" => Some(icons::BIRD),
        "wallet-plus" => Some(icons::WALLET_PLUS),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        "gate-xor" => Some(icons::GATE_XOR),
        "navigation-variant" => Some(icons::NAVIGATION_VARIANT),
        "collapse-all-outline" => Some(icons::COLLAPSE_ALL_OUTLINE),
        "home-export-outline" => Some(icons::HOME_EXPORT_OUTLINE),
        "lingerie" => Some(icons::LINGERIE),
        "arrow-vertical-lock" => Some(icons::ARROW_VERTICAL_LOCK),
        "dog-service" => Some(icons::DOG_SERVICE),
        "book-check" => Some(icons::BOOK_CHECK),
        "clock-time-seven-outline" => Some(icons::CLOCK_TIME_SEVEN_OUTLINE),
        "play-circle-outline" => Some(icons::PLAY_CIRCLE_OUTLINE),
        "fit-to-screen" => Some(icons::FIT_TO_SCREEN),
        "chess-rook" => Some(icons::CHESS_ROOK),
        "account-badge-outline" => Some(icons::ACCOUNT_BADGE_OUTLINE),
        "arrow-collapse-vertical" => Some(icons::ARROW_COLLAPSE_VERTICAL),
        "magnify-plus-outline" => Some(icons::MAGNIFY_PLUS_OUTLINE),
        "drag-vertical" => Some(icons::DRAG_VERTICAL),
        "clipboard-arrow-right" => Some(icons::CLIPBOARD_ARROW_RIGHT),
        "table-chair" => Some(icons::TABLE_CHAIR),
        "microphone-outline" => Some(icons::MICROPHONE_OUTLINE),
        "router" => Some(icons::ROUTER),
        "cloud" => Some(icons::CLOUD),
        "arrow-bottom-left-bold-box" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_BOX),
        "head-dots-horizontal" => Some(icons::HEAD_DOTS_HORIZONTAL),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        "remote" => Some(icons::REMOTE),
        "cloud-question" => Some(icons::CLOUD_QUESTION),
        "magnify-minus-outline" => Some(icons::MAGNIFY_MINUS_OUTLINE),
        "message-plus" => Some(icons::MESSAGE_PLUS),
        "arrow-left-thin" => Some(icons::ARROW_LEFT_THIN),
        "alarm-light-outline" => Some(icons::ALARM_LIGHT_OUTLINE),
        "briefcase-upload" => Some(icons::BRIEFCASE_UPLOAD),
        "current-ac" => Some(icons::CURRENT_AC),
        "math-integral-box" => Some(icons::MATH_INTEGRAL_BOX),
        "battery-80" => Some(icons::BATTERY_80),
        "download-network-outline" => Some(icons::DOWNLOAD_NETWORK_OUTLINE),
        "pencil-off-outline" => Some(icons::PENCIL_OFF_OUTLINE),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        "triangle" => Some(icons::TRIANGLE),
        "printer-settings" => Some(icons::PRINTER_SETTINGS),
        "format-strikethrough-variant" => Some(icons::FORMAT_STRIKETHROUGH_VARIANT),
        "fishbowl" => Some(icons::FISHBOWL),
        "timer-stop-outline" => Some(icons::TIMER_STOP_OUTLINE),
        "airballoon" => Some(icons::AIRBALLOON),
        "set-center-right" => Some(icons::SET_CENTER_RIGHT),
        "star-box-multiple" => Some(icons::STAR_BOX_MULTIPLE),
        "truck-off-road-off" => Some(icons::TRUCK_OFF_ROAD_OFF),
        "calendar-clock-outline" => Some(icons::CALENDAR_CLOCK_OUTLINE),
        "cellphone-arrow-down-variant" => Some(icons::CELLPHONE_ARROW_DOWN_VARIANT),
        "fridge-industrial-alert-outline" => Some(icons::FRIDGE_INDUSTRIAL_ALERT_OUTLINE),
        "baseball-diamond" => Some(icons::BASEBALL_DIAMOND),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "card-account-details-star-outline" => Some(icons::CARD_ACCOUNT_DETAILS_STAR_OUTLINE),
        "numeric-1-circle" => Some(icons::NUMERIC_1_CIRCLE),
        "wifi-strength-alert-outline" => Some(icons::WIFI_STRENGTH_ALERT_OUTLINE),
        "social-distance-2-meters" => Some(icons::SOCIAL_DISTANCE_2_METERS),
        "firewire" => Some(icons::FIREWIRE),
        "filter-outline" => Some(icons::FILTER_OUTLINE),
        "waves-arrow-right" => Some(icons::WAVES_ARROW_RIGHT),
        "chat-minus-outline" => Some(icons::CHAT_MINUS_OUTLINE),
        "battery-charging-medium" => Some(icons::BATTERY_CHARGING_MEDIUM),
        #[allow(deprecated)]
        "language-ruby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUBY)
        }
        "airplane-cog" => Some(icons::AIRPLANE_COG),
        "account-remove" => Some(icons::ACCOUNT_REMOVE),
        "abugida-devanagari" => Some(icons::ABUGIDA_DEVANAGARI),
        "weather-lightning" => Some(icons::WEATHER_LIGHTNING),
        "chart-bell-curve" => Some(icons::CHART_BELL_CURVE),
        "book-arrow-right-outline" => Some(icons::BOOK_ARROW_RIGHT_OUTLINE),
        "multiplication-box" => Some(icons::MULTIPLICATION_BOX),
        "format-columns" => Some(icons::FORMAT_COLUMNS),
        "car-electric-outline" => Some(icons::CAR_ELECTRIC_OUTLINE),
        "skate-off" => Some(icons::SKATE_OFF),
        "diabetes" => Some(icons::DIABETES),
        "flask-empty-minus" => Some(icons::FLASK_EMPTY_MINUS),
        "ear-hearing-loop" => Some(icons::EAR_HEARING_LOOP),
        "reminder" => Some(icons::REMINDER),
        "eye-check-outline" => Some(icons::EYE_CHECK_OUTLINE),
        "shape-rectangle-plus" => Some(icons::SHAPE_RECTANGLE_PLUS),
        "table-column-plus-after" => Some(icons::TABLE_COLUMN_PLUS_AFTER),
        "tooltip-edit" => Some(icons::TOOLTIP_EDIT),
        "alpha-h-circle-outline" => Some(icons::ALPHA_H_CIRCLE_OUTLINE),
        "seat-legroom-extra" => Some(icons::SEAT_LEGROOM_EXTRA),
        "train-car" => Some(icons::TRAIN_CAR),
        "file-xml-box" => Some(icons::FILE_XML_BOX),
        "lock-remove-outline" => Some(icons::LOCK_REMOVE_OUTLINE),
        "pill-off" => Some(icons::PILL_OFF),
        "advertisements" => Some(icons::ADVERTISEMENTS),
        "book-sync" => Some(icons::BOOK_SYNC),
        "cart-plus" => Some(icons::CART_PLUS),
        "head-sync-outline" => Some(icons::HEAD_SYNC_OUTLINE),
        "phone-paused-outline" => Some(icons::PHONE_PAUSED_OUTLINE),
        "monitor-cellphone-star" => Some(icons::MONITOR_CELLPHONE_STAR),
        "alpha-o-box-outline" => Some(icons::ALPHA_O_BOX_OUTLINE),
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "hospital-marker" => Some(icons::HOSPITAL_MARKER),
        "home-search-outline" => Some(icons::HOME_SEARCH_OUTLINE),
        "boom-gate-arrow-up" => Some(icons::BOOM_GATE_ARROW_UP),
        "receipt-text-plus" => Some(icons::RECEIPT_TEXT_PLUS),
        "arrow-right" => Some(icons::ARROW_RIGHT),
        "bell-ring-outline" => Some(icons::BELL_RING_OUTLINE),
        #[allow(deprecated)]
        "salesforce" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'salesforce' is deprecated.").print(py);
            }
            Some(icons::SALESFORCE)
        }
        "arrow-up-down" => Some(icons::ARROW_UP_DOWN),
        "timer-pause-outline" => Some(icons::TIMER_PAUSE_OUTLINE),
        "arrow-up-bold-box" => Some(icons::ARROW_UP_BOLD_BOX),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "content-save-check-outline" => Some(icons::CONTENT_SAVE_CHECK_OUTLINE),
        "gender-female" => Some(icons::GENDER_FEMALE),
        "boom-gate-up" => Some(icons::BOOM_GATE_UP),
        "account-tie-voice-off" => Some(icons::ACCOUNT_TIE_VOICE_OFF),
        "menu-close" => Some(icons::MENU_CLOSE),
        "lightbulb-on-10" => Some(icons::LIGHTBULB_ON_10),
        #[allow(deprecated)]
        "firebase" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firebase' is deprecated.").print(py);
            }
            Some(icons::FIREBASE)
        }
        "battery-charging-wireless-10" => Some(icons::BATTERY_CHARGING_WIRELESS_10),
        "parachute-outline" => Some(icons::PARACHUTE_OUTLINE),
        #[allow(deprecated)]
        "ubuntu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubuntu' is deprecated.").print(py);
            }
            Some(icons::UBUNTU)
        }
        "paragliding" => Some(icons::PARAGLIDING),
        "scan-helper" => Some(icons::SCAN_HELPER),
        "candle" => Some(icons::CANDLE),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        "hammer" => Some(icons::HAMMER),
        "signature" => Some(icons::SIGNATURE),
        "truck-remove-outline" => Some(icons::TRUCK_REMOVE_OUTLINE),
        _ => None,
    }
}
