// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_9(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "delete-alert" => Some(icons::DELETE_ALERT),
        "check-underline-circle" => Some(icons::CHECK_UNDERLINE_CIRCLE),
        "margin" => Some(icons::MARGIN),
        "at" => Some(icons::AT),
        "wall-sconce-flat-variant-outline" => Some(icons::WALL_SCONCE_FLAT_VARIANT_OUTLINE),
        "folder-edit-outline" => Some(icons::FOLDER_EDIT_OUTLINE),
        "archive-eye" => Some(icons::ARCHIVE_EYE),
        "glass-flute" => Some(icons::GLASS_FLUTE),
        "scissors-cutting" => Some(icons::SCISSORS_CUTTING),
        "credit-card-clock-outline" => Some(icons::CREDIT_CARD_CLOCK_OUTLINE),
        "video-account" => Some(icons::VIDEO_ACCOUNT),
        "format-wrap-inline" => Some(icons::FORMAT_WRAP_INLINE),
        "scent-off" => Some(icons::SCENT_OFF),
        "hand-back-right-off" => Some(icons::HAND_BACK_RIGHT_OFF),
        "rickshaw-electric" => Some(icons::RICKSHAW_ELECTRIC),
        "table-off" => Some(icons::TABLE_OFF),
        "chat-processing-outline" => Some(icons::CHAT_PROCESSING_OUTLINE),
        "chili-mild" => Some(icons::CHILI_MILD),
        "recycle-variant" => Some(icons::RECYCLE_VARIANT),
        "send-variant-clock" => Some(icons::SEND_VARIANT_CLOCK),
        "file" => Some(icons::FILE),
        "head-check-outline" => Some(icons::HEAD_CHECK_OUTLINE),
        "phone-check" => Some(icons::PHONE_CHECK),
        "kabaddi" => Some(icons::KABADDI),
        "tally-mark-2" => Some(icons::TALLY_MARK_2),
        "invoice-text-plus" => Some(icons::INVOICE_TEXT_PLUS),
        "rocket-outline" => Some(icons::ROCKET_OUTLINE),
        "vector-radius" => Some(icons::VECTOR_RADIUS),
        "pirate" => Some(icons::PIRATE),
        "source-merge" => Some(icons::SOURCE_MERGE),
        "inbox" => Some(icons::INBOX),
        "view-dashboard" => Some(icons::VIEW_DASHBOARD),
        "arrow-up-thick" => Some(icons::ARROW_UP_THICK),
        "kettle-alert" => Some(icons::KETTLE_ALERT),
        "movie-settings" => Some(icons::MOVIE_SETTINGS),
        "align-horizontal-center" => Some(icons::ALIGN_HORIZONTAL_CENTER),
        "chat-question-outline" => Some(icons::CHAT_QUESTION_OUTLINE),
        "shield-account" => Some(icons::SHIELD_ACCOUNT),
        "tank" => Some(icons::TANK),
        "toy-brick-search" => Some(icons::TOY_BRICK_SEARCH),
        "car-back" => Some(icons::CAR_BACK),
        "triangle-small-down" => Some(icons::TRIANGLE_SMALL_DOWN),
        #[allow(deprecated)]
        "git" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'git' is deprecated.").print(py);
            }
            Some(icons::GIT)
        }
        #[allow(deprecated)]
        "material-design" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'material-design' is deprecated.")
                    .print(py);
            }
            Some(icons::MATERIAL_DESIGN)
        }
        "fireplace-off" => Some(icons::FIREPLACE_OFF),
        "panorama-horizontal" => Some(icons::PANORAMA_HORIZONTAL),
        "flask-empty-plus" => Some(icons::FLASK_EMPTY_PLUS),
        "help-box-multiple-outline" => Some(icons::HELP_BOX_MULTIPLE_OUTLINE),
        "movie-play" => Some(icons::MOVIE_PLAY),
        "zodiac-leo" => Some(icons::ZODIAC_LEO),
        "file-cad-box" => Some(icons::FILE_CAD_BOX),
        "brush" => Some(icons::BRUSH),
        "grease-pencil" => Some(icons::GREASE_PENCIL),
        "pot-outline" => Some(icons::POT_OUTLINE),
        "zodiac-libra" => Some(icons::ZODIAC_LIBRA),
        "phone" => Some(icons::PHONE),
        "emoticon-cool-outline" => Some(icons::EMOTICON_COOL_OUTLINE),
        "diving-scuba-flag" => Some(icons::DIVING_SCUBA_FLAG),
        "et" => Some(icons::ET),
        "book-minus" => Some(icons::BOOK_MINUS),
        "rectangle-outline" => Some(icons::RECTANGLE_OUTLINE),
        "tooltip-check" => Some(icons::TOOLTIP_CHECK),
        "numeric" => Some(icons::NUMERIC),
        "ocarina" => Some(icons::OCARINA),
        "swap-horizontal-circle" => Some(icons::SWAP_HORIZONTAL_CIRCLE),
        "middleware" => Some(icons::MIDDLEWARE),
        "printer-3d-nozzle-outline" => Some(icons::PRINTER_3D_NOZZLE_OUTLINE),
        "pan-top-left" => Some(icons::PAN_TOP_LEFT),
        "beaker-minus-outline" => Some(icons::BEAKER_MINUS_OUTLINE),
        "content-duplicate" => Some(icons::CONTENT_DUPLICATE),
        "traffic-light-outline" => Some(icons::TRAFFIC_LIGHT_OUTLINE),
        "printer-3d-off" => Some(icons::PRINTER_3D_OFF),
        "numeric-1-box" => Some(icons::NUMERIC_1_BOX),
        "unfold-more-horizontal" => Some(icons::UNFOLD_MORE_HORIZONTAL),
        "phone-dial" => Some(icons::PHONE_DIAL),
        "file-percent" => Some(icons::FILE_PERCENT),
        "battery-plus-outline" => Some(icons::BATTERY_PLUS_OUTLINE),
        "flag" => Some(icons::FLAG),
        "thumb-down" => Some(icons::THUMB_DOWN),
        "cards-variant" => Some(icons::CARDS_VARIANT),
        "pen-lock" => Some(icons::PEN_LOCK),
        "cookie-clock" => Some(icons::COOKIE_CLOCK),
        "file-clock" => Some(icons::FILE_CLOCK),
        "fuel-cell" => Some(icons::FUEL_CELL),
        "eye-plus" => Some(icons::EYE_PLUS),
        "baby-bottle" => Some(icons::BABY_BOTTLE),
        #[allow(deprecated)]
        "twitter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'twitter' is deprecated.").print(py);
            }
            Some(icons::TWITTER)
        }
        "attachment-minus" => Some(icons::ATTACHMENT_MINUS),
        "solar-power-variant" => Some(icons::SOLAR_POWER_VARIANT),
        "file-arrow-left-right-outline" => Some(icons::FILE_ARROW_LEFT_RIGHT_OUTLINE),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "power-plug-off" => Some(icons::POWER_PLUG_OFF),
        "cards-playing-heart" => Some(icons::CARDS_PLAYING_HEART),
        "debug-step-over" => Some(icons::DEBUG_STEP_OVER),
        "content-save-cog-outline" => Some(icons::CONTENT_SAVE_COG_OUTLINE),
        "book-alphabet" => Some(icons::BOOK_ALPHABET),
        #[allow(deprecated)]
        "jquery" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'jquery' is deprecated.").print(py);
            }
            Some(icons::JQUERY)
        }
        "car-multiple" => Some(icons::CAR_MULTIPLE),
        "order-bool-ascending-variant" => Some(icons::ORDER_BOOL_ASCENDING_VARIANT),
        "view-carousel" => Some(icons::VIEW_CAROUSEL),
        "devices" => Some(icons::DEVICES),
        "transcribe-close" => Some(icons::TRANSCRIBE_CLOSE),
        "phone-dial-outline" => Some(icons::PHONE_DIAL_OUTLINE),
        #[allow(deprecated)]
        "slack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'slack' is deprecated.").print(py);
            }
            Some(icons::SLACK)
        }
        "cog-clockwise" => Some(icons::COG_CLOCKWISE),
        "flash-triangle" => Some(icons::FLASH_TRIANGLE),
        "invoice-remove-outline" => Some(icons::INVOICE_REMOVE_OUTLINE),
        "database-sync-outline" => Some(icons::DATABASE_SYNC_OUTLINE),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "train-car-passenger-door" => Some(icons::TRAIN_CAR_PASSENGER_DOOR),
        "select-arrow-up" => Some(icons::SELECT_ARROW_UP),
        "link-box-variant-outline" => Some(icons::LINK_BOX_VARIANT_OUTLINE),
        "coffee-maker-outline" => Some(icons::COFFEE_MAKER_OUTLINE),
        "cog-pause" => Some(icons::COG_PAUSE),
        "cart-arrow-up" => Some(icons::CART_ARROW_UP),
        #[allow(deprecated)]
        "microsoft-onedrive" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-onedrive' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ONEDRIVE)
        }
        "server-network" => Some(icons::SERVER_NETWORK),
        "file-sync-outline" => Some(icons::FILE_SYNC_OUTLINE),
        "card-multiple" => Some(icons::CARD_MULTIPLE),
        "toy-brick-marker-outline" => Some(icons::TOY_BRICK_MARKER_OUTLINE),
        "car-off" => Some(icons::CAR_OFF),
        #[allow(deprecated)]
        "google-hangouts" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-hangouts' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_HANGOUTS)
        }
        "timeline-clock-outline" => Some(icons::TIMELINE_CLOCK_OUTLINE),
        "account-eye" => Some(icons::ACCOUNT_EYE),
        "microphone-settings" => Some(icons::MICROPHONE_SETTINGS),
        "relation-zero-or-many-to-zero-or-many" => {
            Some(icons::RELATION_ZERO_OR_MANY_TO_ZERO_OR_MANY)
        }
        "window-maximize" => Some(icons::WINDOW_MAXIMIZE),
        "power-plug-battery-outline" => Some(icons::POWER_PLUG_BATTERY_OUTLINE),
        "floor-lamp-dual-outline" => Some(icons::FLOOR_LAMP_DUAL_OUTLINE),
        "food-variant" => Some(icons::FOOD_VARIANT),
        "alphabet-cyrillic" => Some(icons::ALPHABET_CYRILLIC),
        "typewriter" => Some(icons::TYPEWRITER),
        "clipboard-text-play-outline" => Some(icons::CLIPBOARD_TEXT_PLAY_OUTLINE),
        "silverware-fork-knife" => Some(icons::SILVERWARE_FORK_KNIFE),
        "format-line-spacing" => Some(icons::FORMAT_LINE_SPACING),
        "archive-lock-outline" => Some(icons::ARCHIVE_LOCK_OUTLINE),
        "view-comfy-outline" => Some(icons::VIEW_COMFY_OUTLINE),
        "fridge-variant" => Some(icons::FRIDGE_VARIANT),
        "chat-plus" => Some(icons::CHAT_PLUS),
        "export-variant" => Some(icons::EXPORT_VARIANT),
        "lightbulb-night" => Some(icons::LIGHTBULB_NIGHT),
        "format-annotation-minus" => Some(icons::FORMAT_ANNOTATION_MINUS),
        "format-header-equal" => Some(icons::FORMAT_HEADER_EQUAL),
        "book-alert" => Some(icons::BOOK_ALERT),
        "compass" => Some(icons::COMPASS),
        "numeric-9-plus-box" => Some(icons::NUMERIC_9_PLUS_BOX),
        "ship-wheel" => Some(icons::SHIP_WHEEL),
        "palette-advanced" => Some(icons::PALETTE_ADVANCED),
        "comment-remove" => Some(icons::COMMENT_REMOVE),
        "send-clock" => Some(icons::SEND_CLOCK),
        "cart-outline" => Some(icons::CART_OUTLINE),
        "basket-remove-outline" => Some(icons::BASKET_REMOVE_OUTLINE),
        "bullhorn-variant-outline" => Some(icons::BULLHORN_VARIANT_OUTLINE),
        "city-variant-outline" => Some(icons::CITY_VARIANT_OUTLINE),
        #[allow(deprecated)]
        "xmpp" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'xmpp' is deprecated.").print(py);
            }
            Some(icons::XMPP)
        }
        "invoice-arrow-right-outline" => Some(icons::INVOICE_ARROW_RIGHT_OUTLINE),
        "cat" => Some(icons::CAT),
        "music-clef-alto" => Some(icons::MUSIC_CLEF_ALTO),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "hand-pointing-up" => Some(icons::HAND_POINTING_UP),
        "numeric-2-box-outline" => Some(icons::NUMERIC_2_BOX_OUTLINE),
        "alpha-q-box" => Some(icons::ALPHA_Q_BOX),
        "kettle" => Some(icons::KETTLE),
        "shield-sword-outline" => Some(icons::SHIELD_SWORD_OUTLINE),
        "invoice-list" => Some(icons::INVOICE_LIST),
        "calendar-edit" => Some(icons::CALENDAR_EDIT),
        "keyboard-f3" => Some(icons::KEYBOARD_F3),
        "store-check" => Some(icons::STORE_CHECK),
        "key-alert" => Some(icons::KEY_ALERT),
        "sofa-single-outline" => Some(icons::SOFA_SINGLE_OUTLINE),
        "sprout-outline" => Some(icons::SPROUT_OUTLINE),
        "battery-charging-medium" => Some(icons::BATTERY_CHARGING_MEDIUM),
        "clipboard-off-outline" => Some(icons::CLIPBOARD_OFF_OUTLINE),
        "store-alert-outline" => Some(icons::STORE_ALERT_OUTLINE),
        "image-size-select-large" => Some(icons::IMAGE_SIZE_SELECT_LARGE),
        "wifi-arrow-down" => Some(icons::WIFI_ARROW_DOWN),
        "note-search-outline" => Some(icons::NOTE_SEARCH_OUTLINE),
        "pine-tree-fire" => Some(icons::PINE_TREE_FIRE),
        "piggy-bank-outline" => Some(icons::PIGGY_BANK_OUTLINE),
        "kettle-off" => Some(icons::KETTLE_OFF),
        "dock-bottom" => Some(icons::DOCK_BOTTOM),
        "power-standby" => Some(icons::POWER_STANDBY),
        "skip-previous-circle" => Some(icons::SKIP_PREVIOUS_CIRCLE),
        "gender-male" => Some(icons::GENDER_MALE),
        "screw-flat-top" => Some(icons::SCREW_FLAT_TOP),
        "brightness-1" => Some(icons::BRIGHTNESS_1),
        "toaster-oven" => Some(icons::TOASTER_OVEN),
        "map-marker-left-outline" => Some(icons::MAP_MARKER_LEFT_OUTLINE),
        "truck" => Some(icons::TRUCK),
        "tag-search" => Some(icons::TAG_SEARCH),
        "lighthouse" => Some(icons::LIGHTHOUSE),
        "hand-front-right-outline" => Some(icons::HAND_FRONT_RIGHT_OUTLINE),
        "billiards" => Some(icons::BILLIARDS),
        "coffin" => Some(icons::COFFIN),
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        "decagram-outline" => Some(icons::DECAGRAM_OUTLINE),
        "lightbulb-on-outline" => Some(icons::LIGHTBULB_ON_OUTLINE),
        "ear-hearing-off" => Some(icons::EAR_HEARING_OFF),
        "invoice-send-outline" => Some(icons::INVOICE_SEND_OUTLINE),
        "router" => Some(icons::ROUTER),
        _ => None,
    }
}
