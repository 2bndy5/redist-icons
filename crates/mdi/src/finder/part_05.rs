// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_5(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "account-tie-voice" => Some(icons::ACCOUNT_TIE_VOICE),
        "printer-pos-refresh" => Some(icons::PRINTER_POS_REFRESH),
        "phone-in-talk-outline" => Some(icons::PHONE_IN_TALK_OUTLINE),
        "lipstick" => Some(icons::LIPSTICK),
        "invoice-text-remove" => Some(icons::INVOICE_TEXT_REMOVE),
        "alpha-n-circle-outline" => Some(icons::ALPHA_N_CIRCLE_OUTLINE),
        "cloud-minus" => Some(icons::CLOUD_MINUS),
        #[allow(deprecated)]
        "odnoklassniki" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'odnoklassniki' is deprecated.").print(py);
            }
            Some(icons::ODNOKLASSNIKI)
        }
        "rotate-left" => Some(icons::ROTATE_LEFT),
        #[allow(deprecated)]
        "webpack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'webpack' is deprecated.").print(py);
            }
            Some(icons::WEBPACK)
        }
        "heart-search" => Some(icons::HEART_SEARCH),
        "weather-windy" => Some(icons::WEATHER_WINDY),
        "send-lock-outline" => Some(icons::SEND_LOCK_OUTLINE),
        "tab-minus" => Some(icons::TAB_MINUS),
        "sort-clock-ascending" => Some(icons::SORT_CLOCK_ASCENDING),
        "owl" => Some(icons::OWL),
        "wifi-strength-2-alert" => Some(icons::WIFI_STRENGTH_2_ALERT),
        "brush-outline" => Some(icons::BRUSH_OUTLINE),
        "brightness-6" => Some(icons::BRIGHTNESS_6),
        #[allow(deprecated)]
        "unicode" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'unicode' is deprecated.").print(py);
            }
            Some(icons::UNICODE)
        }
        "bus-wrench" => Some(icons::BUS_WRENCH),
        "qrcode-plus" => Some(icons::QRCODE_PLUS),
        "table-account" => Some(icons::TABLE_ACCOUNT),
        "checkbox-intermediate-variant" => Some(icons::CHECKBOX_INTERMEDIATE_VARIANT),
        "store-alert" => Some(icons::STORE_ALERT),
        "virus" => Some(icons::VIRUS),
        "briefcase-upload" => Some(icons::BRIEFCASE_UPLOAD),
        "gate-or" => Some(icons::GATE_OR),
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "code-less-than-or-equal" => Some(icons::CODE_LESS_THAN_OR_EQUAL),
        "arrange-bring-to-front" => Some(icons::ARRANGE_BRING_TO_FRONT),
        "brightness-auto" => Some(icons::BRIGHTNESS_AUTO),
        "movie-open" => Some(icons::MOVIE_OPEN),
        "upload-off-outline" => Some(icons::UPLOAD_OFF_OUTLINE),
        "bash" => Some(icons::BASH),
        "relation-only-one-to-one" => Some(icons::RELATION_ONLY_ONE_TO_ONE),
        "sawtooth-wave" => Some(icons::SAWTOOTH_WAVE),
        "arrow-bottom-right-thin" => Some(icons::ARROW_BOTTOM_RIGHT_THIN),
        "tooltip-check" => Some(icons::TOOLTIP_CHECK),
        "book-multiple" => Some(icons::BOOK_MULTIPLE),
        "gate-nor" => Some(icons::GATE_NOR),
        #[allow(deprecated)]
        "ubuntu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ubuntu' is deprecated.").print(py);
            }
            Some(icons::UBUNTU)
        }
        "roman-numeral-10" => Some(icons::ROMAN_NUMERAL_10),
        "glass-cocktail-off" => Some(icons::GLASS_COCKTAIL_OFF),
        "battery-50" => Some(icons::BATTERY_50),
        "format-letter-spacing" => Some(icons::FORMAT_LETTER_SPACING),
        "car-side" => Some(icons::CAR_SIDE),
        "usb-port" => Some(icons::USB_PORT),
        "image-refresh" => Some(icons::IMAGE_REFRESH),
        "book-edit-outline" => Some(icons::BOOK_EDIT_OUTLINE),
        "weather-snowy-rainy" => Some(icons::WEATHER_SNOWY_RAINY),
        "arrow-collapse-all" => Some(icons::ARROW_COLLAPSE_ALL),
        "bell-cancel" => Some(icons::BELL_CANCEL),
        "cards-playing" => Some(icons::CARDS_PLAYING),
        "access-point-network-off" => Some(icons::ACCESS_POINT_NETWORK_OFF),
        "bow-tie" => Some(icons::BOW_TIE),
        "coolant-temperature" => Some(icons::COOLANT_TEMPERATURE),
        "baby-face-outline" => Some(icons::BABY_FACE_OUTLINE),
        "basket" => Some(icons::BASKET),
        "format-header-4" => Some(icons::FORMAT_HEADER_4),
        "forum-minus" => Some(icons::FORUM_MINUS),
        "application-brackets" => Some(icons::APPLICATION_BRACKETS),
        "tangram" => Some(icons::TANGRAM),
        "timer-cog" => Some(icons::TIMER_COG),
        "football-australian" => Some(icons::FOOTBALL_AUSTRALIAN),
        "alpha-o-box" => Some(icons::ALPHA_O_BOX),
        "eye" => Some(icons::EYE),
        "download-multiple" => Some(icons::DOWNLOAD_MULTIPLE),
        "eye-refresh" => Some(icons::EYE_REFRESH),
        "battery-charging-outline" => Some(icons::BATTERY_CHARGING_OUTLINE),
        "settings-helper" => Some(icons::SETTINGS_HELPER),
        "arrow-decision-auto-outline" => Some(icons::ARROW_DECISION_AUTO_OUTLINE),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "earth-arrow-right" => Some(icons::EARTH_ARROW_RIGHT),
        "file-refresh" => Some(icons::FILE_REFRESH),
        "briefcase-arrow-up-down" => Some(icons::BRIEFCASE_ARROW_UP_DOWN),
        "vector-polyline-minus" => Some(icons::VECTOR_POLYLINE_MINUS),
        "zodiac-sagittarius" => Some(icons::ZODIAC_SAGITTARIUS),
        "power-socket-jp" => Some(icons::POWER_SOCKET_JP),
        "fridge-variant" => Some(icons::FRIDGE_VARIANT),
        "eye-outline" => Some(icons::EYE_OUTLINE),
        "view-module" => Some(icons::VIEW_MODULE),
        "toaster-off" => Some(icons::TOASTER_OFF),
        "comment-alert" => Some(icons::COMMENT_ALERT),
        #[allow(deprecated)]
        "folder-google-drive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'folder-google-drive' is deprecated.")
                    .print(py);
            }
            Some(icons::FOLDER_GOOGLE_DRIVE)
        }
        "plus-outline" => Some(icons::PLUS_OUTLINE),
        "calendar-refresh-outline" => Some(icons::CALENDAR_REFRESH_OUTLINE),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "barley-off" => Some(icons::BARLEY_OFF),
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "blinds-horizontal-closed" => Some(icons::BLINDS_HORIZONTAL_CLOSED),
        "bell-plus-outline" => Some(icons::BELL_PLUS_OUTLINE),
        "timetable" => Some(icons::TIMETABLE),
        "tally-mark-3" => Some(icons::TALLY_MARK_3),
        "emoticon-cool-outline" => Some(icons::EMOTICON_COOL_OUTLINE),
        "pound" => Some(icons::POUND),
        "washing-machine" => Some(icons::WASHING_MACHINE),
        "tally-mark-4" => Some(icons::TALLY_MARK_4),
        "flask-empty-minus" => Some(icons::FLASK_EMPTY_MINUS),
        "seat-outline" => Some(icons::SEAT_OUTLINE),
        "select-all" => Some(icons::SELECT_ALL),
        "card-account-details-outline" => Some(icons::CARD_ACCOUNT_DETAILS_OUTLINE),
        "calendar-start-outline" => Some(icons::CALENDAR_START_OUTLINE),
        "printer-pos-check" => Some(icons::PRINTER_POS_CHECK),
        "ring" => Some(icons::RING),
        "filter-remove" => Some(icons::FILTER_REMOVE),
        "cog" => Some(icons::COG),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "signal-4g" => Some(icons::SIGNAL_4G),
        "relation-many-to-many" => Some(icons::RELATION_MANY_TO_MANY),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "calendar-star-four-points" => Some(icons::CALENDAR_STAR_FOUR_POINTS),
        "reply-all" => Some(icons::REPLY_ALL),
        "message-text-lock-outline" => Some(icons::MESSAGE_TEXT_LOCK_OUTLINE),
        "folder-question-outline" => Some(icons::FOLDER_QUESTION_OUTLINE),
        "folder-arrow-up" => Some(icons::FOLDER_ARROW_UP),
        "volcano" => Some(icons::VOLCANO),
        "bookmark-plus-outline" => Some(icons::BOOKMARK_PLUS_OUTLINE),
        "table-row-plus-before" => Some(icons::TABLE_ROW_PLUS_BEFORE),
        "eye-check-outline" => Some(icons::EYE_CHECK_OUTLINE),
        "emoticon-lol-outline" => Some(icons::EMOTICON_LOL_OUTLINE),
        "vpn" => Some(icons::VPN),
        "battery-lock" => Some(icons::BATTERY_LOCK),
        "arrow-left-bold-box" => Some(icons::ARROW_LEFT_BOLD_BOX),
        "currency-twd" => Some(icons::CURRENCY_TWD),
        "star-four-points-box" => Some(icons::STAR_FOUR_POINTS_BOX),
        "piston" => Some(icons::PISTON),
        "account-multiple-check" => Some(icons::ACCOUNT_MULTIPLE_CHECK),
        "alpha-l" => Some(icons::ALPHA_L),
        "vacuum" => Some(icons::VACUUM),
        "tag-remove" => Some(icons::TAG_REMOVE),
        "sack" => Some(icons::SACK),
        "connection" => Some(icons::CONNECTION),
        "account-eye" => Some(icons::ACCOUNT_EYE),
        "network-strength-outline" => Some(icons::NETWORK_STRENGTH_OUTLINE),
        "alert-remove" => Some(icons::ALERT_REMOVE),
        "tablet-dashboard" => Some(icons::TABLET_DASHBOARD),
        "perspective-less" => Some(icons::PERSPECTIVE_LESS),
        "ski" => Some(icons::SKI),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "home-map-marker" => Some(icons::HOME_MAP_MARKER),
        "signal-off" => Some(icons::SIGNAL_OFF),
        "home-battery" => Some(icons::HOME_BATTERY),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "circle" => Some(icons::CIRCLE),
        "ethernet-cable-off" => Some(icons::ETHERNET_CABLE_OFF),
        "size-xs" => Some(icons::SIZE_XS),
        "thermometer-alert" => Some(icons::THERMOMETER_ALERT),
        "chat-remove-outline" => Some(icons::CHAT_REMOVE_OUTLINE),
        "kettlebell" => Some(icons::KETTLEBELL),
        "clipboard-outline" => Some(icons::CLIPBOARD_OUTLINE),
        "numeric-3" => Some(icons::NUMERIC_3),
        "zodiac-scorpio" => Some(icons::ZODIAC_SCORPIO),
        "phone-hangup" => Some(icons::PHONE_HANGUP),
        "math-norm-box" => Some(icons::MATH_NORM_BOX),
        "pasta" => Some(icons::PASTA),
        "toslink" => Some(icons::TOSLINK),
        "alarm" => Some(icons::ALARM),
        "food-takeout-box" => Some(icons::FOOD_TAKEOUT_BOX),
        "home-plus" => Some(icons::HOME_PLUS),
        "store-24-hour" => Some(icons::STORE_24_HOUR),
        "zip-box" => Some(icons::ZIP_BOX),
        "hand-cycle" => Some(icons::HAND_CYCLE),
        "account-question" => Some(icons::ACCOUNT_QUESTION),
        "lightbulb-on-10" => Some(icons::LIGHTBULB_ON_10),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "seat-individual-suite" => Some(icons::SEAT_INDIVIDUAL_SUITE),
        "fridge-alert-outline" => Some(icons::FRIDGE_ALERT_OUTLINE),
        "radio-fm" => Some(icons::RADIO_FM),
        "currency-fra" => Some(icons::CURRENCY_FRA),
        "cards-playing-heart-outline" => Some(icons::CARDS_PLAYING_HEART_OUTLINE),
        "loading" => Some(icons::LOADING),
        "code-json" => Some(icons::CODE_JSON),
        "head-plus" => Some(icons::HEAD_PLUS),
        "vanish-quarter" => Some(icons::VANISH_QUARTER),
        "sort-alphabetical-ascending" => Some(icons::SORT_ALPHABETICAL_ASCENDING),
        "selection-ellipse-arrow-inside" => Some(icons::SELECTION_ELLIPSE_ARROW_INSIDE),
        "clipboard-alert-outline" => Some(icons::CLIPBOARD_ALERT_OUTLINE),
        "land-fields" => Some(icons::LAND_FIELDS),
        "vector-polygon-variant" => Some(icons::VECTOR_POLYGON_VARIANT),
        "note-check-outline" => Some(icons::NOTE_CHECK_OUTLINE),
        "selection-ellipse-remove" => Some(icons::SELECTION_ELLIPSE_REMOVE),
        "briefcase-download" => Some(icons::BRIEFCASE_DOWNLOAD),
        "kitesurfing" => Some(icons::KITESURFING),
        "wifi-strength-4-alert" => Some(icons::WIFI_STRENGTH_4_ALERT),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "phone-incoming-outgoing-outline" => Some(icons::PHONE_INCOMING_OUTGOING_OUTLINE),
        "briefcase-download-outline" => Some(icons::BRIEFCASE_DOWNLOAD_OUTLINE),
        "paperclip-remove" => Some(icons::PAPERCLIP_REMOVE),
        "devices" => Some(icons::DEVICES),
        "alert-decagram-outline" => Some(icons::ALERT_DECAGRAM_OUTLINE),
        "alpha-r-circle-outline" => Some(icons::ALPHA_R_CIRCLE_OUTLINE),
        "invoice-text-arrow-left" => Some(icons::INVOICE_TEXT_ARROW_LEFT),
        "waveform" => Some(icons::WAVEFORM),
        "ev-plug-tesla" => Some(icons::EV_PLUG_TESLA),
        "file-document" => Some(icons::FILE_DOCUMENT),
        "thermometer-off" => Some(icons::THERMOMETER_OFF),
        "apple-keyboard-command" => Some(icons::APPLE_KEYBOARD_COMMAND),
        _ => None,
    }
}
