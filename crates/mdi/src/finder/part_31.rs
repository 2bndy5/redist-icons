// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_31(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "alpha-e-circle" => Some(icons::ALPHA_E_CIRCLE),
        "set-right" => Some(icons::SET_RIGHT),
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        "pool" => Some(icons::POOL),
        "battery-40-bluetooth" => Some(icons::BATTERY_40_BLUETOOTH),
        "file-cabinet" => Some(icons::FILE_CABINET),
        "battery-charging-70" => Some(icons::BATTERY_CHARGING_70),
        "bug-check" => Some(icons::BUG_CHECK),
        "book-minus-outline" => Some(icons::BOOK_MINUS_OUTLINE),
        "account-box-multiple" => Some(icons::ACCOUNT_BOX_MULTIPLE),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "barley-off" => Some(icons::BARLEY_OFF),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        "file-check" => Some(icons::FILE_CHECK),
        "bowl-outline" => Some(icons::BOWL_OUTLINE),
        "chevron-right" => Some(icons::CHEVRON_RIGHT),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "pocket" => Some(icons::POCKET),
        #[allow(deprecated)]
        "youtube-tv" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-tv' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_TV)
        }
        "train-car-centerbeam-full" => Some(icons::TRAIN_CAR_CENTERBEAM_FULL),
        "hand-front-right-outline" => Some(icons::HAND_FRONT_RIGHT_OUTLINE),
        "sort-bool-ascending" => Some(icons::SORT_BOOL_ASCENDING),
        "clock-time-eleven-outline" => Some(icons::CLOCK_TIME_ELEVEN_OUTLINE),
        "currency-php" => Some(icons::CURRENCY_PHP),
        "size-l" => Some(icons::SIZE_L),
        "battery-heart-variant" => Some(icons::BATTERY_HEART_VARIANT),
        "key-wireless" => Some(icons::KEY_WIRELESS),
        "wrap" => Some(icons::WRAP),
        "account-eye" => Some(icons::ACCOUNT_EYE),
        "pan-bottom-left" => Some(icons::PAN_BOTTOM_LEFT),
        "volume-mute" => Some(icons::VOLUME_MUTE),
        "emoticon-devil" => Some(icons::EMOTICON_DEVIL),
        "note-remove" => Some(icons::NOTE_REMOVE),
        "file-jpg-box" => Some(icons::FILE_JPG_BOX),
        "web-minus" => Some(icons::WEB_MINUS),
        "alert-circle-check-outline" => Some(icons::ALERT_CIRCLE_CHECK_OUTLINE),
        "bus-stop-covered" => Some(icons::BUS_STOP_COVERED),
        "comment-text-multiple" => Some(icons::COMMENT_TEXT_MULTIPLE),
        "battery-high" => Some(icons::BATTERY_HIGH),
        "smoking-pipe-off" => Some(icons::SMOKING_PIPE_OFF),
        "folder-cog" => Some(icons::FOLDER_COG),
        "note-plus-outline" => Some(icons::NOTE_PLUS_OUTLINE),
        "timer-settings-outline" => Some(icons::TIMER_SETTINGS_OUTLINE),
        "sprinkler-variant" => Some(icons::SPRINKLER_VARIANT),
        "car-parking-lights" => Some(icons::CAR_PARKING_LIGHTS),
        "comment-search-outline" => Some(icons::COMMENT_SEARCH_OUTLINE),
        "bottle-wine-outline" => Some(icons::BOTTLE_WINE_OUTLINE),
        "folder-key-network-outline" => Some(icons::FOLDER_KEY_NETWORK_OUTLINE),
        "raspberry-pi" => Some(icons::RASPBERRY_PI),
        "toy-brick-minus-outline" => Some(icons::TOY_BRICK_MINUS_OUTLINE),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "receipt-text-send-outline" => Some(icons::RECEIPT_TEXT_SEND_OUTLINE),
        "palette-swatch-variant" => Some(icons::PALETTE_SWATCH_VARIANT),
        "briefcase-search-outline" => Some(icons::BRIEFCASE_SEARCH_OUTLINE),
        "truck-remove" => Some(icons::TRUCK_REMOVE),
        "account-child-outline" => Some(icons::ACCOUNT_CHILD_OUTLINE),
        "alert-circle-outline" => Some(icons::ALERT_CIRCLE_OUTLINE),
        "wall-sconce-outline" => Some(icons::WALL_SCONCE_OUTLINE),
        "tab-unselected" => Some(icons::TAB_UNSELECTED),
        "link-box-variant" => Some(icons::LINK_BOX_VARIANT),
        "lamp-outline" => Some(icons::LAMP_OUTLINE),
        "chevron-up-box-outline" => Some(icons::CHEVRON_UP_BOX_OUTLINE),
        "package-variant-closed" => Some(icons::PACKAGE_VARIANT_CLOSED),
        "database-search" => Some(icons::DATABASE_SEARCH),
        "source-commit-start-next-local" => Some(icons::SOURCE_COMMIT_START_NEXT_LOCAL),
        "delete-sweep-outline" => Some(icons::DELETE_SWEEP_OUTLINE),
        "trending-down" => Some(icons::TRENDING_DOWN),
        "fuse" => Some(icons::FUSE),
        "message-question" => Some(icons::MESSAGE_QUESTION),
        "filter-multiple" => Some(icons::FILTER_MULTIPLE),
        "storefront-remove-outline" => Some(icons::STOREFRONT_REMOVE_OUTLINE),
        "prescription" => Some(icons::PRESCRIPTION),
        "cloud-arrow-down-outline" => Some(icons::CLOUD_ARROW_DOWN_OUTLINE),
        "cellphone-off" => Some(icons::CELLPHONE_OFF),
        "package-variant-closed-remove" => Some(icons::PACKAGE_VARIANT_CLOSED_REMOVE),
        "vector-polyline-edit" => Some(icons::VECTOR_POLYLINE_EDIT),
        "watch-import" => Some(icons::WATCH_IMPORT),
        "repeat-variant" => Some(icons::REPEAT_VARIANT),
        "post-lamp" => Some(icons::POST_LAMP),
        "store-settings-outline" => Some(icons::STORE_SETTINGS_OUTLINE),
        "home-silo" => Some(icons::HOME_SILO),
        "file-edit" => Some(icons::FILE_EDIT),
        "stocking" => Some(icons::STOCKING),
        "backspace" => Some(icons::BACKSPACE),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        #[allow(deprecated)]
        "box" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'box' is deprecated.").print(py);
            }
            Some(icons::BOX)
        }
        "border-all-variant" => Some(icons::BORDER_ALL_VARIANT),
        #[allow(deprecated)]
        "linux-mint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux-mint' is deprecated.").print(py);
            }
            Some(icons::LINUX_MINT)
        }
        "movie" => Some(icons::MOVIE),
        "cloud-lock-open-outline" => Some(icons::CLOUD_LOCK_OPEN_OUTLINE),
        "hospital-marker" => Some(icons::HOSPITAL_MARKER),
        "bag-personal-off-outline" => Some(icons::BAG_PERSONAL_OFF_OUTLINE),
        "comment-check-outline" => Some(icons::COMMENT_CHECK_OUTLINE),
        "chili-mild-outline" => Some(icons::CHILI_MILD_OUTLINE),
        "help-network-outline" => Some(icons::HELP_NETWORK_OUTLINE),
        "dice-d4" => Some(icons::DICE_D4),
        "cactus" => Some(icons::CACTUS),
        "robot-outline" => Some(icons::ROBOT_OUTLINE),
        "chat" => Some(icons::CHAT),
        "database-clock" => Some(icons::DATABASE_CLOCK),
        "table-column-width" => Some(icons::TABLE_COLUMN_WIDTH),
        "car-clutch" => Some(icons::CAR_CLUTCH),
        "account-supervisor-circle-outline" => Some(icons::ACCOUNT_SUPERVISOR_CIRCLE_OUTLINE),
        "keyboard-f4" => Some(icons::KEYBOARD_F4),
        "debug-step-into" => Some(icons::DEBUG_STEP_INTO),
        "eye-refresh-outline" => Some(icons::EYE_REFRESH_OUTLINE),
        "monitor-arrow-down-variant" => Some(icons::MONITOR_ARROW_DOWN_VARIANT),
        "rhombus-split-outline" => Some(icons::RHOMBUS_SPLIT_OUTLINE),
        "candelabra-fire" => Some(icons::CANDELABRA_FIRE),
        #[allow(deprecated)]
        "google-downasaur" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-downasaur' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_DOWNASAUR)
        }
        "polo" => Some(icons::POLO),
        "fruit-grapes" => Some(icons::FRUIT_GRAPES),
        "file-lock-outline" => Some(icons::FILE_LOCK_OUTLINE),
        "emoticon-remove-outline" => Some(icons::EMOTICON_REMOVE_OUTLINE),
        "earbuds-off-outline" => Some(icons::EARBUDS_OFF_OUTLINE),
        "network-strength-off-outline" => Some(icons::NETWORK_STRENGTH_OFF_OUTLINE),
        "candle" => Some(icons::CANDLE),
        "camera-front" => Some(icons::CAMERA_FRONT),
        "book-lock" => Some(icons::BOOK_LOCK),
        "invoice-text-remove-outline" => Some(icons::INVOICE_TEXT_REMOVE_OUTLINE),
        "wifi-minus" => Some(icons::WIFI_MINUS),
        "puzzle-minus-outline" => Some(icons::PUZZLE_MINUS_OUTLINE),
        "office-building-cog" => Some(icons::OFFICE_BUILDING_COG),
        "cloud-sync" => Some(icons::CLOUD_SYNC),
        "crop-portrait" => Some(icons::CROP_PORTRAIT),
        "table-picnic" => Some(icons::TABLE_PICNIC),
        "playlist-music" => Some(icons::PLAYLIST_MUSIC),
        "cloud-arrow-left-outline" => Some(icons::CLOUD_ARROW_LEFT_OUTLINE),
        "database-outline" => Some(icons::DATABASE_OUTLINE),
        "inbox-outline" => Some(icons::INBOX_OUTLINE),
        "camera-flip" => Some(icons::CAMERA_FLIP),
        "vector-polygon" => Some(icons::VECTOR_POLYGON),
        "keyboard-outline" => Some(icons::KEYBOARD_OUTLINE),
        "speaker-stop" => Some(icons::SPEAKER_STOP),
        "comment-edit-outline" => Some(icons::COMMENT_EDIT_OUTLINE),
        "file-eye-outline" => Some(icons::FILE_EYE_OUTLINE),
        "attachment-remove" => Some(icons::ATTACHMENT_REMOVE),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "comment-remove" => Some(icons::COMMENT_REMOVE),
        "wifi-strength-alert-outline" => Some(icons::WIFI_STRENGTH_ALERT_OUTLINE),
        "star-crescent" => Some(icons::STAR_CRESCENT),
        "chip" => Some(icons::CHIP),
        "view-dashboard-edit" => Some(icons::VIEW_DASHBOARD_EDIT),
        "clock-edit" => Some(icons::CLOCK_EDIT),
        "distribute-vertical-bottom" => Some(icons::DISTRIBUTE_VERTICAL_BOTTOM),
        "weather-lightning-rainy" => Some(icons::WEATHER_LIGHTNING_RAINY),
        "cog-stop" => Some(icons::COG_STOP),
        "note-remove-outline" => Some(icons::NOTE_REMOVE_OUTLINE),
        "file-download" => Some(icons::FILE_DOWNLOAD),
        "invoice-text-arrow-right-outline" => Some(icons::INVOICE_TEXT_ARROW_RIGHT_OUTLINE),
        "bulkhead-light" => Some(icons::BULKHEAD_LIGHT),
        "file-check-outline" => Some(icons::FILE_CHECK_OUTLINE),
        "boxing-glove" => Some(icons::BOXING_GLOVE),
        "file-music-outline" => Some(icons::FILE_MUSIC_OUTLINE),
        "battery-50" => Some(icons::BATTERY_50),
        "folder-network" => Some(icons::FOLDER_NETWORK),
        "music-note-plus" => Some(icons::MUSIC_NOTE_PLUS),
        "message-video" => Some(icons::MESSAGE_VIDEO),
        "music-note-bluetooth-off" => Some(icons::MUSIC_NOTE_BLUETOOTH_OFF),
        "chat-alert" => Some(icons::CHAT_ALERT),
        "alarm-light" => Some(icons::ALARM_LIGHT),
        "cup" => Some(icons::CUP),
        "treasure-chest" => Some(icons::TREASURE_CHEST),
        "format-list-text" => Some(icons::FORMAT_LIST_TEXT),
        "chart-timeline-variant" => Some(icons::CHART_TIMELINE_VARIANT),
        "fullscreen" => Some(icons::FULLSCREEN),
        "home-thermometer-outline" => Some(icons::HOME_THERMOMETER_OUTLINE),
        "archive-check-outline" => Some(icons::ARCHIVE_CHECK_OUTLINE),
        "gesture-tap-hold" => Some(icons::GESTURE_TAP_HOLD),
        "weather-hurricane" => Some(icons::WEATHER_HURRICANE),
        "cards-outline" => Some(icons::CARDS_OUTLINE),
        "timeline-text-outline" => Some(icons::TIMELINE_TEXT_OUTLINE),
        "message-plus" => Some(icons::MESSAGE_PLUS),
        "rabbit" => Some(icons::RABBIT),
        "comment-multiple" => Some(icons::COMMENT_MULTIPLE),
        #[allow(deprecated)]
        "ember" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ember' is deprecated.").print(py);
            }
            Some(icons::EMBER)
        }
        "dots-vertical-circle" => Some(icons::DOTS_VERTICAL_CIRCLE),
        "near-me" => Some(icons::NEAR_ME),
        "cylinder-off" => Some(icons::CYLINDER_OFF),
        "rewind" => Some(icons::REWIND),
        "human-male-female-child" => Some(icons::HUMAN_MALE_FEMALE_CHILD),
        "wallet-plus-outline" => Some(icons::WALLET_PLUS_OUTLINE),
        "swap-horizontal-hidden" => Some(icons::SWAP_HORIZONTAL_HIDDEN),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "folder-lock-outline" => Some(icons::FOLDER_LOCK_OUTLINE),
        "handshake-outline" => Some(icons::HANDSHAKE_OUTLINE),
        #[allow(deprecated)]
        "google-ads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-ads' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_ADS)
        }
        "select-arrow-down" => Some(icons::SELECT_ARROW_DOWN),
        "account-arrow-right" => Some(icons::ACCOUNT_ARROW_RIGHT),
        "folder-table" => Some(icons::FOLDER_TABLE),
        "trophy" => Some(icons::TROPHY),
        "group" => Some(icons::GROUP),
        "animation-outline" => Some(icons::ANIMATION_OUTLINE),
        "bell-off-outline" => Some(icons::BELL_OFF_OUTLINE),
        "flask-plus-outline" => Some(icons::FLASK_PLUS_OUTLINE),
        "feature-search" => Some(icons::FEATURE_SEARCH),
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
        "archive-arrow-down-outline" => Some(icons::ARCHIVE_ARROW_DOWN_OUTLINE),
        "align-horizontal-right" => Some(icons::ALIGN_HORIZONTAL_RIGHT),
        "home-city-outline" => Some(icons::HOME_CITY_OUTLINE),
        _ => None,
    }
}
