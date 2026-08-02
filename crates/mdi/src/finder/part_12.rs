// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "calendar-edit-outline" => Some(icons::CALENDAR_EDIT_OUTLINE),
        "egg-off" => Some(icons::EGG_OFF),
        "exclamation-thick" => Some(icons::EXCLAMATION_THICK),
        "web-box" => Some(icons::WEB_BOX),
        "radiator" => Some(icons::RADIATOR),
        "call-received" => Some(icons::CALL_RECEIVED),
        "cash" => Some(icons::CASH),
        "microwave-off" => Some(icons::MICROWAVE_OFF),
        "source-commit" => Some(icons::SOURCE_COMMIT),
        "alpha-b-circle-outline" => Some(icons::ALPHA_B_CIRCLE_OUTLINE),
        "bowl-outline" => Some(icons::BOWL_OUTLINE),
        "lan" => Some(icons::LAN),
        "looks" => Some(icons::LOOKS),
        "lock-minus-outline" => Some(icons::LOCK_MINUS_OUTLINE),
        "gate-and" => Some(icons::GATE_AND),
        #[allow(deprecated)]
        "reddit" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'reddit' is deprecated.").print(py);
            }
            Some(icons::REDDIT)
        }
        "lock-off-outline" => Some(icons::LOCK_OFF_OUTLINE),
        "radius-outline" => Some(icons::RADIUS_OUTLINE),
        "file-rotate-right" => Some(icons::FILE_ROTATE_RIGHT),
        "clock-check-outline" => Some(icons::CLOCK_CHECK_OUTLINE),
        "heart-half-outline" => Some(icons::HEART_HALF_OUTLINE),
        "hand-pointing-right" => Some(icons::HAND_POINTING_RIGHT),
        "thermometer-plus" => Some(icons::THERMOMETER_PLUS),
        "briefcase-clock" => Some(icons::BRIEFCASE_CLOCK),
        "air-filter" => Some(icons::AIR_FILTER),
        "wrench-outline" => Some(icons::WRENCH_OUTLINE),
        "gamepad-round-down" => Some(icons::GAMEPAD_ROUND_DOWN),
        "head-alert-outline" => Some(icons::HEAD_ALERT_OUTLINE),
        "timeline-alert-outline" => Some(icons::TIMELINE_ALERT_OUTLINE),
        #[allow(deprecated)]
        "microsoft-excel" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-excel' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_EXCEL)
        }
        "loupe" => Some(icons::LOUPE),
        "wallet-travel" => Some(icons::WALLET_TRAVEL),
        "clock-end" => Some(icons::CLOCK_END),
        "image-outline" => Some(icons::IMAGE_OUTLINE),
        "bacteria-outline" => Some(icons::BACTERIA_OUTLINE),
        "home-export-outline" => Some(icons::HOME_EXPORT_OUTLINE),
        "car-door-lock-open" => Some(icons::CAR_DOOR_LOCK_OPEN),
        "play-box-lock" => Some(icons::PLAY_BOX_LOCK),
        "human-handsup" => Some(icons::HUMAN_HANDSUP),
        "printer-pos-edit" => Some(icons::PRINTER_POS_EDIT),
        "flag-plus" => Some(icons::FLAG_PLUS),
        "silverware-fork" => Some(icons::SILVERWARE_FORK),
        "file-word-outline" => Some(icons::FILE_WORD_OUTLINE),
        "bandage" => Some(icons::BANDAGE),
        "podcast" => Some(icons::PODCAST),
        "wall-sconce-flat-variant" => Some(icons::WALL_SCONCE_FLAT_VARIANT),
        "fan-clock" => Some(icons::FAN_CLOCK),
        "shoe-ballet" => Some(icons::SHOE_BALLET),
        "home-battery-outline" => Some(icons::HOME_BATTERY_OUTLINE),
        "handcuffs" => Some(icons::HANDCUFFS),
        "arrow-left-top" => Some(icons::ARROW_LEFT_TOP),
        "microphone-outline" => Some(icons::MICROPHONE_OUTLINE),
        "camera-metering-spot" => Some(icons::CAMERA_METERING_SPOT),
        "phone-outgoing" => Some(icons::PHONE_OUTGOING),
        "currency-gbp" => Some(icons::CURRENCY_GBP),
        "bag-personal-outline" => Some(icons::BAG_PERSONAL_OUTLINE),
        "food-fork-drink" => Some(icons::FOOD_FORK_DRINK),
        "pickaxe" => Some(icons::PICKAXE),
        "checkbox-marked-outline" => Some(icons::CHECKBOX_MARKED_OUTLINE),
        "invoice-text-outline" => Some(icons::INVOICE_TEXT_OUTLINE),
        "cellphone-message" => Some(icons::CELLPHONE_MESSAGE),
        "car-arrow-right" => Some(icons::CAR_ARROW_RIGHT),
        #[allow(deprecated)]
        "dropbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dropbox' is deprecated.").print(py);
            }
            Some(icons::DROPBOX)
        }
        "close-circle-multiple-outline" => Some(icons::CLOSE_CIRCLE_MULTIPLE_OUTLINE),
        "sofa" => Some(icons::SOFA),
        "paper-roll-outline" => Some(icons::PAPER_ROLL_OUTLINE),
        "barrel" => Some(icons::BARREL),
        "select-multiple-marker" => Some(icons::SELECT_MULTIPLE_MARKER),
        "alert-box" => Some(icons::ALERT_BOX),
        "relation-one-to-only-one" => Some(icons::RELATION_ONE_TO_ONLY_ONE),
        "calendar-export" => Some(icons::CALENDAR_EXPORT),
        "monitor-arrow-down-variant" => Some(icons::MONITOR_ARROW_DOWN_VARIANT),
        "train-car-hopper-full" => Some(icons::TRAIN_CAR_HOPPER_FULL),
        "image-multiple-outline" => Some(icons::IMAGE_MULTIPLE_OUTLINE),
        "plus-lock" => Some(icons::PLUS_LOCK),
        "head-check" => Some(icons::HEAD_CHECK),
        "transit-detour" => Some(icons::TRANSIT_DETOUR),
        "restore-alert" => Some(icons::RESTORE_ALERT),
        "cellphone-key" => Some(icons::CELLPHONE_KEY),
        "battery-charging-wireless" => Some(icons::BATTERY_CHARGING_WIRELESS),
        "flash-triangle-outline" => Some(icons::FLASH_TRIANGLE_OUTLINE),
        "hours-12" => Some(icons::HOURS_12),
        "camcorder" => Some(icons::CAMCORDER),
        "volume-source" => Some(icons::VOLUME_SOURCE),
        "exit-to-app" => Some(icons::EXIT_TO_APP),
        "content-save-cog" => Some(icons::CONTENT_SAVE_COG),
        "projector-screen-outline" => Some(icons::PROJECTOR_SCREEN_OUTLINE),
        "share-off" => Some(icons::SHARE_OFF),
        "view-split-horizontal" => Some(icons::VIEW_SPLIT_HORIZONTAL),
        "home-edit" => Some(icons::HOME_EDIT),
        "scanner" => Some(icons::SCANNER),
        "format-list-numbered" => Some(icons::FORMAT_LIST_NUMBERED),
        "broadcast-off" => Some(icons::BROADCAST_OFF),
        "raspberry-pi" => Some(icons::RASPBERRY_PI),
        "arrow-projectile" => Some(icons::ARROW_PROJECTILE),
        #[allow(deprecated)]
        "google-nearby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-nearby' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_NEARBY)
        }
        "seat-legroom-extra" => Some(icons::SEAT_LEGROOM_EXTRA),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "rocket" => Some(icons::ROCKET),
        "chevron-right-circle" => Some(icons::CHEVRON_RIGHT_CIRCLE),
        "clock-edit" => Some(icons::CLOCK_EDIT),
        "printer-pos-plus" => Some(icons::PRINTER_POS_PLUS),
        "account-cog" => Some(icons::ACCOUNT_COG),
        "clipboard-arrow-right-outline" => Some(icons::CLIPBOARD_ARROW_RIGHT_OUTLINE),
        "arrow-bottom-right-bold-box" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX),
        "asterisk-circle-outline" => Some(icons::ASTERISK_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "gatsby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gatsby' is deprecated.").print(py);
            }
            Some(icons::GATSBY)
        }
        "arrow-down-circle" => Some(icons::ARROW_DOWN_CIRCLE),
        "chat" => Some(icons::CHAT),
        "numeric-0-box-multiple" => Some(icons::NUMERIC_0_BOX_MULTIPLE),
        "battery-heart-outline" => Some(icons::BATTERY_HEART_OUTLINE),
        "flag-minus-outline" => Some(icons::FLAG_MINUS_OUTLINE),
        "timer-sand" => Some(icons::TIMER_SAND),
        "deathly-hallows" => Some(icons::DEATHLY_HALLOWS),
        "blinds-horizontal" => Some(icons::BLINDS_HORIZONTAL),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "wind-power" => Some(icons::WIND_POWER),
        "ray-vertex" => Some(icons::RAY_VERTEX),
        "text" => Some(icons::TEXT),
        "firework" => Some(icons::FIREWORK),
        "link-box-variant" => Some(icons::LINK_BOX_VARIANT),
        "human-capacity-increase" => Some(icons::HUMAN_CAPACITY_INCREASE),
        "fridge-variant-alert" => Some(icons::FRIDGE_VARIANT_ALERT),
        "find-replace" => Some(icons::FIND_REPLACE),
        #[allow(deprecated)]
        "emby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'emby' is deprecated.").print(py);
            }
            Some(icons::EMBY)
        }
        "submarine" => Some(icons::SUBMARINE),
        "carrot" => Some(icons::CARROT),
        "file-table-box-multiple-outline" => Some(icons::FILE_TABLE_BOX_MULTIPLE_OUTLINE),
        "cards-playing-diamond-multiple" => Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE),
        "cookie-minus" => Some(icons::COOKIE_MINUS),
        "delete-sweep-outline" => Some(icons::DELETE_SWEEP_OUTLINE),
        "library" => Some(icons::LIBRARY),
        "battery-charging-30" => Some(icons::BATTERY_CHARGING_30),
        "radio-off" => Some(icons::RADIO_OFF),
        "share-all-outline" => Some(icons::SHARE_ALL_OUTLINE),
        "chevron-left-circle" => Some(icons::CHEVRON_LEFT_CIRCLE),
        "clipboard-plus" => Some(icons::CLIPBOARD_PLUS),
        "page-next" => Some(icons::PAGE_NEXT),
        "face-man-profile" => Some(icons::FACE_MAN_PROFILE),
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "timer-sand-empty" => Some(icons::TIMER_SAND_EMPTY),
        "gamepad-left" => Some(icons::GAMEPAD_LEFT),
        "arrow-bottom-right-thin" => Some(icons::ARROW_BOTTOM_RIGHT_THIN),
        "sim-outline" => Some(icons::SIM_OUTLINE),
        "timeline-text-outline" => Some(icons::TIMELINE_TEXT_OUTLINE),
        "bookmark-plus-outline" => Some(icons::BOOKMARK_PLUS_OUTLINE),
        "camera-lock" => Some(icons::CAMERA_LOCK),
        "alert-outline" => Some(icons::ALERT_OUTLINE),
        "fast-forward-10" => Some(icons::FAST_FORWARD_10),
        "receipt" => Some(icons::RECEIPT),
        "gate-or" => Some(icons::GATE_OR),
        "database" => Some(icons::DATABASE),
        "tooltip-text" => Some(icons::TOOLTIP_TEXT),
        "lock-percent" => Some(icons::LOCK_PERCENT),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "mushroom-outline" => Some(icons::MUSHROOM_OUTLINE),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "trumpet" => Some(icons::TRUMPET),
        "auto-download" => Some(icons::AUTO_DOWNLOAD),
        "curling" => Some(icons::CURLING),
        "star-four-points" => Some(icons::STAR_FOUR_POINTS),
        "file-image-marker" => Some(icons::FILE_IMAGE_MARKER),
        "shovel-off" => Some(icons::SHOVEL_OFF),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "roman-numeral-5" => Some(icons::ROMAN_NUMERAL_5),
        "printer-pos-play-outline" => Some(icons::PRINTER_POS_PLAY_OUTLINE),
        "seat" => Some(icons::SEAT),
        "folder-arrow-down" => Some(icons::FOLDER_ARROW_DOWN),
        "transit-connection" => Some(icons::TRANSIT_CONNECTION),
        "office-building-minus" => Some(icons::OFFICE_BUILDING_MINUS),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "food-kosher" => Some(icons::FOOD_KOSHER),
        "logout" => Some(icons::LOGOUT),
        "alert-rhombus" => Some(icons::ALERT_RHOMBUS),
        "battery-30-bluetooth" => Some(icons::BATTERY_30_BLUETOOTH),
        "sleep-off" => Some(icons::SLEEP_OFF),
        "format-align-center" => Some(icons::FORMAT_ALIGN_CENTER),
        "fast-forward-outline" => Some(icons::FAST_FORWARD_OUTLINE),
        "carabiner" => Some(icons::CARABINER),
        "invoice-text-clock" => Some(icons::INVOICE_TEXT_CLOCK),
        "hand-back-right-off-outline" => Some(icons::HAND_BACK_RIGHT_OFF_OUTLINE),
        "rewind-30" => Some(icons::REWIND_30),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        "camera-retake" => Some(icons::CAMERA_RETAKE),
        "database-arrow-up-outline" => Some(icons::DATABASE_ARROW_UP_OUTLINE),
        "update" => Some(icons::UPDATE),
        "lightbulb-group-off-outline" => Some(icons::LIGHTBULB_GROUP_OFF_OUTLINE),
        "camera-enhance" => Some(icons::CAMERA_ENHANCE),
        "page-layout-header-footer" => Some(icons::PAGE_LAYOUT_HEADER_FOOTER),
        "air-purifier-off" => Some(icons::AIR_PURIFIER_OFF),
        "beekeeper" => Some(icons::BEEKEEPER),
        "cable-data" => Some(icons::CABLE_DATA),
        "airplane-clock" => Some(icons::AIRPLANE_CLOCK),
        "eye-minus" => Some(icons::EYE_MINUS),
        "compare-horizontal" => Some(icons::COMPARE_HORIZONTAL),
        "view-dashboard-variant" => Some(icons::VIEW_DASHBOARD_VARIANT),
        "boom-gate" => Some(icons::BOOM_GATE),
        "arrow-collapse-down" => Some(icons::ARROW_COLLAPSE_DOWN),
        "tent" => Some(icons::TENT),
        "lightbulb-off" => Some(icons::LIGHTBULB_OFF),
        _ => None,
    }
}
