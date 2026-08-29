// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_21(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "seed-off-outline" => Some(icons::SEED_OFF_OUTLINE),
        "calendar-multiselect-outline" => Some(icons::CALENDAR_MULTISELECT_OUTLINE),
        "arrow-right-bold-box-outline" => Some(icons::ARROW_RIGHT_BOLD_BOX_OUTLINE),
        "file-excel-box-outline" => Some(icons::FILE_EXCEL_BOX_OUTLINE),
        "help-circle-outline" => Some(icons::HELP_CIRCLE_OUTLINE),
        "home-clock-outline" => Some(icons::HOME_CLOCK_OUTLINE),
        "cards-playing-diamond-multiple" => Some(icons::CARDS_PLAYING_DIAMOND_MULTIPLE),
        "image-minus-outline" => Some(icons::IMAGE_MINUS_OUTLINE),
        "cloud-cancel-outline" => Some(icons::CLOUD_CANCEL_OUTLINE),
        "archive-cancel-outline" => Some(icons::ARCHIVE_CANCEL_OUTLINE),
        "ornament-variant" => Some(icons::ORNAMENT_VARIANT),
        "dice-4" => Some(icons::DICE_4),
        "decimal-increase" => Some(icons::DECIMAL_INCREASE),
        "information-slab-circle" => Some(icons::INFORMATION_SLAB_CIRCLE),
        "alpha-h-box" => Some(icons::ALPHA_H_BOX),
        "dip-switch" => Some(icons::DIP_SWITCH),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-empty" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-empty' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_EMPTY)
        }
        "folder-heart" => Some(icons::FOLDER_HEART),
        "archive-minus" => Some(icons::ARCHIVE_MINUS),
        "alpha-o-circle" => Some(icons::ALPHA_O_CIRCLE),
        "account-wrench" => Some(icons::ACCOUNT_WRENCH),
        "head-lightbulb" => Some(icons::HEAD_LIGHTBULB),
        "bottle-tonic-skull" => Some(icons::BOTTLE_TONIC_SKULL),
        "silverware-fork-knife" => Some(icons::SILVERWARE_FORK_KNIFE),
        "heart-minus" => Some(icons::HEART_MINUS),
        "train-car-box-full" => Some(icons::TRAIN_CAR_BOX_FULL),
        "download-off-outline" => Some(icons::DOWNLOAD_OFF_OUTLINE),
        "alpha-j-box" => Some(icons::ALPHA_J_BOX),
        "image-size-select-large" => Some(icons::IMAGE_SIZE_SELECT_LARGE),
        "wall-sconce-flat" => Some(icons::WALL_SCONCE_FLAT),
        "earth-box-minus" => Some(icons::EARTH_BOX_MINUS),
        "fast-forward-60" => Some(icons::FAST_FORWARD_60),
        "micro-sd" => Some(icons::MICRO_SD),
        "camera-off-outline" => Some(icons::CAMERA_OFF_OUTLINE),
        "wardrobe-outline" => Some(icons::WARDROBE_OUTLINE),
        "book-search" => Some(icons::BOOK_SEARCH),
        "music-note-whole" => Some(icons::MUSIC_NOTE_WHOLE),
        "keyboard-close-outline" => Some(icons::KEYBOARD_CLOSE_OUTLINE),
        "eye-minus" => Some(icons::EYE_MINUS),
        "cloud-sync" => Some(icons::CLOUD_SYNC),
        "lightbulb-cfl-spiral" => Some(icons::LIGHTBULB_CFL_SPIRAL),
        "ear-hearing-off" => Some(icons::EAR_HEARING_OFF),
        "food" => Some(icons::FOOD),
        "surround-sound-2-0" => Some(icons::SURROUND_SOUND_2_0),
        "filter" => Some(icons::FILTER),
        "alpha-l-circle" => Some(icons::ALPHA_L_CIRCLE),
        "tag-hidden" => Some(icons::TAG_HIDDEN),
        "table-column-plus-after" => Some(icons::TABLE_COLUMN_PLUS_AFTER),
        "food-off-outline" => Some(icons::FOOD_OFF_OUTLINE),
        "file-chart-outline" => Some(icons::FILE_CHART_OUTLINE),
        "washing-machine-off" => Some(icons::WASHING_MACHINE_OFF),
        "transition-masked" => Some(icons::TRANSITION_MASKED),
        "tag-arrow-down" => Some(icons::TAG_ARROW_DOWN),
        "alpha-t-circle-outline" => Some(icons::ALPHA_T_CIRCLE_OUTLINE),
        "filter-remove-outline" => Some(icons::FILTER_REMOVE_OUTLINE),
        "close-outline" => Some(icons::CLOSE_OUTLINE),
        "chart-areaspline" => Some(icons::CHART_AREASPLINE),
        "lightbulb-on" => Some(icons::LIGHTBULB_ON),
        "crown" => Some(icons::CROWN),
        "select-group" => Some(icons::SELECT_GROUP),
        "barcode-off" => Some(icons::BARCODE_OFF),
        "paw-off" => Some(icons::PAW_OFF),
        "snowflake-variant" => Some(icons::SNOWFLAKE_VARIANT),
        "home-outline" => Some(icons::HOME_OUTLINE),
        "book-off-outline" => Some(icons::BOOK_OFF_OUTLINE),
        "airplane-remove" => Some(icons::AIRPLANE_REMOVE),
        "alpha-s" => Some(icons::ALPHA_S),
        "timer-sync" => Some(icons::TIMER_SYNC),
        "gift-off-outline" => Some(icons::GIFT_OFF_OUTLINE),
        "archive-check" => Some(icons::ARCHIVE_CHECK),
        "keyboard-f12" => Some(icons::KEYBOARD_F12),
        "eye-off" => Some(icons::EYE_OFF),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "file-document-check" => Some(icons::FILE_DOCUMENT_CHECK),
        "content-save-cog" => Some(icons::CONTENT_SAVE_COG),
        "square-off-outline" => Some(icons::SQUARE_OFF_OUTLINE),
        "music-rest-sixteenth" => Some(icons::MUSIC_REST_SIXTEENTH),
        "vector-point-minus" => Some(icons::VECTOR_POINT_MINUS),
        "projector-off" => Some(icons::PROJECTOR_OFF),
        "minus" => Some(icons::MINUS),
        "bicycle" => Some(icons::BICYCLE),
        "record-rec" => Some(icons::RECORD_REC),
        "ribbon" => Some(icons::RIBBON),
        "update" => Some(icons::UPDATE),
        "code-parentheses-box" => Some(icons::CODE_PARENTHESES_BOX),
        "charity" => Some(icons::CHARITY),
        "equalizer" => Some(icons::EQUALIZER),
        "content-save-settings-outline" => Some(icons::CONTENT_SAVE_SETTINGS_OUTLINE),
        "door-sliding" => Some(icons::DOOR_SLIDING),
        "volume-minus" => Some(icons::VOLUME_MINUS),
        "flag-plus" => Some(icons::FLAG_PLUS),
        #[allow(deprecated)]
        "language-fortran" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-fortran' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_FORTRAN)
        }
        "tooltip-minus-outline" => Some(icons::TOOLTIP_MINUS_OUTLINE),
        "television-stop" => Some(icons::TELEVISION_STOP),
        "notebook-heart" => Some(icons::NOTEBOOK_HEART),
        "earth-box-plus" => Some(icons::EARTH_BOX_PLUS),
        "mouse-move-vertical" => Some(icons::MOUSE_MOVE_VERTICAL),
        "table-star" => Some(icons::TABLE_STAR),
        "lock-minus" => Some(icons::LOCK_MINUS),
        "pen-plus" => Some(icons::PEN_PLUS),
        "car-arrow-right" => Some(icons::CAR_ARROW_RIGHT),
        "printer-3d-nozzle-heat" => Some(icons::PRINTER_3D_NOZZLE_HEAT),
        "eraser-variant" => Some(icons::ERASER_VARIANT),
        "hand-pointing-up" => Some(icons::HAND_POINTING_UP),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "brightness-5" => Some(icons::BRIGHTNESS_5),
        "alpha-e-box" => Some(icons::ALPHA_E_BOX),
        "book-arrow-down" => Some(icons::BOOK_ARROW_DOWN),
        "arrow-left-bottom" => Some(icons::ARROW_LEFT_BOTTOM),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        "coffee-maker-check" => Some(icons::COFFEE_MAKER_CHECK),
        "timer-sand-paused" => Some(icons::TIMER_SAND_PAUSED),
        "cellphone-text" => Some(icons::CELLPHONE_TEXT),
        "alert-octagram" => Some(icons::ALERT_OCTAGRAM),
        "play-pause" => Some(icons::PLAY_PAUSE),
        "numeric-10-box" => Some(icons::NUMERIC_10_BOX),
        "checkbox-marked-circle-auto-outline" => Some(icons::CHECKBOX_MARKED_CIRCLE_AUTO_OUTLINE),
        "sd" => Some(icons::SD),
        "cat" => Some(icons::CAT),
        "advertisements" => Some(icons::ADVERTISEMENTS),
        "chili-off" => Some(icons::CHILI_OFF),
        "road" => Some(icons::ROAD),
        "chart-box-multiple" => Some(icons::CHART_BOX_MULTIPLE),
        "account-edit" => Some(icons::ACCOUNT_EDIT),
        "pipe-leak" => Some(icons::PIPE_LEAK),
        "file-clock-outline" => Some(icons::FILE_CLOCK_OUTLINE),
        "axis-arrow-lock" => Some(icons::AXIS_ARROW_LOCK),
        "key" => Some(icons::KEY),
        "beer-outline" => Some(icons::BEER_OUTLINE),
        "scatter-plot" => Some(icons::SCATTER_PLOT),
        "inbox-arrow-down-outline" => Some(icons::INBOX_ARROW_DOWN_OUTLINE),
        "delete-empty-outline" => Some(icons::DELETE_EMPTY_OUTLINE),
        "fax" => Some(icons::FAX),
        "credit-card-refund-outline" => Some(icons::CREDIT_CARD_REFUND_OUTLINE),
        "fan-speed-3" => Some(icons::FAN_SPEED_3),
        "horse-variant" => Some(icons::HORSE_VARIANT),
        "bed-king" => Some(icons::BED_KING),
        #[allow(deprecated)]
        "facebook-workplace" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-workplace' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_WORKPLACE)
        }
        "snowflake-off" => Some(icons::SNOWFLAKE_OFF),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "border-top-variant" => Some(icons::BORDER_TOP_VARIANT),
        "peanut-off-outline" => Some(icons::PEANUT_OFF_OUTLINE),
        "water-alert" => Some(icons::WATER_ALERT),
        "upload-multiple" => Some(icons::UPLOAD_MULTIPLE),
        "alpha-b-circle-outline" => Some(icons::ALPHA_B_CIRCLE_OUTLINE),
        "qrcode-remove" => Some(icons::QRCODE_REMOVE),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        "transit-connection-horizontal" => Some(icons::TRANSIT_CONNECTION_HORIZONTAL),
        "numeric-6-circle-outline" => Some(icons::NUMERIC_6_CIRCLE_OUTLINE),
        #[allow(deprecated)]
        "language-typescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-typescript' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_TYPESCRIPT)
        }
        "arrow-expand-up" => Some(icons::ARROW_EXPAND_UP),
        "nature" => Some(icons::NATURE),
        "timer-alert-outline" => Some(icons::TIMER_ALERT_OUTLINE),
        "qrcode-minus" => Some(icons::QRCODE_MINUS),
        "raw-off" => Some(icons::RAW_OFF),
        "chart-tree" => Some(icons::CHART_TREE),
        "car-2-plus" => Some(icons::CAR_2_PLUS),
        "human-walker" => Some(icons::HUMAN_WALKER),
        "space-invaders" => Some(icons::SPACE_INVADERS),
        "align-horizontal-center" => Some(icons::ALIGN_HORIZONTAL_CENTER),
        "file-sync" => Some(icons::FILE_SYNC),
        "airplane-takeoff" => Some(icons::AIRPLANE_TAKEOFF),
        "chart-multiline" => Some(icons::CHART_MULTILINE),
        "cloud-alert-outline" => Some(icons::CLOUD_ALERT_OUTLINE),
        "fit-to-page" => Some(icons::FIT_TO_PAGE),
        "arrow-right-drop-circle" => Some(icons::ARROW_RIGHT_DROP_CIRCLE),
        "invoice-plus" => Some(icons::INVOICE_PLUS),
        "shield-half-full" => Some(icons::SHIELD_HALF_FULL),
        "list-box-outline" => Some(icons::LIST_BOX_OUTLINE),
        "phone-plus" => Some(icons::PHONE_PLUS),
        "star-minus" => Some(icons::STAR_MINUS),
        "bell-cog" => Some(icons::BELL_COG),
        "ray-start-vertex-end" => Some(icons::RAY_START_VERTEX_END),
        "archive-settings-outline" => Some(icons::ARCHIVE_SETTINGS_OUTLINE),
        "razor-single-edge" => Some(icons::RAZOR_SINGLE_EDGE),
        "movie-search-outline" => Some(icons::MOVIE_SEARCH_OUTLINE),
        "arrow-right-circle-outline" => Some(icons::ARROW_RIGHT_CIRCLE_OUTLINE),
        "mailbox-open-up" => Some(icons::MAILBOX_OPEN_UP),
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "zodiac-capricorn" => Some(icons::ZODIAC_CAPRICORN),
        "lock-open-plus" => Some(icons::LOCK_OPEN_PLUS),
        "web-clock" => Some(icons::WEB_CLOCK),
        "clipboard-text-clock" => Some(icons::CLIPBOARD_TEXT_CLOCK),
        "repeat-off" => Some(icons::REPEAT_OFF),
        "relation-one-or-many-to-one" => Some(icons::RELATION_ONE_OR_MANY_TO_ONE),
        "archive-star" => Some(icons::ARCHIVE_STAR),
        "account-multiple" => Some(icons::ACCOUNT_MULTIPLE),
        "trending-neutral" => Some(icons::TRENDING_NEUTRAL),
        "file-rotate-right-outline" => Some(icons::FILE_ROTATE_RIGHT_OUTLINE),
        "email-remove-outline" => Some(icons::EMAIL_REMOVE_OUTLINE),
        "chart-line-variant" => Some(icons::CHART_LINE_VARIANT),
        "warehouse" => Some(icons::WAREHOUSE),
        "account-arrow-right" => Some(icons::ACCOUNT_ARROW_RIGHT),
        "flask" => Some(icons::FLASK),
        "fan-speed-2" => Some(icons::FAN_SPEED_2),
        "led-variant-outline" => Some(icons::LED_VARIANT_OUTLINE),
        #[allow(deprecated)]
        "youtube-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-gaming' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_GAMING)
        }
        "folder-alert" => Some(icons::FOLDER_ALERT),
        "clock-star-four-points-outline" => Some(icons::CLOCK_STAR_FOUR_POINTS_OUTLINE),
        "music-accidental-natural" => Some(icons::MUSIC_ACCIDENTAL_NATURAL),
        _ => None,
    }
}
