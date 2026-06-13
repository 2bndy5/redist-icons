// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_23(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "receipt-text-arrow-right" => Some(icons::RECEIPT_TEXT_ARROW_RIGHT),
        "spade" => Some(icons::SPADE),
        "shipping-pallet" => Some(icons::SHIPPING_PALLET),
        "car-side" => Some(icons::CAR_SIDE),
        #[allow(deprecated)]
        "gitlab" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gitlab' is deprecated.").print(py);
            }
            Some(icons::GITLAB)
        }
        "image-plus" => Some(icons::IMAGE_PLUS),
        "roman-numeral-5" => Some(icons::ROMAN_NUMERAL_5),
        "shoe-formal" => Some(icons::SHOE_FORMAL),
        "target" => Some(icons::TARGET),
        "gate-or" => Some(icons::GATE_OR),
        "warehouse" => Some(icons::WAREHOUSE),
        "water-polo" => Some(icons::WATER_POLO),
        "bank-plus" => Some(icons::BANK_PLUS),
        "numeric-1-box-multiple" => Some(icons::NUMERIC_1_BOX_MULTIPLE),
        "chess-pawn" => Some(icons::CHESS_PAWN),
        "thermometer-chevron-down" => Some(icons::THERMOMETER_CHEVRON_DOWN),
        "flask-off" => Some(icons::FLASK_OFF),
        "star-box-multiple-outline" => Some(icons::STAR_BOX_MULTIPLE_OUTLINE),
        "land-rows-vertical" => Some(icons::LAND_ROWS_VERTICAL),
        "account-card-outline" => Some(icons::ACCOUNT_CARD_OUTLINE),
        "generator-mobile" => Some(icons::GENERATOR_MOBILE),
        "chandelier" => Some(icons::CHANDELIER),
        "truck-flatbed" => Some(icons::TRUCK_FLATBED),
        #[allow(deprecated)]
        "google-ads" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-ads' is deprecated.").print(py);
            }
            Some(icons::GOOGLE_ADS)
        }
        "medical-cotton-swab" => Some(icons::MEDICAL_COTTON_SWAB),
        "toggle-switch" => Some(icons::TOGGLE_SWITCH),
        "dots-square" => Some(icons::DOTS_SQUARE),
        "cellphone-remove" => Some(icons::CELLPHONE_REMOVE),
        "file-delimited" => Some(icons::FILE_DELIMITED),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "thermometer-alert" => Some(icons::THERMOMETER_ALERT),
        "car-key" => Some(icons::CAR_KEY),
        "hair-dryer-outline" => Some(icons::HAIR_DRYER_OUTLINE),
        "check-decagram" => Some(icons::CHECK_DECAGRAM),
        #[allow(deprecated)]
        "redhat" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'redhat' is deprecated.").print(py);
            }
            Some(icons::REDHAT)
        }
        "file-powerpoint-outline" => Some(icons::FILE_POWERPOINT_OUTLINE),
        "battery-minus" => Some(icons::BATTERY_MINUS),
        "lightbulb-group" => Some(icons::LIGHTBULB_GROUP),
        "form-select" => Some(icons::FORM_SELECT),
        "format-title" => Some(icons::FORMAT_TITLE),
        "file-document-arrow-right" => Some(icons::FILE_DOCUMENT_ARROW_RIGHT),
        "hook-off" => Some(icons::HOOK_OFF),
        "format-wrap-inline" => Some(icons::FORMAT_WRAP_INLINE),
        "credit-card-scan" => Some(icons::CREDIT_CARD_SCAN),
        "iron-board" => Some(icons::IRON_BOARD),
        "keyboard-f8" => Some(icons::KEYBOARD_F8),
        "camera-party-mode" => Some(icons::CAMERA_PARTY_MODE),
        "speedometer-medium" => Some(icons::SPEEDOMETER_MEDIUM),
        "folder-account-outline" => Some(icons::FOLDER_ACCOUNT_OUTLINE),
        "battery-off" => Some(icons::BATTERY_OFF),
        #[allow(deprecated)]
        "spotify" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'spotify' is deprecated.").print(py);
            }
            Some(icons::SPOTIFY)
        }
        "toaster-oven" => Some(icons::TOASTER_OVEN),
        "ticket-percent-outline" => Some(icons::TICKET_PERCENT_OUTLINE),
        "tennis" => Some(icons::TENNIS),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "note-check" => Some(icons::NOTE_CHECK),
        "watermark" => Some(icons::WATERMARK),
        "ceiling-fan" => Some(icons::CEILING_FAN),
        "image-filter-vintage" => Some(icons::IMAGE_FILTER_VINTAGE),
        "pyramid-off" => Some(icons::PYRAMID_OFF),
        "circle-slice-1" => Some(icons::CIRCLE_SLICE_1),
        #[allow(deprecated)]
        "linkedin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linkedin' is deprecated.").print(py);
            }
            Some(icons::LINKEDIN)
        }
        "message-fast-outline" => Some(icons::MESSAGE_FAST_OUTLINE),
        "file-marker-outline" => Some(icons::FILE_MARKER_OUTLINE),
        #[allow(deprecated)]
        "stack-overflow" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'stack-overflow' is deprecated.").print(py);
            }
            Some(icons::STACK_OVERFLOW)
        }
        "tag-hidden" => Some(icons::TAG_HIDDEN),
        "bullhorn-variant" => Some(icons::BULLHORN_VARIANT),
        "file-music-outline" => Some(icons::FILE_MUSIC_OUTLINE),
        "checkbox-blank-off" => Some(icons::CHECKBOX_BLANK_OFF),
        "image-minus-outline" => Some(icons::IMAGE_MINUS_OUTLINE),
        "ocarina" => Some(icons::OCARINA),
        "calendar-remove-outline" => Some(icons::CALENDAR_REMOVE_OUTLINE),
        "eye-arrow-right" => Some(icons::EYE_ARROW_RIGHT),
        "water-thermometer-outline" => Some(icons::WATER_THERMOMETER_OUTLINE),
        "alien-outline" => Some(icons::ALIEN_OUTLINE),
        "file-import-outline" => Some(icons::FILE_IMPORT_OUTLINE),
        "target-account" => Some(icons::TARGET_ACCOUNT),
        "star-half" => Some(icons::STAR_HALF),
        "bird" => Some(icons::BIRD),
        "music-note-whole-dotted" => Some(icons::MUSIC_NOTE_WHOLE_DOTTED),
        "office-building-minus-outline" => Some(icons::OFFICE_BUILDING_MINUS_OUTLINE),
        "message-bulleted-off" => Some(icons::MESSAGE_BULLETED_OFF),
        "head-cog-outline" => Some(icons::HEAD_COG_OUTLINE),
        "flash-alert" => Some(icons::FLASH_ALERT),
        "spray" => Some(icons::SPRAY),
        "notebook-plus-outline" => Some(icons::NOTEBOOK_PLUS_OUTLINE),
        "speaker-wireless" => Some(icons::SPEAKER_WIRELESS),
        "theater" => Some(icons::THEATER),
        "message-reply-text-outline" => Some(icons::MESSAGE_REPLY_TEXT_OUTLINE),
        "bookmark-plus-outline" => Some(icons::BOOKMARK_PLUS_OUTLINE),
        "flag-variant-off-outline" => Some(icons::FLAG_VARIANT_OFF_OUTLINE),
        "head-lightbulb-outline" => Some(icons::HEAD_LIGHTBULB_OUTLINE),
        "login-variant" => Some(icons::LOGIN_VARIANT),
        "human-male-height" => Some(icons::HUMAN_MALE_HEIGHT),
        "fan-alert" => Some(icons::FAN_ALERT),
        "package-variant-remove" => Some(icons::PACKAGE_VARIANT_REMOVE),
        "eyedropper-minus" => Some(icons::EYEDROPPER_MINUS),
        "printer-3d-nozzle-off" => Some(icons::PRINTER_3D_NOZZLE_OFF),
        "reflect-horizontal" => Some(icons::REFLECT_HORIZONTAL),
        "ev-station" => Some(icons::EV_STATION),
        "printer-3d" => Some(icons::PRINTER_3D),
        "timer-sync" => Some(icons::TIMER_SYNC),
        "trumpet" => Some(icons::TRUMPET),
        "flag-outline" => Some(icons::FLAG_OUTLINE),
        "french-fries" => Some(icons::FRENCH_FRIES),
        "star-shooting" => Some(icons::STAR_SHOOTING),
        "hexagon-slice-5" => Some(icons::HEXAGON_SLICE_5),
        "closed-caption" => Some(icons::CLOSED_CAPTION),
        "rivet" => Some(icons::RIVET),
        "tag-edit-outline" => Some(icons::TAG_EDIT_OUTLINE),
        "car-3-plus" => Some(icons::CAR_3_PLUS),
        "tournament" => Some(icons::TOURNAMENT),
        "phone-alert" => Some(icons::PHONE_ALERT),
        "home-floor-a" => Some(icons::HOME_FLOOR_A),
        "radiator" => Some(icons::RADIATOR),
        "earth-box-remove" => Some(icons::EARTH_BOX_REMOVE),
        "relative-scale" => Some(icons::RELATIVE_SCALE),
        "ev-plug-ccs2" => Some(icons::EV_PLUG_CCS2),
        "heat-pump-outline" => Some(icons::HEAT_PUMP_OUTLINE),
        "bicycle-cargo" => Some(icons::BICYCLE_CARGO),
        "form-textbox-password" => Some(icons::FORM_TEXTBOX_PASSWORD),
        "hexadecimal" => Some(icons::HEXADECIMAL),
        "human-female-boy" => Some(icons::HUMAN_FEMALE_BOY),
        "notebook-heart-outline" => Some(icons::NOTEBOOK_HEART_OUTLINE),
        "reminder" => Some(icons::REMINDER),
        "air-filter" => Some(icons::AIR_FILTER),
        "dock-right" => Some(icons::DOCK_RIGHT),
        "domain-off" => Some(icons::DOMAIN_OFF),
        "ice-cream-off" => Some(icons::ICE_CREAM_OFF),
        "clipboard-account" => Some(icons::CLIPBOARD_ACCOUNT),
        "source-branch-minus" => Some(icons::SOURCE_BRANCH_MINUS),
        "silo-outline" => Some(icons::SILO_OUTLINE),
        "database-eye" => Some(icons::DATABASE_EYE),
        "video-plus-outline" => Some(icons::VIDEO_PLUS_OUTLINE),
        "book-plus-multiple-outline" => Some(icons::BOOK_PLUS_MULTIPLE_OUTLINE),
        "battery-charging-70" => Some(icons::BATTERY_CHARGING_70),
        "arrow-top-left" => Some(icons::ARROW_TOP_LEFT),
        "rewind-45" => Some(icons::REWIND_45),
        "file-pdf-box" => Some(icons::FILE_PDF_BOX),
        "flag-triangle" => Some(icons::FLAG_TRIANGLE),
        "cellphone-dock" => Some(icons::CELLPHONE_DOCK),
        "cash-sync" => Some(icons::CASH_SYNC),
        "card-plus" => Some(icons::CARD_PLUS),
        "silverware-variant" => Some(icons::SILVERWARE_VARIANT),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "button-cursor" => Some(icons::BUTTON_CURSOR),
        "volume-minus" => Some(icons::VOLUME_MINUS),
        "cellphone-arrow-down" => Some(icons::CELLPHONE_ARROW_DOWN),
        "solar-panel-large" => Some(icons::SOLAR_PANEL_LARGE),
        #[allow(deprecated)]
        "blender-software" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'blender-software' is deprecated.")
                    .print(py);
            }
            Some(icons::BLENDER_SOFTWARE)
        }
        "water-check" => Some(icons::WATER_CHECK),
        "map-legend" => Some(icons::MAP_LEGEND),
        "bottle-tonic-outline" => Some(icons::BOTTLE_TONIC_OUTLINE),
        "currency-bdt" => Some(icons::CURRENCY_BDT),
        "account-school" => Some(icons::ACCOUNT_SCHOOL),
        "bathtub-outline" => Some(icons::BATHTUB_OUTLINE),
        "horse-variant" => Some(icons::HORSE_VARIANT),
        "invoice-arrow-right-outline" => Some(icons::INVOICE_ARROW_RIGHT_OUTLINE),
        "ph" => Some(icons::PH),
        "battery-clock" => Some(icons::BATTERY_CLOCK),
        "fire-truck" => Some(icons::FIRE_TRUCK),
        "diving-scuba" => Some(icons::DIVING_SCUBA),
        "database-outline" => Some(icons::DATABASE_OUTLINE),
        "lightbulb-cfl-spiral-off" => Some(icons::LIGHTBULB_CFL_SPIRAL_OFF),
        "invoice-text-minus-outline" => Some(icons::INVOICE_TEXT_MINUS_OUTLINE),
        "plus-circle-multiple-outline" => Some(icons::PLUS_CIRCLE_MULTIPLE_OUTLINE),
        "star-four-points-box" => Some(icons::STAR_FOUR_POINTS_BOX),
        "code-greater-than" => Some(icons::CODE_GREATER_THAN),
        "table" => Some(icons::TABLE),
        "island-variant" => Some(icons::ISLAND_VARIANT),
        "account-star" => Some(icons::ACCOUNT_STAR),
        "format-bold" => Some(icons::FORMAT_BOLD),
        "numeric-5-circle" => Some(icons::NUMERIC_5_CIRCLE),
        "email-fast-outline" => Some(icons::EMAIL_FAST_OUTLINE),
        "mail" => Some(icons::MAIL),
        "human" => Some(icons::HUMAN),
        "relation-zero-or-many-to-only-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONLY_ONE),
        "clipboard-arrow-right-outline" => Some(icons::CLIPBOARD_ARROW_RIGHT_OUTLINE),
        "train-car-passenger" => Some(icons::TRAIN_CAR_PASSENGER),
        "progress-clock" => Some(icons::PROGRESS_CLOCK),
        "moon-waxing-crescent" => Some(icons::MOON_WAXING_CRESCENT),
        "database-minus-outline" => Some(icons::DATABASE_MINUS_OUTLINE),
        #[allow(deprecated)]
        "material-ui" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-ui' is deprecated.").print(py);
            }
            Some(icons::MATERIAL_UI)
        }
        "sun-angle-outline" => Some(icons::SUN_ANGLE_OUTLINE),
        "box-cutter" => Some(icons::BOX_CUTTER),
        "watering-can" => Some(icons::WATERING_CAN),
        "axis-x-rotate-clockwise" => Some(icons::AXIS_X_ROTATE_CLOCKWISE),
        "store-alert-outline" => Some(icons::STORE_ALERT_OUTLINE),
        "blinds-horizontal" => Some(icons::BLINDS_HORIZONTAL),
        "book-heart-outline" => Some(icons::BOOK_HEART_OUTLINE),
        "augmented-reality" => Some(icons::AUGMENTED_REALITY),
        "view-grid-plus-outline" => Some(icons::VIEW_GRID_PLUS_OUTLINE),
        "land-plots-circle-variant" => Some(icons::LAND_PLOTS_CIRCLE_VARIANT),
        "database-plus-outline" => Some(icons::DATABASE_PLUS_OUTLINE),
        "stretch-to-page-outline" => Some(icons::STRETCH_TO_PAGE_OUTLINE),
        "disc-alert" => Some(icons::DISC_ALERT),
        "water-percent" => Some(icons::WATER_PERCENT),
        "sine-wave" => Some(icons::SINE_WAVE),
        "cookie-settings-outline" => Some(icons::COOKIE_SETTINGS_OUTLINE),
        "archive-minus-outline" => Some(icons::ARCHIVE_MINUS_OUTLINE),
        _ => None,
    }
}
