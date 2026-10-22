// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_3(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "chart-timeline-variant-shimmer" => Some(icons::CHART_TIMELINE_VARIANT_SHIMMER),
        "menu-up-outline" => Some(icons::MENU_UP_OUTLINE),
        #[allow(deprecated)]
        "graphql" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'graphql' is deprecated.").print(py);
            }
            Some(icons::GRAPHQL)
        }
        "sort-calendar-ascending" => Some(icons::SORT_CALENDAR_ASCENDING),
        "home-roof" => Some(icons::HOME_ROOF),
        "numeric-5-circle-outline" => Some(icons::NUMERIC_5_CIRCLE_OUTLINE),
        "account-box-multiple-outline" => Some(icons::ACCOUNT_BOX_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "google-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'google-assistant' is deprecated.")
                    .print(py);
            }
            Some(icons::GOOGLE_ASSISTANT)
        }
        "weather-sunset-down" => Some(icons::WEATHER_SUNSET_DOWN),
        "calendar-week-begin-outline" => Some(icons::CALENDAR_WEEK_BEGIN_OUTLINE),
        "arrow-bottom-right" => Some(icons::ARROW_BOTTOM_RIGHT),
        #[allow(deprecated)]
        "apple-icloud" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-icloud' is deprecated.").print(py);
            }
            Some(icons::APPLE_ICLOUD)
        }
        "magnify" => Some(icons::MAGNIFY),
        "filter-variant" => Some(icons::FILTER_VARIANT),
        "spear" => Some(icons::SPEAR),
        "hands-pray" => Some(icons::HANDS_PRAY),
        "book-minus-multiple" => Some(icons::BOOK_MINUS_MULTIPLE),
        "ultra-high-definition" => Some(icons::ULTRA_HIGH_DEFINITION),
        "briefcase-search" => Some(icons::BRIEFCASE_SEARCH),
        #[allow(deprecated)]
        "drupal" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'drupal' is deprecated.").print(py);
            }
            Some(icons::DRUPAL)
        }
        "family-tree" => Some(icons::FAMILY_TREE),
        "cart" => Some(icons::CART),
        "book-off" => Some(icons::BOOK_OFF),
        "camera-account" => Some(icons::CAMERA_ACCOUNT),
        "account-settings" => Some(icons::ACCOUNT_SETTINGS),
        "printer-pos-network" => Some(icons::PRINTER_POS_NETWORK),
        "glass-mug-variant-off" => Some(icons::GLASS_MUG_VARIANT_OFF),
        "battery-charging-80" => Some(icons::BATTERY_CHARGING_80),
        "translate-off" => Some(icons::TRANSLATE_OFF),
        "paper-roll-outline" => Some(icons::PAPER_ROLL_OUTLINE),
        "mail" => Some(icons::MAIL),
        "fan-auto" => Some(icons::FAN_AUTO),
        "music-clef-treble" => Some(icons::MUSIC_CLEF_TREBLE),
        "ship-wheel" => Some(icons::SHIP_WHEEL),
        "alert-remove-outline" => Some(icons::ALERT_REMOVE_OUTLINE),
        "checkbox-intermediate-variant" => Some(icons::CHECKBOX_INTERMEDIATE_VARIANT),
        "book-search" => Some(icons::BOOK_SEARCH),
        "video-3d" => Some(icons::VIDEO_3D),
        "snake" => Some(icons::SNAKE),
        "arrow-up-bold-outline" => Some(icons::ARROW_UP_BOLD_OUTLINE),
        "account-badge" => Some(icons::ACCOUNT_BADGE),
        "alarm-light-off" => Some(icons::ALARM_LIGHT_OFF),
        "card-remove" => Some(icons::CARD_REMOVE),
        "alpha-v-circle" => Some(icons::ALPHA_V_CIRCLE),
        "mailbox-open-outline" => Some(icons::MAILBOX_OPEN_OUTLINE),
        "bugle" => Some(icons::BUGLE),
        "comma" => Some(icons::COMMA),
        "vector-square" => Some(icons::VECTOR_SQUARE),
        "checkbox-blank-off-outline" => Some(icons::CHECKBOX_BLANK_OFF_OUTLINE),
        "earth-box-minus" => Some(icons::EARTH_BOX_MINUS),
        "power-sleep" => Some(icons::POWER_SLEEP),
        "clipboard-multiple-outline" => Some(icons::CLIPBOARD_MULTIPLE_OUTLINE),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "lightbulb-fluorescent-tube-outline" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE_OUTLINE),
        "arrow-left-thin" => Some(icons::ARROW_LEFT_THIN),
        "apple-keyboard-caps" => Some(icons::APPLE_KEYBOARD_CAPS),
        "card-search" => Some(icons::CARD_SEARCH),
        #[allow(deprecated)]
        "polymer" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'polymer' is deprecated.").print(py);
            }
            Some(icons::POLYMER)
        }
        "swim" => Some(icons::SWIM),
        "puzzle-star" => Some(icons::PUZZLE_STAR),
        "home-remove" => Some(icons::HOME_REMOVE),
        "flask-empty-plus-outline" => Some(icons::FLASK_EMPTY_PLUS_OUTLINE),
        "battery-remove-outline" => Some(icons::BATTERY_REMOVE_OUTLINE),
        "message-minus-outline" => Some(icons::MESSAGE_MINUS_OUTLINE),
        "virus-off-outline" => Some(icons::VIRUS_OFF_OUTLINE),
        "timeline-plus" => Some(icons::TIMELINE_PLUS),
        #[allow(deprecated)]
        "gog" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gog' is deprecated.").print(py);
            }
            Some(icons::GOG)
        }
        "tag-text-outline" => Some(icons::TAG_TEXT_OUTLINE),
        "format-float-none" => Some(icons::FORMAT_FLOAT_NONE),
        "book-open-blank-variant" => Some(icons::BOOK_OPEN_BLANK_VARIANT),
        "food-outline" => Some(icons::FOOD_OUTLINE),
        "invoice-edit" => Some(icons::INVOICE_EDIT),
        "book-variant" => Some(icons::BOOK_VARIANT),
        "car-pickup" => Some(icons::CAR_PICKUP),
        "oil-lamp" => Some(icons::OIL_LAMP),
        "chess-king" => Some(icons::CHESS_KING),
        "battery-medium" => Some(icons::BATTERY_MEDIUM),
        "relation-one-to-zero-or-many" => Some(icons::RELATION_ONE_TO_ZERO_OR_MANY),
        "file-sign" => Some(icons::FILE_SIGN),
        "file-remove" => Some(icons::FILE_REMOVE),
        "format-text-variant" => Some(icons::FORMAT_TEXT_VARIANT),
        #[allow(deprecated)]
        "slack" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'slack' is deprecated.").print(py);
            }
            Some(icons::SLACK)
        }
        "flask-empty-remove-outline" => Some(icons::FLASK_EMPTY_REMOVE_OUTLINE),
        "content-save-alert" => Some(icons::CONTENT_SAVE_ALERT),
        "format-font" => Some(icons::FORMAT_FONT),
        "view-compact-outline" => Some(icons::VIEW_COMPACT_OUTLINE),
        #[allow(deprecated)]
        "atlassian" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'atlassian' is deprecated.").print(py);
            }
            Some(icons::ATLASSIAN)
        }
        "owl" => Some(icons::OWL),
        "human-handsup" => Some(icons::HUMAN_HANDSUP),
        "water-boiler-auto" => Some(icons::WATER_BOILER_AUTO),
        "format-header-5" => Some(icons::FORMAT_HEADER_5),
        "lock-plus-outline" => Some(icons::LOCK_PLUS_OUTLINE),
        "pinwheel" => Some(icons::PINWHEEL),
        "gift-open-outline" => Some(icons::GIFT_OPEN_OUTLINE),
        "hoop-house" => Some(icons::HOOP_HOUSE),
        "truck-alert" => Some(icons::TRUCK_ALERT),
        "checkbox-intermediate" => Some(icons::CHECKBOX_INTERMEDIATE),
        "delete-clock-outline" => Some(icons::DELETE_CLOCK_OUTLINE),
        "toy-brick-plus-outline" => Some(icons::TOY_BRICK_PLUS_OUTLINE),
        "movie-star-outline" => Some(icons::MOVIE_STAR_OUTLINE),
        "text-box-minus" => Some(icons::TEXT_BOX_MINUS),
        "pail-minus-outline" => Some(icons::PAIL_MINUS_OUTLINE),
        "text-box-remove" => Some(icons::TEXT_BOX_REMOVE),
        "drone" => Some(icons::DRONE),
        "record-circle-outline" => Some(icons::RECORD_CIRCLE_OUTLINE),
        "cookie-remove-outline" => Some(icons::COOKIE_REMOVE_OUTLINE),
        "cursor-default-gesture" => Some(icons::CURSOR_DEFAULT_GESTURE),
        "city" => Some(icons::CITY),
        "weather-tornado" => Some(icons::WEATHER_TORNADO),
        "map-search-outline" => Some(icons::MAP_SEARCH_OUTLINE),
        "coach-lamp" => Some(icons::COACH_LAMP),
        "umbrella-outline" => Some(icons::UMBRELLA_OUTLINE),
        "file-table-outline" => Some(icons::FILE_TABLE_OUTLINE),
        "smoke-detector-variant-off" => Some(icons::SMOKE_DETECTOR_VARIANT_OFF),
        "format-title" => Some(icons::FORMAT_TITLE),
        "chat-minus" => Some(icons::CHAT_MINUS),
        "sticker-text" => Some(icons::STICKER_TEXT),
        "escalator-box" => Some(icons::ESCALATOR_BOX),
        #[allow(deprecated)]
        "freebsd" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'freebsd' is deprecated.").print(py);
            }
            Some(icons::FREEBSD)
        }
        "printer-outline" => Some(icons::PRINTER_OUTLINE),
        "head-heart" => Some(icons::HEAD_HEART),
        "camera-marker" => Some(icons::CAMERA_MARKER),
        "road" => Some(icons::ROAD),
        "eyedropper" => Some(icons::EYEDROPPER),
        "star-minus" => Some(icons::STAR_MINUS),
        "contactless-payment-circle" => Some(icons::CONTACTLESS_PAYMENT_CIRCLE),
        "upload-multiple" => Some(icons::UPLOAD_MULTIPLE),
        "octagram-plus" => Some(icons::OCTAGRAM_PLUS),
        "storefront-edit" => Some(icons::STOREFRONT_EDIT),
        "projector-screen-off-outline" => Some(icons::PROJECTOR_SCREEN_OFF_OUTLINE),
        "shield-off-outline" => Some(icons::SHIELD_OFF_OUTLINE),
        "image-minus" => Some(icons::IMAGE_MINUS),
        "star-circle-outline" => Some(icons::STAR_CIRCLE_OUTLINE),
        "seat-passenger" => Some(icons::SEAT_PASSENGER),
        "lock-minus-outline" => Some(icons::LOCK_MINUS_OUTLINE),
        "bookmark-box-multiple" => Some(icons::BOOKMARK_BOX_MULTIPLE),
        "database-edit-outline" => Some(icons::DATABASE_EDIT_OUTLINE),
        #[allow(deprecated)]
        "nfc" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'nfc' is deprecated.").print(py);
            }
            Some(icons::NFC)
        }
        "book-arrow-left" => Some(icons::BOOK_ARROW_LEFT),
        "car-search-outline" => Some(icons::CAR_SEARCH_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-off" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-off' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_OFF)
        }
        "rotate-right" => Some(icons::ROTATE_RIGHT),
        "calculator-variant" => Some(icons::CALCULATOR_VARIANT),
        "smoking-pipe" => Some(icons::SMOKING_PIPE),
        "comment-processing" => Some(icons::COMMENT_PROCESSING),
        #[allow(deprecated)]
        "youtube-studio" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'youtube-studio' is deprecated.").print(py);
            }
            Some(icons::YOUTUBE_STUDIO)
        }
        "cards-playing-spade-multiple" => Some(icons::CARDS_PLAYING_SPADE_MULTIPLE),
        "layers-off-outline" => Some(icons::LAYERS_OFF_OUTLINE),
        "iron" => Some(icons::IRON),
        "cards-playing-club-multiple" => Some(icons::CARDS_PLAYING_CLUB_MULTIPLE),
        "source-commit-local" => Some(icons::SOURCE_COMMIT_LOCAL),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "checkerboard-plus" => Some(icons::CHECKERBOARD_PLUS),
        "heart-off-outline" => Some(icons::HEART_OFF_OUTLINE),
        "cards-playing-outline" => Some(icons::CARDS_PLAYING_OUTLINE),
        "bowling" => Some(icons::BOWLING),
        "arrow-right-bold-box" => Some(icons::ARROW_RIGHT_BOLD_BOX),
        #[allow(deprecated)]
        "home-assistant" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'home-assistant' is deprecated.").print(py);
            }
            Some(icons::HOME_ASSISTANT)
        }
        "panorama-outline" => Some(icons::PANORAMA_OUTLINE),
        "rabbit-variant-outline" => Some(icons::RABBIT_VARIANT_OUTLINE),
        "credit-card-remove-outline" => Some(icons::CREDIT_CARD_REMOVE_OUTLINE),
        "emoticon-kiss-outline" => Some(icons::EMOTICON_KISS_OUTLINE),
        "tally-mark-5" => Some(icons::TALLY_MARK_5),
        "arrow-right-bold" => Some(icons::ARROW_RIGHT_BOLD),
        "arrange-bring-to-front" => Some(icons::ARRANGE_BRING_TO_FRONT),
        "heart-box-outline" => Some(icons::HEART_BOX_OUTLINE),
        "phone-ring" => Some(icons::PHONE_RING),
        "human-male" => Some(icons::HUMAN_MALE),
        "human-female" => Some(icons::HUMAN_FEMALE),
        "battery-heart-outline" => Some(icons::BATTERY_HEART_OUTLINE),
        "mailbox" => Some(icons::MAILBOX),
        "view-grid-plus-outline" => Some(icons::VIEW_GRID_PLUS_OUTLINE),
        "wifi-alert" => Some(icons::WIFI_ALERT),
        "fast-forward-5" => Some(icons::FAST_FORWARD_5),
        "relation-only-one-to-one-or-many" => Some(icons::RELATION_ONLY_ONE_TO_ONE_OR_MANY),
        "card-off" => Some(icons::CARD_OFF),
        "code-parentheses" => Some(icons::CODE_PARENTHESES),
        "skate" => Some(icons::SKATE),
        "axis-y-rotate-clockwise" => Some(icons::AXIS_Y_ROTATE_CLOCKWISE),
        "vector-point-minus" => Some(icons::VECTOR_POINT_MINUS),
        "seed-off" => Some(icons::SEED_OFF),
        "volume-vibrate" => Some(icons::VOLUME_VIBRATE),
        "sort-variant-lock-open" => Some(icons::SORT_VARIANT_LOCK_OPEN),
        "guy-fawkes-mask" => Some(icons::GUY_FAWKES_MASK),
        #[allow(deprecated)]
        "deviantart" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'deviantart' is deprecated.").print(py);
            }
            Some(icons::DEVIANTART)
        }
        "cog-off-outline" => Some(icons::COG_OFF_OUTLINE),
        "clippy" => Some(icons::CLIPPY),
        "bowl-mix-outline" => Some(icons::BOWL_MIX_OUTLINE),
        "movie-open-cog" => Some(icons::MOVIE_OPEN_COG),
        "speaker" => Some(icons::SPEAKER),
        "account-lock-open" => Some(icons::ACCOUNT_LOCK_OPEN),
        "shield-half" => Some(icons::SHIELD_HALF),
        "town-hall" => Some(icons::TOWN_HALL),
        "chef-hat" => Some(icons::CHEF_HAT),
        "invoice-text-edit-outline" => Some(icons::INVOICE_TEXT_EDIT_OUTLINE),
        "bell-check" => Some(icons::BELL_CHECK),
        "video-stabilization" => Some(icons::VIDEO_STABILIZATION),
        "view-split-horizontal" => Some(icons::VIEW_SPLIT_HORIZONTAL),
        "landslide-outline" => Some(icons::LANDSLIDE_OUTLINE),
        "hand-peace" => Some(icons::HAND_PEACE),
        _ => None,
    }
}
