// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_24(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "solar-power" => Some(icons::SOLAR_POWER),
        "latitude" => Some(icons::LATITUDE),
        #[allow(deprecated)]
        "rollupjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'rollupjs' is deprecated.").print(py);
            }
            Some(icons::ROLLUPJS)
        }
        "arrow-left-right-bold" => Some(icons::ARROW_LEFT_RIGHT_BOLD),
        "send-circle" => Some(icons::SEND_CIRCLE),
        "pen-remove" => Some(icons::PEN_REMOVE),
        "train-bus" => Some(icons::TRAIN_BUS),
        "shield-account-variant-outline" => Some(icons::SHIELD_ACCOUNT_VARIANT_OUTLINE),
        "routes-clock" => Some(icons::ROUTES_CLOCK),
        "unfold-more-horizontal" => Some(icons::UNFOLD_MORE_HORIZONTAL),
        "spoon-sugar" => Some(icons::SPOON_SUGAR),
        "spotlight" => Some(icons::SPOTLIGHT),
        "routes" => Some(icons::ROUTES),
        "camera-document" => Some(icons::CAMERA_DOCUMENT),
        "vector-union" => Some(icons::VECTOR_UNION),
        "sigma" => Some(icons::SIGMA),
        "head-plus" => Some(icons::HEAD_PLUS),
        "alpha-c" => Some(icons::ALPHA_C),
        "file-link-outline" => Some(icons::FILE_LINK_OUTLINE),
        "radio-handheld" => Some(icons::RADIO_HANDHELD),
        "clock-time-six" => Some(icons::CLOCK_TIME_SIX),
        #[allow(deprecated)]
        "google-chrome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-chrome' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_CHROME)
        }
        "store-marker-outline" => Some(icons::STORE_MARKER_OUTLINE),
        "gamepad-variant" => Some(icons::GAMEPAD_VARIANT),
        "chess-king" => Some(icons::CHESS_KING),
        "face-recognition" => Some(icons::FACE_RECOGNITION),
        "calendar-range" => Some(icons::CALENDAR_RANGE),
        "folder-download-outline" => Some(icons::FOLDER_DOWNLOAD_OUTLINE),
        "handshake-outline" => Some(icons::HANDSHAKE_OUTLINE),
        "account-hard-hat-outline" => Some(icons::ACCOUNT_HARD_HAT_OUTLINE),
        "rhombus-split" => Some(icons::RHOMBUS_SPLIT),
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "flask-round-bottom-empty-outline" => Some(icons::FLASK_ROUND_BOTTOM_EMPTY_OUTLINE),
        "nfc-tap" => Some(icons::NFC_TAP),
        "database-settings" => Some(icons::DATABASE_SETTINGS),
        "transmission-tower-export" => Some(icons::TRANSMISSION_TOWER_EXPORT),
        "closed-caption" => Some(icons::CLOSED_CAPTION),
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "heating-coil" => Some(icons::HEATING_COIL),
        "track-light" => Some(icons::TRACK_LIGHT),
        "monitor-edit" => Some(icons::MONITOR_EDIT),
        "calendar-filter-outline" => Some(icons::CALENDAR_FILTER_OUTLINE),
        "shoe-sneaker" => Some(icons::SHOE_SNEAKER),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "battery-lock-open" => Some(icons::BATTERY_LOCK_OPEN),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "bag-personal-tag" => Some(icons::BAG_PERSONAL_TAG),
        "page-layout-sidebar-left" => Some(icons::PAGE_LAYOUT_SIDEBAR_LEFT),
        "database" => Some(icons::DATABASE),
        "tag-text-outline" => Some(icons::TAG_TEXT_OUTLINE),
        "sort-bool-ascending-variant" => Some(icons::SORT_BOOL_ASCENDING_VARIANT),
        "cog-refresh" => Some(icons::COG_REFRESH),
        "television-shimmer" => Some(icons::TELEVISION_SHIMMER),
        "file-delimited-outline" => Some(icons::FILE_DELIMITED_OUTLINE),
        "human-greeting" => Some(icons::HUMAN_GREETING),
        "wrench-clock" => Some(icons::WRENCH_CLOCK),
        "plus-box" => Some(icons::PLUS_BOX),
        "chat-processing-outline" => Some(icons::CHAT_PROCESSING_OUTLINE),
        "web-sync" => Some(icons::WEB_SYNC),
        "timer-lock-outline" => Some(icons::TIMER_LOCK_OUTLINE),
        "heat-pump" => Some(icons::HEAT_PUMP),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "head-snowflake-outline" => Some(icons::HEAD_SNOWFLAKE_OUTLINE),
        "toilet" => Some(icons::TOILET),
        "wizard-hat" => Some(icons::WIZARD_HAT),
        "kettle-steam-outline" => Some(icons::KETTLE_STEAM_OUTLINE),
        "swap-horizontal-circle" => Some(icons::SWAP_HORIZONTAL_CIRCLE),
        "bus-stop" => Some(icons::BUS_STOP),
        "car-seat-cooler" => Some(icons::CAR_SEAT_COOLER),
        "play-box-lock-open" => Some(icons::PLAY_BOX_LOCK_OPEN),
        "label-off" => Some(icons::LABEL_OFF),
        "data-matrix-plus" => Some(icons::DATA_MATRIX_PLUS),
        "cube-outline" => Some(icons::CUBE_OUTLINE),
        "key-plus" => Some(icons::KEY_PLUS),
        "charity-search" => Some(icons::CHARITY_SEARCH),
        #[allow(deprecated)]
        "terraform" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'terraform' is deprecated.").print(py);
            }
            Some(icons::TERRAFORM)
        }
        "format-font-size-increase" => Some(icons::FORMAT_FONT_SIZE_INCREASE),
        "numeric" => Some(icons::NUMERIC),
        "key-minus" => Some(icons::KEY_MINUS),
        "dice-d8" => Some(icons::DICE_D8),
        "cellphone-play" => Some(icons::CELLPHONE_PLAY),
        "bus-school" => Some(icons::BUS_SCHOOL),
        "weight-lifter" => Some(icons::WEIGHT_LIFTER),
        "bluetooth-connect" => Some(icons::BLUETOOTH_CONNECT),
        "email-minus-outline" => Some(icons::EMAIL_MINUS_OUTLINE),
        "arrow-up-down-bold" => Some(icons::ARROW_UP_DOWN_BOLD),
        "vector-radius" => Some(icons::VECTOR_RADIUS),
        "hub" => Some(icons::HUB),
        #[allow(deprecated)]
        "microsoft-xbox-controller-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-view' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_VIEW)
        }
        "gantry-crane" => Some(icons::GANTRY_CRANE),
        "format-float-center" => Some(icons::FORMAT_FLOAT_CENTER),
        "town-hall" => Some(icons::TOWN_HALL),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "table-minus" => Some(icons::TABLE_MINUS),
        "timer-check-outline" => Some(icons::TIMER_CHECK_OUTLINE),
        "size-m" => Some(icons::SIZE_M),
        "alpha-n-box" => Some(icons::ALPHA_N_BOX),
        "pine-tree-box" => Some(icons::PINE_TREE_BOX),
        "ballot-recount-outline" => Some(icons::BALLOT_RECOUNT_OUTLINE),
        "fruit-watermelon" => Some(icons::FRUIT_WATERMELON),
        #[allow(deprecated)]
        "apple" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple' is deprecated.").print(py);
            }
            Some(icons::APPLE)
        }
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "stadium-outline" => Some(icons::STADIUM_OUTLINE),
        "toy-brick-marker-outline" => Some(icons::TOY_BRICK_MARKER_OUTLINE),
        "message-reply" => Some(icons::MESSAGE_REPLY),
        "delete-circle-outline" => Some(icons::DELETE_CIRCLE_OUTLINE),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "file-download-outline" => Some(icons::FILE_DOWNLOAD_OUTLINE),
        "alpha-g-circle-outline" => Some(icons::ALPHA_G_CIRCLE_OUTLINE),
        "axis-z-arrow" => Some(icons::AXIS_Z_ARROW),
        "shield-account-variant" => Some(icons::SHIELD_ACCOUNT_VARIANT),
        "lightbulb-on-outline" => Some(icons::LIGHTBULB_ON_OUTLINE),
        "file-presentation-box" => Some(icons::FILE_PRESENTATION_BOX),
        "image-refresh" => Some(icons::IMAGE_REFRESH),
        "cursor-default-click" => Some(icons::CURSOR_DEFAULT_CLICK),
        "credit-card-settings-outline" => Some(icons::CREDIT_CARD_SETTINGS_OUTLINE),
        "numeric-7-box-outline" => Some(icons::NUMERIC_7_BOX_OUTLINE),
        "gamepad-circle-outline" => Some(icons::GAMEPAD_CIRCLE_OUTLINE),
        "chart-scatter-plot-hexbin" => Some(icons::CHART_SCATTER_PLOT_HEXBIN),
        "ev-plug-type2" => Some(icons::EV_PLUG_TYPE2),
        "arrow-top-left-bold-box" => Some(icons::ARROW_TOP_LEFT_BOLD_BOX),
        "test-tube-empty" => Some(icons::TEST_TUBE_EMPTY),
        "razor-single-edge" => Some(icons::RAZOR_SINGLE_EDGE),
        "folder-star-outline" => Some(icons::FOLDER_STAR_OUTLINE),
        "page-next-outline" => Some(icons::PAGE_NEXT_OUTLINE),
        "lightbulb-on-50" => Some(icons::LIGHTBULB_ON_50),
        "settings-helper" => Some(icons::SETTINGS_HELPER),
        "cow-off" => Some(icons::COW_OFF),
        #[allow(deprecated)]
        "meteor" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'meteor' is deprecated.").print(py);
            }
            Some(icons::METEOR)
        }
        "golf-tee" => Some(icons::GOLF_TEE),
        "gondola" => Some(icons::GONDOLA),
        "format-list-text" => Some(icons::FORMAT_LIST_TEXT),
        "bell-check" => Some(icons::BELL_CHECK),
        "bell-plus-outline" => Some(icons::BELL_PLUS_OUTLINE),
        "server-plus-outline" => Some(icons::SERVER_PLUS_OUTLINE),
        "head-heart" => Some(icons::HEAD_HEART),
        "cards-club" => Some(icons::CARDS_CLUB),
        "candy-off" => Some(icons::CANDY_OFF),
        "format-annotation-minus" => Some(icons::FORMAT_ANNOTATION_MINUS),
        "bug-play" => Some(icons::BUG_PLAY),
        "camera-party-mode" => Some(icons::CAMERA_PARTY_MODE),
        "magnify-scan" => Some(icons::MAGNIFY_SCAN),
        "download-multiple-outline" => Some(icons::DOWNLOAD_MULTIPLE_OUTLINE),
        "package-variant-closed-remove" => Some(icons::PACKAGE_VARIANT_CLOSED_REMOVE),
        "list-box-outline" => Some(icons::LIST_BOX_OUTLINE),
        "file-alert" => Some(icons::FILE_ALERT),
        "battery-40" => Some(icons::BATTERY_40),
        "format-color-fill" => Some(icons::FORMAT_COLOR_FILL),
        "alpha-x-box-outline" => Some(icons::ALPHA_X_BOX_OUTLINE),
        "beehive-outline" => Some(icons::BEEHIVE_OUTLINE),
        "keyboard-tab" => Some(icons::KEYBOARD_TAB),
        "arrow-expand-left" => Some(icons::ARROW_EXPAND_LEFT),
        "ideogram-cjk-variant" => Some(icons::IDEOGRAM_CJK_VARIANT),
        "bunk-bed" => Some(icons::BUNK_BED),
        "ray-start-vertex-end" => Some(icons::RAY_START_VERTEX_END),
        "folder-multiple-plus-outline" => Some(icons::FOLDER_MULTIPLE_PLUS_OUTLINE),
        "tools" => Some(icons::TOOLS),
        "clock-fast" => Some(icons::CLOCK_FAST),
        "quadcopter" => Some(icons::QUADCOPTER),
        "send-clock" => Some(icons::SEND_CLOCK),
        "robot-excited-outline" => Some(icons::ROBOT_EXCITED_OUTLINE),
        #[allow(deprecated)]
        "google-glass" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-glass' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_GLASS)
        }
        "desktop-classic" => Some(icons::DESKTOP_CLASSIC),
        "currency-btc" => Some(icons::CURRENCY_BTC),
        "alpha-k" => Some(icons::ALPHA_K),
        "hydraulic-oil-level" => Some(icons::HYDRAULIC_OIL_LEVEL),
        "dice-6-outline" => Some(icons::DICE_6_OUTLINE),
        "pail-minus" => Some(icons::PAIL_MINUS),
        "image-album" => Some(icons::IMAGE_ALBUM),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "power-standby" => Some(icons::POWER_STANDBY),
        "stadium" => Some(icons::STADIUM),
        "menu-up-outline" => Some(icons::MENU_UP_OUTLINE),
        "wave" => Some(icons::WAVE),
        "align-horizontal-center" => Some(icons::ALIGN_HORIZONTAL_CENTER),
        "trophy-award" => Some(icons::TROPHY_AWARD),
        "mirror-variant" => Some(icons::MIRROR_VARIANT),
        "wiper-wash-alert" => Some(icons::WIPER_WASH_ALERT),
        "tally-mark-5" => Some(icons::TALLY_MARK_5),
        "calendar-multiselect-outline" => Some(icons::CALENDAR_MULTISELECT_OUTLINE),
        "eye-plus" => Some(icons::EYE_PLUS),
        "flash-red-eye" => Some(icons::FLASH_RED_EYE),
        "silo" => Some(icons::SILO),
        "file-document-refresh-outline" => Some(icons::FILE_DOCUMENT_REFRESH_OUTLINE),
        "format-header-5" => Some(icons::FORMAT_HEADER_5),
        "mushroom-outline" => Some(icons::MUSHROOM_OUTLINE),
        "cloud-alert-outline" => Some(icons::CLOUD_ALERT_OUTLINE),
        "invoice-list-outline" => Some(icons::INVOICE_LIST_OUTLINE),
        "invoice-list" => Some(icons::INVOICE_LIST),
        "text-search-variant" => Some(icons::TEXT_SEARCH_VARIANT),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "smoking" => Some(icons::SMOKING),
        "sort-clock-ascending" => Some(icons::SORT_CLOCK_ASCENDING),
        "nfc-variant" => Some(icons::NFC_VARIANT),
        "robot-mower-outline" => Some(icons::ROBOT_MOWER_OUTLINE),
        "clock-time-four-outline" => Some(icons::CLOCK_TIME_FOUR_OUTLINE),
        "message-star-outline" => Some(icons::MESSAGE_STAR_OUTLINE),
        "weather-partly-lightning" => Some(icons::WEATHER_PARTLY_LIGHTNING),
        "puzzle-plus-outline" => Some(icons::PUZZLE_PLUS_OUTLINE),
        "lock-open-variant" => Some(icons::LOCK_OPEN_VARIANT),
        _ => None,
    }
}
