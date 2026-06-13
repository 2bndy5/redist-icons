// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_10(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "panorama-fisheye" => Some(icons::PANORAMA_FISHEYE),
        "airplane-off" => Some(icons::AIRPLANE_OFF),
        "ray-end-arrow" => Some(icons::RAY_END_ARROW),
        "circle-multiple-outline" => Some(icons::CIRCLE_MULTIPLE_OUTLINE),
        "close-network" => Some(icons::CLOSE_NETWORK),
        "cellphone-check" => Some(icons::CELLPHONE_CHECK),
        "camera-retake" => Some(icons::CAMERA_RETAKE),
        "soy-sauce-off" => Some(icons::SOY_SAUCE_OFF),
        "opacity" => Some(icons::OPACITY),
        "heart-minus" => Some(icons::HEART_MINUS),
        "transcribe-close" => Some(icons::TRANSCRIBE_CLOSE),
        "office-building-marker-outline" => Some(icons::OFFICE_BUILDING_MARKER_OUTLINE),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "crown" => Some(icons::CROWN),
        "balloon" => Some(icons::BALLOON),
        "arrow-left-drop-circle-outline" => Some(icons::ARROW_LEFT_DROP_CIRCLE_OUTLINE),
        "shopping-search" => Some(icons::SHOPPING_SEARCH),
        "source-commit-end" => Some(icons::SOURCE_COMMIT_END),
        "paperclip-plus" => Some(icons::PAPERCLIP_PLUS),
        "arrow-up-drop-circle" => Some(icons::ARROW_UP_DROP_CIRCLE),
        "card-text" => Some(icons::CARD_TEXT),
        "truck-remove-outline" => Some(icons::TRUCK_REMOVE_OUTLINE),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "archive-lock" => Some(icons::ARCHIVE_LOCK),
        #[allow(deprecated)]
        "unreal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unreal' is deprecated.").print(py);
            }
            Some(icons::UNREAL)
        }
        "server-network-outline" => Some(icons::SERVER_NETWORK_OUTLINE),
        "battery-charging-90" => Some(icons::BATTERY_CHARGING_90),
        "hops" => Some(icons::HOPS),
        "axis-arrow" => Some(icons::AXIS_ARROW),
        "heart-cog-outline" => Some(icons::HEART_COG_OUTLINE),
        "closed-caption-outline" => Some(icons::CLOSED_CAPTION_OUTLINE),
        "shovel" => Some(icons::SHOVEL),
        "shield-account-variant" => Some(icons::SHIELD_ACCOUNT_VARIANT),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "flip-vertical" => Some(icons::FLIP_VERTICAL),
        "package-variant-closed-plus" => Some(icons::PACKAGE_VARIANT_CLOSED_PLUS),
        "braille" => Some(icons::BRAILLE),
        "link" => Some(icons::LINK),
        "magic-staff" => Some(icons::MAGIC_STAFF),
        "file-cloud-outline" => Some(icons::FILE_CLOUD_OUTLINE),
        "arrow-down-bold-box-outline" => Some(icons::ARROW_DOWN_BOLD_BOX_OUTLINE),
        "access-point-network-off" => Some(icons::ACCESS_POINT_NETWORK_OFF),
        "timer-check-outline" => Some(icons::TIMER_CHECK_OUTLINE),
        "lightbulb-outline" => Some(icons::LIGHTBULB_OUTLINE),
        "database-export" => Some(icons::DATABASE_EXPORT),
        "set-right" => Some(icons::SET_RIGHT),
        "battery-lock" => Some(icons::BATTERY_LOCK),
        "head-lightbulb" => Some(icons::HEAD_LIGHTBULB),
        "restart" => Some(icons::RESTART),
        "find-replace" => Some(icons::FIND_REPLACE),
        #[allow(deprecated)]
        "google-spreadsheet" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-spreadsheet' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_SPREADSHEET)
        }
        "account-tag-outline" => Some(icons::ACCOUNT_TAG_OUTLINE),
        #[allow(deprecated)]
        "language-html5" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-html5' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_HTML5)
        }
        "ssh" => Some(icons::SSH),
        "arrow-expand-up" => Some(icons::ARROW_EXPAND_UP),
        #[allow(deprecated)]
        "microsoft-powerpoint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-powerpoint' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_POWERPOINT)
        }
        "home-group" => Some(icons::HOME_GROUP),
        "file-multiple-outline" => Some(icons::FILE_MULTIPLE_OUTLINE),
        "ballot-outline" => Some(icons::BALLOT_OUTLINE),
        "page-next" => Some(icons::PAGE_NEXT),
        "phone-outline" => Some(icons::PHONE_OUTLINE),
        "toggle-switch-outline" => Some(icons::TOGGLE_SWITCH_OUTLINE),
        #[allow(deprecated)]
        "apple-safari" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-safari' is deprecated.").print(py);
            }
            Some(icons::APPLE_SAFARI)
        }
        "alphabetical-variant" => Some(icons::ALPHABETICAL_VARIANT),
        "phone-remove" => Some(icons::PHONE_REMOVE),
        "file-certificate" => Some(icons::FILE_CERTIFICATE),
        "alpha-p-box" => Some(icons::ALPHA_P_BOX),
        "music-note-eighth-dotted" => Some(icons::MUSIC_NOTE_EIGHTH_DOTTED),
        "message-text-lock" => Some(icons::MESSAGE_TEXT_LOCK),
        "archive-cog-outline" => Some(icons::ARCHIVE_COG_OUTLINE),
        "slot-machine-outline" => Some(icons::SLOT_MACHINE_OUTLINE),
        "leaf-maple-off" => Some(icons::LEAF_MAPLE_OFF),
        "sort-clock-descending-outline" => Some(icons::SORT_CLOCK_DESCENDING_OUTLINE),
        "silverware-fork-knife" => Some(icons::SILVERWARE_FORK_KNIFE),
        "square" => Some(icons::SQUARE),
        "file-document-plus-outline" => Some(icons::FILE_DOCUMENT_PLUS_OUTLINE),
        "briefcase-check" => Some(icons::BRIEFCASE_CHECK),
        "play-box-multiple" => Some(icons::PLAY_BOX_MULTIPLE),
        "water-boiler-alert" => Some(icons::WATER_BOILER_ALERT),
        "doorbell" => Some(icons::DOORBELL),
        "numeric-5-box-outline" => Some(icons::NUMERIC_5_BOX_OUTLINE),
        "arrow-u-down-left-bold" => Some(icons::ARROW_U_DOWN_LEFT_BOLD),
        "arrow-bottom-right-thick" => Some(icons::ARROW_BOTTOM_RIGHT_THICK),
        "podium" => Some(icons::PODIUM),
        "book-plus-outline" => Some(icons::BOOK_PLUS_OUTLINE),
        "robot-confused" => Some(icons::ROBOT_CONFUSED),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "keyboard-return" => Some(icons::KEYBOARD_RETURN),
        "transfer-up" => Some(icons::TRANSFER_UP),
        "currency-mnt" => Some(icons::CURRENCY_MNT),
        "truck-check-outline" => Some(icons::TRUCK_CHECK_OUTLINE),
        "skull-crossbones" => Some(icons::SKULL_CROSSBONES),
        "format-header-6" => Some(icons::FORMAT_HEADER_6),
        "robot-angry-outline" => Some(icons::ROBOT_ANGRY_OUTLINE),
        #[allow(deprecated)]
        "arch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'arch' is deprecated.").print(py);
            }
            Some(icons::ARCH)
        }
        "arrow-left-circle-outline" => Some(icons::ARROW_LEFT_CIRCLE_OUTLINE),
        "kettle-steam" => Some(icons::KETTLE_STEAM),
        "book-off" => Some(icons::BOOK_OFF),
        "smoke-detector-variant-alert" => Some(icons::SMOKE_DETECTOR_VARIANT_ALERT),
        "movie-off-outline" => Some(icons::MOVIE_OFF_OUTLINE),
        "arrow-top-right-bottom-left-bold" => Some(icons::ARROW_TOP_RIGHT_BOTTOM_LEFT_BOLD),
        "fridge-industrial-off" => Some(icons::FRIDGE_INDUSTRIAL_OFF),
        #[allow(deprecated)]
        "nintendo-switch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-switch' is deprecated.")
                    .print(py);
            }
            Some(icons::NINTENDO_SWITCH)
        }
        "clipboard-flow-outline" => Some(icons::CLIPBOARD_FLOW_OUTLINE),
        "bluetooth-audio" => Some(icons::BLUETOOTH_AUDIO),
        "highway" => Some(icons::HIGHWAY),
        "car-pickup" => Some(icons::CAR_PICKUP),
        "flag-variant-plus" => Some(icons::FLAG_VARIANT_PLUS),
        "instrument-triangle" => Some(icons::INSTRUMENT_TRIANGLE),
        "file-search-outline" => Some(icons::FILE_SEARCH_OUTLINE),
        "hand-clap" => Some(icons::HAND_CLAP),
        "transit-connection-variant" => Some(icons::TRANSIT_CONNECTION_VARIANT),
        "sort-alphabetical-variant" => Some(icons::SORT_ALPHABETICAL_VARIANT),
        "shield-bug" => Some(icons::SHIELD_BUG),
        "calendar-range" => Some(icons::CALENDAR_RANGE),
        "human-edit" => Some(icons::HUMAN_EDIT),
        "ideogram-cjk-variant" => Some(icons::IDEOGRAM_CJK_VARIANT),
        "alpha-w-circle-outline" => Some(icons::ALPHA_W_CIRCLE_OUTLINE),
        "car-emergency" => Some(icons::CAR_EMERGENCY),
        "swim" => Some(icons::SWIM),
        "cookie" => Some(icons::COOKIE),
        "help-circle-outline" => Some(icons::HELP_CIRCLE_OUTLINE),
        "bed-queen-outline" => Some(icons::BED_QUEEN_OUTLINE),
        "dog" => Some(icons::DOG),
        "account-minus" => Some(icons::ACCOUNT_MINUS),
        "baguette" => Some(icons::BAGUETTE),
        "animation" => Some(icons::ANIMATION),
        "umbrella-closed" => Some(icons::UMBRELLA_CLOSED),
        "arrow-top-left-bold-outline" => Some(icons::ARROW_TOP_LEFT_BOLD_OUTLINE),
        "alpha-g-circle-outline" => Some(icons::ALPHA_G_CIRCLE_OUTLINE),
        "sort-bool-descending-variant" => Some(icons::SORT_BOOL_DESCENDING_VARIANT),
        "podcast" => Some(icons::PODCAST),
        "dns-outline" => Some(icons::DNS_OUTLINE),
        "lightbulb-on-40" => Some(icons::LIGHTBULB_ON_40),
        "cog-sync-outline" => Some(icons::COG_SYNC_OUTLINE),
        "arrow-up-down" => Some(icons::ARROW_UP_DOWN),
        "gesture-swipe-horizontal" => Some(icons::GESTURE_SWIPE_HORIZONTAL),
        "playlist-music" => Some(icons::PLAYLIST_MUSIC),
        "file-certificate-outline" => Some(icons::FILE_CERTIFICATE_OUTLINE),
        "weather-hail" => Some(icons::WEATHER_HAIL),
        "pencil-remove" => Some(icons::PENCIL_REMOVE),
        "share-off" => Some(icons::SHARE_OFF),
        "timeline-minus-outline" => Some(icons::TIMELINE_MINUS_OUTLINE),
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "format-pilcrow-arrow-right" => Some(icons::FORMAT_PILCROW_ARROW_RIGHT),
        "mailbox-open-outline" => Some(icons::MAILBOX_OPEN_OUTLINE),
        "file-move" => Some(icons::FILE_MOVE),
        "clock-end" => Some(icons::CLOCK_END),
        #[allow(deprecated)]
        "zigbee" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zigbee' is deprecated.").print(py);
            }
            Some(icons::ZIGBEE)
        }
        "eye-off" => Some(icons::EYE_OFF),
        "lasso" => Some(icons::LASSO),
        "wifi-arrow-right" => Some(icons::WIFI_ARROW_RIGHT),
        "camera-metering-matrix" => Some(icons::CAMERA_METERING_MATRIX),
        "format-color-highlight" => Some(icons::FORMAT_COLOR_HIGHLIGHT),
        "dice-1-outline" => Some(icons::DICE_1_OUTLINE),
        "mouse-off" => Some(icons::MOUSE_OFF),
        "notebook-edit" => Some(icons::NOTEBOOK_EDIT),
        "alpha-y-box" => Some(icons::ALPHA_Y_BOX),
        "view-dashboard" => Some(icons::VIEW_DASHBOARD),
        "run-fast" => Some(icons::RUN_FAST),
        "web-cancel" => Some(icons::WEB_CANCEL),
        "baseball-bat" => Some(icons::BASEBALL_BAT),
        "music-note-whole" => Some(icons::MUSIC_NOTE_WHOLE),
        "view-agenda-outline" => Some(icons::VIEW_AGENDA_OUTLINE),
        "alert-box-outline" => Some(icons::ALERT_BOX_OUTLINE),
        "folder-heart" => Some(icons::FOLDER_HEART),
        "image-refresh" => Some(icons::IMAGE_REFRESH),
        "view-array-outline" => Some(icons::VIEW_ARRAY_OUTLINE),
        "calendar-sync-outline" => Some(icons::CALENDAR_SYNC_OUTLINE),
        "power-sleep" => Some(icons::POWER_SLEEP),
        "water-circle" => Some(icons::WATER_CIRCLE),
        "arrow-expand-down" => Some(icons::ARROW_EXPAND_DOWN),
        "map-marker-plus-outline" => Some(icons::MAP_MARKER_PLUS_OUTLINE),
        "qrcode-plus" => Some(icons::QRCODE_PLUS),
        "microphone-variant" => Some(icons::MICROPHONE_VARIANT),
        "power-off" => Some(icons::POWER_OFF),
        "diving-scuba-flag" => Some(icons::DIVING_SCUBA_FLAG),
        "account-cowboy-hat" => Some(icons::ACCOUNT_COWBOY_HAT),
        "calendar-alert" => Some(icons::CALENDAR_ALERT),
        "basket-remove" => Some(icons::BASKET_REMOVE),
        "timer-sand-complete" => Some(icons::TIMER_SAND_COMPLETE),
        "phone-minus-outline" => Some(icons::PHONE_MINUS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-visual-studio-code" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-visual-studio-code' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO_CODE)
        }
        "watch-import" => Some(icons::WATCH_IMPORT),
        "message-text-clock-outline" => Some(icons::MESSAGE_TEXT_CLOCK_OUTLINE),
        "truck-plus-outline" => Some(icons::TRUCK_PLUS_OUTLINE),
        "subway-alert-variant" => Some(icons::SUBWAY_ALERT_VARIANT),
        "file-document-outline" => Some(icons::FILE_DOCUMENT_OUTLINE),
        "surfing" => Some(icons::SURFING),
        "note-remove" => Some(icons::NOTE_REMOVE),
        "file-document" => Some(icons::FILE_DOCUMENT),
        "account-sync-outline" => Some(icons::ACCOUNT_SYNC_OUTLINE),
        "pencil-remove-outline" => Some(icons::PENCIL_REMOVE_OUTLINE),
        "view-parallel-outline" => Some(icons::VIEW_PARALLEL_OUTLINE),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        "folder-home-outline" => Some(icons::FOLDER_HOME_OUTLINE),
        "bell-remove" => Some(icons::BELL_REMOVE),
        "skull-outline" => Some(icons::SKULL_OUTLINE),
        "border-radius" => Some(icons::BORDER_RADIUS),
        _ => None,
    }
}
