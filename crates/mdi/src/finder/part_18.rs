// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_18(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        #[allow(deprecated)]
        "deviantart" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'deviantart' is deprecated.").print(py);
            }
            Some(icons::DEVIANTART)
        }
        "sticker-text-outline" => Some(icons::STICKER_TEXT_OUTLINE),
        "alpha-i-box" => Some(icons::ALPHA_I_BOX),
        "sign-direction-minus" => Some(icons::SIGN_DIRECTION_MINUS),
        "file-document-check" => Some(icons::FILE_DOCUMENT_CHECK),
        "store-search-outline" => Some(icons::STORE_SEARCH_OUTLINE),
        "molecule-co2" => Some(icons::MOLECULE_CO2),
        "file-lock" => Some(icons::FILE_LOCK),
        "heart-circle-outline" => Some(icons::HEART_CIRCLE_OUTLINE),
        "view-dashboard" => Some(icons::VIEW_DASHBOARD),
        "artboard" => Some(icons::ARTBOARD),
        "flash-red-eye" => Some(icons::FLASH_RED_EYE),
        "motion-sensor" => Some(icons::MOTION_SENSOR),
        "car-door-lock-open" => Some(icons::CAR_DOOR_LOCK_OPEN),
        "marker-check" => Some(icons::MARKER_CHECK),
        "close-thick" => Some(icons::CLOSE_THICK),
        "table-row-plus-before" => Some(icons::TABLE_ROW_PLUS_BEFORE),
        "vector-triangle" => Some(icons::VECTOR_TRIANGLE),
        "table-headers-eye-off" => Some(icons::TABLE_HEADERS_EYE_OFF),
        "subway" => Some(icons::SUBWAY),
        "earth-remove" => Some(icons::EARTH_REMOVE),
        "dice-1-outline" => Some(icons::DICE_1_OUTLINE),
        "movie-open-play-outline" => Some(icons::MOVIE_OPEN_PLAY_OUTLINE),
        "traffic-light" => Some(icons::TRAFFIC_LIGHT),
        "alarm-panel" => Some(icons::ALARM_PANEL),
        "account-remove-outline" => Some(icons::ACCOUNT_REMOVE_OUTLINE),
        "popcorn" => Some(icons::POPCORN),
        "numeric-2-circle" => Some(icons::NUMERIC_2_CIRCLE),
        "sale-outline" => Some(icons::SALE_OUTLINE),
        "earth-box-minus" => Some(icons::EARTH_BOX_MINUS),
        "plus-box-multiple-outline" => Some(icons::PLUS_BOX_MULTIPLE_OUTLINE),
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        "smoke-detector-alert-outline" => Some(icons::SMOKE_DETECTOR_ALERT_OUTLINE),
        "gamepad-circle-outline" => Some(icons::GAMEPAD_CIRCLE_OUTLINE),
        "timeline-alert" => Some(icons::TIMELINE_ALERT),
        "car-lifted-pickup" => Some(icons::CAR_LIFTED_PICKUP),
        "square-root" => Some(icons::SQUARE_ROOT),
        "email-sync" => Some(icons::EMAIL_SYNC),
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "format-wrap-square" => Some(icons::FORMAT_WRAP_SQUARE),
        "send-check" => Some(icons::SEND_CHECK),
        "format-pilcrow-arrow-left" => Some(icons::FORMAT_PILCROW_ARROW_LEFT),
        "thermometer-high" => Some(icons::THERMOMETER_HIGH),
        "tag-outline" => Some(icons::TAG_OUTLINE),
        "pipe-leak" => Some(icons::PIPE_LEAK),
        #[allow(deprecated)]
        "manjaro" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'manjaro' is deprecated.").print(py);
            }
            Some(icons::MANJARO)
        }
        "chemical-weapon" => Some(icons::CHEMICAL_WEAPON),
        "calendar-expand-horizontal" => Some(icons::CALENDAR_EXPAND_HORIZONTAL),
        "rhombus-medium" => Some(icons::RHOMBUS_MEDIUM),
        "credit-card-minus" => Some(icons::CREDIT_CARD_MINUS),
        "flask-empty-off" => Some(icons::FLASK_EMPTY_OFF),
        "archive-refresh-outline" => Some(icons::ARCHIVE_REFRESH_OUTLINE),
        "chat-alert-outline" => Some(icons::CHAT_ALERT_OUTLINE),
        "file-video" => Some(icons::FILE_VIDEO),
        "tumble-dryer" => Some(icons::TUMBLE_DRYER),
        "filter-off" => Some(icons::FILTER_OFF),
        "virus-off" => Some(icons::VIRUS_OFF),
        "window-restore" => Some(icons::WINDOW_RESTORE),
        "vector-circle-variant" => Some(icons::VECTOR_CIRCLE_VARIANT),
        "content-save-move-outline" => Some(icons::CONTENT_SAVE_MOVE_OUTLINE),
        "check-underline-circle-outline" => Some(icons::CHECK_UNDERLINE_CIRCLE_OUTLINE),
        "format-list-bulleted-type" => Some(icons::FORMAT_LIST_BULLETED_TYPE),
        "home-map-marker" => Some(icons::HOME_MAP_MARKER),
        "kettle" => Some(icons::KETTLE),
        "list-box" => Some(icons::LIST_BOX),
        "vector-difference-ba" => Some(icons::VECTOR_DIFFERENCE_BA),
        "thermostat-auto" => Some(icons::THERMOSTAT_AUTO),
        "email-remove-outline" => Some(icons::EMAIL_REMOVE_OUTLINE),
        "email-seal" => Some(icons::EMAIL_SEAL),
        "weather-cloudy-alert" => Some(icons::WEATHER_CLOUDY_ALERT),
        "music-note-plus" => Some(icons::MUSIC_NOTE_PLUS),
        "format-list-numbered" => Some(icons::FORMAT_LIST_NUMBERED),
        "step-forward" => Some(icons::STEP_FORWARD),
        "comment-search-outline" => Some(icons::COMMENT_SEARCH_OUTLINE),
        "diamond" => Some(icons::DIAMOND),
        "chart-bar" => Some(icons::CHART_BAR),
        "signal-variant" => Some(icons::SIGNAL_VARIANT),
        "shield-sync-outline" => Some(icons::SHIELD_SYNC_OUTLINE),
        "comment-arrow-left" => Some(icons::COMMENT_ARROW_LEFT),
        "arrow-right-bottom" => Some(icons::ARROW_RIGHT_BOTTOM),
        "tally-mark-4" => Some(icons::TALLY_MARK_4),
        "account-cog-outline" => Some(icons::ACCOUNT_COG_OUTLINE),
        "map-marker-off" => Some(icons::MAP_MARKER_OFF),
        "filter-variant" => Some(icons::FILTER_VARIANT),
        "tag-remove-outline" => Some(icons::TAG_REMOVE_OUTLINE),
        "checkbox-blank-off" => Some(icons::CHECKBOX_BLANK_OFF),
        "cloud-sync" => Some(icons::CLOUD_SYNC),
        #[allow(deprecated)]
        "nuxt" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nuxt' is deprecated.").print(py);
            }
            Some(icons::NUXT)
        }
        "server-remove" => Some(icons::SERVER_REMOVE),
        #[allow(deprecated)]
        "vuejs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuejs' is deprecated.").print(py);
            }
            Some(icons::VUEJS)
        }
        "alpha-n-box" => Some(icons::ALPHA_N_BOX),
        "invoice-text-send-outline" => Some(icons::INVOICE_TEXT_SEND_OUTLINE),
        "account-tag-outline" => Some(icons::ACCOUNT_TAG_OUTLINE),
        "chart-box" => Some(icons::CHART_BOX),
        "download" => Some(icons::DOWNLOAD),
        "zodiac-pisces" => Some(icons::ZODIAC_PISCES),
        "heart-off" => Some(icons::HEART_OFF),
        "arrow-down-bold-circle-outline" => Some(icons::ARROW_DOWN_BOLD_CIRCLE_OUTLINE),
        "replay" => Some(icons::REPLAY),
        "folder-account" => Some(icons::FOLDER_ACCOUNT),
        #[allow(deprecated)]
        "apple-ios" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-ios' is deprecated.").print(py);
            }
            Some(icons::APPLE_IOS)
        }
        "relation-only-one-to-one" => Some(icons::RELATION_ONLY_ONE_TO_ONE),
        "car-2-plus" => Some(icons::CAR_2_PLUS),
        "account-arrow-right-outline" => Some(icons::ACCOUNT_ARROW_RIGHT_OUTLINE),
        "car-defrost-front" => Some(icons::CAR_DEFROST_FRONT),
        "file-excel-box-outline" => Some(icons::FILE_EXCEL_BOX_OUTLINE),
        "table-arrow-left" => Some(icons::TABLE_ARROW_LEFT),
        "kabaddi" => Some(icons::KABADDI),
        "sprout" => Some(icons::SPROUT),
        "alpha-e" => Some(icons::ALPHA_E),
        "account-star-outline" => Some(icons::ACCOUNT_STAR_OUTLINE),
        "inbox-full-outline" => Some(icons::INBOX_FULL_OUTLINE),
        "fountain" => Some(icons::FOUNTAIN),
        "bug-check-outline" => Some(icons::BUG_CHECK_OUTLINE),
        "nut" => Some(icons::NUT),
        "sticker-remove" => Some(icons::STICKER_REMOVE),
        "message-question" => Some(icons::MESSAGE_QUESTION),
        "wallet" => Some(icons::WALLET),
        "math-compass" => Some(icons::MATH_COMPASS),
        "speaker-message" => Some(icons::SPEAKER_MESSAGE),
        "counter" => Some(icons::COUNTER),
        "wifi-strength-3-alert" => Some(icons::WIFI_STRENGTH_3_ALERT),
        "account-injury-outline" => Some(icons::ACCOUNT_INJURY_OUTLINE),
        "ticket-percent-outline" => Some(icons::TICKET_PERCENT_OUTLINE),
        "car-esp" => Some(icons::CAR_ESP),
        "garage-lock" => Some(icons::GARAGE_LOCK),
        "paperclip-remove" => Some(icons::PAPERCLIP_REMOVE),
        "train-car-passenger-door-open" => Some(icons::TRAIN_CAR_PASSENGER_DOOR_OPEN),
        "view-sequential" => Some(icons::VIEW_SEQUENTIAL),
        "movie-open-off-outline" => Some(icons::MOVIE_OPEN_OFF_OUTLINE),
        "passport-cancel" => Some(icons::PASSPORT_CANCEL),
        "lightbulb" => Some(icons::LIGHTBULB),
        "alert-box" => Some(icons::ALERT_BOX),
        "button-pointer" => Some(icons::BUTTON_POINTER),
        "image-size-select-small" => Some(icons::IMAGE_SIZE_SELECT_SMALL),
        "face-woman-outline" => Some(icons::FACE_WOMAN_OUTLINE),
        "food-turkey" => Some(icons::FOOD_TURKEY),
        "swap-horizontal-circle-outline" => Some(icons::SWAP_HORIZONTAL_CIRCLE_OUTLINE),
        "sort-alphabetical-variant" => Some(icons::SORT_ALPHABETICAL_VARIANT),
        "brightness-5" => Some(icons::BRIGHTNESS_5),
        "cross" => Some(icons::CROSS),
        "monitor-edit" => Some(icons::MONITOR_EDIT),
        "brightness-1" => Some(icons::BRIGHTNESS_1),
        "weather-cloudy-clock" => Some(icons::WEATHER_CLOUDY_CLOCK),
        "ethernet-off" => Some(icons::ETHERNET_OFF),
        "flag-variant" => Some(icons::FLAG_VARIANT),
        "folder-lock" => Some(icons::FOLDER_LOCK),
        "music-accidental-natural" => Some(icons::MUSIC_ACCIDENTAL_NATURAL),
        "page-last" => Some(icons::PAGE_LAST),
        #[allow(deprecated)]
        "linkedin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linkedin' is deprecated.").print(py);
            }
            Some(icons::LINKEDIN)
        }
        "filter-cog" => Some(icons::FILTER_COG),
        "chat-plus-outline" => Some(icons::CHAT_PLUS_OUTLINE),
        "gamepad-variant-outline" => Some(icons::GAMEPAD_VARIANT_OUTLINE),
        "heart-pulse" => Some(icons::HEART_PULSE),
        "slide" => Some(icons::SLIDE),
        "email-edit-outline" => Some(icons::EMAIL_EDIT_OUTLINE),
        "room-service-outline" => Some(icons::ROOM_SERVICE_OUTLINE),
        "music-note-eighth-dotted" => Some(icons::MUSIC_NOTE_EIGHTH_DOTTED),
        "battery-charging-30" => Some(icons::BATTERY_CHARGING_30),
        "arrow-up-circle" => Some(icons::ARROW_UP_CIRCLE),
        "account-group" => Some(icons::ACCOUNT_GROUP),
        "chevron-double-left" => Some(icons::CHEVRON_DOUBLE_LEFT),
        "account-eye" => Some(icons::ACCOUNT_EYE),
        "clock-minus-outline" => Some(icons::CLOCK_MINUS_OUTLINE),
        "emoticon-kiss-outline" => Some(icons::EMOTICON_KISS_OUTLINE),
        "hand-extended-outline" => Some(icons::HAND_EXTENDED_OUTLINE),
        "file-document-outline" => Some(icons::FILE_DOCUMENT_OUTLINE),
        "help-rhombus" => Some(icons::HELP_RHOMBUS),
        "tag-minus" => Some(icons::TAG_MINUS),
        "cog-pause" => Some(icons::COG_PAUSE),
        #[allow(deprecated)]
        "microsoft-xbox-controller" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER)
        }
        "postage-stamp" => Some(icons::POSTAGE_STAMP),
        "credit-card-sync" => Some(icons::CREDIT_CARD_SYNC),
        "table-star" => Some(icons::TABLE_STAR),
        "airport" => Some(icons::AIRPORT),
        "comment" => Some(icons::COMMENT),
        "rounded-corner" => Some(icons::ROUNDED_CORNER),
        "star-circle" => Some(icons::STAR_CIRCLE),
        "roman-numeral-5" => Some(icons::ROMAN_NUMERAL_5),
        "hub-outline" => Some(icons::HUB_OUTLINE),
        "format-line-weight" => Some(icons::FORMAT_LINE_WEIGHT),
        "less-than-or-equal" => Some(icons::LESS_THAN_OR_EQUAL),
        "clock-plus" => Some(icons::CLOCK_PLUS),
        "tag-text-outline" => Some(icons::TAG_TEXT_OUTLINE),
        "pipe-disconnected" => Some(icons::PIPE_DISCONNECTED),
        "lan-connect" => Some(icons::LAN_CONNECT),
        "calendar-today" => Some(icons::CALENDAR_TODAY),
        "briefcase-arrow-up-down-outline" => Some(icons::BRIEFCASE_ARROW_UP_DOWN_OUTLINE),
        "alpha-x-circle" => Some(icons::ALPHA_X_CIRCLE),
        "weather-rainy" => Some(icons::WEATHER_RAINY),
        "code-equal" => Some(icons::CODE_EQUAL),
        "menu-left" => Some(icons::MENU_LEFT),
        "movie-play" => Some(icons::MOVIE_PLAY),
        "format-align-top" => Some(icons::FORMAT_ALIGN_TOP),
        "file-music-outline" => Some(icons::FILE_MUSIC_OUTLINE),
        "chevron-down" => Some(icons::CHEVRON_DOWN),
        #[allow(deprecated)]
        "aws" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'aws' is deprecated.").print(py);
            }
            Some(icons::AWS)
        }
        "alpha-l-circle-outline" => Some(icons::ALPHA_L_CIRCLE_OUTLINE),
        "battery-charging-wireless-40" => Some(icons::BATTERY_CHARGING_WIRELESS_40),
        "tooltip-check-outline" => Some(icons::TOOLTIP_CHECK_OUTLINE),
        _ => None,
    }
}
