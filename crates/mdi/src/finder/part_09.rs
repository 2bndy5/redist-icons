// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_9(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "circle-slice-8" => Some(icons::CIRCLE_SLICE_8),
        "tally-mark-1" => Some(icons::TALLY_MARK_1),
        "floppy-variant" => Some(icons::FLOPPY_VARIANT),
        "skip-forward-outline" => Some(icons::SKIP_FORWARD_OUTLINE),
        "dots-circle" => Some(icons::DOTS_CIRCLE),
        "information-slab-circle-outline" => Some(icons::INFORMATION_SLAB_CIRCLE_OUTLINE),
        "sort-calendar-descending" => Some(icons::SORT_CALENDAR_DESCENDING),
        "baby-face" => Some(icons::BABY_FACE),
        "brush-outline" => Some(icons::BRUSH_OUTLINE),
        "format-strikethrough" => Some(icons::FORMAT_STRIKETHROUGH),
        "tshirt-v" => Some(icons::TSHIRT_V),
        "calendar-today" => Some(icons::CALENDAR_TODAY),
        "crop-free" => Some(icons::CROP_FREE),
        "ladder" => Some(icons::LADDER),
        "golf-cart" => Some(icons::GOLF_CART),
        "rollerblade" => Some(icons::ROLLERBLADE),
        "bus-double-decker" => Some(icons::BUS_DOUBLE_DECKER),
        "solar-panel" => Some(icons::SOLAR_PANEL),
        "bulldozer" => Some(icons::BULLDOZER),
        "palette-outline" => Some(icons::PALETTE_OUTLINE),
        "led-outline" => Some(icons::LED_OUTLINE),
        "briefcase-eye-outline" => Some(icons::BRIEFCASE_EYE_OUTLINE),
        "cube" => Some(icons::CUBE),
        "cookie-remove" => Some(icons::COOKIE_REMOVE),
        "glass-tulip" => Some(icons::GLASS_TULIP),
        "clock-time-eleven" => Some(icons::CLOCK_TIME_ELEVEN),
        "pot-steam" => Some(icons::POT_STEAM),
        "minus-network" => Some(icons::MINUS_NETWORK),
        "phone-alert-outline" => Some(icons::PHONE_ALERT_OUTLINE),
        "pill-multiple" => Some(icons::PILL_MULTIPLE),
        "sign-direction-plus" => Some(icons::SIGN_DIRECTION_PLUS),
        "swap-vertical-variant" => Some(icons::SWAP_VERTICAL_VARIANT),
        "bow-tie" => Some(icons::BOW_TIE),
        "table-refresh" => Some(icons::TABLE_REFRESH),
        "tag-edit-outline" => Some(icons::TAG_EDIT_OUTLINE),
        "panorama-vertical-outline" => Some(icons::PANORAMA_VERTICAL_OUTLINE),
        "numeric-6-box-outline" => Some(icons::NUMERIC_6_BOX_OUTLINE),
        "play-box-lock-outline" => Some(icons::PLAY_BOX_LOCK_OUTLINE),
        #[allow(deprecated)]
        "microsoft-edge" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-edge' is deprecated.").print(py);
            }
            Some(icons::MICROSOFT_EDGE)
        }
        "toy-brick-remove" => Some(icons::TOY_BRICK_REMOVE),
        "format-paint" => Some(icons::FORMAT_PAINT),
        "skew-less" => Some(icons::SKEW_LESS),
        "network-off-outline" => Some(icons::NETWORK_OFF_OUTLINE),
        "folder-cancel-outline" => Some(icons::FOLDER_CANCEL_OUTLINE),
        "airplane-settings" => Some(icons::AIRPLANE_SETTINGS),
        "augmented-reality" => Some(icons::AUGMENTED_REALITY),
        "lock-alert" => Some(icons::LOCK_ALERT),
        "account-star" => Some(icons::ACCOUNT_STAR),
        "cloud-print-outline" => Some(icons::CLOUD_PRINT_OUTLINE),
        "bookmark-plus" => Some(icons::BOOKMARK_PLUS),
        "invoice" => Some(icons::INVOICE),
        "cross-bolnisi" => Some(icons::CROSS_BOLNISI),
        "star-off-outline" => Some(icons::STAR_OFF_OUTLINE),
        "refresh-circle" => Some(icons::REFRESH_CIRCLE),
        "storefront-edit-outline" => Some(icons::STOREFRONT_EDIT_OUTLINE),
        "format-letter-case-upper" => Some(icons::FORMAT_LETTER_CASE_UPPER),
        "account-lock-open" => Some(icons::ACCOUNT_LOCK_OPEN),
        "diving-scuba" => Some(icons::DIVING_SCUBA),
        "sim-alert" => Some(icons::SIM_ALERT),
        "application-braces" => Some(icons::APPLICATION_BRACES),
        "folder-key" => Some(icons::FOLDER_KEY),
        "percent-circle-outline" => Some(icons::PERCENT_CIRCLE_OUTLINE),
        "shield-cross" => Some(icons::SHIELD_CROSS),
        "receipt-text-check-outline" => Some(icons::RECEIPT_TEXT_CHECK_OUTLINE),
        "volume-high" => Some(icons::VOLUME_HIGH),
        "dance-pole" => Some(icons::DANCE_POLE),
        "cloud-print" => Some(icons::CLOUD_PRINT),
        "stool" => Some(icons::STOOL),
        "alpha-k-box-outline" => Some(icons::ALPHA_K_BOX_OUTLINE),
        "video-standard-definition" => Some(icons::VIDEO_STANDARD_DEFINITION),
        "receipt-text-outline" => Some(icons::RECEIPT_TEXT_OUTLINE),
        "collapse-all" => Some(icons::COLLAPSE_ALL),
        "bank-remove" => Some(icons::BANK_REMOVE),
        "star-three-points-outline" => Some(icons::STAR_THREE_POINTS_OUTLINE),
        "aurora" => Some(icons::AURORA),
        "temperature-kelvin" => Some(icons::TEMPERATURE_KELVIN),
        "alpha-v-box" => Some(icons::ALPHA_V_BOX),
        "arrow-right-top" => Some(icons::ARROW_RIGHT_TOP),
        "table-headers-eye-off" => Some(icons::TABLE_HEADERS_EYE_OFF),
        "shield-crown" => Some(icons::SHIELD_CROWN),
        "arrow-expand-up" => Some(icons::ARROW_EXPAND_UP),
        "web-box" => Some(icons::WEB_BOX),
        "eject-circle-outline" => Some(icons::EJECT_CIRCLE_OUTLINE),
        "upload-off-outline" => Some(icons::UPLOAD_OFF_OUTLINE),
        "monitor-lock" => Some(icons::MONITOR_LOCK),
        "baby" => Some(icons::BABY),
        "folder-play" => Some(icons::FOLDER_PLAY),
        "airplane-landing" => Some(icons::AIRPLANE_LANDING),
        "water-boiler" => Some(icons::WATER_BOILER),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "surround-sound-5-1-2" => Some(icons::SURROUND_SOUND_5_1_2),
        "send-clock-outline" => Some(icons::SEND_CLOCK_OUTLINE),
        #[allow(deprecated)]
        "font-awesome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'font-awesome' is deprecated.").print(py);
            }
            Some(icons::FONT_AWESOME)
        }
        "format-list-group-plus" => Some(icons::FORMAT_LIST_GROUP_PLUS),
        "select-color" => Some(icons::SELECT_COLOR),
        "format-list-checks" => Some(icons::FORMAT_LIST_CHECKS),
        "briefcase" => Some(icons::BRIEFCASE),
        "land-rows-vertical" => Some(icons::LAND_ROWS_VERTICAL),
        "plus-box-outline" => Some(icons::PLUS_BOX_OUTLINE),
        "cube-unfolded" => Some(icons::CUBE_UNFOLDED),
        "dock-top" => Some(icons::DOCK_TOP),
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
        "cash-lock" => Some(icons::CASH_LOCK),
        "hands-pray" => Some(icons::HANDS_PRAY),
        "octagon" => Some(icons::OCTAGON),
        "table-row" => Some(icons::TABLE_ROW),
        "car-door-lock-open" => Some(icons::CAR_DOOR_LOCK_OPEN),
        "comment-arrow-right-outline" => Some(icons::COMMENT_ARROW_RIGHT_OUTLINE),
        "turbine" => Some(icons::TURBINE),
        "numeric-7-box-multiple" => Some(icons::NUMERIC_7_BOX_MULTIPLE),
        "pause-box-outline" => Some(icons::PAUSE_BOX_OUTLINE),
        "vector-difference-ba" => Some(icons::VECTOR_DIFFERENCE_BA),
        "book-open-outline" => Some(icons::BOOK_OPEN_OUTLINE),
        "bug-check-outline" => Some(icons::BUG_CHECK_OUTLINE),
        "clock-edit-outline" => Some(icons::CLOCK_EDIT_OUTLINE),
        "truck-fast" => Some(icons::TRUCK_FAST),
        "heart-remove-outline" => Some(icons::HEART_REMOVE_OUTLINE),
        "square" => Some(icons::SQUARE),
        "selection-remove" => Some(icons::SELECTION_REMOVE),
        "emoticon-tongue" => Some(icons::EMOTICON_TONGUE),
        "table-edit" => Some(icons::TABLE_EDIT),
        "account-cowboy-hat-outline" => Some(icons::ACCOUNT_COWBOY_HAT_OUTLINE),
        "format-text-rotation-none" => Some(icons::FORMAT_TEXT_ROTATION_NONE),
        "alpha-w-box-outline" => Some(icons::ALPHA_W_BOX_OUTLINE),
        "currency-sign" => Some(icons::CURRENCY_SIGN),
        "dock-right" => Some(icons::DOCK_RIGHT),
        "fuse-blade" => Some(icons::FUSE_BLADE),
        "source-commit-start-next-local" => Some(icons::SOURCE_COMMIT_START_NEXT_LOCAL),
        "text-box-minus" => Some(icons::TEXT_BOX_MINUS),
        "theme-light-dark" => Some(icons::THEME_LIGHT_DARK),
        "cloud-minus" => Some(icons::CLOUD_MINUS),
        "usb-flash-drive" => Some(icons::USB_FLASH_DRIVE),
        "eyedropper-variant" => Some(icons::EYEDROPPER_VARIANT),
        "help-rhombus" => Some(icons::HELP_RHOMBUS),
        "filter-cog-outline" => Some(icons::FILTER_COG_OUTLINE),
        "calendar-multiselect" => Some(icons::CALENDAR_MULTISELECT),
        "lock-percent" => Some(icons::LOCK_PERCENT),
        "timer-music" => Some(icons::TIMER_MUSIC),
        "car-brake-abs" => Some(icons::CAR_BRAKE_ABS),
        "arrow-right-bold" => Some(icons::ARROW_RIGHT_BOLD),
        "egg-off-outline" => Some(icons::EGG_OFF_OUTLINE),
        "invoice-check-outline" => Some(icons::INVOICE_CHECK_OUTLINE),
        "fan-clock" => Some(icons::FAN_CLOCK),
        "help-box" => Some(icons::HELP_BOX),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "comment-plus" => Some(icons::COMMENT_PLUS),
        "oil-temperature" => Some(icons::OIL_TEMPERATURE),
        #[allow(deprecated)]
        "qi" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'qi' is deprecated.").print(py);
            }
            Some(icons::QI)
        }
        "wifi-strength-3-alert" => Some(icons::WIFI_STRENGTH_3_ALERT),
        "cloud-key" => Some(icons::CLOUD_KEY),
        "flask-empty" => Some(icons::FLASK_EMPTY),
        "chevron-double-right" => Some(icons::CHEVRON_DOUBLE_RIGHT),
        "table-key" => Some(icons::TABLE_KEY),
        "tooltip-remove" => Some(icons::TOOLTIP_REMOVE),
        "bike-pedal-clipless" => Some(icons::BIKE_PEDAL_CLIPLESS),
        "led-variant-on" => Some(icons::LED_VARIANT_ON),
        "flash-outline" => Some(icons::FLASH_OUTLINE),
        "source-commit" => Some(icons::SOURCE_COMMIT),
        "clipboard-text-clock" => Some(icons::CLIPBOARD_TEXT_CLOCK),
        "notebook-plus" => Some(icons::NOTEBOOK_PLUS),
        "pan-top-right" => Some(icons::PAN_TOP_RIGHT),
        "invoice-text-plus-outline" => Some(icons::INVOICE_TEXT_PLUS_OUTLINE),
        "robot-off-outline" => Some(icons::ROBOT_OFF_OUTLINE),
        "alpha-c-circle-outline" => Some(icons::ALPHA_C_CIRCLE_OUTLINE),
        "format-letter-starts-with" => Some(icons::FORMAT_LETTER_STARTS_WITH),
        "widgets-outline" => Some(icons::WIDGETS_OUTLINE),
        "power-socket-eu" => Some(icons::POWER_SOCKET_EU),
        "forum-remove-outline" => Some(icons::FORUM_REMOVE_OUTLINE),
        "bell-circle-outline" => Some(icons::BELL_CIRCLE_OUTLINE),
        "music-box-multiple" => Some(icons::MUSIC_BOX_MULTIPLE),
        "music-clef-treble" => Some(icons::MUSIC_CLEF_TREBLE),
        "folder-arrow-up-down" => Some(icons::FOLDER_ARROW_UP_DOWN),
        "gesture-swipe-down" => Some(icons::GESTURE_SWIPE_DOWN),
        "coffee-maker-check" => Some(icons::COFFEE_MAKER_CHECK),
        "information-slab-box" => Some(icons::INFORMATION_SLAB_BOX),
        #[allow(deprecated)]
        "whatsapp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'whatsapp' is deprecated.").print(py);
            }
            Some(icons::WHATSAPP)
        }
        "folder-swap" => Some(icons::FOLDER_SWAP),
        "chart-line-stacked" => Some(icons::CHART_LINE_STACKED),
        "upload-network-outline" => Some(icons::UPLOAD_NETWORK_OUTLINE),
        "math-norm" => Some(icons::MATH_NORM),
        "application-edit-outline" => Some(icons::APPLICATION_EDIT_OUTLINE),
        "bread-slice" => Some(icons::BREAD_SLICE),
        "bed-outline" => Some(icons::BED_OUTLINE),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "tournament" => Some(icons::TOURNAMENT),
        "sale" => Some(icons::SALE),
        "watermark" => Some(icons::WATERMARK),
        "file-table-box-outline" => Some(icons::FILE_TABLE_BOX_OUTLINE),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "battery-charging-low" => Some(icons::BATTERY_CHARGING_LOW),
        "image-edit-outline" => Some(icons::IMAGE_EDIT_OUTLINE),
        "clipboard-text-clock-outline" => Some(icons::CLIPBOARD_TEXT_CLOCK_OUTLINE),
        "database-alert" => Some(icons::DATABASE_ALERT),
        "pencil-circle-outline" => Some(icons::PENCIL_CIRCLE_OUTLINE),
        "invoice-text-clock" => Some(icons::INVOICE_TEXT_CLOCK),
        "cog-clockwise" => Some(icons::COG_CLOCKWISE),
        "ray-start-arrow" => Some(icons::RAY_START_ARROW),
        "glass-flute" => Some(icons::GLASS_FLUTE),
        "emoticon-excited" => Some(icons::EMOTICON_EXCITED),
        "fuse" => Some(icons::FUSE),
        _ => None,
    }
}
