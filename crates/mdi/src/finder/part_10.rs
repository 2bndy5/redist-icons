// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_10(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "cog-sync-outline" => Some(icons::COG_SYNC_OUTLINE),
        "file-word-outline" => Some(icons::FILE_WORD_OUTLINE),
        "delete-circle-outline" => Some(icons::DELETE_CIRCLE_OUTLINE),
        "head-flash-outline" => Some(icons::HEAD_FLASH_OUTLINE),
        "shield-sword-outline" => Some(icons::SHIELD_SWORD_OUTLINE),
        "iron-board" => Some(icons::IRON_BOARD),
        "delete-circle" => Some(icons::DELETE_CIRCLE),
        #[allow(deprecated)]
        "github" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'github' is deprecated.").print(py);
            }
            Some(icons::GITHUB)
        }
        "archive-plus" => Some(icons::ARCHIVE_PLUS),
        "shark-fin-outline" => Some(icons::SHARK_FIN_OUTLINE),
        "format-header-1" => Some(icons::FORMAT_HEADER_1),
        "looks" => Some(icons::LOOKS),
        "wall-sconce-round-outline" => Some(icons::WALL_SCONCE_ROUND_OUTLINE),
        "tray-minus" => Some(icons::TRAY_MINUS),
        "star-crescent" => Some(icons::STAR_CRESCENT),
        "vector-polyline-plus" => Some(icons::VECTOR_POLYLINE_PLUS),
        "menu-up" => Some(icons::MENU_UP),
        "arrow-right-bold-box-outline" => Some(icons::ARROW_RIGHT_BOLD_BOX_OUTLINE),
        "credit-card-wireless-off" => Some(icons::CREDIT_CARD_WIRELESS_OFF),
        "key-chain-variant" => Some(icons::KEY_CHAIN_VARIANT),
        "clock-time-eleven" => Some(icons::CLOCK_TIME_ELEVEN),
        "store-cog" => Some(icons::STORE_COG),
        #[allow(deprecated)]
        "google-keep" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-keep' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_KEEP)
        }
        "gesture-pinch" => Some(icons::GESTURE_PINCH),
        "file-image-marker-outline" => Some(icons::FILE_IMAGE_MARKER_OUTLINE),
        "cards-spade-outline" => Some(icons::CARDS_SPADE_OUTLINE),
        "share" => Some(icons::SHARE),
        "bio" => Some(icons::BIO),
        "rotate-right-variant" => Some(icons::ROTATE_RIGHT_VARIANT),
        "playlist-music-outline" => Some(icons::PLAYLIST_MUSIC_OUTLINE),
        "sort-clock-descending" => Some(icons::SORT_CLOCK_DESCENDING),
        "movie-open-edit" => Some(icons::MOVIE_OPEN_EDIT),
        "beach" => Some(icons::BEACH),
        "nfc-tap" => Some(icons::NFC_TAP),
        "plus-box-outline" => Some(icons::PLUS_BOX_OUTLINE),
        "music-note-sixteenth" => Some(icons::MUSIC_NOTE_SIXTEENTH),
        "storefront-edit-outline" => Some(icons::STOREFRONT_EDIT_OUTLINE),
        "chair-school" => Some(icons::CHAIR_SCHOOL),
        "television-play" => Some(icons::TELEVISION_PLAY),
        "alpha-l-box" => Some(icons::ALPHA_L_BOX),
        "music-note" => Some(icons::MUSIC_NOTE),
        "fruit-pear" => Some(icons::FRUIT_PEAR),
        "signature-image" => Some(icons::SIGNATURE_IMAGE),
        "script-outline" => Some(icons::SCRIPT_OUTLINE),
        "book-open-variant-outline" => Some(icons::BOOK_OPEN_VARIANT_OUTLINE),
        "note-search-outline" => Some(icons::NOTE_SEARCH_OUTLINE),
        "battery-check" => Some(icons::BATTERY_CHECK),
        "file-settings" => Some(icons::FILE_SETTINGS),
        "store-alert" => Some(icons::STORE_ALERT),
        "baby-carriage-off" => Some(icons::BABY_CARRIAGE_OFF),
        "layers-search-outline" => Some(icons::LAYERS_SEARCH_OUTLINE),
        "archive-alert" => Some(icons::ARCHIVE_ALERT),
        "chat-processing" => Some(icons::CHAT_PROCESSING),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "muffin" => Some(icons::MUFFIN),
        "size-xl" => Some(icons::SIZE_XL),
        "invoice-import-outline" => Some(icons::INVOICE_IMPORT_OUTLINE),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        "head-question-outline" => Some(icons::HEAD_QUESTION_OUTLINE),
        "map-marker-plus-outline" => Some(icons::MAP_MARKER_PLUS_OUTLINE),
        "shield-plus" => Some(icons::SHIELD_PLUS),
        "invoice-check-outline" => Some(icons::INVOICE_CHECK_OUTLINE),
        "silo-outline" => Some(icons::SILO_OUTLINE),
        "file-question-outline" => Some(icons::FILE_QUESTION_OUTLINE),
        "fridge-variant-outline" => Some(icons::FRIDGE_VARIANT_OUTLINE),
        "hand-front-left" => Some(icons::HAND_FRONT_LEFT),
        "relation-many-to-zero-or-one" => Some(icons::RELATION_MANY_TO_ZERO_OR_ONE),
        "view-split-horizontal" => Some(icons::VIEW_SPLIT_HORIZONTAL),
        "numeric-9-box-multiple-outline" => Some(icons::NUMERIC_9_BOX_MULTIPLE_OUTLINE),
        "cloud-remove" => Some(icons::CLOUD_REMOVE),
        "surround-sound-2-0" => Some(icons::SURROUND_SOUND_2_0),
        "clock-out" => Some(icons::CLOCK_OUT),
        "shield-refresh-outline" => Some(icons::SHIELD_REFRESH_OUTLINE),
        "blood-bag" => Some(icons::BLOOD_BAG),
        "transit-skip" => Some(icons::TRANSIT_SKIP),
        "briefcase-outline" => Some(icons::BRIEFCASE_OUTLINE),
        "database-marker-outline" => Some(icons::DATABASE_MARKER_OUTLINE),
        "battery-arrow-up-outline" => Some(icons::BATTERY_ARROW_UP_OUTLINE),
        "reorder-horizontal" => Some(icons::REORDER_HORIZONTAL),
        "car-wash" => Some(icons::CAR_WASH),
        "meter-electric" => Some(icons::METER_ELECTRIC),
        "file-document-multiple-outline" => Some(icons::FILE_DOCUMENT_MULTIPLE_OUTLINE),
        "roman-numeral-10" => Some(icons::ROMAN_NUMERAL_10),
        "page-layout-body" => Some(icons::PAGE_LAYOUT_BODY),
        "horse-human" => Some(icons::HORSE_HUMAN),
        "cellphone-dock" => Some(icons::CELLPHONE_DOCK),
        "minus-box-multiple-outline" => Some(icons::MINUS_BOX_MULTIPLE_OUTLINE),
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "bag-personal-plus" => Some(icons::BAG_PERSONAL_PLUS),
        "pipe-valve" => Some(icons::PIPE_VALVE),
        "invoice-text-check" => Some(icons::INVOICE_TEXT_CHECK),
        "progress-close" => Some(icons::PROGRESS_CLOSE),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "water-off" => Some(icons::WATER_OFF),
        "headset-dock" => Some(icons::HEADSET_DOCK),
        "currency-inr" => Some(icons::CURRENCY_INR),
        "bus-marker" => Some(icons::BUS_MARKER),
        "human-capacity-increase" => Some(icons::HUMAN_CAPACITY_INCREASE),
        "radiator" => Some(icons::RADIATOR),
        "gift" => Some(icons::GIFT),
        "brightness-2" => Some(icons::BRIGHTNESS_2),
        "content-save-move" => Some(icons::CONTENT_SAVE_MOVE),
        "diabetes" => Some(icons::DIABETES),
        "note-search" => Some(icons::NOTE_SEARCH),
        "comment-multiple" => Some(icons::COMMENT_MULTIPLE),
        "ballot-outline" => Some(icons::BALLOT_OUTLINE),
        "navigation-outline" => Some(icons::NAVIGATION_OUTLINE),
        "water-plus" => Some(icons::WATER_PLUS),
        "magnify-remove-cursor" => Some(icons::MAGNIFY_REMOVE_CURSOR),
        "camera-image" => Some(icons::CAMERA_IMAGE),
        "hvac-off" => Some(icons::HVAC_OFF),
        "diving-scuba-mask" => Some(icons::DIVING_SCUBA_MASK),
        "hand-peace-variant" => Some(icons::HAND_PEACE_VARIANT),
        "tank" => Some(icons::TANK),
        "engine" => Some(icons::ENGINE),
        "cellphone-arrow-down" => Some(icons::CELLPHONE_ARROW_DOWN),
        "inbox-multiple-outline" => Some(icons::INBOX_MULTIPLE_OUTLINE),
        "lightbulb-on-outline" => Some(icons::LIGHTBULB_ON_OUTLINE),
        "ssh" => Some(icons::SSH),
        "vector-polyline" => Some(icons::VECTOR_POLYLINE),
        "progress-helper" => Some(icons::PROGRESS_HELPER),
        "folder-cog" => Some(icons::FOLDER_COG),
        "file-cloud" => Some(icons::FILE_CLOUD),
        "menu-open" => Some(icons::MENU_OPEN),
        "hand-wash-outline" => Some(icons::HAND_WASH_OUTLINE),
        "moped-electric" => Some(icons::MOPED_ELECTRIC),
        "home-alert" => Some(icons::HOME_ALERT),
        "heart-half" => Some(icons::HEART_HALF),
        "refresh-auto" => Some(icons::REFRESH_AUTO),
        "projector-screen-outline" => Some(icons::PROJECTOR_SCREEN_OUTLINE),
        "waves-arrow-left" => Some(icons::WAVES_ARROW_LEFT),
        "arrow-down-bold" => Some(icons::ARROW_DOWN_BOLD),
        "barley" => Some(icons::BARLEY),
        "wallpaper" => Some(icons::WALLPAPER),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "gamepad-round-up" => Some(icons::GAMEPAD_ROUND_UP),
        "mailbox-up-outline" => Some(icons::MAILBOX_UP_OUTLINE),
        "rss" => Some(icons::RSS),
        "chart-pie-outline" => Some(icons::CHART_PIE_OUTLINE),
        "hexagon-multiple" => Some(icons::HEXAGON_MULTIPLE),
        "bookmark-minus" => Some(icons::BOOKMARK_MINUS),
        "boomerang" => Some(icons::BOOMERANG),
        "jeepney" => Some(icons::JEEPNEY),
        "church" => Some(icons::CHURCH),
        "speedometer" => Some(icons::SPEEDOMETER),
        "battery-charging-wireless-50" => Some(icons::BATTERY_CHARGING_WIRELESS_50),
        "circle-multiple" => Some(icons::CIRCLE_MULTIPLE),
        "account-eye-outline" => Some(icons::ACCOUNT_EYE_OUTLINE),
        "text-box-edit-outline" => Some(icons::TEXT_BOX_EDIT_OUTLINE),
        "reproduction" => Some(icons::REPRODUCTION),
        "car-hatchback" => Some(icons::CAR_HATCHBACK),
        "sd" => Some(icons::SD),
        "train" => Some(icons::TRAIN),
        "cursor-default" => Some(icons::CURSOR_DEFAULT),
        "security" => Some(icons::SECURITY),
        #[allow(deprecated)]
        "jira" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jira' is deprecated.").print(py);
            }
            Some(icons::JIRA)
        }
        "link-circle" => Some(icons::LINK_CIRCLE),
        "file-upload-outline" => Some(icons::FILE_UPLOAD_OUTLINE),
        "sticker-outline" => Some(icons::STICKER_OUTLINE),
        "archive-sync" => Some(icons::ARCHIVE_SYNC),
        "comment-plus" => Some(icons::COMMENT_PLUS),
        "phone-in-talk-outline" => Some(icons::PHONE_IN_TALK_OUTLINE),
        "airplane-edit" => Some(icons::AIRPLANE_EDIT),
        "generator-mobile" => Some(icons::GENERATOR_MOBILE),
        "briefcase-variant-off" => Some(icons::BRIEFCASE_VARIANT_OFF),
        "account-network" => Some(icons::ACCOUNT_NETWORK),
        "clipboard-minus" => Some(icons::CLIPBOARD_MINUS),
        "checkbox-intermediate-variant" => Some(icons::CHECKBOX_INTERMEDIATE_VARIANT),
        "pencil-ruler-outline" => Some(icons::PENCIL_RULER_OUTLINE),
        "opacity" => Some(icons::OPACITY),
        "sun-clock" => Some(icons::SUN_CLOCK),
        "notebook-heart-outline" => Some(icons::NOTEBOOK_HEART_OUTLINE),
        "seat" => Some(icons::SEAT),
        "close-octagon-outline" => Some(icons::CLOSE_OCTAGON_OUTLINE),
        "view-grid-plus" => Some(icons::VIEW_GRID_PLUS),
        "heart-half-outline" => Some(icons::HEART_HALF_OUTLINE),
        "account-plus-outline" => Some(icons::ACCOUNT_PLUS_OUTLINE),
        "leaf-circle-outline" => Some(icons::LEAF_CIRCLE_OUTLINE),
        "map-marker-check" => Some(icons::MAP_MARKER_CHECK),
        "unicycle" => Some(icons::UNICYCLE),
        "help-circle-outline" => Some(icons::HELP_CIRCLE_OUTLINE),
        "cellphone-screenshot" => Some(icons::CELLPHONE_SCREENSHOT),
        "numeric-6-box" => Some(icons::NUMERIC_6_BOX),
        "hand-pointing-down" => Some(icons::HAND_POINTING_DOWN),
        "vector-arrange-below" => Some(icons::VECTOR_ARRANGE_BELOW),
        "timeline-minus" => Some(icons::TIMELINE_MINUS),
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
        #[allow(deprecated)]
        "creative-commons" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'creative-commons' is deprecated.")
                    .print(py);
            }
            Some(icons::CREATIVE_COMMONS)
        }
        "timer-music-outline" => Some(icons::TIMER_MUSIC_OUTLINE),
        "gold" => Some(icons::GOLD),
        "numeric-10-box-multiple-outline" => Some(icons::NUMERIC_10_BOX_MULTIPLE_OUTLINE),
        "seat-flat" => Some(icons::SEAT_FLAT),
        "window-closed-variant" => Some(icons::WINDOW_CLOSED_VARIANT),
        "alpha-r-circle" => Some(icons::ALPHA_R_CIRCLE),
        "volume-equal" => Some(icons::VOLUME_EQUAL),
        "star-off-outline" => Some(icons::STAR_OFF_OUTLINE),
        "cog-box" => Some(icons::COG_BOX),
        "package-variant-closed-minus" => Some(icons::PACKAGE_VARIANT_CLOSED_MINUS),
        "star-shooting-outline" => Some(icons::STAR_SHOOTING_OUTLINE),
        "phone-remove-outline" => Some(icons::PHONE_REMOVE_OUTLINE),
        _ => None,
    }
}
