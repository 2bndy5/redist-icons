// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_3(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "gift-open-outline" => Some(icons::GIFT_OPEN_OUTLINE),
        "beer-outline" => Some(icons::BEER_OUTLINE),
        "emoticon-happy-outline" => Some(icons::EMOTICON_HAPPY_OUTLINE),
        "exit-run" => Some(icons::EXIT_RUN),
        "flag-triangle" => Some(icons::FLAG_TRIANGLE),
        #[allow(deprecated)]
        "pi-hole" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'pi-hole' is deprecated.").print(py);
            }
            Some(icons::PI_HOLE)
        }
        "account-sync-outline" => Some(icons::ACCOUNT_SYNC_OUTLINE),
        "credit-card-edit" => Some(icons::CREDIT_CARD_EDIT),
        "source-branch-minus" => Some(icons::SOURCE_BRANCH_MINUS),
        "cursor-text" => Some(icons::CURSOR_TEXT),
        "comment-arrow-right" => Some(icons::COMMENT_ARROW_RIGHT),
        #[allow(deprecated)]
        "mastodon" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'mastodon' is deprecated.").print(py);
            }
            Some(icons::MASTODON)
        }
        "clock-outline" => Some(icons::CLOCK_OUTLINE),
        "transfer" => Some(icons::TRANSFER),
        "gamepad-round-outline" => Some(icons::GAMEPAD_ROUND_OUTLINE),
        "minus" => Some(icons::MINUS),
        "pencil-lock" => Some(icons::PENCIL_LOCK),
        "alpha-m-circle-outline" => Some(icons::ALPHA_M_CIRCLE_OUTLINE),
        "ski" => Some(icons::SKI),
        "forum-outline" => Some(icons::FORUM_OUTLINE),
        "ear-hearing" => Some(icons::EAR_HEARING),
        "snowmobile" => Some(icons::SNOWMOBILE),
        "monitor-speaker-off" => Some(icons::MONITOR_SPEAKER_OFF),
        "store-settings" => Some(icons::STORE_SETTINGS),
        "file-upload" => Some(icons::FILE_UPLOAD),
        "shaker" => Some(icons::SHAKER),
        "information-slab-box" => Some(icons::INFORMATION_SLAB_BOX),
        "invoice-export-outline" => Some(icons::INVOICE_EXPORT_OUTLINE),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "pan-top-left" => Some(icons::PAN_TOP_LEFT),
        "battery-charging-wireless-outline" => Some(icons::BATTERY_CHARGING_WIRELESS_OUTLINE),
        "email-check-outline" => Some(icons::EMAIL_CHECK_OUTLINE),
        "arrow-vertical-lock" => Some(icons::ARROW_VERTICAL_LOCK),
        "robot-dead-outline" => Some(icons::ROBOT_DEAD_OUTLINE),
        "numeric-9-plus-box-outline" => Some(icons::NUMERIC_9_PLUS_BOX_OUTLINE),
        "expansion-card" => Some(icons::EXPANSION_CARD),
        "coach-lamp" => Some(icons::COACH_LAMP),
        "fast-forward-15" => Some(icons::FAST_FORWARD_15),
        "chip" => Some(icons::CHIP),
        "router-wireless" => Some(icons::ROUTER_WIRELESS),
        "robot-angry-outline" => Some(icons::ROBOT_ANGRY_OUTLINE),
        "border-none" => Some(icons::BORDER_NONE),
        "flask-empty-remove" => Some(icons::FLASK_EMPTY_REMOVE),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        "car-seat-heater" => Some(icons::CAR_SEAT_HEATER),
        "email-open-heart-outline" => Some(icons::EMAIL_OPEN_HEART_OUTLINE),
        "camera-lock-open" => Some(icons::CAMERA_LOCK_OPEN),
        "checkbox-marked-circle-minus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_MINUS_OUTLINE),
        "upload-off-outline" => Some(icons::UPLOAD_OFF_OUTLINE),
        "shipping-pallet" => Some(icons::SHIPPING_PALLET),
        "shape-outline" => Some(icons::SHAPE_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-low" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-low' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_LOW)
        }
        "office-building-cog-outline" => Some(icons::OFFICE_BUILDING_COG_OUTLINE),
        "database-minus" => Some(icons::DATABASE_MINUS),
        "wind-turbine" => Some(icons::WIND_TURBINE),
        "map-marker-circle" => Some(icons::MAP_MARKER_CIRCLE),
        "archive-alert-outline" => Some(icons::ARCHIVE_ALERT_OUTLINE),
        "signal-off" => Some(icons::SIGNAL_OFF),
        "airplane-check" => Some(icons::AIRPLANE_CHECK),
        "sort-ascending" => Some(icons::SORT_ASCENDING),
        "timer-sand" => Some(icons::TIMER_SAND),
        "power-socket-de" => Some(icons::POWER_SOCKET_DE),
        "bell-remove-outline" => Some(icons::BELL_REMOVE_OUTLINE),
        "numeric-9-plus-box-multiple" => Some(icons::NUMERIC_9_PLUS_BOX_MULTIPLE),
        "pen-off" => Some(icons::PEN_OFF),
        "emoticon-minus-outline" => Some(icons::EMOTICON_MINUS_OUTLINE),
        "cellphone" => Some(icons::CELLPHONE),
        "white-balance-sunny" => Some(icons::WHITE_BALANCE_SUNNY),
        "cookie-clock" => Some(icons::COOKIE_CLOCK),
        "selection-off" => Some(icons::SELECTION_OFF),
        "layers" => Some(icons::LAYERS),
        "iron-outline" => Some(icons::IRON_OUTLINE),
        "file-percent" => Some(icons::FILE_PERCENT),
        #[allow(deprecated)]
        "language-kotlin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-kotlin' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_KOTLIN)
        }
        "cloud-check" => Some(icons::CLOUD_CHECK),
        "account-reactivate" => Some(icons::ACCOUNT_REACTIVATE),
        "package-variant-closed-check" => Some(icons::PACKAGE_VARIANT_CLOSED_CHECK),
        "close-box-multiple-outline" => Some(icons::CLOSE_BOX_MULTIPLE_OUTLINE),
        "battery-sync" => Some(icons::BATTERY_SYNC),
        "rectangle" => Some(icons::RECTANGLE),
        #[allow(deprecated)]
        "youtube-tv" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-tv' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_TV)
        }
        "ring" => Some(icons::RING),
        "ladder" => Some(icons::LADDER),
        "puzzle-check" => Some(icons::PUZZLE_CHECK),
        "eye" => Some(icons::EYE),
        "arrow-top-right-bold-outline" => Some(icons::ARROW_TOP_RIGHT_BOLD_OUTLINE),
        "information-variant-circle" => Some(icons::INFORMATION_VARIANT_CIRCLE),
        "reload" => Some(icons::RELOAD),
        "rhombus-outline" => Some(icons::RHOMBUS_OUTLINE),
        "controller-off" => Some(icons::CONTROLLER_OFF),
        "file-star-four-points-outline" => Some(icons::FILE_STAR_FOUR_POINTS_OUTLINE),
        "cloud-lock" => Some(icons::CLOUD_LOCK),
        "wifi-star" => Some(icons::WIFI_STAR),
        "lightbulb-outline" => Some(icons::LIGHTBULB_OUTLINE),
        "feature-search" => Some(icons::FEATURE_SEARCH),
        "format-text-wrapping-clip" => Some(icons::FORMAT_TEXT_WRAPPING_CLIP),
        "alpha-v-box" => Some(icons::ALPHA_V_BOX),
        "cog-transfer" => Some(icons::COG_TRANSFER),
        "image-check" => Some(icons::IMAGE_CHECK),
        "transfer-down" => Some(icons::TRANSFER_DOWN),
        "stop-circle" => Some(icons::STOP_CIRCLE),
        "chart-sankey-variant" => Some(icons::CHART_SANKEY_VARIANT),
        "car-brake-temperature" => Some(icons::CAR_BRAKE_TEMPERATURE),
        "view-list" => Some(icons::VIEW_LIST),
        "invert-colors-off" => Some(icons::INVERT_COLORS_OFF),
        "invoice-text" => Some(icons::INVOICE_TEXT),
        "numeric-8-circle" => Some(icons::NUMERIC_8_CIRCLE),
        "axis-z-rotate-clockwise" => Some(icons::AXIS_Z_ROTATE_CLOCKWISE),
        "home-remove" => Some(icons::HOME_REMOVE),
        "gamepad-circle-down" => Some(icons::GAMEPAD_CIRCLE_DOWN),
        "lightbulb-off" => Some(icons::LIGHTBULB_OFF),
        "table-column" => Some(icons::TABLE_COLUMN),
        "tally-mark-5" => Some(icons::TALLY_MARK_5),
        "alpha-b-box-outline" => Some(icons::ALPHA_B_BOX_OUTLINE),
        "network-strength-3-alert" => Some(icons::NETWORK_STRENGTH_3_ALERT),
        "credit-card-marker-outline" => Some(icons::CREDIT_CARD_MARKER_OUTLINE),
        "timer-star-outline" => Some(icons::TIMER_STAR_OUTLINE),
        "currency-twd" => Some(icons::CURRENCY_TWD),
        "video-high-definition" => Some(icons::VIDEO_HIGH_DEFINITION),
        "pasta" => Some(icons::PASTA),
        "image-filter-vintage" => Some(icons::IMAGE_FILTER_VINTAGE),
        "link-box-variant" => Some(icons::LINK_BOX_VARIANT),
        "account-arrow-down-outline" => Some(icons::ACCOUNT_ARROW_DOWN_OUTLINE),
        "numeric-8-box-multiple-outline" => Some(icons::NUMERIC_8_BOX_MULTIPLE_OUTLINE),
        "power-off" => Some(icons::POWER_OFF),
        "head-check-outline" => Some(icons::HEAD_CHECK_OUTLINE),
        "table-multiple" => Some(icons::TABLE_MULTIPLE),
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "calendar-weekend-outline" => Some(icons::CALENDAR_WEEKEND_OUTLINE),
        "image-filter-drama-outline" => Some(icons::IMAGE_FILTER_DRAMA_OUTLINE),
        "biohazard" => Some(icons::BIOHAZARD),
        "circle-slice-3" => Some(icons::CIRCLE_SLICE_3),
        "cloud-refresh-variant-outline" => Some(icons::CLOUD_REFRESH_VARIANT_OUTLINE),
        "package-variant-minus" => Some(icons::PACKAGE_VARIANT_MINUS),
        #[allow(deprecated)]
        "snapchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'snapchat' is deprecated.").print(py);
            }
            Some(icons::SNAPCHAT)
        }
        "information-symbol" => Some(icons::INFORMATION_SYMBOL),
        "clipboard-play-multiple" => Some(icons::CLIPBOARD_PLAY_MULTIPLE),
        "currency-cny" => Some(icons::CURRENCY_CNY),
        "basketball-hoop-outline" => Some(icons::BASKETBALL_HOOP_OUTLINE),
        "flag-variant-remove" => Some(icons::FLAG_VARIANT_REMOVE),
        "snake" => Some(icons::SNAKE),
        "table-chair" => Some(icons::TABLE_CHAIR),
        "alpha-m-circle" => Some(icons::ALPHA_M_CIRCLE),
        "thermometer-plus" => Some(icons::THERMOMETER_PLUS),
        "near-me" => Some(icons::NEAR_ME),
        "arrow-up-bold-hexagon-outline" => Some(icons::ARROW_UP_BOLD_HEXAGON_OUTLINE),
        "battery-charging-60" => Some(icons::BATTERY_CHARGING_60),
        "thermometer-bluetooth" => Some(icons::THERMOMETER_BLUETOOTH),
        "weather-windy-variant" => Some(icons::WEATHER_WINDY_VARIANT),
        "bulldozer" => Some(icons::BULLDOZER),
        "account-file-text-outline" => Some(icons::ACCOUNT_FILE_TEXT_OUTLINE),
        "text-box-multiple-outline" => Some(icons::TEXT_BOX_MULTIPLE_OUTLINE),
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        #[allow(deprecated)]
        "instagram" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'instagram' is deprecated.").print(py);
            }
            Some(icons::INSTAGRAM)
        }
        "step-backward" => Some(icons::STEP_BACKWARD),
        "fuse" => Some(icons::FUSE),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        #[allow(deprecated)]
        "teamviewer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'teamviewer' is deprecated.").print(py);
            }
            Some(icons::TEAMVIEWER)
        }
        "filter-minus-outline" => Some(icons::FILTER_MINUS_OUTLINE),
        "source-repository" => Some(icons::SOURCE_REPOSITORY),
        "alpha" => Some(icons::ALPHA),
        "television-off" => Some(icons::TELEVISION_OFF),
        "reply-outline" => Some(icons::REPLY_OUTLINE),
        "database-remove" => Some(icons::DATABASE_REMOVE),
        "boom-gate-arrow-down-outline" => Some(icons::BOOM_GATE_ARROW_DOWN_OUTLINE),
        "spirit-level" => Some(icons::SPIRIT_LEVEL),
        #[allow(deprecated)]
        "eslint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'eslint' is deprecated.").print(py);
            }
            Some(icons::ESLINT)
        }
        "timer-refresh-outline" => Some(icons::TIMER_REFRESH_OUTLINE),
        "calendar-collapse-horizontal" => Some(icons::CALENDAR_COLLAPSE_HORIZONTAL),
        "alpha-p-circle" => Some(icons::ALPHA_P_CIRCLE),
        "axis-arrow" => Some(icons::AXIS_ARROW),
        #[allow(deprecated)]
        "facebook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook' is deprecated.").print(py);
            }
            Some(icons::FACEBOOK)
        }
        "cog-transfer-outline" => Some(icons::COG_TRANSFER_OUTLINE),
        "moped-electric-outline" => Some(icons::MOPED_ELECTRIC_OUTLINE),
        "yin-yang" => Some(icons::YIN_YANG),
        "hands-pray" => Some(icons::HANDS_PRAY),
        "cassette" => Some(icons::CASSETTE),
        "account-remove" => Some(icons::ACCOUNT_REMOVE),
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        "picture-in-picture-bottom-right-outline" => {
            Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT_OUTLINE)
        }
        "multicast" => Some(icons::MULTICAST),
        "text-box-search-outline" => Some(icons::TEXT_BOX_SEARCH_OUTLINE),
        "phone-incoming" => Some(icons::PHONE_INCOMING),
        "database-check" => Some(icons::DATABASE_CHECK),
        "email-arrow-left-outline" => Some(icons::EMAIL_ARROW_LEFT_OUTLINE),
        "list-status" => Some(icons::LIST_STATUS),
        "account-details-outline" => Some(icons::ACCOUNT_DETAILS_OUTLINE),
        "weather-sunny-off" => Some(icons::WEATHER_SUNNY_OFF),
        "sim-outline" => Some(icons::SIM_OUTLINE),
        "usb-port" => Some(icons::USB_PORT),
        "candy-off-outline" => Some(icons::CANDY_OFF_OUTLINE),
        "vector-link" => Some(icons::VECTOR_LINK),
        "timeline-remove-outline" => Some(icons::TIMELINE_REMOVE_OUTLINE),
        "skip-next-circle" => Some(icons::SKIP_NEXT_CIRCLE),
        "floor-lamp-torchiere-outline" => Some(icons::FLOOR_LAMP_TORCHIERE_OUTLINE),
        "scale" => Some(icons::SCALE),
        "clipboard-play-outline" => Some(icons::CLIPBOARD_PLAY_OUTLINE),
        "alpha-y-box-outline" => Some(icons::ALPHA_Y_BOX_OUTLINE),
        "monitor" => Some(icons::MONITOR),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        _ => None,
    }
}
