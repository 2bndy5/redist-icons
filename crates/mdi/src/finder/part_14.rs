// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_14(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "cellphone" => Some(icons::CELLPHONE),
        "movie-star-outline" => Some(icons::MOVIE_STAR_OUTLINE),
        "crane" => Some(icons::CRANE),
        "message-text-clock-outline" => Some(icons::MESSAGE_TEXT_CLOCK_OUTLINE),
        "notebook-minus-outline" => Some(icons::NOTEBOOK_MINUS_OUTLINE),
        "phone-settings-outline" => Some(icons::PHONE_SETTINGS_OUTLINE),
        "card-bulleted-off-outline" => Some(icons::CARD_BULLETED_OFF_OUTLINE),
        #[allow(deprecated)]
        "fedora" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'fedora' is deprecated.").print(py);
            }
            Some(icons::FEDORA)
        }
        "shield-half-full" => Some(icons::SHIELD_HALF_FULL),
        "home-search" => Some(icons::HOME_SEARCH),
        "account-arrow-right" => Some(icons::ACCOUNT_ARROW_RIGHT),
        "mower" => Some(icons::MOWER),
        "dice-2" => Some(icons::DICE_2),
        "approximately-equal" => Some(icons::APPROXIMATELY_EQUAL),
        "podium-silver" => Some(icons::PODIUM_SILVER),
        "alpha-j-circle" => Some(icons::ALPHA_J_CIRCLE),
        "numeric-0-circle-outline" => Some(icons::NUMERIC_0_CIRCLE_OUTLINE),
        "robot-happy-outline" => Some(icons::ROBOT_HAPPY_OUTLINE),
        "rewind-45" => Some(icons::REWIND_45),
        "face-mask-outline" => Some(icons::FACE_MASK_OUTLINE),
        "briefcase-eye" => Some(icons::BRIEFCASE_EYE),
        "bio" => Some(icons::BIO),
        "signal-5g" => Some(icons::SIGNAL_5G),
        "alpha-n" => Some(icons::ALPHA_N),
        "food-halal" => Some(icons::FOOD_HALAL),
        "alarm-off" => Some(icons::ALARM_OFF),
        "gradient-vertical" => Some(icons::GRADIENT_VERTICAL),
        "size-xxs" => Some(icons::SIZE_XXS),
        "food-steak-off" => Some(icons::FOOD_STEAK_OFF),
        "notebook-multiple" => Some(icons::NOTEBOOK_MULTIPLE),
        "clipboard-flow-outline" => Some(icons::CLIPBOARD_FLOW_OUTLINE),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "chart-ppf" => Some(icons::CHART_PPF),
        "campfire" => Some(icons::CAMPFIRE),
        "scale" => Some(icons::SCALE),
        "comment-minus" => Some(icons::COMMENT_MINUS),
        "pencil-minus-outline" => Some(icons::PENCIL_MINUS_OUTLINE),
        #[allow(deprecated)]
        "android-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'android-studio' is deprecated.").print(py);
            }
            Some(icons::ANDROID_STUDIO)
        }
        "clipboard-plus-outline" => Some(icons::CLIPBOARD_PLUS_OUTLINE),
        "progress-question" => Some(icons::PROGRESS_QUESTION),
        "tag-check" => Some(icons::TAG_CHECK),
        "move-resize-variant" => Some(icons::MOVE_RESIZE_VARIANT),
        "axe-battle" => Some(icons::AXE_BATTLE),
        "numeric-0-box-multiple-outline" => Some(icons::NUMERIC_0_BOX_MULTIPLE_OUTLINE),
        "format-clear" => Some(icons::FORMAT_CLEAR),
        "mouse-move-vertical" => Some(icons::MOUSE_MOVE_VERTICAL),
        "file-star" => Some(icons::FILE_STAR),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "gesture-two-tap" => Some(icons::GESTURE_TWO_TAP),
        "eye-lock-open" => Some(icons::EYE_LOCK_OPEN),
        "layers" => Some(icons::LAYERS),
        "sort-variant-off" => Some(icons::SORT_VARIANT_OFF),
        "room-service" => Some(icons::ROOM_SERVICE),
        "face-man" => Some(icons::FACE_MAN),
        "plus-circle-multiple" => Some(icons::PLUS_CIRCLE_MULTIPLE),
        "controller" => Some(icons::CONTROLLER),
        "music-rest-quarter" => Some(icons::MUSIC_REST_QUARTER),
        "contactless-payment" => Some(icons::CONTACTLESS_PAYMENT),
        "format-underline" => Some(icons::FORMAT_UNDERLINE),
        "cast-audio-variant" => Some(icons::CAST_AUDIO_VARIANT),
        "store-cog" => Some(icons::STORE_COG),
        "format-horizontal-align-center" => Some(icons::FORMAT_HORIZONTAL_ALIGN_CENTER),
        "food-kosher" => Some(icons::FOOD_KOSHER),
        "minus-circle" => Some(icons::MINUS_CIRCLE),
        "tunnel" => Some(icons::TUNNEL),
        "guitar-pick" => Some(icons::GUITAR_PICK),
        "image-refresh-outline" => Some(icons::IMAGE_REFRESH_OUTLINE),
        "check-decagram-outline" => Some(icons::CHECK_DECAGRAM_OUTLINE),
        "tea-outline" => Some(icons::TEA_OUTLINE),
        "radiology-box" => Some(icons::RADIOLOGY_BOX),
        "calendar-expand-horizontal-outline" => Some(icons::CALENDAR_EXPAND_HORIZONTAL_OUTLINE),
        "poker-chip" => Some(icons::POKER_CHIP),
        "star-outline" => Some(icons::STAR_OUTLINE),
        "alpha-i-box-outline" => Some(icons::ALPHA_I_BOX_OUTLINE),
        "car-turbocharger" => Some(icons::CAR_TURBOCHARGER),
        "graph-outline" => Some(icons::GRAPH_OUTLINE),
        "memory-arrow-down" => Some(icons::MEMORY_ARROW_DOWN),
        "wallet-membership" => Some(icons::WALLET_MEMBERSHIP),
        "view-carousel-outline" => Some(icons::VIEW_CAROUSEL_OUTLINE),
        "projector-screen-outline" => Some(icons::PROJECTOR_SCREEN_OUTLINE),
        "folder-network" => Some(icons::FOLDER_NETWORK),
        "account-cancel-outline" => Some(icons::ACCOUNT_CANCEL_OUTLINE),
        "email-fast" => Some(icons::EMAIL_FAST),
        "box-cutter-off" => Some(icons::BOX_CUTTER_OFF),
        "power-socket-au" => Some(icons::POWER_SOCKET_AU),
        "flag-variant-plus-outline" => Some(icons::FLAG_VARIANT_PLUS_OUTLINE),
        "key-chain" => Some(icons::KEY_CHAIN),
        "koala" => Some(icons::KOALA),
        "vector-difference-ab" => Some(icons::VECTOR_DIFFERENCE_AB),
        "pillar" => Some(icons::PILLAR),
        "human-child" => Some(icons::HUMAN_CHILD),
        "arrow-right-bold-box" => Some(icons::ARROW_RIGHT_BOLD_BOX),
        "star-box" => Some(icons::STAR_BOX),
        "calendar-weekend" => Some(icons::CALENDAR_WEEKEND),
        "bookmark-remove" => Some(icons::BOOKMARK_REMOVE),
        "arrow-left-bold-box" => Some(icons::ARROW_LEFT_BOLD_BOX),
        "home-sound-out-outline" => Some(icons::HOME_SOUND_OUT_OUTLINE),
        "note-search-outline" => Some(icons::NOTE_SEARCH_OUTLINE),
        "compass-off" => Some(icons::COMPASS_OFF),
        "bed-double" => Some(icons::BED_DOUBLE),
        "ferris-wheel" => Some(icons::FERRIS_WHEEL),
        "home-variant-outline" => Some(icons::HOME_VARIANT_OUTLINE),
        "land-rows-horizontal" => Some(icons::LAND_ROWS_HORIZONTAL),
        "file-lock-open" => Some(icons::FILE_LOCK_OPEN),
        "currency-eth" => Some(icons::CURRENCY_ETH),
        #[allow(deprecated)]
        "wikipedia" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'wikipedia' is deprecated.").print(py);
            }
            Some(icons::WIKIPEDIA)
        }
        "sprinkler-variant" => Some(icons::SPRINKLER_VARIANT),
        "compare" => Some(icons::COMPARE),
        "briefcase-check-outline" => Some(icons::BRIEFCASE_CHECK_OUTLINE),
        "gauge" => Some(icons::GAUGE),
        "account-box" => Some(icons::ACCOUNT_BOX),
        "camera-marker-outline" => Some(icons::CAMERA_MARKER_OUTLINE),
        "calendar-badge" => Some(icons::CALENDAR_BADGE),
        "smart-card-off" => Some(icons::SMART_CARD_OFF),
        "account-circle" => Some(icons::ACCOUNT_CIRCLE),
        "archive-marker-outline" => Some(icons::ARCHIVE_MARKER_OUTLINE),
        "arrow-top-right-thin" => Some(icons::ARROW_TOP_RIGHT_THIN),
        "beaker-question-outline" => Some(icons::BEAKER_QUESTION_OUTLINE),
        "account-eye" => Some(icons::ACCOUNT_EYE),
        "store-search-outline" => Some(icons::STORE_SEARCH_OUTLINE),
        "clipboard-play" => Some(icons::CLIPBOARD_PLAY),
        "clipboard-arrow-up-outline" => Some(icons::CLIPBOARD_ARROW_UP_OUTLINE),
        "thermostat" => Some(icons::THERMOSTAT),
        "coffee-outline" => Some(icons::COFFEE_OUTLINE),
        "resize" => Some(icons::RESIZE),
        "trophy-variant-outline" => Some(icons::TROPHY_VARIANT_OUTLINE),
        "script-text-outline" => Some(icons::SCRIPT_TEXT_OUTLINE),
        "email-open-multiple-outline" => Some(icons::EMAIL_OPEN_MULTIPLE_OUTLINE),
        "eyedropper-off" => Some(icons::EYEDROPPER_OFF),
        "folder-outline" => Some(icons::FOLDER_OUTLINE),
        "account-multiple-minus" => Some(icons::ACCOUNT_MULTIPLE_MINUS),
        "hiking" => Some(icons::HIKING),
        "console-network" => Some(icons::CONSOLE_NETWORK),
        "wifi-strength-off-outline" => Some(icons::WIFI_STRENGTH_OFF_OUTLINE),
        "account-injury" => Some(icons::ACCOUNT_INJURY),
        "numeric-10-box-multiple" => Some(icons::NUMERIC_10_BOX_MULTIPLE),
        "train-car-hopper" => Some(icons::TRAIN_CAR_HOPPER),
        "format-subscript" => Some(icons::FORMAT_SUBSCRIPT),
        "sort-numeric-ascending-variant" => Some(icons::SORT_NUMERIC_ASCENDING_VARIANT),
        "jellyfish-outline" => Some(icons::JELLYFISH_OUTLINE),
        "lightbulb-multiple-off-outline" => Some(icons::LIGHTBULB_MULTIPLE_OFF_OUTLINE),
        "play-network" => Some(icons::PLAY_NETWORK),
        #[allow(deprecated)]
        "apache-kafka" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apache-kafka' is deprecated.").print(py);
            }
            Some(icons::APACHE_KAFKA)
        }
        "temple-hindu" => Some(icons::TEMPLE_HINDU),
        "scoreboard-outline" => Some(icons::SCOREBOARD_OUTLINE),
        "reload" => Some(icons::RELOAD),
        "lamps-outline" => Some(icons::LAMPS_OUTLINE),
        "boom-gate-arrow-up-outline" => Some(icons::BOOM_GATE_ARROW_UP_OUTLINE),
        "human-handsdown" => Some(icons::HUMAN_HANDSDOWN),
        "periodic-table" => Some(icons::PERIODIC_TABLE),
        "upload-off" => Some(icons::UPLOAD_OFF),
        "sign-direction-minus" => Some(icons::SIGN_DIRECTION_MINUS),
        "timeline-remove" => Some(icons::TIMELINE_REMOVE),
        "human-handsup" => Some(icons::HUMAN_HANDSUP),
        "file-gif-box" => Some(icons::FILE_GIF_BOX),
        "swap-vertical-circle-outline" => Some(icons::SWAP_VERTICAL_CIRCLE_OUTLINE),
        "book-outline" => Some(icons::BOOK_OUTLINE),
        "alpha-k-box" => Some(icons::ALPHA_K_BOX),
        "home-import-outline" => Some(icons::HOME_IMPORT_OUTLINE),
        "clipboard-file-outline" => Some(icons::CLIPBOARD_FILE_OUTLINE),
        "magnify-plus-cursor" => Some(icons::MAGNIFY_PLUS_CURSOR),
        "page-next" => Some(icons::PAGE_NEXT),
        "minus-circle-outline" => Some(icons::MINUS_CIRCLE_OUTLINE),
        "sack-outline" => Some(icons::SACK_OUTLINE),
        "cookie-outline" => Some(icons::COOKIE_OUTLINE),
        "card-minus" => Some(icons::CARD_MINUS),
        "fridge-variant-outline" => Some(icons::FRIDGE_VARIANT_OUTLINE),
        "cursor-pointer" => Some(icons::CURSOR_POINTER),
        "generator-portable" => Some(icons::GENERATOR_PORTABLE),
        "shopping-outline" => Some(icons::SHOPPING_OUTLINE),
        "bullet" => Some(icons::BULLET),
        "balloon" => Some(icons::BALLOON),
        "food-apple-outline" => Some(icons::FOOD_APPLE_OUTLINE),
        "fuel" => Some(icons::FUEL),
        "serial-port" => Some(icons::SERIAL_PORT),
        "volume-plus" => Some(icons::VOLUME_PLUS),
        "file-export" => Some(icons::FILE_EXPORT),
        "family-tree" => Some(icons::FAMILY_TREE),
        "format-text-rotation-down" => Some(icons::FORMAT_TEXT_ROTATION_DOWN),
        #[allow(deprecated)]
        "youtube-tv" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-tv' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_TV)
        }
        #[allow(deprecated)]
        "evernote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'evernote' is deprecated.").print(py);
            }
            Some(icons::EVERNOTE)
        }
        "table-pivot" => Some(icons::TABLE_PIVOT),
        "multiplication" => Some(icons::MULTIPLICATION),
        "inbox-arrow-up" => Some(icons::INBOX_ARROW_UP),
        "buffet" => Some(icons::BUFFET),
        "arrow-oscillating-off" => Some(icons::ARROW_OSCILLATING_OFF),
        "storefront-plus-outline" => Some(icons::STOREFRONT_PLUS_OUTLINE),
        "keyboard-tab-reverse" => Some(icons::KEYBOARD_TAB_REVERSE),
        "string-lights-off" => Some(icons::STRING_LIGHTS_OFF),
        "seat" => Some(icons::SEAT),
        "home-floor-1" => Some(icons::HOME_FLOOR_1),
        "vector-polyline-plus" => Some(icons::VECTOR_POLYLINE_PLUS),
        "text-box-remove-outline" => Some(icons::TEXT_BOX_REMOVE_OUTLINE),
        "account-file-text-outline" => Some(icons::ACCOUNT_FILE_TEXT_OUTLINE),
        "tower-fire" => Some(icons::TOWER_FIRE),
        "file-alert-outline" => Some(icons::FILE_ALERT_OUTLINE),
        "vector-polyline-remove" => Some(icons::VECTOR_POLYLINE_REMOVE),
        "battery-minus-outline" => Some(icons::BATTERY_MINUS_OUTLINE),
        "account-key" => Some(icons::ACCOUNT_KEY),
        "emoticon-happy-outline" => Some(icons::EMOTICON_HAPPY_OUTLINE),
        _ => None,
    }
}
