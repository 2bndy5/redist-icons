// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_27(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "vhs" => Some(icons::VHS),
        "alpha-f-box-outline" => Some(icons::ALPHA_F_BOX_OUTLINE),
        "arrow-left-right-bold-outline" => Some(icons::ARROW_LEFT_RIGHT_BOLD_OUTLINE),
        "delete-clock-outline" => Some(icons::DELETE_CLOCK_OUTLINE),
        "alpha-d-circle" => Some(icons::ALPHA_D_CIRCLE),
        "send-variant-outline" => Some(icons::SEND_VARIANT_OUTLINE),
        "arrow-bottom-left-bold-outline" => Some(icons::ARROW_BOTTOM_LEFT_BOLD_OUTLINE),
        #[allow(deprecated)]
        "apple" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple' is deprecated.").print(py);
            }
            Some(icons::APPLE)
        }
        "lock-open" => Some(icons::LOCK_OPEN),
        "arrow-up-circle" => Some(icons::ARROW_UP_CIRCLE),
        "washing-machine" => Some(icons::WASHING_MACHINE),
        "bell-cancel-outline" => Some(icons::BELL_CANCEL_OUTLINE),
        "ev-plug-tesla" => Some(icons::EV_PLUG_TESLA),
        "gymnastics" => Some(icons::GYMNASTICS),
        "bow-tie" => Some(icons::BOW_TIE),
        "note-plus-outline" => Some(icons::NOTE_PLUS_OUTLINE),
        "sprinkler" => Some(icons::SPRINKLER),
        "music-off" => Some(icons::MUSIC_OFF),
        "map" => Some(icons::MAP),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "led-strip-variant" => Some(icons::LED_STRIP_VARIANT),
        "power-socket-eu" => Some(icons::POWER_SOCKET_EU),
        "vector-polyline" => Some(icons::VECTOR_POLYLINE),
        "webhook" => Some(icons::WEBHOOK),
        "spotlight" => Some(icons::SPOTLIGHT),
        "lightbulb-off" => Some(icons::LIGHTBULB_OFF),
        "table-row-remove" => Some(icons::TABLE_ROW_REMOVE),
        "newspaper-plus" => Some(icons::NEWSPAPER_PLUS),
        "database-check" => Some(icons::DATABASE_CHECK),
        "monitor-eye" => Some(icons::MONITOR_EYE),
        "pencil-lock-outline" => Some(icons::PENCIL_LOCK_OUTLINE),
        "currency-php" => Some(icons::CURRENCY_PHP),
        "playlist-plus" => Some(icons::PLAYLIST_PLUS),
        "dock-left" => Some(icons::DOCK_LEFT),
        "seat-flat-angled" => Some(icons::SEAT_FLAT_ANGLED),
        "toothbrush-paste" => Some(icons::TOOTHBRUSH_PASTE),
        "invoice-import" => Some(icons::INVOICE_IMPORT),
        "roman-numeral-6" => Some(icons::ROMAN_NUMERAL_6),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "floor-lamp-torchiere-variant-outline" => Some(icons::FLOOR_LAMP_TORCHIERE_VARIANT_OUTLINE),
        "comment-text" => Some(icons::COMMENT_TEXT),
        "home-floor-3" => Some(icons::HOME_FLOOR_3),
        "contrast-circle" => Some(icons::CONTRAST_CIRCLE),
        "information-symbol" => Some(icons::INFORMATION_SYMBOL),
        "bookmark-music" => Some(icons::BOOKMARK_MUSIC),
        "page-layout-header" => Some(icons::PAGE_LAYOUT_HEADER),
        "gender-non-binary" => Some(icons::GENDER_NON_BINARY),
        "arrow-right-bottom" => Some(icons::ARROW_RIGHT_BOTTOM),
        "format-wrap-top-bottom" => Some(icons::FORMAT_WRAP_TOP_BOTTOM),
        "phone-outgoing" => Some(icons::PHONE_OUTGOING),
        "file-alert-outline" => Some(icons::FILE_ALERT_OUTLINE),
        "sleep" => Some(icons::SLEEP),
        #[allow(deprecated)]
        "font-awesome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'font-awesome' is deprecated.").print(py);
            }
            Some(icons::FONT_AWESOME)
        }
        "arrow-vertical-lock" => Some(icons::ARROW_VERTICAL_LOCK),
        "circle-slice-7" => Some(icons::CIRCLE_SLICE_7),
        "book-refresh" => Some(icons::BOOK_REFRESH),
        "book-arrow-left-outline" => Some(icons::BOOK_ARROW_LEFT_OUTLINE),
        "baby-carriage-off" => Some(icons::BABY_CARRIAGE_OFF),
        "head-heart-outline" => Some(icons::HEAD_HEART_OUTLINE),
        "cards-heart-outline" => Some(icons::CARDS_HEART_OUTLINE),
        "floor-plan" => Some(icons::FLOOR_PLAN),
        "collage" => Some(icons::COLLAGE),
        "table-pivot" => Some(icons::TABLE_PIVOT),
        #[allow(deprecated)]
        "microsoft-windows-classic" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-windows-classic' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_WINDOWS_CLASSIC)
        }
        "arrow-top-right-bold-outline" => Some(icons::ARROW_TOP_RIGHT_BOLD_OUTLINE),
        "khanda" => Some(icons::KHANDA),
        "chevron-up" => Some(icons::CHEVRON_UP),
        "hydro-power" => Some(icons::HYDRO_POWER),
        "compare-vertical" => Some(icons::COMPARE_VERTICAL),
        "filter-multiple-outline" => Some(icons::FILTER_MULTIPLE_OUTLINE),
        "alpha-n-box-outline" => Some(icons::ALPHA_N_BOX_OUTLINE),
        "text-box-check-outline" => Some(icons::TEXT_BOX_CHECK_OUTLINE),
        "content-save-cog" => Some(icons::CONTENT_SAVE_COG),
        "cube" => Some(icons::CUBE),
        "play-box-edit-outline" => Some(icons::PLAY_BOX_EDIT_OUTLINE),
        "video-input-component" => Some(icons::VIDEO_INPUT_COMPONENT),
        "account-key-outline" => Some(icons::ACCOUNT_KEY_OUTLINE),
        "battery-charging-low" => Some(icons::BATTERY_CHARGING_LOW),
        "cards-playing-diamond" => Some(icons::CARDS_PLAYING_DIAMOND),
        "resistor" => Some(icons::RESISTOR),
        "counter" => Some(icons::COUNTER),
        "alpha-y-circle-outline" => Some(icons::ALPHA_Y_CIRCLE_OUTLINE),
        "not-equal-variant" => Some(icons::NOT_EQUAL_VARIANT),
        "alpha-w-box" => Some(icons::ALPHA_W_BOX),
        "garage-variant" => Some(icons::GARAGE_VARIANT),
        "water-sync" => Some(icons::WATER_SYNC),
        "earth-box-plus" => Some(icons::EARTH_BOX_PLUS),
        "bucket" => Some(icons::BUCKET),
        "windsock" => Some(icons::WINDSOCK),
        "robot-vacuum" => Some(icons::ROBOT_VACUUM),
        "file-table" => Some(icons::FILE_TABLE),
        "water-thermometer" => Some(icons::WATER_THERMOMETER),
        "book-open-blank-variant-outline" => Some(icons::BOOK_OPEN_BLANK_VARIANT_OUTLINE),
        "comment-edit-outline" => Some(icons::COMMENT_EDIT_OUTLINE),
        "solar-power" => Some(icons::SOLAR_POWER),
        "ethernet" => Some(icons::ETHERNET),
        "arrow-top-left-thin-circle-outline" => Some(icons::ARROW_TOP_LEFT_THIN_CIRCLE_OUTLINE),
        "alert-circle-outline" => Some(icons::ALERT_CIRCLE_OUTLINE),
        "golf-tee" => Some(icons::GOLF_TEE),
        "cog-outline" => Some(icons::COG_OUTLINE),
        "arrow-bottom-right-bold-box" => Some(icons::ARROW_BOTTOM_RIGHT_BOLD_BOX),
        "ungroup" => Some(icons::UNGROUP),
        "account-injury" => Some(icons::ACCOUNT_INJURY),
        "go-kart-track" => Some(icons::GO_KART_TRACK),
        "image-filter-center-focus" => Some(icons::IMAGE_FILTER_CENTER_FOCUS),
        "numeric-8-box-multiple" => Some(icons::NUMERIC_8_BOX_MULTIPLE),
        "thumbs-up-down-outline" => Some(icons::THUMBS_UP_DOWN_OUTLINE),
        "gamepad-circle" => Some(icons::GAMEPAD_CIRCLE),
        "hammer" => Some(icons::HAMMER),
        "currency-ngn" => Some(icons::CURRENCY_NGN),
        "chess-knight" => Some(icons::CHESS_KNIGHT),
        "bike" => Some(icons::BIKE),
        "outdoor-lamp" => Some(icons::OUTDOOR_LAMP),
        "card-remove" => Some(icons::CARD_REMOVE),
        "bookmark-remove-outline" => Some(icons::BOOKMARK_REMOVE_OUTLINE),
        "lightbulb-variant" => Some(icons::LIGHTBULB_VARIANT),
        #[allow(deprecated)]
        "oci" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'oci' is deprecated.").print(py);
            }
            Some(icons::OCI)
        }
        "tag-minus-outline" => Some(icons::TAG_MINUS_OUTLINE),
        "format-font" => Some(icons::FORMAT_FONT),
        "email-fast" => Some(icons::EMAIL_FAST),
        "phone-in-talk-outline" => Some(icons::PHONE_IN_TALK_OUTLINE),
        #[allow(deprecated)]
        "odnoklassniki" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'odnoklassniki' is deprecated.").print(py);
            }
            Some(icons::ODNOKLASSNIKI)
        }
        "wifi-strength-lock-outline" => Some(icons::WIFI_STRENGTH_LOCK_OUTLINE),
        "movie-cog-outline" => Some(icons::MOVIE_COG_OUTLINE),
        "eye-arrow-left" => Some(icons::EYE_ARROW_LEFT),
        "truck-alert" => Some(icons::TRUCK_ALERT),
        "image-remove-outline" => Some(icons::IMAGE_REMOVE_OUTLINE),
        "baby-face-outline" => Some(icons::BABY_FACE_OUTLINE),
        "candy" => Some(icons::CANDY),
        "table-clock" => Some(icons::TABLE_CLOCK),
        "format-list-bulleted-triangle" => Some(icons::FORMAT_LIST_BULLETED_TRIANGLE),
        #[allow(deprecated)]
        "microsoft-teams" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-teams' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_TEAMS)
        }
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "fingerprint" => Some(icons::FINGERPRINT),
        "comment" => Some(icons::COMMENT),
        "bug-check-outline" => Some(icons::BUG_CHECK_OUTLINE),
        "passport-minus" => Some(icons::PASSPORT_MINUS),
        "mouse-right-click" => Some(icons::MOUSE_RIGHT_CLICK),
        "movie-plus" => Some(icons::MOVIE_PLUS),
        "clipboard-flow" => Some(icons::CLIPBOARD_FLOW),
        "standard-definition" => Some(icons::STANDARD_DEFINITION),
        "emoticon-wink" => Some(icons::EMOTICON_WINK),
        "table-column" => Some(icons::TABLE_COLUMN),
        "horizontal-rotate-clockwise" => Some(icons::HORIZONTAL_ROTATE_CLOCKWISE),
        "archive-refresh-outline" => Some(icons::ARCHIVE_REFRESH_OUTLINE),
        "school-outline" => Some(icons::SCHOOL_OUTLINE),
        "calendar-range-outline" => Some(icons::CALENDAR_RANGE_OUTLINE),
        "cloud-plus" => Some(icons::CLOUD_PLUS),
        "forwardburger" => Some(icons::FORWARDBURGER),
        "movie-outline" => Some(icons::MOVIE_OUTLINE),
        "content-save-settings" => Some(icons::CONTENT_SAVE_SETTINGS),
        "office-building-cog-outline" => Some(icons::OFFICE_BUILDING_COG_OUTLINE),
        "pipe-leak" => Some(icons::PIPE_LEAK),
        "inbox-multiple-outline" => Some(icons::INBOX_MULTIPLE_OUTLINE),
        "upload-off" => Some(icons::UPLOAD_OFF),
        "call-merge" => Some(icons::CALL_MERGE),
        "octahedron" => Some(icons::OCTAHEDRON),
        "brush" => Some(icons::BRUSH),
        "delete" => Some(icons::DELETE),
        "puzzle-star-outline" => Some(icons::PUZZLE_STAR_OUTLINE),
        "theme-light-dark" => Some(icons::THEME_LIGHT_DARK),
        "close-circle-multiple" => Some(icons::CLOSE_CIRCLE_MULTIPLE),
        "chart-gantt" => Some(icons::CHART_GANTT),
        "step-backward-2" => Some(icons::STEP_BACKWARD_2),
        "pan-right" => Some(icons::PAN_RIGHT),
        "sword-cross" => Some(icons::SWORD_CROSS),
        "octagram-edit-outline" => Some(icons::OCTAGRAM_EDIT_OUTLINE),
        "human-handsup" => Some(icons::HUMAN_HANDSUP),
        "arrow-right-circle" => Some(icons::ARROW_RIGHT_CIRCLE),
        "glasses" => Some(icons::GLASSES),
        "code-block-parentheses" => Some(icons::CODE_BLOCK_PARENTHESES),
        "led-outline" => Some(icons::LED_OUTLINE),
        "yeast" => Some(icons::YEAST),
        "string-lights" => Some(icons::STRING_LIGHTS),
        "source-branch-remove" => Some(icons::SOURCE_BRANCH_REMOVE),
        "sort-numeric-variant" => Some(icons::SORT_NUMERIC_VARIANT),
        "video-minus-outline" => Some(icons::VIDEO_MINUS_OUTLINE),
        "projector-screen-off" => Some(icons::PROJECTOR_SCREEN_OFF),
        "fountain" => Some(icons::FOUNTAIN),
        "volleyball" => Some(icons::VOLLEYBALL),
        "light-switch-off" => Some(icons::LIGHT_SWITCH_OFF),
        "folder-eye" => Some(icons::FOLDER_EYE),
        "email-box" => Some(icons::EMAIL_BOX),
        "knife" => Some(icons::KNIFE),
        "bug-pause-outline" => Some(icons::BUG_PAUSE_OUTLINE),
        "bank-check" => Some(icons::BANK_CHECK),
        "monitor-arrow-down" => Some(icons::MONITOR_ARROW_DOWN),
        "pencil-off-outline" => Some(icons::PENCIL_OFF_OUTLINE),
        "triangle-small-down" => Some(icons::TRIANGLE_SMALL_DOWN),
        "folder-multiple" => Some(icons::FOLDER_MULTIPLE),
        "update" => Some(icons::UPDATE),
        "rewind" => Some(icons::REWIND),
        "brush-off" => Some(icons::BRUSH_OFF),
        "car-coolant-level" => Some(icons::CAR_COOLANT_LEVEL),
        "movie-open-settings-outline" => Some(icons::MOVIE_OPEN_SETTINGS_OUTLINE),
        "mirror-variant" => Some(icons::MIRROR_VARIANT),
        "slide" => Some(icons::SLIDE),
        "podium-gold" => Some(icons::PODIUM_GOLD),
        "numeric-8" => Some(icons::NUMERIC_8),
        "floor-lamp-torchiere-outline" => Some(icons::FLOOR_LAMP_TORCHIERE_OUTLINE),
        _ => None,
    }
}
