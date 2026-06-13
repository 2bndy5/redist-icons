// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_8(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "comment-question-outline" => Some(icons::COMMENT_QUESTION_OUTLINE),
        "clock-time-eleven" => Some(icons::CLOCK_TIME_ELEVEN),
        "printer-pos-play" => Some(icons::PRINTER_POS_PLAY),
        "source-commit-start" => Some(icons::SOURCE_COMMIT_START),
        "account-search-outline" => Some(icons::ACCOUNT_SEARCH_OUTLINE),
        "dots-horizontal-circle-outline" => Some(icons::DOTS_HORIZONTAL_CIRCLE_OUTLINE),
        "arrow-left-bold-hexagon-outline" => Some(icons::ARROW_LEFT_BOLD_HEXAGON_OUTLINE),
        "account-lock-outline" => Some(icons::ACCOUNT_LOCK_OUTLINE),
        "credit-card-multiple" => Some(icons::CREDIT_CARD_MULTIPLE),
        "gradient-vertical" => Some(icons::GRADIENT_VERTICAL),
        "basket-outline" => Some(icons::BASKET_OUTLINE),
        "currency-uah" => Some(icons::CURRENCY_UAH),
        "ev-plug-ccs1" => Some(icons::EV_PLUG_CCS1),
        "lightbulb-spot" => Some(icons::LIGHTBULB_SPOT),
        "archive-marker" => Some(icons::ARCHIVE_MARKER),
        "cog-stop-outline" => Some(icons::COG_STOP_OUTLINE),
        "map-marker-circle" => Some(icons::MAP_MARKER_CIRCLE),
        "movie-edit" => Some(icons::MOVIE_EDIT),
        "case-sensitive-alt" => Some(icons::CASE_SENSITIVE_ALT),
        #[allow(deprecated)]
        "zend" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'zend' is deprecated.").print(py);
            }
            Some(icons::ZEND)
        }
        "arrow-right-bottom-bold" => Some(icons::ARROW_RIGHT_BOTTOM_BOLD),
        "redo" => Some(icons::REDO),
        "triangle-small-up" => Some(icons::TRIANGLE_SMALL_UP),
        "piston" => Some(icons::PISTON),
        "fruit-watermelon" => Some(icons::FRUIT_WATERMELON),
        "eject-circle" => Some(icons::EJECT_CIRCLE),
        "account-child-circle" => Some(icons::ACCOUNT_CHILD_CIRCLE),
        "minus-network-outline" => Some(icons::MINUS_NETWORK_OUTLINE),
        "border-outside" => Some(icons::BORDER_OUTSIDE),
        "record-circle" => Some(icons::RECORD_CIRCLE),
        "book-heart" => Some(icons::BOOK_HEART),
        "island" => Some(icons::ISLAND),
        "arrow-right-box" => Some(icons::ARROW_RIGHT_BOX),
        "pig-variant-outline" => Some(icons::PIG_VARIANT_OUTLINE),
        "clock" => Some(icons::CLOCK),
        "application-array-outline" => Some(icons::APPLICATION_ARRAY_OUTLINE),
        "television-guide" => Some(icons::TELEVISION_GUIDE),
        "kettle-alert-outline" => Some(icons::KETTLE_ALERT_OUTLINE),
        "account-music" => Some(icons::ACCOUNT_MUSIC),
        "order-numeric-ascending" => Some(icons::ORDER_NUMERIC_ASCENDING),
        "human-male-board-poll" => Some(icons::HUMAN_MALE_BOARD_POLL),
        "database-sync" => Some(icons::DATABASE_SYNC),
        "roller-shade" => Some(icons::ROLLER_SHADE),
        "clipboard-play" => Some(icons::CLIPBOARD_PLAY),
        "cake-variant-outline" => Some(icons::CAKE_VARIANT_OUTLINE),
        "emoticon-cry" => Some(icons::EMOTICON_CRY),
        "battery-unknown" => Some(icons::BATTERY_UNKNOWN),
        "notebook-remove" => Some(icons::NOTEBOOK_REMOVE),
        "arrange-bring-forward" => Some(icons::ARRANGE_BRING_FORWARD),
        "sign-direction-minus" => Some(icons::SIGN_DIRECTION_MINUS),
        "database" => Some(icons::DATABASE),
        "checkbook" => Some(icons::CHECKBOOK),
        "star-cog" => Some(icons::STAR_COG),
        "account-eye" => Some(icons::ACCOUNT_EYE),
        "snowflake-thermometer" => Some(icons::SNOWFLAKE_THERMOMETER),
        "basket-unfill" => Some(icons::BASKET_UNFILL),
        "folder-refresh" => Some(icons::FOLDER_REFRESH),
        "relation-zero-or-many-to-one-or-many" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONE_OR_MANY),
        "glass-mug-variant-off" => Some(icons::GLASS_MUG_VARIANT_OFF),
        "chart-donut-variant" => Some(icons::CHART_DONUT_VARIANT),
        "gauge-empty" => Some(icons::GAUGE_EMPTY),
        "arrow-u-up-right" => Some(icons::ARROW_U_UP_RIGHT),
        "close-octagon" => Some(icons::CLOSE_OCTAGON),
        "archive-lock-open" => Some(icons::ARCHIVE_LOCK_OPEN),
        "label-multiple" => Some(icons::LABEL_MULTIPLE),
        "arrow-right-thin-circle-outline" => Some(icons::ARROW_RIGHT_THIN_CIRCLE_OUTLINE),
        "chili-alert" => Some(icons::CHILI_ALERT),
        "blood-bag" => Some(icons::BLOOD_BAG),
        "format-letter-case" => Some(icons::FORMAT_LETTER_CASE),
        "thermometer-chevron-up" => Some(icons::THERMOMETER_CHEVRON_UP),
        "bookmark-check-outline" => Some(icons::BOOKMARK_CHECK_OUTLINE),
        "priority-low" => Some(icons::PRIORITY_LOW),
        "lock-percent-open" => Some(icons::LOCK_PERCENT_OPEN),
        "seesaw" => Some(icons::SEESAW),
        "smart-card-outline" => Some(icons::SMART_CARD_OUTLINE),
        "table-settings" => Some(icons::TABLE_SETTINGS),
        "weather-moonset-up" => Some(icons::WEATHER_MOONSET_UP),
        "cookie-remove-outline" => Some(icons::COOKIE_REMOVE_OUTLINE),
        "bolt" => Some(icons::BOLT),
        "circle-outline" => Some(icons::CIRCLE_OUTLINE),
        "drama-masks" => Some(icons::DRAMA_MASKS),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        "alpha-l" => Some(icons::ALPHA_L),
        "table-plus" => Some(icons::TABLE_PLUS),
        "movie-open-minus-outline" => Some(icons::MOVIE_OPEN_MINUS_OUTLINE),
        "cup-water" => Some(icons::CUP_WATER),
        "alpha-t-box-outline" => Some(icons::ALPHA_T_BOX_OUTLINE),
        "food-croissant" => Some(icons::FOOD_CROISSANT),
        "flag-variant-remove-outline" => Some(icons::FLAG_VARIANT_REMOVE_OUTLINE),
        "bed-king-outline" => Some(icons::BED_KING_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-view" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-view' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_VIEW)
        }
        "folder-open" => Some(icons::FOLDER_OPEN),
        "scatter-plot-outline" => Some(icons::SCATTER_PLOT_OUTLINE),
        "hand-wave-outline" => Some(icons::HAND_WAVE_OUTLINE),
        "axis-y-rotate-clockwise" => Some(icons::AXIS_Y_ROTATE_CLOCKWISE),
        "eye-lock" => Some(icons::EYE_LOCK),
        "alpha-l-circle" => Some(icons::ALPHA_L_CIRCLE),
        #[allow(deprecated)]
        "language-rust" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-rust' is deprecated.").print(py);
            }
            Some(icons::LANGUAGE_RUST)
        }
        "package-variant-minus" => Some(icons::PACKAGE_VARIANT_MINUS),
        "format-vertical-align-center" => Some(icons::FORMAT_VERTICAL_ALIGN_CENTER),
        "pencil-plus-outline" => Some(icons::PENCIL_PLUS_OUTLINE),
        "clipboard-text-clock" => Some(icons::CLIPBOARD_TEXT_CLOCK),
        "numeric" => Some(icons::NUMERIC),
        "clipboard-play-multiple" => Some(icons::CLIPBOARD_PLAY_MULTIPLE),
        "comma-circle-outline" => Some(icons::COMMA_CIRCLE_OUTLINE),
        "chevron-triple-right" => Some(icons::CHEVRON_TRIPLE_RIGHT),
        "summit" => Some(icons::SUMMIT),
        "head-flash" => Some(icons::HEAD_FLASH),
        "smart-card-reader" => Some(icons::SMART_CARD_READER),
        "ticket-confirmation" => Some(icons::TICKET_CONFIRMATION),
        "land-fields" => Some(icons::LAND_FIELDS),
        "motorbike-electric" => Some(icons::MOTORBIKE_ELECTRIC),
        "help" => Some(icons::HELP),
        "arrow-down-drop-circle-outline" => Some(icons::ARROW_DOWN_DROP_CIRCLE_OUTLINE),
        "diameter" => Some(icons::DIAMETER),
        "puzzle-minus" => Some(icons::PUZZLE_MINUS),
        "human-male-board" => Some(icons::HUMAN_MALE_BOARD),
        "package-variant-plus" => Some(icons::PACKAGE_VARIANT_PLUS),
        "delete-alert-outline" => Some(icons::DELETE_ALERT_OUTLINE),
        "calendar-lock-open" => Some(icons::CALENDAR_LOCK_OPEN),
        "cart-arrow-up" => Some(icons::CART_ARROW_UP),
        "fire-hydrant-off" => Some(icons::FIRE_HYDRANT_OFF),
        "briefcase-upload-outline" => Some(icons::BRIEFCASE_UPLOAD_OUTLINE),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        "puzzle-plus" => Some(icons::PUZZLE_PLUS),
        "glass-mug" => Some(icons::GLASS_MUG),
        #[allow(deprecated)]
        "nativescript" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nativescript' is deprecated.").print(py);
            }
            Some(icons::NATIVESCRIPT)
        }
        "wrench-cog" => Some(icons::WRENCH_COG),
        "usb" => Some(icons::USB),
        "folder-star-multiple-outline" => Some(icons::FOLDER_STAR_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "vuetify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'vuetify' is deprecated.").print(py);
            }
            Some(icons::VUETIFY)
        }
        "format-header-4" => Some(icons::FORMAT_HEADER_4),
        "circle-half" => Some(icons::CIRCLE_HALF),
        "abacus" => Some(icons::ABACUS),
        "razor-double-edge" => Some(icons::RAZOR_DOUBLE_EDGE),
        "triangle-outline" => Some(icons::TRIANGLE_OUTLINE),
        "movie" => Some(icons::MOVIE),
        "temple-hindu" => Some(icons::TEMPLE_HINDU),
        "file-rotate-left" => Some(icons::FILE_ROTATE_LEFT),
        "cloud-percent-outline" => Some(icons::CLOUD_PERCENT_OUTLINE),
        "comment-quote" => Some(icons::COMMENT_QUOTE),
        "crosshairs-gps" => Some(icons::CROSSHAIRS_GPS),
        "view-dashboard-outline" => Some(icons::VIEW_DASHBOARD_OUTLINE),
        "alpha-k" => Some(icons::ALPHA_K),
        "dots-grid" => Some(icons::DOTS_GRID),
        "comment-search" => Some(icons::COMMENT_SEARCH),
        "crowd" => Some(icons::CROWD),
        "lamp" => Some(icons::LAMP),
        "home-thermometer" => Some(icons::HOME_THERMOMETER),
        "spellcheck" => Some(icons::SPELLCHECK),
        "calendar-lock-open-outline" => Some(icons::CALENDAR_LOCK_OPEN_OUTLINE),
        "timer-refresh-outline" => Some(icons::TIMER_REFRESH_OUTLINE),
        "message-plus-outline" => Some(icons::MESSAGE_PLUS_OUTLINE),
        "ticket-account" => Some(icons::TICKET_ACCOUNT),
        "store-marker" => Some(icons::STORE_MARKER),
        "compass-rose" => Some(icons::COMPASS_ROSE),
        "color-helper" => Some(icons::COLOR_HELPER),
        "atm" => Some(icons::ATM),
        "sun-wireless-outline" => Some(icons::SUN_WIRELESS_OUTLINE),
        "shield-key" => Some(icons::SHIELD_KEY),
        "skull-scan-outline" => Some(icons::SKULL_SCAN_OUTLINE),
        "block-helper" => Some(icons::BLOCK_HELPER),
        "axis-y-arrow-lock" => Some(icons::AXIS_Y_ARROW_LOCK),
        "dice-d6" => Some(icons::DICE_D6),
        "account-off" => Some(icons::ACCOUNT_OFF),
        "account-supervisor" => Some(icons::ACCOUNT_SUPERVISOR),
        "menu-down-outline" => Some(icons::MENU_DOWN_OUTLINE),
        "briefcase-account-outline" => Some(icons::BRIEFCASE_ACCOUNT_OUTLINE),
        "folder-arrow-left-right-outline" => Some(icons::FOLDER_ARROW_LEFT_RIGHT_OUTLINE),
        "octagram-outline" => Some(icons::OCTAGRAM_OUTLINE),
        "music-note-sixteenth" => Some(icons::MUSIC_NOTE_SIXTEENTH),
        "phone-alert-outline" => Some(icons::PHONE_ALERT_OUTLINE),
        "alert-plus-outline" => Some(icons::ALERT_PLUS_OUTLINE),
        "format-horizontal-align-right" => Some(icons::FORMAT_HORIZONTAL_ALIGN_RIGHT),
        "play-box" => Some(icons::PLAY_BOX),
        "human-baby-changing-table" => Some(icons::HUMAN_BABY_CHANGING_TABLE),
        "account-box-outline" => Some(icons::ACCOUNT_BOX_OUTLINE),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "battery-charging-wireless-20" => Some(icons::BATTERY_CHARGING_WIRELESS_20),
        "seat-legroom-normal" => Some(icons::SEAT_LEGROOM_NORMAL),
        "flash-auto" => Some(icons::FLASH_AUTO),
        "badge-account-alert" => Some(icons::BADGE_ACCOUNT_ALERT),
        "emoticon-remove" => Some(icons::EMOTICON_REMOVE),
        "sticker-minus-outline" => Some(icons::STICKER_MINUS_OUTLINE),
        "chevron-right-box" => Some(icons::CHEVRON_RIGHT_BOX),
        "shield-plus-outline" => Some(icons::SHIELD_PLUS_OUTLINE),
        "comma-box" => Some(icons::COMMA_BOX),
        "information-outline" => Some(icons::INFORMATION_OUTLINE),
        "car-esp" => Some(icons::CAR_ESP),
        "basket" => Some(icons::BASKET),
        "subway" => Some(icons::SUBWAY),
        "clipboard-outline" => Some(icons::CLIPBOARD_OUTLINE),
        "fridge-industrial" => Some(icons::FRIDGE_INDUSTRIAL),
        "cart-plus" => Some(icons::CART_PLUS),
        "account-outline" => Some(icons::ACCOUNT_OUTLINE),
        "eye-plus" => Some(icons::EYE_PLUS),
        "sticker" => Some(icons::STICKER),
        "chart-line" => Some(icons::CHART_LINE),
        "bell-cog" => Some(icons::BELL_COG),
        "chart-box-multiple" => Some(icons::CHART_BOX_MULTIPLE),
        _ => None,
    }
}
