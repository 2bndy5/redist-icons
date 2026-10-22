// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_24(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "dolphin" => Some(icons::DOLPHIN),
        "glass-fragile" => Some(icons::GLASS_FRAGILE),
        "weather-sunny-alert" => Some(icons::WEATHER_SUNNY_ALERT),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "thermostat-box-auto" => Some(icons::THERMOSTAT_BOX_AUTO),
        "smoke-detector-alert-outline" => Some(icons::SMOKE_DETECTOR_ALERT_OUTLINE),
        "send-outline" => Some(icons::SEND_OUTLINE),
        #[allow(deprecated)]
        "language-typescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-typescript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_TYPESCRIPT)
        }
        "sticker-minus-outline" => Some(icons::STICKER_MINUS_OUTLINE),
        "bicycle-basket" => Some(icons::BICYCLE_BASKET),
        "golf" => Some(icons::GOLF),
        "arrow-collapse-all" => Some(icons::ARROW_COLLAPSE_ALL),
        "shield-star-outline" => Some(icons::SHIELD_STAR_OUTLINE),
        "code-parentheses-box" => Some(icons::CODE_PARENTHESES_BOX),
        "clock-time-ten-outline" => Some(icons::CLOCK_TIME_TEN_OUTLINE),
        "arrow-expand" => Some(icons::ARROW_EXPAND),
        "table-row-remove" => Some(icons::TABLE_ROW_REMOVE),
        "transfer-left" => Some(icons::TRANSFER_LEFT),
        "map-marker-left-outline" => Some(icons::MAP_MARKER_LEFT_OUTLINE),
        "hubspot" => Some(icons::HUBSPOT),
        "book-arrow-right-outline" => Some(icons::BOOK_ARROW_RIGHT_OUTLINE),
        "lightbulb" => Some(icons::LIGHTBULB),
        "alpha-k-circle-outline" => Some(icons::ALPHA_K_CIRCLE_OUTLINE),
        "alpha-g-box-outline" => Some(icons::ALPHA_G_BOX_OUTLINE),
        "message-star" => Some(icons::MESSAGE_STAR),
        "book-open-page-variant" => Some(icons::BOOK_OPEN_PAGE_VARIANT),
        "clock-out" => Some(icons::CLOCK_OUT),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "tab-remove" => Some(icons::TAB_REMOVE),
        "list-box-outline" => Some(icons::LIST_BOX_OUTLINE),
        "lock-open-alert" => Some(icons::LOCK_OPEN_ALERT),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "folder-arrow-right-outline" => Some(icons::FOLDER_ARROW_RIGHT_OUTLINE),
        "lightbulb-night" => Some(icons::LIGHTBULB_NIGHT),
        "arrow-bottom-right-thin" => Some(icons::ARROW_BOTTOM_RIGHT_THIN),
        "compost" => Some(icons::COMPOST),
        "movie-open-cog-outline" => Some(icons::MOVIE_OPEN_COG_OUTLINE),
        "checkbox-marked-outline" => Some(icons::CHECKBOX_MARKED_OUTLINE),
        "tab-minus" => Some(icons::TAB_MINUS),
        "shaker-outline" => Some(icons::SHAKER_OUTLINE),
        "lightbulb-variant" => Some(icons::LIGHTBULB_VARIANT),
        "archive-sync-outline" => Some(icons::ARCHIVE_SYNC_OUTLINE),
        "ballot-recount-outline" => Some(icons::BALLOT_RECOUNT_OUTLINE),
        "share" => Some(icons::SHARE),
        "solar-panel-large" => Some(icons::SOLAR_PANEL_LARGE),
        "power-plug-battery" => Some(icons::POWER_PLUG_BATTERY),
        "eraser" => Some(icons::ERASER),
        "arrow-bottom-right-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_RIGHT_THIN_CIRCLE_OUTLINE)
        }
        "sort-alphabetical-ascending-variant" => Some(icons::SORT_ALPHABETICAL_ASCENDING_VARIANT),
        "box-cutter-off" => Some(icons::BOX_CUTTER_OFF),
        "swap-vertical-variant" => Some(icons::SWAP_VERTICAL_VARIANT),
        "arrow-left-bold-circle" => Some(icons::ARROW_LEFT_BOLD_CIRCLE),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "alpha-f" => Some(icons::ALPHA_F),
        "page-previous" => Some(icons::PAGE_PREVIOUS),
        #[allow(deprecated)]
        "google-plus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-plus' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PLUS)
        }
        "truck-outline" => Some(icons::TRUCK_OUTLINE),
        "folder-plus-outline" => Some(icons::FOLDER_PLUS_OUTLINE),
        "smoke" => Some(icons::SMOKE),
        "sign-yield" => Some(icons::SIGN_YIELD),
        "wifi-strength-4" => Some(icons::WIFI_STRENGTH_4),
        "key-minus" => Some(icons::KEY_MINUS),
        "cart-check" => Some(icons::CART_CHECK),
        "key-link" => Some(icons::KEY_LINK),
        "camera-lock" => Some(icons::CAMERA_LOCK),
        "comment-off-outline" => Some(icons::COMMENT_OFF_OUTLINE),
        "power-plug-outline" => Some(icons::POWER_PLUG_OUTLINE),
        "inbox-remove" => Some(icons::INBOX_REMOVE),
        "human-male-board-poll" => Some(icons::HUMAN_MALE_BOARD_POLL),
        "hexagon-slice-6" => Some(icons::HEXAGON_SLICE_6),
        "printer-settings" => Some(icons::PRINTER_SETTINGS),
        "account-file-text" => Some(icons::ACCOUNT_FILE_TEXT),
        "sticker-emoji" => Some(icons::STICKER_EMOJI),
        "beekeeper" => Some(icons::BEEKEEPER),
        "folder-key-outline" => Some(icons::FOLDER_KEY_OUTLINE),
        "window-shutter-cog" => Some(icons::WINDOW_SHUTTER_COG),
        "surround-sound-5-1-2" => Some(icons::SURROUND_SOUND_5_1_2),
        "skull-scan" => Some(icons::SKULL_SCAN),
        "head-minus-outline" => Some(icons::HEAD_MINUS_OUTLINE),
        "animation-play-outline" => Some(icons::ANIMATION_PLAY_OUTLINE),
        "battery-charging-wireless-80" => Some(icons::BATTERY_CHARGING_WIRELESS_80),
        #[allow(deprecated)]
        "google-podcast" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-podcast' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_PODCAST)
        }
        "ballot-recount" => Some(icons::BALLOT_RECOUNT),
        "format-indent-increase" => Some(icons::FORMAT_INDENT_INCREASE),
        "ray-end-arrow" => Some(icons::RAY_END_ARROW),
        "floppy-variant" => Some(icons::FLOPPY_VARIANT),
        "launch" => Some(icons::LAUNCH),
        "numeric-9-circle" => Some(icons::NUMERIC_9_CIRCLE),
        "sort-clock-descending" => Some(icons::SORT_CLOCK_DESCENDING),
        "chevron-left-box" => Some(icons::CHEVRON_LEFT_BOX),
        "alpha-l" => Some(icons::ALPHA_L),
        "card-account-phone" => Some(icons::CARD_ACCOUNT_PHONE),
        "screw-lag" => Some(icons::SCREW_LAG),
        "remote" => Some(icons::REMOTE),
        "kayaking" => Some(icons::KAYAKING),
        "access-point-remove" => Some(icons::ACCESS_POINT_REMOVE),
        "bag-carry-on-check" => Some(icons::BAG_CARRY_ON_CHECK),
        "alpha-e" => Some(icons::ALPHA_E),
        "application-variable-outline" => Some(icons::APPLICATION_VARIABLE_OUTLINE),
        "motion-outline" => Some(icons::MOTION_OUTLINE),
        "snowflake-variant" => Some(icons::SNOWFLAKE_VARIANT),
        "content-save-plus" => Some(icons::CONTENT_SAVE_PLUS),
        "table-large" => Some(icons::TABLE_LARGE),
        "ethernet" => Some(icons::ETHERNET),
        "select-arrow-up" => Some(icons::SELECT_ARROW_UP),
        "comment-alert-outline" => Some(icons::COMMENT_ALERT_OUTLINE),
        "clipboard-account-outline" => Some(icons::CLIPBOARD_ACCOUNT_OUTLINE),
        "image-multiple-outline" => Some(icons::IMAGE_MULTIPLE_OUTLINE),
        "expand-all" => Some(icons::EXPAND_ALL),
        "emoticon-outline" => Some(icons::EMOTICON_OUTLINE),
        "clock-star-four-points" => Some(icons::CLOCK_STAR_FOUR_POINTS),
        "fountain" => Some(icons::FOUNTAIN),
        "virus-outline" => Some(icons::VIRUS_OUTLINE),
        "battery-positive" => Some(icons::BATTERY_POSITIVE),
        "store-remove" => Some(icons::STORE_REMOVE),
        "comment-eye-outline" => Some(icons::COMMENT_EYE_OUTLINE),
        "map-marker-account-outline" => Some(icons::MAP_MARKER_ACCOUNT_OUTLINE),
        "toothbrush" => Some(icons::TOOTHBRUSH),
        "diameter-variant" => Some(icons::DIAMETER_VARIANT),
        "send-variant-outline" => Some(icons::SEND_VARIANT_OUTLINE),
        "grave-stone" => Some(icons::GRAVE_STONE),
        "car-search" => Some(icons::CAR_SEARCH),
        "function" => Some(icons::FUNCTION),
        #[allow(deprecated)]
        "tailwind" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'tailwind' is deprecated.").print(py);
            }
            Some(icons::TAILWIND)
        }
        "file-key-outline" => Some(icons::FILE_KEY_OUTLINE),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "multiplication" => Some(icons::MULTIPLICATION),
        "format-text-rotation-down-vertical" => Some(icons::FORMAT_TEXT_ROTATION_DOWN_VERTICAL),
        "home-floor-2" => Some(icons::HOME_FLOOR_2),
        "script-text-play-outline" => Some(icons::SCRIPT_TEXT_PLAY_OUTLINE),
        "hexagon-slice-1" => Some(icons::HEXAGON_SLICE_1),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "ticket-percent-outline" => Some(icons::TICKET_PERCENT_OUTLINE),
        "bullhorn-outline" => Some(icons::BULLHORN_OUTLINE),
        "printer-check" => Some(icons::PRINTER_CHECK),
        "underwear-outline" => Some(icons::UNDERWEAR_OUTLINE),
        "play-pause" => Some(icons::PLAY_PAUSE),
        "umbrella" => Some(icons::UMBRELLA),
        "briefcase-variant-off-outline" => Some(icons::BRIEFCASE_VARIANT_OFF_OUTLINE),
        "cash-sync" => Some(icons::CASH_SYNC),
        "vector-curve" => Some(icons::VECTOR_CURVE),
        "eye-remove-outline" => Some(icons::EYE_REMOVE_OUTLINE),
        "signal-3g" => Some(icons::SIGNAL_3G),
        "plus-minus-variant" => Some(icons::PLUS_MINUS_VARIANT),
        "fish-off" => Some(icons::FISH_OFF),
        "folder-file-outline" => Some(icons::FOLDER_FILE_OUTLINE),
        "incognito-circle-off" => Some(icons::INCOGNITO_CIRCLE_OFF),
        "database-settings" => Some(icons::DATABASE_SETTINGS),
        "eye-lock-outline" => Some(icons::EYE_LOCK_OUTLINE),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "mouse-right-click-outline" => Some(icons::MOUSE_RIGHT_CLICK_OUTLINE),
        "hand-coin" => Some(icons::HAND_COIN),
        "temple-hindu-outline" => Some(icons::TEMPLE_HINDU_OUTLINE),
        "laptop-account" => Some(icons::LAPTOP_ACCOUNT),
        "lock-open-variant-outline" => Some(icons::LOCK_OPEN_VARIANT_OUTLINE),
        "bell-outline" => Some(icons::BELL_OUTLINE),
        "chart-multiple" => Some(icons::CHART_MULTIPLE),
        "table-settings" => Some(icons::TABLE_SETTINGS),
        "arrow-decision-outline" => Some(icons::ARROW_DECISION_OUTLINE),
        "archive-cog" => Some(icons::ARCHIVE_COG),
        "account-filter" => Some(icons::ACCOUNT_FILTER),
        "movie-open-minus-outline" => Some(icons::MOVIE_OPEN_MINUS_OUTLINE),
        "credit-card-search-outline" => Some(icons::CREDIT_CARD_SEARCH_OUTLINE),
        "panorama-horizontal-outline" => Some(icons::PANORAMA_HORIZONTAL_OUTLINE),
        "chart-box-multiple-outline" => Some(icons::CHART_BOX_MULTIPLE_OUTLINE),
        "file-word" => Some(icons::FILE_WORD),
        "flag" => Some(icons::FLAG),
        "hanger" => Some(icons::HANGER),
        "progress-check" => Some(icons::PROGRESS_CHECK),
        #[allow(deprecated)]
        "google" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google' is deprecated.").print(py);
            }
            Some(icons::GOOGLE)
        }
        "comment-question" => Some(icons::COMMENT_QUESTION),
        "coffee-maker-check" => Some(icons::COFFEE_MAKER_CHECK),
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
        "perspective-more" => Some(icons::PERSPECTIVE_MORE),
        "induction" => Some(icons::INDUCTION),
        "overscan" => Some(icons::OVERSCAN),
        "grid-large" => Some(icons::GRID_LARGE),
        "timeline-remove-outline" => Some(icons::TIMELINE_REMOVE_OUTLINE),
        "basket-off-outline" => Some(icons::BASKET_OFF_OUTLINE),
        "video-wireless" => Some(icons::VIDEO_WIRELESS),
        "star-box-outline" => Some(icons::STAR_BOX_OUTLINE),
        #[allow(deprecated)]
        "redhat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'redhat' is deprecated.").print(py);
            }
            Some(icons::REDHAT)
        }
        "ghost" => Some(icons::GHOST),
        "connection" => Some(icons::CONNECTION),
        "format-underline" => Some(icons::FORMAT_UNDERLINE),
        "atom-variant" => Some(icons::ATOM_VARIANT),
        "cupcake" => Some(icons::CUPCAKE),
        "sort-numeric-descending-variant" => Some(icons::SORT_NUMERIC_DESCENDING_VARIANT),
        "ladybug" => Some(icons::LADYBUG),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "inbox-full" => Some(icons::INBOX_FULL),
        "printer-pos-play-outline" => Some(icons::PRINTER_POS_PLAY_OUTLINE),
        #[allow(deprecated)]
        "language-css3" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-css3' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_CSS3)
        }
        "shield-alert" => Some(icons::SHIELD_ALERT),
        "shield-crown-outline" => Some(icons::SHIELD_CROWN_OUTLINE),
        "battery-negative" => Some(icons::BATTERY_NEGATIVE),
        "image-filter-hdr-outline" => Some(icons::IMAGE_FILTER_HDR_OUTLINE),
        "minidisc" => Some(icons::MINIDISC),
        "battery-20" => Some(icons::BATTERY_20),
        "signature" => Some(icons::SIGNATURE),
        _ => None,
    }
}
