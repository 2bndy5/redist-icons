// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "book-cog" => Some(icons::BOOK_COG),
        "counter" => Some(icons::COUNTER),
        "smart-card" => Some(icons::SMART_CARD),
        "database-cog-outline" => Some(icons::DATABASE_COG_OUTLINE),
        "skip-previous-outline" => Some(icons::SKIP_PREVIOUS_OUTLINE),
        "baseball-outline" => Some(icons::BASEBALL_OUTLINE),
        "arrow-left-box" => Some(icons::ARROW_LEFT_BOX),
        "paperclip-minus" => Some(icons::PAPERCLIP_MINUS),
        "timeline-alert" => Some(icons::TIMELINE_ALERT),
        "lightbulb-on-80" => Some(icons::LIGHTBULB_ON_80),
        "paw-off-outline" => Some(icons::PAW_OFF_OUTLINE),
        "console-line" => Some(icons::CONSOLE_LINE),
        "power-socket-au" => Some(icons::POWER_SOCKET_AU),
        "file-phone-outline" => Some(icons::FILE_PHONE_OUTLINE),
        "timer-remove" => Some(icons::TIMER_REMOVE),
        "microsoft-xbox-controller-battery-unknown" => {
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_UNKNOWN)
        }
        "format-wrap-tight" => Some(icons::FORMAT_WRAP_TIGHT),
        "battery-charging-wireless-60" => Some(icons::BATTERY_CHARGING_WIRELESS_60),
        "cards-heart-outline" => Some(icons::CARDS_HEART_OUTLINE),
        "inbox-full-outline" => Some(icons::INBOX_FULL_OUTLINE),
        "weather-moonset" => Some(icons::WEATHER_MOONSET),
        "arrow-u-up-right" => Some(icons::ARROW_U_UP_RIGHT),
        "glass-flute" => Some(icons::GLASS_FLUTE),
        "file-restore-outline" => Some(icons::FILE_RESTORE_OUTLINE),
        "align-vertical-top" => Some(icons::ALIGN_VERTICAL_TOP),
        "file-document-minus-outline" => Some(icons::FILE_DOCUMENT_MINUS_OUTLINE),
        "rickshaw" => Some(icons::RICKSHAW),
        #[allow(deprecated)]
        "google-street-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-street-view' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_STREET_VIEW)
        }
        "minus-circle-multiple-outline" => Some(icons::MINUS_CIRCLE_MULTIPLE_OUTLINE),
        "seat-legroom-extra" => Some(icons::SEAT_LEGROOM_EXTRA),
        "new-box" => Some(icons::NEW_BOX),
        "sofa-outline" => Some(icons::SOFA_OUTLINE),
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "alphabet-piqad" => Some(icons::ALPHABET_PIQAD),
        "arrow-all" => Some(icons::ARROW_ALL),
        "close-box-outline" => Some(icons::CLOSE_BOX_OUTLINE),
        "credit-card-minus" => Some(icons::CREDIT_CARD_MINUS),
        "tag-arrow-right-outline" => Some(icons::TAG_ARROW_RIGHT_OUTLINE),
        "flask-off-outline" => Some(icons::FLASK_OFF_OUTLINE),
        "basket-fill" => Some(icons::BASKET_FILL),
        "arrow-top-left" => Some(icons::ARROW_TOP_LEFT),
        "flag-variant-minus-outline" => Some(icons::FLAG_VARIANT_MINUS_OUTLINE),
        "clock-check" => Some(icons::CLOCK_CHECK),
        "clock-in" => Some(icons::CLOCK_IN),
        "credit-card-edit-outline" => Some(icons::CREDIT_CARD_EDIT_OUTLINE),
        "file-arrow-up-down" => Some(icons::FILE_ARROW_UP_DOWN),
        "page-previous-outline" => Some(icons::PAGE_PREVIOUS_OUTLINE),
        "alpha-o-box" => Some(icons::ALPHA_O_BOX),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "currency-rupee" => Some(icons::CURRENCY_RUPEE),
        "fridge" => Some(icons::FRIDGE),
        "package" => Some(icons::PACKAGE),
        "storefront-edit-outline" => Some(icons::STOREFRONT_EDIT_OUTLINE),
        "grid-off" => Some(icons::GRID_OFF),
        "alpha-p-circle" => Some(icons::ALPHA_P_CIRCLE),
        "bookmark-check-outline" => Some(icons::BOOKMARK_CHECK_OUTLINE),
        "arrow-up" => Some(icons::ARROW_UP),
        #[allow(deprecated)]
        "firefox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'firefox' is deprecated.").print(py);
            }
            Some(icons::FIREFOX)
        }
        "microphone-message-off" => Some(icons::MICROPHONE_MESSAGE_OFF),
        "account-arrow-right-outline" => Some(icons::ACCOUNT_ARROW_RIGHT_OUTLINE),
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        #[allow(deprecated)]
        "trello" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'trello' is deprecated.").print(py);
            }
            Some(icons::TRELLO)
        }
        "account-convert" => Some(icons::ACCOUNT_CONVERT),
        "arrow-left-drop-circle" => Some(icons::ARROW_LEFT_DROP_CIRCLE),
        "view-parallel" => Some(icons::VIEW_PARALLEL),
        "page-layout-body" => Some(icons::PAGE_LAYOUT_BODY),
        "tunnel-outline" => Some(icons::TUNNEL_OUTLINE),
        "head-remove" => Some(icons::HEAD_REMOVE),
        "data-matrix-minus" => Some(icons::DATA_MATRIX_MINUS),
        "arrow-right-bottom" => Some(icons::ARROW_RIGHT_BOTTOM),
        "battery-60-bluetooth" => Some(icons::BATTERY_60_BLUETOOTH),
        "email-minus" => Some(icons::EMAIL_MINUS),
        "format-page-break" => Some(icons::FORMAT_PAGE_BREAK),
        "shield-account-variant" => Some(icons::SHIELD_ACCOUNT_VARIANT),
        "folder-home-outline" => Some(icons::FOLDER_HOME_OUTLINE),
        "table-column-plus-after" => Some(icons::TABLE_COLUMN_PLUS_AFTER),
        "label-percent" => Some(icons::LABEL_PERCENT),
        "arrow-left-bold-hexagon-outline" => Some(icons::ARROW_LEFT_BOLD_HEXAGON_OUTLINE),
        "alpha-n-box-outline" => Some(icons::ALPHA_N_BOX_OUTLINE),
        "bicycle-cargo" => Some(icons::BICYCLE_CARGO),
        "briefcase-download" => Some(icons::BRIEFCASE_DOWNLOAD),
        "human-male-height" => Some(icons::HUMAN_MALE_HEIGHT),
        "earth" => Some(icons::EARTH),
        "flask-round-bottom-empty" => Some(icons::FLASK_ROUND_BOTTOM_EMPTY),
        "account-arrow-left-outline" => Some(icons::ACCOUNT_ARROW_LEFT_OUTLINE),
        "numeric-9-box-outline" => Some(icons::NUMERIC_9_BOX_OUTLINE),
        "door-sliding-lock" => Some(icons::DOOR_SLIDING_LOCK),
        "reply" => Some(icons::REPLY),
        "magnify-minus" => Some(icons::MAGNIFY_MINUS),
        "iron-board" => Some(icons::IRON_BOARD),
        "table-eye-off" => Some(icons::TABLE_EYE_OFF),
        "close-outline" => Some(icons::CLOSE_OUTLINE),
        "tally-mark-3" => Some(icons::TALLY_MARK_3),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        "calendar-account-outline" => Some(icons::CALENDAR_ACCOUNT_OUTLINE),
        "cow" => Some(icons::COW),
        "numeric-4-box-outline" => Some(icons::NUMERIC_4_BOX_OUTLINE),
        "battery-arrow-up-outline" => Some(icons::BATTERY_ARROW_UP_OUTLINE),
        "currency-usd" => Some(icons::CURRENCY_USD),
        "mushroom-outline" => Some(icons::MUSHROOM_OUTLINE),
        "beaker-check-outline" => Some(icons::BEAKER_CHECK_OUTLINE),
        "cross-outline" => Some(icons::CROSS_OUTLINE),
        "texture-box" => Some(icons::TEXTURE_BOX),
        "label-percent-outline" => Some(icons::LABEL_PERCENT_OUTLINE),
        "email-multiple-outline" => Some(icons::EMAIL_MULTIPLE_OUTLINE),
        "numeric-1-circle-outline" => Some(icons::NUMERIC_1_CIRCLE_OUTLINE),
        "brightness-percent" => Some(icons::BRIGHTNESS_PERCENT),
        "message-star-outline" => Some(icons::MESSAGE_STAR_OUTLINE),
        "phone" => Some(icons::PHONE),
        "desk-lamp-off" => Some(icons::DESK_LAMP_OFF),
        "cash-plus" => Some(icons::CASH_PLUS),
        "video-off" => Some(icons::VIDEO_OFF),
        "fire-off" => Some(icons::FIRE_OFF),
        "layers-edit" => Some(icons::LAYERS_EDIT),
        "liquid-spot" => Some(icons::LIQUID_SPOT),
        "skull-crossbones-outline" => Some(icons::SKULL_CROSSBONES_OUTLINE),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "signal-cellular-3" => Some(icons::SIGNAL_CELLULAR_3),
        #[allow(deprecated)]
        "emby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'emby' is deprecated.").print(py);
            }
            Some(icons::EMBY)
        }
        "cart-arrow-up" => Some(icons::CART_ARROW_UP),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "book-cog-outline" => Some(icons::BOOK_COG_OUTLINE),
        "format-header-6" => Some(icons::FORMAT_HEADER_6),
        "book-clock-outline" => Some(icons::BOOK_CLOCK_OUTLINE),
        "waves-arrow-up" => Some(icons::WAVES_ARROW_UP),
        "football" => Some(icons::FOOTBALL),
        "snowflake-alert" => Some(icons::SNOWFLAKE_ALERT),
        "paperclip-check" => Some(icons::PAPERCLIP_CHECK),
        "bike-pedal-mountain" => Some(icons::BIKE_PEDAL_MOUNTAIN),
        "sun-thermometer" => Some(icons::SUN_THERMOMETER),
        "truck-minus" => Some(icons::TRUCK_MINUS),
        "movie-roll" => Some(icons::MOVIE_ROLL),
        "folder-refresh" => Some(icons::FOLDER_REFRESH),
        "earbuds" => Some(icons::EARBUDS),
        "ev-plug-ccs1" => Some(icons::EV_PLUG_CCS1),
        "store-marker" => Some(icons::STORE_MARKER),
        "store-search" => Some(icons::STORE_SEARCH),
        "heart-half-full" => Some(icons::HEART_HALF_FULL),
        "locker" => Some(icons::LOCKER),
        "clock-minus" => Some(icons::CLOCK_MINUS),
        "clock-time-six" => Some(icons::CLOCK_TIME_SIX),
        "folder-alert-outline" => Some(icons::FOLDER_ALERT_OUTLINE),
        "content-save-alert-outline" => Some(icons::CONTENT_SAVE_ALERT_OUTLINE),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        "text-box-plus" => Some(icons::TEXT_BOX_PLUS),
        "food-drumstick-off-outline" => Some(icons::FOOD_DRUMSTICK_OFF_OUTLINE),
        "home-lightning-bolt-outline" => Some(icons::HOME_LIGHTNING_BOLT_OUTLINE),
        "upload-circle-outline" => Some(icons::UPLOAD_CIRCLE_OUTLINE),
        "format-list-group-plus" => Some(icons::FORMAT_LIST_GROUP_PLUS),
        "book-lock-outline" => Some(icons::BOOK_LOCK_OUTLINE),
        "archive-plus-outline" => Some(icons::ARCHIVE_PLUS_OUTLINE),
        "flash-off-outline" => Some(icons::FLASH_OFF_OUTLINE),
        "nut" => Some(icons::NUT),
        "timer-sand" => Some(icons::TIMER_SAND),
        "moon-waning-crescent" => Some(icons::MOON_WANING_CRESCENT),
        "bag-carry-on-off" => Some(icons::BAG_CARRY_ON_OFF),
        "ear-hearing" => Some(icons::EAR_HEARING),
        #[allow(deprecated)]
        "webrtc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webrtc' is deprecated.").print(py);
            }
            Some(icons::WEBRTC)
        }
        "variable" => Some(icons::VARIABLE),
        "microwave" => Some(icons::MICROWAVE),
        "flash-triangle" => Some(icons::FLASH_TRIANGLE),
        "image-album" => Some(icons::IMAGE_ALBUM),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "thermometer-off" => Some(icons::THERMOMETER_OFF),
        "message-text-lock-outline" => Some(icons::MESSAGE_TEXT_LOCK_OUTLINE),
        "emoticon-poop" => Some(icons::EMOTICON_POOP),
        "mother-heart" => Some(icons::MOTHER_HEART),
        "comment-arrow-left-outline" => Some(icons::COMMENT_ARROW_LEFT_OUTLINE),
        "rename-outline" => Some(icons::RENAME_OUTLINE),
        "file-image" => Some(icons::FILE_IMAGE),
        "gesture-tap" => Some(icons::GESTURE_TAP),
        "alpha-s-circle-outline" => Some(icons::ALPHA_S_CIRCLE_OUTLINE),
        "food-variant" => Some(icons::FOOD_VARIANT),
        "microphone-minus" => Some(icons::MICROPHONE_MINUS),
        "grid" => Some(icons::GRID),
        "bed-king" => Some(icons::BED_KING),
        "lightbulb-night-outline" => Some(icons::LIGHTBULB_NIGHT_OUTLINE),
        "upload-off" => Some(icons::UPLOAD_OFF),
        "calendar-expand-horizontal" => Some(icons::CALENDAR_EXPAND_HORIZONTAL),
        "numeric-10" => Some(icons::NUMERIC_10),
        "leak-off" => Some(icons::LEAK_OFF),
        "beaker-plus" => Some(icons::BEAKER_PLUS),
        "clipboard-play-multiple" => Some(icons::CLIPBOARD_PLAY_MULTIPLE),
        "magnify-close" => Some(icons::MAGNIFY_CLOSE),
        "cash-minus" => Some(icons::CASH_MINUS),
        "zip-disk" => Some(icons::ZIP_DISK),
        "playlist-plus" => Some(icons::PLAYLIST_PLUS),
        "egg-fried" => Some(icons::EGG_FRIED),
        "kite" => Some(icons::KITE),
        "dock-left" => Some(icons::DOCK_LEFT),
        "printer-pos-refresh" => Some(icons::PRINTER_POS_REFRESH),
        "account-circle" => Some(icons::ACCOUNT_CIRCLE),
        "alert-minus-outline" => Some(icons::ALERT_MINUS_OUTLINE),
        "airplane-edit" => Some(icons::AIRPLANE_EDIT),
        "human-child" => Some(icons::HUMAN_CHILD),
        "minus-box-outline" => Some(icons::MINUS_BOX_OUTLINE),
        "router-wireless" => Some(icons::ROUTER_WIRELESS),
        "transition" => Some(icons::TRANSITION),
        #[allow(deprecated)]
        "bulma" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bulma' is deprecated.").print(py);
            }
            Some(icons::BULMA)
        }
        "router" => Some(icons::ROUTER),
        _ => None,
    }
}
