// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_0(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "vpn" => Some(icons::VPN),
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "floor-lamp" => Some(icons::FLOOR_LAMP),
        "leek" => Some(icons::LEEK),
        "drag-variant" => Some(icons::DRAG_VARIANT),
        "face-man-shimmer" => Some(icons::FACE_MAN_SHIMMER),
        "light-switch" => Some(icons::LIGHT_SWITCH),
        "battery-10" => Some(icons::BATTERY_10),
        "movie-edit-outline" => Some(icons::MOVIE_EDIT_OUTLINE),
        "key-alert-outline" => Some(icons::KEY_ALERT_OUTLINE),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "bus-articulated-end" => Some(icons::BUS_ARTICULATED_END),
        "fridge-variant-off-outline" => Some(icons::FRIDGE_VARIANT_OFF_OUTLINE),
        "ferry" => Some(icons::FERRY),
        "shaker-outline" => Some(icons::SHAKER_OUTLINE),
        "currency-gbp" => Some(icons::CURRENCY_GBP),
        "briefcase-arrow-left-right-outline" => Some(icons::BRIEFCASE_ARROW_LEFT_RIGHT_OUTLINE),
        "transit-transfer" => Some(icons::TRANSIT_TRANSFER),
        "dna" => Some(icons::DNA),
        "mailbox-outline" => Some(icons::MAILBOX_OUTLINE),
        "mouse-left-click-outline" => Some(icons::MOUSE_LEFT_CLICK_OUTLINE),
        "signal-distance-variant" => Some(icons::SIGNAL_DISTANCE_VARIANT),
        "pencil-remove-outline" => Some(icons::PENCIL_REMOVE_OUTLINE),
        "clipboard-text" => Some(icons::CLIPBOARD_TEXT),
        "arrow-u-down-left" => Some(icons::ARROW_U_DOWN_LEFT),
        "desktop-tower" => Some(icons::DESKTOP_TOWER),
        "battery-negative" => Some(icons::BATTERY_NEGATIVE),
        "map-marker-alert" => Some(icons::MAP_MARKER_ALERT),
        "video-wireless" => Some(icons::VIDEO_WIRELESS),
        "account-multiple-remove" => Some(icons::ACCOUNT_MULTIPLE_REMOVE),
        "lock-percent-open-variant-outline" => Some(icons::LOCK_PERCENT_OPEN_VARIANT_OUTLINE),
        "toaster-off" => Some(icons::TOASTER_OFF),
        "wifi-strength-1-lock" => Some(icons::WIFI_STRENGTH_1_LOCK),
        "hamburger-minus" => Some(icons::HAMBURGER_MINUS),
        "book-open-variant" => Some(icons::BOOK_OPEN_VARIANT),
        "less-than-or-equal" => Some(icons::LESS_THAN_OR_EQUAL),
        "view-carousel" => Some(icons::VIEW_CAROUSEL),
        "axis-arrow-lock" => Some(icons::AXIS_ARROW_LOCK),
        "earth-off" => Some(icons::EARTH_OFF),
        "sync-circle" => Some(icons::SYNC_CIRCLE),
        "television-ambient-light" => Some(icons::TELEVISION_AMBIENT_LIGHT),
        "eye" => Some(icons::EYE),
        "file-rotate-right-outline" => Some(icons::FILE_ROTATE_RIGHT_OUTLINE),
        "gate-and" => Some(icons::GATE_AND),
        "account-remove-outline" => Some(icons::ACCOUNT_REMOVE_OUTLINE),
        "heart-multiple-outline" => Some(icons::HEART_MULTIPLE_OUTLINE),
        "database-marker" => Some(icons::DATABASE_MARKER),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "emoticon-wink-outline" => Some(icons::EMOTICON_WINK_OUTLINE),
        "monitor-small" => Some(icons::MONITOR_SMALL),
        "smoke-detector-off" => Some(icons::SMOKE_DETECTOR_OFF),
        "sort-alphabetical-descending" => Some(icons::SORT_ALPHABETICAL_DESCENDING),
        "email-multiple" => Some(icons::EMAIL_MULTIPLE),
        "database-import-outline" => Some(icons::DATABASE_IMPORT_OUTLINE),
        "content-save-all-outline" => Some(icons::CONTENT_SAVE_ALL_OUTLINE),
        #[allow(deprecated)]
        "dolby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dolby' is deprecated.").print(py);
            }
            Some(icons::DOLBY)
        }
        "bottle-soda-classic" => Some(icons::BOTTLE_SODA_CLASSIC),
        "folder-move-outline" => Some(icons::FOLDER_MOVE_OUTLINE),
        "filter-minus-outline" => Some(icons::FILTER_MINUS_OUTLINE),
        "star-four-points-box" => Some(icons::STAR_FOUR_POINTS_BOX),
        "rotate-left" => Some(icons::ROTATE_LEFT),
        "music-accidental-double-flat" => Some(icons::MUSIC_ACCIDENTAL_DOUBLE_FLAT),
        "briefcase-outline" => Some(icons::BRIEFCASE_OUTLINE),
        "lock-reset" => Some(icons::LOCK_RESET),
        "battery-charging-80" => Some(icons::BATTERY_CHARGING_80),
        "motion-pause" => Some(icons::MOTION_PAUSE),
        "fan-remove" => Some(icons::FAN_REMOVE),
        "calendar-remove-outline" => Some(icons::CALENDAR_REMOVE_OUTLINE),
        "filmstrip-off" => Some(icons::FILMSTRIP_OFF),
        "dice-multiple" => Some(icons::DICE_MULTIPLE),
        "food-off" => Some(icons::FOOD_OFF),
        "bus-clock" => Some(icons::BUS_CLOCK),
        "wifi-arrow-left" => Some(icons::WIFI_ARROW_LEFT),
        "credit-card-multiple" => Some(icons::CREDIT_CARD_MULTIPLE),
        "set-all" => Some(icons::SET_ALL),
        "badge-account" => Some(icons::BADGE_ACCOUNT),
        "arrow-down-bold-outline" => Some(icons::ARROW_DOWN_BOLD_OUTLINE),
        "link-variant" => Some(icons::LINK_VARIANT),
        #[allow(deprecated)]
        "google-earth" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-earth' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_EARTH)
        }
        #[allow(deprecated)]
        "dropbox" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dropbox' is deprecated.").print(py);
            }
            Some(icons::DROPBOX)
        }
        "email-arrow-right" => Some(icons::EMAIL_ARROW_RIGHT),
        "signature-text" => Some(icons::SIGNATURE_TEXT),
        #[allow(deprecated)]
        "nintendo-wiiu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nintendo-wiiu' is deprecated.").print(py);
            }
            Some(icons::NINTENDO_WIIU)
        }
        "home-minus" => Some(icons::HOME_MINUS),
        "heart-settings" => Some(icons::HEART_SETTINGS),
        "storefront-outline" => Some(icons::STOREFRONT_OUTLINE),
        "hand-coin-outline" => Some(icons::HAND_COIN_OUTLINE),
        "circle-edit-outline" => Some(icons::CIRCLE_EDIT_OUTLINE),
        "spider-web" => Some(icons::SPIDER_WEB),
        "minus-box-multiple-outline" => Some(icons::MINUS_BOX_MULTIPLE_OUTLINE),
        "image-filter-black-white" => Some(icons::IMAGE_FILTER_BLACK_WHITE),
        "billboard" => Some(icons::BILLBOARD),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        "car-brake-alert" => Some(icons::CAR_BRAKE_ALERT),
        "pan-bottom-left" => Some(icons::PAN_BOTTOM_LEFT),
        "emoticon-devil" => Some(icons::EMOTICON_DEVIL),
        "cloud-clock" => Some(icons::CLOUD_CLOCK),
        "apple-keyboard-command" => Some(icons::APPLE_KEYBOARD_COMMAND),
        "contacts-outline" => Some(icons::CONTACTS_OUTLINE),
        "stool-outline" => Some(icons::STOOL_OUTLINE),
        "battery-sync" => Some(icons::BATTERY_SYNC),
        "camera-rear" => Some(icons::CAMERA_REAR),
        "eyedropper" => Some(icons::EYEDROPPER),
        "rewind-60" => Some(icons::REWIND_60),
        "comment-remove" => Some(icons::COMMENT_REMOVE),
        "guitar-electric" => Some(icons::GUITAR_ELECTRIC),
        "bank-circle" => Some(icons::BANK_CIRCLE),
        "alpha-x" => Some(icons::ALPHA_X),
        "alpha-y-circle" => Some(icons::ALPHA_Y_CIRCLE),
        #[allow(deprecated)]
        "react" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'react' is deprecated.").print(py);
            }
            Some(icons::REACT)
        }
        "window-shutter-auto" => Some(icons::WINDOW_SHUTTER_AUTO),
        #[allow(deprecated)]
        "teamviewer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'teamviewer' is deprecated.").print(py);
            }
            Some(icons::TEAMVIEWER)
        }
        "message-text-lock-outline" => Some(icons::MESSAGE_TEXT_LOCK_OUTLINE),
        "receipt-text-minus" => Some(icons::RECEIPT_TEXT_MINUS),
        "message-arrow-left-outline" => Some(icons::MESSAGE_ARROW_LEFT_OUTLINE),
        "book-sync-outline" => Some(icons::BOOK_SYNC_OUTLINE),
        "swim" => Some(icons::SWIM),
        "cloud-cancel" => Some(icons::CLOUD_CANCEL),
        "cloud-lock-open" => Some(icons::CLOUD_LOCK_OPEN),
        "cupcake" => Some(icons::CUPCAKE),
        "delete-empty-outline" => Some(icons::DELETE_EMPTY_OUTLINE),
        "cookie-off-outline" => Some(icons::COOKIE_OFF_OUTLINE),
        "currency-rub" => Some(icons::CURRENCY_RUB),
        "cart-arrow-down" => Some(icons::CART_ARROW_DOWN),
        "content-save-move-outline" => Some(icons::CONTENT_SAVE_MOVE_OUTLINE),
        "mailbox-open-outline" => Some(icons::MAILBOX_OPEN_OUTLINE),
        "thermostat-box" => Some(icons::THERMOSTAT_BOX),
        "hazard-lights" => Some(icons::HAZARD_LIGHTS),
        "text-short" => Some(icons::TEXT_SHORT),
        "book-refresh" => Some(icons::BOOK_REFRESH),
        "clipboard-text-play-outline" => Some(icons::CLIPBOARD_TEXT_PLAY_OUTLINE),
        "pause-circle" => Some(icons::PAUSE_CIRCLE),
        #[allow(deprecated)]
        "microsoft-onenote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onenote' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONENOTE)
        }
        "wifi-arrow-right" => Some(icons::WIFI_ARROW_RIGHT),
        "format-title" => Some(icons::FORMAT_TITLE),
        "page-first" => Some(icons::PAGE_FIRST),
        "axis-x-rotate-counterclockwise" => Some(icons::AXIS_X_ROTATE_COUNTERCLOCKWISE),
        "calendar-account-outline" => Some(icons::CALENDAR_ACCOUNT_OUTLINE),
        "clock-plus" => Some(icons::CLOCK_PLUS),
        "application-braces-outline" => Some(icons::APPLICATION_BRACES_OUTLINE),
        "rocket-launch-outline" => Some(icons::ROCKET_LAUNCH_OUTLINE),
        "mosque" => Some(icons::MOSQUE),
        "vector-polyline" => Some(icons::VECTOR_POLYLINE),
        "thermostat-cog" => Some(icons::THERMOSTAT_COG),
        "roman-numeral-10" => Some(icons::ROMAN_NUMERAL_10),
        "email-edit" => Some(icons::EMAIL_EDIT),
        "view-grid" => Some(icons::VIEW_GRID),
        "numeric-8-circle" => Some(icons::NUMERIC_8_CIRCLE),
        "clock-time-five" => Some(icons::CLOCK_TIME_FIVE),
        "timer-cancel-outline" => Some(icons::TIMER_CANCEL_OUTLINE),
        "numeric-6-box" => Some(icons::NUMERIC_6_BOX),
        "layers-search-outline" => Some(icons::LAYERS_SEARCH_OUTLINE),
        "book-variant" => Some(icons::BOOK_VARIANT),
        "temple-buddhist-outline" => Some(icons::TEMPLE_BUDDHIST_OUTLINE),
        "bike-pedal-mountain" => Some(icons::BIKE_PEDAL_MOUNTAIN),
        "download-off" => Some(icons::DOWNLOAD_OFF),
        "send-variant-clock-outline" => Some(icons::SEND_VARIANT_CLOCK_OUTLINE),
        "cloud-refresh-variant-outline" => Some(icons::CLOUD_REFRESH_VARIANT_OUTLINE),
        "account-tie-woman" => Some(icons::ACCOUNT_TIE_WOMAN),
        "alpha-e-box" => Some(icons::ALPHA_E_BOX),
        "vector-point-plus" => Some(icons::VECTOR_POINT_PLUS),
        "swap-vertical-circle" => Some(icons::SWAP_VERTICAL_CIRCLE),
        "book-lock" => Some(icons::BOOK_LOCK),
        "image-off-outline" => Some(icons::IMAGE_OFF_OUTLINE),
        "table-arrow-down" => Some(icons::TABLE_ARROW_DOWN),
        "shopping-search-outline" => Some(icons::SHOPPING_SEARCH_OUTLINE),
        #[allow(deprecated)]
        "language-ruby-on-rails" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-ruby-on-rails' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_RUBY_ON_RAILS)
        }
        "lock-percent-open-variant" => Some(icons::LOCK_PERCENT_OPEN_VARIANT),
        "blender" => Some(icons::BLENDER),
        "battery-unknown" => Some(icons::BATTERY_UNKNOWN),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "pencil-plus" => Some(icons::PENCIL_PLUS),
        "border-all-variant" => Some(icons::BORDER_ALL_VARIANT),
        "fridge-industrial-off" => Some(icons::FRIDGE_INDUSTRIAL_OFF),
        "bell-alert" => Some(icons::BELL_ALERT),
        "server-off" => Some(icons::SERVER_OFF),
        "horizontal-rotate-clockwise" => Some(icons::HORIZONTAL_ROTATE_CLOCKWISE),
        "airplane-marker" => Some(icons::AIRPLANE_MARKER),
        "virus-off" => Some(icons::VIRUS_OFF),
        "home-off" => Some(icons::HOME_OFF),
        "tag-arrow-right-outline" => Some(icons::TAG_ARROW_RIGHT_OUTLINE),
        "radiobox-indeterminate-variant" => Some(icons::RADIOBOX_INDETERMINATE_VARIANT),
        "repeat-once" => Some(icons::REPEAT_ONCE),
        "clippy" => Some(icons::CLIPPY),
        "scent" => Some(icons::SCENT),
        "earth-plus" => Some(icons::EARTH_PLUS),
        "format-line-height" => Some(icons::FORMAT_LINE_HEIGHT),
        "format-vertical-align-bottom" => Some(icons::FORMAT_VERTICAL_ALIGN_BOTTOM),
        "cloud-plus-outline" => Some(icons::CLOUD_PLUS_OUTLINE),
        "timer-sand-complete" => Some(icons::TIMER_SAND_COMPLETE),
        "cog-sync" => Some(icons::COG_SYNC),
        "shield-off" => Some(icons::SHIELD_OFF),
        #[allow(deprecated)]
        "dlna" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dlna' is deprecated.").print(py);
            }
            Some(icons::DLNA)
        }
        "eyedropper-minus" => Some(icons::EYEDROPPER_MINUS),
        "seed-off" => Some(icons::SEED_OFF),
        "french-fries" => Some(icons::FRENCH_FRIES),
        "timer-sand-empty" => Some(icons::TIMER_SAND_EMPTY),
        "image-marker-outline" => Some(icons::IMAGE_MARKER_OUTLINE),
        "segment" => Some(icons::SEGMENT),
        _ => None,
    }
}
