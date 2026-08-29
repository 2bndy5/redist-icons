// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_32(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "chili-medium-outline" => Some(icons::CHILI_MEDIUM_OUTLINE),
        "relation-many-to-zero-or-one" => Some(icons::RELATION_MANY_TO_ZERO_OR_ONE),
        "image-auto-adjust" => Some(icons::IMAGE_AUTO_ADJUST),
        "basket-remove" => Some(icons::BASKET_REMOVE),
        "credit-card-settings-outline" => Some(icons::CREDIT_CARD_SETTINGS_OUTLINE),
        "floor-lamp-torchiere-variant" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT),
        "axis-arrow" => Some(icons::AXIS_ARROW),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "battery-alert" => Some(icons::BATTERY_ALERT),
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        #[allow(deprecated)]
        "linux" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux' is deprecated.").print(py);
            }
            Some(icons::LINUX)
        }
        "file-plus" => Some(icons::FILE_PLUS),
        "human-male-male" => Some(icons::HUMAN_MALE_MALE),
        "food-steak" => Some(icons::FOOD_STEAK),
        "note-multiple-outline" => Some(icons::NOTE_MULTIPLE_OUTLINE),
        "clock-time-eight-outline" => Some(icons::CLOCK_TIME_EIGHT_OUTLINE),
        "book-sync" => Some(icons::BOOK_SYNC),
        #[allow(deprecated)]
        "kodi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kodi' is deprecated.").print(py);
            }
            Some(icons::KODI)
        }
        "octagram-minus-outline" => Some(icons::OCTAGRAM_MINUS_OUTLINE),
        "city-variant-outline" => Some(icons::CITY_VARIANT_OUTLINE),
        "greenhouse" => Some(icons::GREENHOUSE),
        "bed-single-outline" => Some(icons::BED_SINGLE_OUTLINE),
        "cog-off-outline" => Some(icons::COG_OFF_OUTLINE),
        "tree-outline" => Some(icons::TREE_OUTLINE),
        "sort-variant-lock-open" => Some(icons::SORT_VARIANT_LOCK_OPEN),
        "alpha-g-circle" => Some(icons::ALPHA_G_CIRCLE),
        "star-four-points" => Some(icons::STAR_FOUR_POINTS),
        "music-circle" => Some(icons::MUSIC_CIRCLE),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "file-phone-outline" => Some(icons::FILE_PHONE_OUTLINE),
        "billboard" => Some(icons::BILLBOARD),
        "basket-minus" => Some(icons::BASKET_MINUS),
        "map-minus" => Some(icons::MAP_MINUS),
        "projector" => Some(icons::PROJECTOR),
        "printer-pos-cancel-outline" => Some(icons::PRINTER_POS_CANCEL_OUTLINE),
        "bottle-soda-classic-outline" => Some(icons::BOTTLE_SODA_CLASSIC_OUTLINE),
        "delete-off-outline" => Some(icons::DELETE_OFF_OUTLINE),
        "audio-input-stereo-minijack" => Some(icons::AUDIO_INPUT_STEREO_MINIJACK),
        #[allow(deprecated)]
        "microsoft-visual-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-visual-studio' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_VISUAL_STUDIO)
        }
        "toothbrush" => Some(icons::TOOTHBRUSH),
        "wifi-marker" => Some(icons::WIFI_MARKER),
        "gamepad-right" => Some(icons::GAMEPAD_RIGHT),
        "sort-variant-lock" => Some(icons::SORT_VARIANT_LOCK),
        "calendar-plus-outline" => Some(icons::CALENDAR_PLUS_OUTLINE),
        "mailbox-open-up-outline" => Some(icons::MAILBOX_OPEN_UP_OUTLINE),
        "phone-forward" => Some(icons::PHONE_FORWARD),
        "invoice-text-multiple-outline" => Some(icons::INVOICE_TEXT_MULTIPLE_OUTLINE),
        "set-center-right" => Some(icons::SET_CENTER_RIGHT),
        "vector-polyline" => Some(icons::VECTOR_POLYLINE),
        #[allow(deprecated)]
        "cryengine" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'cryengine' is deprecated.").print(py);
            }
            Some(icons::CRYENGINE)
        }
        #[allow(deprecated)]
        "microsoft-xbox-controller-menu" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-menu' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_MENU)
        }
        "smoke-detector-alert-outline" => Some(icons::SMOKE_DETECTOR_ALERT_OUTLINE),
        "folder-settings-outline" => Some(icons::FOLDER_SETTINGS_OUTLINE),
        "arrow-projectile" => Some(icons::ARROW_PROJECTILE),
        "mouse-variant-off" => Some(icons::MOUSE_VARIANT_OFF),
        "fan-plus" => Some(icons::FAN_PLUS),
        "database-remove" => Some(icons::DATABASE_REMOVE),
        "chevron-left" => Some(icons::CHEVRON_LEFT),
        "chart-pie-outline" => Some(icons::CHART_PIE_OUTLINE),
        "video-2d" => Some(icons::VIDEO_2D),
        "toaster" => Some(icons::TOASTER),
        "string-lights" => Some(icons::STRING_LIGHTS),
        "clipboard-play-outline" => Some(icons::CLIPBOARD_PLAY_OUTLINE),
        "emoticon-poop-outline" => Some(icons::EMOTICON_POOP_OUTLINE),
        "key-star" => Some(icons::KEY_STAR),
        "comma-circle" => Some(icons::COMMA_CIRCLE),
        "currency-brl" => Some(icons::CURRENCY_BRL),
        "roman-numeral-3" => Some(icons::ROMAN_NUMERAL_3),
        "arrow-down-bold-hexagon-outline" => Some(icons::ARROW_DOWN_BOLD_HEXAGON_OUTLINE),
        "server-outline" => Some(icons::SERVER_OUTLINE),
        "star-off-outline" => Some(icons::STAR_OFF_OUTLINE),
        "ticket-percent-outline" => Some(icons::TICKET_PERCENT_OUTLINE),
        "zodiac-cancer" => Some(icons::ZODIAC_CANCER),
        "beaker-plus-outline" => Some(icons::BEAKER_PLUS_OUTLINE),
        "comment-check-outline" => Some(icons::COMMENT_CHECK_OUTLINE),
        "skip-next-circle-outline" => Some(icons::SKIP_NEXT_CIRCLE_OUTLINE),
        "kettle-outline" => Some(icons::KETTLE_OUTLINE),
        "tab-remove" => Some(icons::TAB_REMOVE),
        "label-percent" => Some(icons::LABEL_PERCENT),
        "book-search-outline" => Some(icons::BOOK_SEARCH_OUTLINE),
        "numeric-9-circle-outline" => Some(icons::NUMERIC_9_CIRCLE_OUTLINE),
        "view-carousel-outline" => Some(icons::VIEW_CAROUSEL_OUTLINE),
        #[allow(deprecated)]
        "microsoft-word" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-word' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_WORD)
        }
        "ballot" => Some(icons::BALLOT),
        "crane" => Some(icons::CRANE),
        "bag-carry-on" => Some(icons::BAG_CARRY_ON),
        "truck-snowflake" => Some(icons::TRUCK_SNOWFLAKE),
        "progress-pencil" => Some(icons::PROGRESS_PENCIL),
        "allergy" => Some(icons::ALLERGY),
        "folder-music" => Some(icons::FOLDER_MUSIC),
        "format-color-marker-cancel" => Some(icons::FORMAT_COLOR_MARKER_CANCEL),
        "hand-okay" => Some(icons::HAND_OKAY),
        "folder-key-network" => Some(icons::FOLDER_KEY_NETWORK),
        "calculator-variant-outline" => Some(icons::CALCULATOR_VARIANT_OUTLINE),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "beaker-plus" => Some(icons::BEAKER_PLUS),
        "book-lock" => Some(icons::BOOK_LOCK),
        "package-up" => Some(icons::PACKAGE_UP),
        "movie-open-cog" => Some(icons::MOVIE_OPEN_COG),
        "decimal-comma" => Some(icons::DECIMAL_COMMA),
        "radioactive-circle" => Some(icons::RADIOACTIVE_CIRCLE),
        "karate" => Some(icons::KARATE),
        "train-car-passenger-door" => Some(icons::TRAIN_CAR_PASSENGER_DOOR),
        "email-heart-outline" => Some(icons::EMAIL_HEART_OUTLINE),
        "satellite-uplink" => Some(icons::SATELLITE_UPLINK),
        "cards-playing-outline" => Some(icons::CARDS_PLAYING_OUTLINE),
        "table-minus" => Some(icons::TABLE_MINUS),
        "traffic-cone" => Some(icons::TRAFFIC_CONE),
        "message-off" => Some(icons::MESSAGE_OFF),
        "database-check-outline" => Some(icons::DATABASE_CHECK_OUTLINE),
        "unfold-less-horizontal" => Some(icons::UNFOLD_LESS_HORIZONTAL),
        "campfire" => Some(icons::CAMPFIRE),
        "format-color-highlight" => Some(icons::FORMAT_COLOR_HIGHLIGHT),
        "cookie" => Some(icons::COOKIE),
        "human-male-male-child" => Some(icons::HUMAN_MALE_MALE_CHILD),
        "diaper-outline" => Some(icons::DIAPER_OUTLINE),
        "video-3d-off" => Some(icons::VIDEO_3D_OFF),
        "train-car-caboose" => Some(icons::TRAIN_CAR_CABOOSE),
        "table-lock" => Some(icons::TABLE_LOCK),
        "inbox-remove-outline" => Some(icons::INBOX_REMOVE_OUTLINE),
        "restart-alert" => Some(icons::RESTART_ALERT),
        "fridge-industrial-off" => Some(icons::FRIDGE_INDUSTRIAL_OFF),
        "archive-refresh-outline" => Some(icons::ARCHIVE_REFRESH_OUTLINE),
        "puzzle-star-outline" => Some(icons::PUZZLE_STAR_OUTLINE),
        "arrow-up-left-bold" => Some(icons::ARROW_UP_LEFT_BOLD),
        "star-four-points-circle" => Some(icons::STAR_FOUR_POINTS_CIRCLE),
        "bell-minus-outline" => Some(icons::BELL_MINUS_OUTLINE),
        "sign-text" => Some(icons::SIGN_TEXT),
        "clipboard-off-outline" => Some(icons::CLIPBOARD_OFF_OUTLINE),
        "phone-rotate-landscape" => Some(icons::PHONE_ROTATE_LANDSCAPE),
        "upload-network" => Some(icons::UPLOAD_NETWORK),
        "cellphone-sound" => Some(icons::CELLPHONE_SOUND),
        "arrow-expand-left" => Some(icons::ARROW_EXPAND_LEFT),
        "distribute-horizontal-center" => Some(icons::DISTRIBUTE_HORIZONTAL_CENTER),
        "clock-time-seven" => Some(icons::CLOCK_TIME_SEVEN),
        "wan" => Some(icons::WAN),
        "timer-star" => Some(icons::TIMER_STAR),
        "home-lightbulb" => Some(icons::HOME_LIGHTBULB),
        "flask-empty-remove-outline" => Some(icons::FLASK_EMPTY_REMOVE_OUTLINE),
        "brain" => Some(icons::BRAIN),
        "information-variant-circle-outline" => Some(icons::INFORMATION_VARIANT_CIRCLE_OUTLINE),
        "human-greeting-variant" => Some(icons::HUMAN_GREETING_VARIANT),
        "paperclip-plus" => Some(icons::PAPERCLIP_PLUS),
        "magnify-remove-outline" => Some(icons::MAGNIFY_REMOVE_OUTLINE),
        "shoe-heel" => Some(icons::SHOE_HEEL),
        "network-off-outline" => Some(icons::NETWORK_OFF_OUTLINE),
        "flask-empty-plus-outline" => Some(icons::FLASK_EMPTY_PLUS_OUTLINE),
        "margin" => Some(icons::MARGIN),
        "ev-plug-type2" => Some(icons::EV_PLUG_TYPE2),
        "file-image-remove" => Some(icons::FILE_IMAGE_REMOVE),
        "hexadecimal" => Some(icons::HEXADECIMAL),
        "kettle-off-outline" => Some(icons::KETTLE_OFF_OUTLINE),
        "numeric-9-box-outline" => Some(icons::NUMERIC_9_BOX_OUTLINE),
        "play-outline" => Some(icons::PLAY_OUTLINE),
        "message-reply-outline" => Some(icons::MESSAGE_REPLY_OUTLINE),
        "currency-gbp" => Some(icons::CURRENCY_GBP),
        "quality-medium" => Some(icons::QUALITY_MEDIUM),
        "keyboard-f7" => Some(icons::KEYBOARD_F7),
        "folder-key" => Some(icons::FOLDER_KEY),
        "checkbox-blank-circle-outline" => Some(icons::CHECKBOX_BLANK_CIRCLE_OUTLINE),
        "map-marker-up" => Some(icons::MAP_MARKER_UP),
        "quality-high" => Some(icons::QUALITY_HIGH),
        "wallet-plus" => Some(icons::WALLET_PLUS),
        "emoticon-frown" => Some(icons::EMOTICON_FROWN),
        "layers-minus" => Some(icons::LAYERS_MINUS),
        "phone-in-talk" => Some(icons::PHONE_IN_TALK),
        "bus-multiple" => Some(icons::BUS_MULTIPLE),
        "monitor-arrow-down" => Some(icons::MONITOR_ARROW_DOWN),
        "fuel-cell" => Some(icons::FUEL_CELL),
        "home-lightbulb-outline" => Some(icons::HOME_LIGHTBULB_OUTLINE),
        "comment-account-outline" => Some(icons::COMMENT_ACCOUNT_OUTLINE),
        "database-arrow-down-outline" => Some(icons::DATABASE_ARROW_DOWN_OUTLINE),
        "cellphone-message" => Some(icons::CELLPHONE_MESSAGE),
        "bank-outline" => Some(icons::BANK_OUTLINE),
        "skip-backward-outline" => Some(icons::SKIP_BACKWARD_OUTLINE),
        "calendar-minus-outline" => Some(icons::CALENDAR_MINUS_OUTLINE),
        "food-apple-outline" => Some(icons::FOOD_APPLE_OUTLINE),
        "radiator-off" => Some(icons::RADIATOR_OFF),
        "comment-edit-outline" => Some(icons::COMMENT_EDIT_OUTLINE),
        "monitor-shimmer" => Some(icons::MONITOR_SHIMMER),
        "skip-previous-circle" => Some(icons::SKIP_PREVIOUS_CIRCLE),
        "light-recessed" => Some(icons::LIGHT_RECESSED),
        "map-clock-outline" => Some(icons::MAP_CLOCK_OUTLINE),
        "email-multiple" => Some(icons::EMAIL_MULTIPLE),
        "camera" => Some(icons::CAMERA),
        #[allow(deprecated)]
        "stack-exchange" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-exchange' is deprecated.").print(py);
            }
            Some(icons::STACK_EXCHANGE)
        }
        "pill-off" => Some(icons::PILL_OFF),
        "toggle-switch-off-outline" => Some(icons::TOGGLE_SWITCH_OFF_OUTLINE),
        "newspaper-variant-multiple" => Some(icons::NEWSPAPER_VARIANT_MULTIPLE),
        "triangle-down" => Some(icons::TRIANGLE_DOWN),
        "gesture-swipe" => Some(icons::GESTURE_SWIPE),
        "flag-variant-off" => Some(icons::FLAG_VARIANT_OFF),
        "database-arrow-left" => Some(icons::DATABASE_ARROW_LEFT),
        "head" => Some(icons::HEAD),
        "test-tube-off" => Some(icons::TEST_TUBE_OFF),
        "cloud-cog-outline" => Some(icons::CLOUD_COG_OUTLINE),
        #[allow(deprecated)]
        "material-design" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-design' is deprecated.")
                    .print(py);
            }
            Some(icons::MATERIAL_DESIGN)
        }
        "close-network-outline" => Some(icons::CLOSE_NETWORK_OUTLINE),
        "plus-lock" => Some(icons::PLUS_LOCK),
        "alpha-c-circle" => Some(icons::ALPHA_C_CIRCLE),
        _ => None,
    }
}
