// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_12(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "format-font-size-decrease" => Some(icons::FORMAT_FONT_SIZE_DECREASE),
        "format-quote-open" => Some(icons::FORMAT_QUOTE_OPEN),
        "arrow-down-right" => Some(icons::ARROW_DOWN_RIGHT),
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "file-lock-open-outline" => Some(icons::FILE_LOCK_OPEN_OUTLINE),
        #[allow(deprecated)]
        "redhat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'redhat' is deprecated.").print(py);
            }
            Some(icons::REDHAT)
        }
        "battery-charging-20" => Some(icons::BATTERY_CHARGING_20),
        "timer-sand-paused" => Some(icons::TIMER_SAND_PAUSED),
        "timer-alert" => Some(icons::TIMER_ALERT),
        "meditation" => Some(icons::MEDITATION),
        "account-lock-open-outline" => Some(icons::ACCOUNT_LOCK_OPEN_OUTLINE),
        "terrain" => Some(icons::TERRAIN),
        "tortoise" => Some(icons::TORTOISE),
        "progress-star" => Some(icons::PROGRESS_STAR),
        "account-details" => Some(icons::ACCOUNT_DETAILS),
        "dice-5" => Some(icons::DICE_5),
        "folder-key-outline" => Some(icons::FOLDER_KEY_OUTLINE),
        "clipboard-check-multiple-outline" => Some(icons::CLIPBOARD_CHECK_MULTIPLE_OUTLINE),
        "mushroom-off" => Some(icons::MUSHROOM_OFF),
        "music-note-whole" => Some(icons::MUSIC_NOTE_WHOLE),
        "office-building-outline" => Some(icons::OFFICE_BUILDING_OUTLINE),
        "text-box" => Some(icons::TEXT_BOX),
        "database-arrow-up" => Some(icons::DATABASE_ARROW_UP),
        "human-male-girl" => Some(icons::HUMAN_MALE_GIRL),
        "focus-field-vertical" => Some(icons::FOCUS_FIELD_VERTICAL),
        "phone-rotate-landscape" => Some(icons::PHONE_ROTATE_LANDSCAPE),
        "feature-search" => Some(icons::FEATURE_SEARCH),
        "size-xxxl" => Some(icons::SIZE_XXXL),
        "file-image-plus-outline" => Some(icons::FILE_IMAGE_PLUS_OUTLINE),
        "recycle" => Some(icons::RECYCLE),
        "help-box-multiple-outline" => Some(icons::HELP_BOX_MULTIPLE_OUTLINE),
        "microsoft-xbox-controller-battery-unknown" => {
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_UNKNOWN)
        }
        "diving-flippers" => Some(icons::DIVING_FLIPPERS),
        "fire-alert" => Some(icons::FIRE_ALERT),
        "distribute-vertical-top" => Some(icons::DISTRIBUTE_VERTICAL_TOP),
        "bunk-bed-outline" => Some(icons::BUNK_BED_OUTLINE),
        "link-box-variant" => Some(icons::LINK_BOX_VARIANT),
        "pen-lock" => Some(icons::PEN_LOCK),
        "alert-box" => Some(icons::ALERT_BOX),
        "moped" => Some(icons::MOPED),
        "tooth" => Some(icons::TOOTH),
        "calendar-sync" => Some(icons::CALENDAR_SYNC),
        "network-outline" => Some(icons::NETWORK_OUTLINE),
        "tooltip-question-outline" => Some(icons::TOOLTIP_QUESTION_OUTLINE),
        "application-parentheses" => Some(icons::APPLICATION_PARENTHESES),
        "timeline-check-outline" => Some(icons::TIMELINE_CHECK_OUTLINE),
        "signal-cellular-1" => Some(icons::SIGNAL_CELLULAR_1),
        "chevron-left-circle" => Some(icons::CHEVRON_LEFT_CIRCLE),
        "monitor-share" => Some(icons::MONITOR_SHARE),
        "asterisk-circle-outline" => Some(icons::ASTERISK_CIRCLE_OUTLINE),
        "archive-outline" => Some(icons::ARCHIVE_OUTLINE),
        "crosshairs" => Some(icons::CROSSHAIRS),
        "bacteria" => Some(icons::BACTERIA),
        "file-powerpoint-box" => Some(icons::FILE_POWERPOINT_BOX),
        "skip-previous" => Some(icons::SKIP_PREVIOUS),
        "hammer-wrench" => Some(icons::HAMMER_WRENCH),
        "ice-cream-off" => Some(icons::ICE_CREAM_OFF),
        "alpha-m" => Some(icons::ALPHA_M),
        "circle-slice-6" => Some(icons::CIRCLE_SLICE_6),
        #[allow(deprecated)]
        "box" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'box' is deprecated.").print(py);
            }
            Some(icons::BOX)
        }
        "texture-box" => Some(icons::TEXTURE_BOX),
        "train-car-autorack" => Some(icons::TRAIN_CAR_AUTORACK),
        "arrange-bring-to-front" => Some(icons::ARRANGE_BRING_TO_FRONT),
        "shield-sun" => Some(icons::SHIELD_SUN),
        "square-circle" => Some(icons::SQUARE_CIRCLE),
        "home-battery-outline" => Some(icons::HOME_BATTERY_OUTLINE),
        "hospital-building" => Some(icons::HOSPITAL_BUILDING),
        "train-car-centerbeam" => Some(icons::TRAIN_CAR_CENTERBEAM),
        "logout-variant" => Some(icons::LOGOUT_VARIANT),
        "set-center" => Some(icons::SET_CENTER),
        "movie-open-edit-outline" => Some(icons::MOVIE_OPEN_EDIT_OUTLINE),
        "water-percent" => Some(icons::WATER_PERCENT),
        "arrow-decision-auto-outline" => Some(icons::ARROW_DECISION_AUTO_OUTLINE),
        "cards-playing-diamond" => Some(icons::CARDS_PLAYING_DIAMOND),
        "receipt-text-minus-outline" => Some(icons::RECEIPT_TEXT_MINUS_OUTLINE),
        "pail-plus" => Some(icons::PAIL_PLUS),
        "cloud-refresh" => Some(icons::CLOUD_REFRESH),
        "database-arrow-right-outline" => Some(icons::DATABASE_ARROW_RIGHT_OUTLINE),
        #[allow(deprecated)]
        "snapchat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'snapchat' is deprecated.").print(py);
            }
            Some(icons::SNAPCHAT)
        }
        "account-tie-voice-outline" => Some(icons::ACCOUNT_TIE_VOICE_OUTLINE),
        #[allow(deprecated)]
        "language-rust" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-rust' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUST)
        }
        "book-plus-multiple" => Some(icons::BOOK_PLUS_MULTIPLE),
        "transmission-tower" => Some(icons::TRANSMISSION_TOWER),
        "phone-bluetooth" => Some(icons::PHONE_BLUETOOTH),
        "car-electric" => Some(icons::CAR_ELECTRIC),
        "rickshaw" => Some(icons::RICKSHAW),
        "cellphone-link-off" => Some(icons::CELLPHONE_LINK_OFF),
        "wifi-strength-2-lock-open" => Some(icons::WIFI_STRENGTH_2_LOCK_OPEN),
        "stomach" => Some(icons::STOMACH),
        "chat-remove" => Some(icons::CHAT_REMOVE),
        "store-alert-outline" => Some(icons::STORE_ALERT_OUTLINE),
        "water-off-outline" => Some(icons::WATER_OFF_OUTLINE),
        "crowd" => Some(icons::CROWD),
        #[allow(deprecated)]
        "linux-mint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux-mint' is deprecated.").print(py);
            }
            Some(icons::LINUX_MINT)
        }
        "network-strength-2-alert" => Some(icons::NETWORK_STRENGTH_2_ALERT),
        "tooltip-text" => Some(icons::TOOLTIP_TEXT),
        "storefront-edit" => Some(icons::STOREFRONT_EDIT),
        "circle-slice-7" => Some(icons::CIRCLE_SLICE_7),
        "printer-eye" => Some(icons::PRINTER_EYE),
        "image-frame" => Some(icons::IMAGE_FRAME),
        "cpu-32-bit" => Some(icons::CPU_32_BIT),
        "biohazard" => Some(icons::BIOHAZARD),
        "fan-speed-1" => Some(icons::FAN_SPEED_1),
        "alphabetical" => Some(icons::ALPHABETICAL),
        "delete-clock" => Some(icons::DELETE_CLOCK),
        "nas" => Some(icons::NAS),
        "crop" => Some(icons::CROP),
        "train-car-passenger-door-open" => Some(icons::TRAIN_CAR_PASSENGER_DOOR_OPEN),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "battery-charging-wireless-30" => Some(icons::BATTERY_CHARGING_WIRELESS_30),
        "heat-pump-outline" => Some(icons::HEAT_PUMP_OUTLINE),
        "puzzle-heart-outline" => Some(icons::PUZZLE_HEART_OUTLINE),
        "weather-pouring" => Some(icons::WEATHER_POURING),
        "sprinkler" => Some(icons::SPRINKLER),
        "crown-circle" => Some(icons::CROWN_CIRCLE),
        "pencil" => Some(icons::PENCIL),
        "egg-easter" => Some(icons::EGG_EASTER),
        "launch" => Some(icons::LAUNCH),
        "tow-truck" => Some(icons::TOW_TRUCK),
        "battery-unknown-bluetooth" => Some(icons::BATTERY_UNKNOWN_BLUETOOTH),
        "playlist-edit" => Some(icons::PLAYLIST_EDIT),
        #[allow(deprecated)]
        "bulma" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bulma' is deprecated.").print(py);
            }
            Some(icons::BULMA)
        }
        "checkbox-blank-off" => Some(icons::CHECKBOX_BLANK_OFF),
        "ceiling-light-outline" => Some(icons::CEILING_LIGHT_OUTLINE),
        "relation-one-or-many-to-zero-or-many" => Some(icons::RELATION_ONE_OR_MANY_TO_ZERO_OR_MANY),
        "content-save-check" => Some(icons::CONTENT_SAVE_CHECK),
        "select-arrow-up" => Some(icons::SELECT_ARROW_UP),
        "phone-refresh" => Some(icons::PHONE_REFRESH),
        "lightbulb-cfl-off" => Some(icons::LIGHTBULB_CFL_OFF),
        "bookmark-box" => Some(icons::BOOKMARK_BOX),
        "cart-outline" => Some(icons::CART_OUTLINE),
        "phone-settings" => Some(icons::PHONE_SETTINGS),
        "tooltip-minus-outline" => Some(icons::TOOLTIP_MINUS_OUTLINE),
        "progress-helper" => Some(icons::PROGRESS_HELPER),
        "shield-key" => Some(icons::SHIELD_KEY),
        "scale-bathroom" => Some(icons::SCALE_BATHROOM),
        "curtains-closed" => Some(icons::CURTAINS_CLOSED),
        "cookie-cog" => Some(icons::COOKIE_COG),
        "shape-oval-plus" => Some(icons::SHAPE_OVAL_PLUS),
        "lightbulb-fluorescent-tube-outline" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE_OUTLINE),
        "window-close" => Some(icons::WINDOW_CLOSE),
        "emoticon-neutral-outline" => Some(icons::EMOTICON_NEUTRAL_OUTLINE),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
        "keyboard-close-outline" => Some(icons::KEYBOARD_CLOSE_OUTLINE),
        "battery-70-bluetooth" => Some(icons::BATTERY_70_BLUETOOTH),
        "music-note-minus" => Some(icons::MUSIC_NOTE_MINUS),
        "relation-zero-or-many-to-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE),
        "pencil-minus" => Some(icons::PENCIL_MINUS),
        "basket-off" => Some(icons::BASKET_OFF),
        "magnify-plus" => Some(icons::MAGNIFY_PLUS),
        "gamepad-square" => Some(icons::GAMEPAD_SQUARE),
        "television-off" => Some(icons::TELEVISION_OFF),
        "numeric-0-box-multiple" => Some(icons::NUMERIC_0_BOX_MULTIPLE),
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "clock-time-ten-outline" => Some(icons::CLOCK_TIME_TEN_OUTLINE),
        "calendar-edit" => Some(icons::CALENDAR_EDIT),
        "size-xs" => Some(icons::SIZE_XS),
        "microphone-question-outline" => Some(icons::MICROPHONE_QUESTION_OUTLINE),
        "spray" => Some(icons::SPRAY),
        "keyboard-return" => Some(icons::KEYBOARD_RETURN),
        "countertop-outline" => Some(icons::COUNTERTOP_OUTLINE),
        "home-alert-outline" => Some(icons::HOME_ALERT_OUTLINE),
        "chemical-weapon" => Some(icons::CHEMICAL_WEAPON),
        "decagram-outline" => Some(icons::DECAGRAM_OUTLINE),
        "lan" => Some(icons::LAN),
        "desk-lamp" => Some(icons::DESK_LAMP),
        "basket-outline" => Some(icons::BASKET_OUTLINE),
        "email-search" => Some(icons::EMAIL_SEARCH),
        "backup-restore" => Some(icons::BACKUP_RESTORE),
        "trumpet" => Some(icons::TRUMPET),
        #[allow(deprecated)]
        "ansible" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ansible' is deprecated.").print(py);
            }
            Some(icons::ANSIBLE)
        }
        "paperclip-plus" => Some(icons::PAPERCLIP_PLUS),
        "door-open" => Some(icons::DOOR_OPEN),
        #[allow(deprecated)]
        "google-maps" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-maps' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_MAPS)
        }
        "castle" => Some(icons::CASTLE),
        "arrow-up-right-bold" => Some(icons::ARROW_UP_RIGHT_BOLD),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "car-convertible" => Some(icons::CAR_CONVERTIBLE),
        "smoking-pipe" => Some(icons::SMOKING_PIPE),
        "induction" => Some(icons::INDUCTION),
        "note-check-outline" => Some(icons::NOTE_CHECK_OUTLINE),
        "file-edit" => Some(icons::FILE_EDIT),
        "projector-screen-variant-off" => Some(icons::PROJECTOR_SCREEN_VARIANT_OFF),
        "movie-check" => Some(icons::MOVIE_CHECK),
        "cellphone-lock" => Some(icons::CELLPHONE_LOCK),
        "note-minus-outline" => Some(icons::NOTE_MINUS_OUTLINE),
        "chevron-double-up" => Some(icons::CHEVRON_DOUBLE_UP),
        "cloud-braces" => Some(icons::CLOUD_BRACES),
        "traffic-light" => Some(icons::TRAFFIC_LIGHT),
        #[allow(deprecated)]
        "youtube-subscription" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-subscription' is deprecated.")
                    .print(py);
            }
            Some(icons::YOUTUBE_SUBSCRIPTION)
        }
        "star-cog-outline" => Some(icons::STAR_COG_OUTLINE),
        "shape-polygon-plus" => Some(icons::SHAPE_POLYGON_PLUS),
        "message-plus-outline" => Some(icons::MESSAGE_PLUS_OUTLINE),
        #[allow(deprecated)]
        "language-html5" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-html5' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_HTML5)
        }
        "mouse-scroll-wheel" => Some(icons::MOUSE_SCROLL_WHEEL),
        "yoga" => Some(icons::YOGA),
        "movie-open-settings" => Some(icons::MOVIE_OPEN_SETTINGS),
        "folder-arrow-right" => Some(icons::FOLDER_ARROW_RIGHT),
        "gauge-low" => Some(icons::GAUGE_LOW),
        "invoice-edit-outline" => Some(icons::INVOICE_EDIT_OUTLINE),
        _ => None,
    }
}
