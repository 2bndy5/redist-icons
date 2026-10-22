// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_14(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "hours-24" => Some(icons::HOURS_24),
        "latitude" => Some(icons::LATITUDE),
        "ufo-outline" => Some(icons::UFO_OUTLINE),
        "puzzle-star" => Some(icons::PUZZLE_STAR),
        "laser-pointer" => Some(icons::LASER_POINTER),
        "smoke-detector-off" => Some(icons::SMOKE_DETECTOR_OFF),
        "waves-arrow-left" => Some(icons::WAVES_ARROW_LEFT),
        "image-filter-center-focus-strong" => Some(icons::IMAGE_FILTER_CENTER_FOCUS_STRONG),
        "share-all" => Some(icons::SHARE_ALL),
        "shield-bug-outline" => Some(icons::SHIELD_BUG_OUTLINE),
        "alert-remove" => Some(icons::ALERT_REMOVE),
        "train-car-box-open" => Some(icons::TRAIN_CAR_BOX_OPEN),
        "cloud-arrow-up-outline" => Some(icons::CLOUD_ARROW_UP_OUTLINE),
        "remote-tv-off" => Some(icons::REMOTE_TV_OFF),
        "human" => Some(icons::HUMAN),
        "transit-connection" => Some(icons::TRANSIT_CONNECTION),
        "lightbulb-fluorescent-tube" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE),
        "bug-outline" => Some(icons::BUG_OUTLINE),
        "calendar-blank-multiple" => Some(icons::CALENDAR_BLANK_MULTIPLE),
        "book-play" => Some(icons::BOOK_PLAY),
        "cursor-default-click-outline" => Some(icons::CURSOR_DEFAULT_CLICK_OUTLINE),
        "send-check" => Some(icons::SEND_CHECK),
        "engine-off" => Some(icons::ENGINE_OFF),
        "shield-sync" => Some(icons::SHIELD_SYNC),
        "wifi-strength-lock-outline" => Some(icons::WIFI_STRENGTH_LOCK_OUTLINE),
        "counter" => Some(icons::COUNTER),
        "fast-forward-45" => Some(icons::FAST_FORWARD_45),
        "microphone" => Some(icons::MICROPHONE),
        #[allow(deprecated)]
        "vuetify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuetify' is deprecated.").print(py);
            }
            Some(icons::VUETIFY)
        }
        "pump-off" => Some(icons::PUMP_OFF),
        "playlist-music-outline" => Some(icons::PLAYLIST_MUSIC_OUTLINE),
        "delete-forever" => Some(icons::DELETE_FOREVER),
        "cards-playing-outline" => Some(icons::CARDS_PLAYING_OUTLINE),
        "file-swap" => Some(icons::FILE_SWAP),
        "bell" => Some(icons::BELL),
        "printer-pos-sync" => Some(icons::PRINTER_POS_SYNC),
        "moon-waning-crescent" => Some(icons::MOON_WANING_CRESCENT),
        "vanish" => Some(icons::VANISH),
        "map-marker-outline" => Some(icons::MAP_MARKER_OUTLINE),
        "folder-download-outline" => Some(icons::FOLDER_DOWNLOAD_OUTLINE),
        "arrow-collapse-up" => Some(icons::ARROW_COLLAPSE_UP),
        "compost" => Some(icons::COMPOST),
        "equalizer" => Some(icons::EQUALIZER),
        "home-lock-open" => Some(icons::HOME_LOCK_OPEN),
        "close-box-multiple" => Some(icons::CLOSE_BOX_MULTIPLE),
        "table-multiple" => Some(icons::TABLE_MULTIPLE),
        "thermometer-probe-off" => Some(icons::THERMOMETER_PROBE_OFF),
        "tally-mark-3" => Some(icons::TALLY_MARK_3),
        #[allow(deprecated)]
        "nintendo-switch" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-switch' is deprecated.")
                    .print(py);
            }
            Some(icons::NINTENDO_SWITCH)
        }
        "toothbrush" => Some(icons::TOOTHBRUSH),
        "valve" => Some(icons::VALVE),
        "bus-marker" => Some(icons::BUS_MARKER),
        "file-document-arrow-right" => Some(icons::FILE_DOCUMENT_ARROW_RIGHT),
        "printer-pos-alert" => Some(icons::PRINTER_POS_ALERT),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "clipboard-text-multiple-outline" => Some(icons::CLIPBOARD_TEXT_MULTIPLE_OUTLINE),
        "chart-waterfall" => Some(icons::CHART_WATERFALL),
        "gate" => Some(icons::GATE),
        "current-ac" => Some(icons::CURRENT_AC),
        "hand-peace" => Some(icons::HAND_PEACE),
        "alpha-g-circle" => Some(icons::ALPHA_G_CIRCLE),
        "kettle" => Some(icons::KETTLE),
        "calendar-question-outline" => Some(icons::CALENDAR_QUESTION_OUTLINE),
        "archive-arrow-down" => Some(icons::ARCHIVE_ARROW_DOWN),
        #[allow(deprecated)]
        "language-ruby-on-rails" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby-on-rails' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_RUBY_ON_RAILS)
        }
        "sign-direction-remove" => Some(icons::SIGN_DIRECTION_REMOVE),
        "format-header-decrease" => Some(icons::FORMAT_HEADER_DECREASE),
        "disc" => Some(icons::DISC),
        "shield-sword-outline" => Some(icons::SHIELD_SWORD_OUTLINE),
        "forum-minus" => Some(icons::FORUM_MINUS),
        "timer-edit-outline" => Some(icons::TIMER_EDIT_OUTLINE),
        "invert-colors-off" => Some(icons::INVERT_COLORS_OFF),
        "cellphone-off" => Some(icons::CELLPHONE_OFF),
        "book-edit-outline" => Some(icons::BOOK_EDIT_OUTLINE),
        "food-halal" => Some(icons::FOOD_HALAL),
        "magnify-plus-cursor" => Some(icons::MAGNIFY_PLUS_CURSOR),
        "food-takeout-box-outline" => Some(icons::FOOD_TAKEOUT_BOX_OUTLINE),
        "white-balance-incandescent" => Some(icons::WHITE_BALANCE_INCANDESCENT),
        "folder-open" => Some(icons::FOLDER_OPEN),
        "checkbox-marked-circle-plus-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_PLUS_OUTLINE),
        "folder-wrench-outline" => Some(icons::FOLDER_WRENCH_OUTLINE),
        "alpha-z-box" => Some(icons::ALPHA_Z_BOX),
        "picture-in-picture-bottom-right-outline" => {
            Some(icons::PICTURE_IN_PICTURE_BOTTOM_RIGHT_OUTLINE)
        }
        "ballot" => Some(icons::BALLOT),
        "square-rounded-badge" => Some(icons::SQUARE_ROUNDED_BADGE),
        "campfire" => Some(icons::CAMPFIRE),
        "pencil-plus" => Some(icons::PENCIL_PLUS),
        "relation-only-one-to-zero-or-one" => Some(icons::RELATION_ONLY_ONE_TO_ZERO_OR_ONE),
        "file-table-box-multiple-outline" => Some(icons::FILE_TABLE_BOX_MULTIPLE_OUTLINE),
        "television-shimmer" => Some(icons::TELEVISION_SHIMMER),
        "numeric-9-plus-box-outline" => Some(icons::NUMERIC_9_PLUS_BOX_OUTLINE),
        "hydro-power" => Some(icons::HYDRO_POWER),
        "piano-off" => Some(icons::PIANO_OFF),
        "script-text-play-outline" => Some(icons::SCRIPT_TEXT_PLAY_OUTLINE),
        "fast-forward-30" => Some(icons::FAST_FORWARD_30),
        "message-arrow-left-outline" => Some(icons::MESSAGE_ARROW_LEFT_OUTLINE),
        "alpha-r-box-outline" => Some(icons::ALPHA_R_BOX_OUTLINE),
        "ethernet-cable" => Some(icons::ETHERNET_CABLE),
        "arrow-down-bold-outline" => Some(icons::ARROW_DOWN_BOLD_OUTLINE),
        "numeric-2-box" => Some(icons::NUMERIC_2_BOX),
        "bottle-tonic-skull-outline" => Some(icons::BOTTLE_TONIC_SKULL_OUTLINE),
        "scatter-plot" => Some(icons::SCATTER_PLOT),
        "baseball-diamond" => Some(icons::BASEBALL_DIAMOND),
        "home-automation" => Some(icons::HOME_AUTOMATION),
        "layers-search" => Some(icons::LAYERS_SEARCH),
        "sort-ascending" => Some(icons::SORT_ASCENDING),
        "truck-plus-outline" => Some(icons::TRUCK_PLUS_OUTLINE),
        "arrow-down-circle" => Some(icons::ARROW_DOWN_CIRCLE),
        "file-search" => Some(icons::FILE_SEARCH),
        "bank-transfer-out" => Some(icons::BANK_TRANSFER_OUT),
        "head-snowflake-outline" => Some(icons::HEAD_SNOWFLAKE_OUTLINE),
        "email-off" => Some(icons::EMAIL_OFF),
        "arrow-vertical-lock" => Some(icons::ARROW_VERTICAL_LOCK),
        "seatbelt" => Some(icons::SEATBELT),
        "wheelchair" => Some(icons::WHEELCHAIR),
        "anvil" => Some(icons::ANVIL),
        "awning-outline" => Some(icons::AWNING_OUTLINE),
        "window-closed" => Some(icons::WINDOW_CLOSED),
        "message-draw" => Some(icons::MESSAGE_DRAW),
        "plane-car" => Some(icons::PLANE_CAR),
        "comment-question-outline" => Some(icons::COMMENT_QUESTION_OUTLINE),
        "email-multiple-outline" => Some(icons::EMAIL_MULTIPLE_OUTLINE),
        "math-cos" => Some(icons::MATH_COS),
        "image-plus" => Some(icons::IMAGE_PLUS),
        "numeric-5" => Some(icons::NUMERIC_5),
        "message-settings-outline" => Some(icons::MESSAGE_SETTINGS_OUTLINE),
        "database-export-outline" => Some(icons::DATABASE_EXPORT_OUTLINE),
        "meter-gas" => Some(icons::METER_GAS),
        "swap-vertical-variant" => Some(icons::SWAP_VERTICAL_VARIANT),
        "battery-60-bluetooth" => Some(icons::BATTERY_60_BLUETOOTH),
        "screen-rotation" => Some(icons::SCREEN_ROTATION),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "notification-clear-all" => Some(icons::NOTIFICATION_CLEAR_ALL),
        "music-box-outline" => Some(icons::MUSIC_BOX_OUTLINE),
        "playlist-music" => Some(icons::PLAYLIST_MUSIC),
        "resize" => Some(icons::RESIZE),
        "pen-lock" => Some(icons::PEN_LOCK),
        "octagram-outline" => Some(icons::OCTAGRAM_OUTLINE),
        "cog-clockwise" => Some(icons::COG_CLOCKWISE),
        "leaf-off" => Some(icons::LEAF_OFF),
        "microphone-settings" => Some(icons::MICROPHONE_SETTINGS),
        "kettle-steam-outline" => Some(icons::KETTLE_STEAM_OUTLINE),
        "power-socket-us" => Some(icons::POWER_SOCKET_US),
        "nut" => Some(icons::NUT),
        "wifi-lock-open" => Some(icons::WIFI_LOCK_OPEN),
        "palette" => Some(icons::PALETTE),
        "home-floor-negative-1" => Some(icons::HOME_FLOOR_NEGATIVE_1),
        "bash" => Some(icons::BASH),
        #[allow(deprecated)]
        "stackpath" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stackpath' is deprecated.").print(py);
            }
            Some(icons::STACKPATH)
        }
        "clipboard-file-outline" => Some(icons::CLIPBOARD_FILE_OUTLINE),
        "gamepad-variant" => Some(icons::GAMEPAD_VARIANT),
        "camera-plus-outline" => Some(icons::CAMERA_PLUS_OUTLINE),
        "timer-sand-paused" => Some(icons::TIMER_SAND_PAUSED),
        "motion-pause-outline" => Some(icons::MOTION_PAUSE_OUTLINE),
        "redo" => Some(icons::REDO),
        "hospital-box-outline" => Some(icons::HOSPITAL_BOX_OUTLINE),
        "city-variant-outline" => Some(icons::CITY_VARIANT_OUTLINE),
        "coffee-maker-outline" => Some(icons::COFFEE_MAKER_OUTLINE),
        "weather-sunset-down" => Some(icons::WEATHER_SUNSET_DOWN),
        "sprout" => Some(icons::SPROUT),
        "ski-cross-country" => Some(icons::SKI_CROSS_COUNTRY),
        "fast-forward" => Some(icons::FAST_FORWARD),
        "menu-right-outline" => Some(icons::MENU_RIGHT_OUTLINE),
        "video-4k-box" => Some(icons::VIDEO_4K_BOX),
        "sticker-outline" => Some(icons::STICKER_OUTLINE),
        "book-open-blank-variant-outline" => Some(icons::BOOK_OPEN_BLANK_VARIANT_OUTLINE),
        "server-network-outline" => Some(icons::SERVER_NETWORK_OUTLINE),
        "radiobox-indeterminate-variant" => Some(icons::RADIOBOX_INDETERMINATE_VARIANT),
        "approximately-equal" => Some(icons::APPROXIMATELY_EQUAL),
        "format-horizontal-align-left" => Some(icons::FORMAT_HORIZONTAL_ALIGN_LEFT),
        "information-outline" => Some(icons::INFORMATION_OUTLINE),
        "view-agenda" => Some(icons::VIEW_AGENDA),
        "camera-front-variant" => Some(icons::CAMERA_FRONT_VARIANT),
        "dialpad" => Some(icons::DIALPAD),
        "responsive" => Some(icons::RESPONSIVE),
        "car-electric-outline" => Some(icons::CAR_ELECTRIC_OUTLINE),
        "distribute-vertical-center" => Some(icons::DISTRIBUTE_VERTICAL_CENTER),
        "vector-circle" => Some(icons::VECTOR_CIRCLE),
        "email-minus" => Some(icons::EMAIL_MINUS),
        "printer-pos-wrench-outline" => Some(icons::PRINTER_POS_WRENCH_OUTLINE),
        "album" => Some(icons::ALBUM),
        "apple-keyboard-caps" => Some(icons::APPLE_KEYBOARD_CAPS),
        "flag-variant-plus" => Some(icons::FLAG_VARIANT_PLUS),
        "label" => Some(icons::LABEL),
        "chart-timeline-variant" => Some(icons::CHART_TIMELINE_VARIANT),
        "instrument-triangle" => Some(icons::INSTRUMENT_TRIANGLE),
        "file-chart-check-outline" => Some(icons::FILE_CHART_CHECK_OUTLINE),
        "water-boiler-alert" => Some(icons::WATER_BOILER_ALERT),
        "gavel" => Some(icons::GAVEL),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "pan-bottom-left" => Some(icons::PAN_BOTTOM_LEFT),
        "view-grid-plus-outline" => Some(icons::VIEW_GRID_PLUS_OUTLINE),
        "currency-usd-off" => Some(icons::CURRENCY_USD_OFF),
        "cart-plus" => Some(icons::CART_PLUS),
        "wifi-strength-2-lock" => Some(icons::WIFI_STRENGTH_2_LOCK),
        "playlist-play" => Some(icons::PLAYLIST_PLAY),
        "toothbrush-electric" => Some(icons::TOOTHBRUSH_ELECTRIC),
        "minus-network-outline" => Some(icons::MINUS_NETWORK_OUTLINE),
        "vibrate" => Some(icons::VIBRATE),
        "comment-remove-outline" => Some(icons::COMMENT_REMOVE_OUTLINE),
        _ => None,
    }
}
