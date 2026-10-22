// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_29(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "fence" => Some(icons::FENCE),
        "format-color-highlight" => Some(icons::FORMAT_COLOR_HIGHLIGHT),
        "palette-swatch-outline" => Some(icons::PALETTE_SWATCH_OUTLINE),
        "battery-charging-wireless-alert" => Some(icons::BATTERY_CHARGING_WIRELESS_ALERT),
        "credit-card-off" => Some(icons::CREDIT_CARD_OFF),
        "mouse" => Some(icons::MOUSE),
        "basket-unfill" => Some(icons::BASKET_UNFILL),
        "arrow-right-box" => Some(icons::ARROW_RIGHT_BOX),
        "image-sync-outline" => Some(icons::IMAGE_SYNC_OUTLINE),
        "tooltip" => Some(icons::TOOLTIP),
        "turtle" => Some(icons::TURTLE),
        "account-details-outline" => Some(icons::ACCOUNT_DETAILS_OUTLINE),
        "content-save-check-outline" => Some(icons::CONTENT_SAVE_CHECK_OUTLINE),
        "dice-d6-outline" => Some(icons::DICE_D6_OUTLINE),
        "emoticon-sad-outline" => Some(icons::EMOTICON_SAD_OUTLINE),
        "file-upload" => Some(icons::FILE_UPLOAD),
        "numeric-0-box-outline" => Some(icons::NUMERIC_0_BOX_OUTLINE),
        "relation-zero-or-many-to-only-one" => Some(icons::RELATION_ZERO_OR_MANY_TO_ONLY_ONE),
        "database-edit" => Some(icons::DATABASE_EDIT),
        "signal-hspa" => Some(icons::SIGNAL_HSPA),
        "archive-settings-outline" => Some(icons::ARCHIVE_SETTINGS_OUTLINE),
        #[allow(deprecated)]
        "microsoft-xbox-controller-battery-medium" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                    "The icon 'microsoft-xbox-controller-battery-medium' is deprecated.",
                )
                .print(py);
            }
            Some(icons::MICROSOFT_XBOX_CONTROLLER_BATTERY_MEDIUM)
        }
        "send-variant-clock-outline" => Some(icons::SEND_VARIANT_CLOCK_OUTLINE),
        "smog" => Some(icons::SMOG),
        "arrow-bottom-left-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_LEFT_THIN_CIRCLE_OUTLINE)
        }
        "swap-horizontal-bold" => Some(icons::SWAP_HORIZONTAL_BOLD),
        "gantry-crane" => Some(icons::GANTRY_CRANE),
        "printer-3d-nozzle-heat-outline" => Some(icons::PRINTER_3D_NOZZLE_HEAT_OUTLINE),
        "account-tie-voice-off-outline" => Some(icons::ACCOUNT_TIE_VOICE_OFF_OUTLINE),
        "train-car-hopper-covered" => Some(icons::TRAIN_CAR_HOPPER_COVERED),
        "water-percent-alert" => Some(icons::WATER_PERCENT_ALERT),
        "weather-windy-variant" => Some(icons::WEATHER_WINDY_VARIANT),
        "rewind-10" => Some(icons::REWIND_10),
        "middleware-outline" => Some(icons::MIDDLEWARE_OUTLINE),
        "share-variant" => Some(icons::SHARE_VARIANT),
        "numeric-5-box" => Some(icons::NUMERIC_5_BOX),
        "fan-speed-1" => Some(icons::FAN_SPEED_1),
        "home-clock-outline" => Some(icons::HOME_CLOCK_OUTLINE),
        "select-drag" => Some(icons::SELECT_DRAG),
        "invoice-multiple" => Some(icons::INVOICE_MULTIPLE),
        "file-star" => Some(icons::FILE_STAR),
        "numeric-9-plus-box" => Some(icons::NUMERIC_9_PLUS_BOX),
        "projector-off" => Some(icons::PROJECTOR_OFF),
        "vector-square-edit" => Some(icons::VECTOR_SQUARE_EDIT),
        "calendar-search-outline" => Some(icons::CALENDAR_SEARCH_OUTLINE),
        "network-strength-4-cog" => Some(icons::NETWORK_STRENGTH_4_COG),
        "thermostat-auto" => Some(icons::THERMOSTAT_AUTO),
        #[allow(deprecated)]
        "disqus" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'disqus' is deprecated.").print(py);
            }
            Some(icons::DISQUS)
        }
        "format-header-decrease" => Some(icons::FORMAT_HEADER_DECREASE),
        "power-socket-uk" => Some(icons::POWER_SOCKET_UK),
        "account-minus" => Some(icons::ACCOUNT_MINUS),
        "surround-sound-2-0" => Some(icons::SURROUND_SOUND_2_0),
        "alpha-y" => Some(icons::ALPHA_Y),
        "sticker-plus" => Some(icons::STICKER_PLUS),
        "account-box-minus-outline" => Some(icons::ACCOUNT_BOX_MINUS_OUTLINE),
        "video-image" => Some(icons::VIDEO_IMAGE),
        "comment-minus" => Some(icons::COMMENT_MINUS),
        "rocket-outline" => Some(icons::ROCKET_OUTLINE),
        "rotate-left-variant" => Some(icons::ROTATE_LEFT_VARIANT),
        "dice-multiple-outline" => Some(icons::DICE_MULTIPLE_OUTLINE),
        "account-tie-hat" => Some(icons::ACCOUNT_TIE_HAT),
        "archive-remove" => Some(icons::ARCHIVE_REMOVE),
        #[allow(deprecated)]
        "angularjs" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'angularjs' is deprecated.").print(py);
            }
            Some(icons::ANGULARJS)
        }
        "transfer-down" => Some(icons::TRANSFER_DOWN),
        "lightbulb-on-30" => Some(icons::LIGHTBULB_ON_30),
        "hand-front-right" => Some(icons::HAND_FRONT_RIGHT),
        "transmission-tower" => Some(icons::TRANSMISSION_TOWER),
        "format-text-wrapping-overflow" => Some(icons::FORMAT_TEXT_WRAPPING_OVERFLOW),
        "crop-free" => Some(icons::CROP_FREE),
        "chevron-up-circle-outline" => Some(icons::CHEVRON_UP_CIRCLE_OUTLINE),
        "translate" => Some(icons::TRANSLATE),
        "shopping-music" => Some(icons::SHOPPING_MUSIC),
        "file-tree" => Some(icons::FILE_TREE),
        "message-flash" => Some(icons::MESSAGE_FLASH),
        "faucet" => Some(icons::FAUCET),
        "cellphone-text" => Some(icons::CELLPHONE_TEXT),
        "baseball" => Some(icons::BASEBALL),
        "window-closed-variant" => Some(icons::WINDOW_CLOSED_VARIANT),
        "wifi-strength-lock-open-outline" => Some(icons::WIFI_STRENGTH_LOCK_OPEN_OUTLINE),
        "hospital-building" => Some(icons::HOSPITAL_BUILDING),
        "abjad-hebrew" => Some(icons::ABJAD_HEBREW),
        "glass-pint-outline" => Some(icons::GLASS_PINT_OUTLINE),
        "bookmark-remove-outline" => Some(icons::BOOKMARK_REMOVE_OUTLINE),
        "emoticon-plus-outline" => Some(icons::EMOTICON_PLUS_OUTLINE),
        "script-outline" => Some(icons::SCRIPT_OUTLINE),
        "dog-side" => Some(icons::DOG_SIDE),
        "numeric-4-circle" => Some(icons::NUMERIC_4_CIRCLE),
        "island-variant" => Some(icons::ISLAND_VARIANT),
        "arrow-down-thick" => Some(icons::ARROW_DOWN_THICK),
        "printer-pos-minus" => Some(icons::PRINTER_POS_MINUS),
        "barcode" => Some(icons::BARCODE),
        "clock-time-eight" => Some(icons::CLOCK_TIME_EIGHT),
        "bank-remove" => Some(icons::BANK_REMOVE),
        "land-plots-circle-variant" => Some(icons::LAND_PLOTS_CIRCLE_VARIANT),
        "clipboard-remove-outline" => Some(icons::CLIPBOARD_REMOVE_OUTLINE),
        "hand-heart" => Some(icons::HAND_HEART),
        "slope-downhill" => Some(icons::SLOPE_DOWNHILL),
        "cake" => Some(icons::CAKE),
        "battery-charging" => Some(icons::BATTERY_CHARGING),
        "chili-hot" => Some(icons::CHILI_HOT),
        "shield-sword" => Some(icons::SHIELD_SWORD),
        "anchor" => Some(icons::ANCHOR),
        #[allow(deprecated)]
        "bitcoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'bitcoin' is deprecated.").print(py);
            }
            Some(icons::BITCOIN)
        }
        "motorbike-electric" => Some(icons::MOTORBIKE_ELECTRIC),
        "sheep" => Some(icons::SHEEP),
        #[allow(deprecated)]
        "dot-net" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'dot-net' is deprecated.").print(py);
            }
            Some(icons::DOT_NET)
        }
        "vector-difference-ab" => Some(icons::VECTOR_DIFFERENCE_AB),
        "email-arrow-right-outline" => Some(icons::EMAIL_ARROW_RIGHT_OUTLINE),
        "food" => Some(icons::FOOD),
        "pig-variant" => Some(icons::PIG_VARIANT),
        "axis-x-rotate-counterclockwise" => Some(icons::AXIS_X_ROTATE_COUNTERCLOCKWISE),
        "piano" => Some(icons::PIANO),
        "numeric-5-box-multiple-outline" => Some(icons::NUMERIC_5_BOX_MULTIPLE_OUTLINE),
        "book-account-outline" => Some(icons::BOOK_ACCOUNT_OUTLINE),
        "roman-numeral-1" => Some(icons::ROMAN_NUMERAL_1),
        "map-marker-radius" => Some(icons::MAP_MARKER_RADIUS),
        "cellphone" => Some(icons::CELLPHONE),
        "creation" => Some(icons::CREATION),
        #[allow(deprecated)]
        "apple-finder" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'apple-finder' is deprecated.").print(py);
            }
            Some(icons::APPLE_FINDER)
        }
        "keyboard-f12" => Some(icons::KEYBOARD_F12),
        "format-text-rotation-none" => Some(icons::FORMAT_TEXT_ROTATION_NONE),
        "egg-outline" => Some(icons::EGG_OUTLINE),
        "play-box" => Some(icons::PLAY_BOX),
        "database-off" => Some(icons::DATABASE_OFF),
        "dns-outline" => Some(icons::DNS_OUTLINE),
        "resistor-nodes" => Some(icons::RESISTOR_NODES),
        "cards-playing-spade-outline" => Some(icons::CARDS_PLAYING_SPADE_OUTLINE),
        "map-minus" => Some(icons::MAP_MINUS),
        "file-lock" => Some(icons::FILE_LOCK),
        "fridge-variant-off" => Some(icons::FRIDGE_VARIANT_OFF),
        "align-vertical-distribute" => Some(icons::ALIGN_VERTICAL_DISTRIBUTE),
        "store-marker-outline" => Some(icons::STORE_MARKER_OUTLINE),
        "alpha-y-box" => Some(icons::ALPHA_Y_BOX),
        "alpha-n-box" => Some(icons::ALPHA_N_BOX),
        "comment-quote-outline" => Some(icons::COMMENT_QUOTE_OUTLINE),
        "view-list-outline" => Some(icons::VIEW_LIST_OUTLINE),
        "poll" => Some(icons::POLL),
        "close-circle-outline" => Some(icons::CLOSE_CIRCLE_OUTLINE),
        "video-minus" => Some(icons::VIDEO_MINUS),
        "office-building-minus" => Some(icons::OFFICE_BUILDING_MINUS),
        "train-car" => Some(icons::TRAIN_CAR),
        "emoticon-minus-outline" => Some(icons::EMOTICON_MINUS_OUTLINE),
        "alpha-a" => Some(icons::ALPHA_A),
        "voicemail" => Some(icons::VOICEMAIL),
        "credit-card-refresh" => Some(icons::CREDIT_CARD_REFRESH),
        "drag-vertical-variant" => Some(icons::DRAG_VERTICAL_VARIANT),
        "account-child" => Some(icons::ACCOUNT_CHILD),
        "file-table-box-multiple-outline" => Some(icons::FILE_TABLE_BOX_MULTIPLE_OUTLINE),
        "vote-outline" => Some(icons::VOTE_OUTLINE),
        "star-cog" => Some(icons::STAR_COG),
        "account-voice-off" => Some(icons::ACCOUNT_VOICE_OFF),
        "handcuffs" => Some(icons::HANDCUFFS),
        "toggle-switch" => Some(icons::TOGGLE_SWITCH),
        "rss" => Some(icons::RSS),
        "cloud-clock" => Some(icons::CLOUD_CLOCK),
        "garage" => Some(icons::GARAGE),
        "briefcase-variant" => Some(icons::BRIEFCASE_VARIANT),
        "account-multiple-check" => Some(icons::ACCOUNT_MULTIPLE_CHECK),
        "rodent" => Some(icons::RODENT),
        "timer-off" => Some(icons::TIMER_OFF),
        "keyboard-close" => Some(icons::KEYBOARD_CLOSE),
        "format-quote-open-outline" => Some(icons::FORMAT_QUOTE_OPEN_OUTLINE),
        "not-equal" => Some(icons::NOT_EQUAL),
        "passport-minus" => Some(icons::PASSPORT_MINUS),
        "pail-outline" => Some(icons::PAIL_OUTLINE),
        "star-remove-outline" => Some(icons::STAR_REMOVE_OUTLINE),
        "cursor-default-click" => Some(icons::CURSOR_DEFAULT_CLICK),
        "mower-bag-on" => Some(icons::MOWER_BAG_ON),
        "draw" => Some(icons::DRAW),
        "archive-cog-outline" => Some(icons::ARCHIVE_COG_OUTLINE),
        "pencil-plus" => Some(icons::PENCIL_PLUS),
        "comment-bookmark" => Some(icons::COMMENT_BOOKMARK),
        "email-search-outline" => Some(icons::EMAIL_SEARCH_OUTLINE),
        "numeric-8-box" => Some(icons::NUMERIC_8_BOX),
        "file-cog" => Some(icons::FILE_COG),
        "numeric-9-box" => Some(icons::NUMERIC_9_BOX),
        "fast-forward" => Some(icons::FAST_FORWARD),
        "pill-multiple" => Some(icons::PILL_MULTIPLE),
        "page-layout-sidebar-right" => Some(icons::PAGE_LAYOUT_SIDEBAR_RIGHT),
        "clipboard-arrow-up-outline" => Some(icons::CLIPBOARD_ARROW_UP_OUTLINE),
        "toothbrush-electric" => Some(icons::TOOTHBRUSH_ELECTRIC),
        "fan" => Some(icons::FAN),
        "tally-mark-2" => Some(icons::TALLY_MARK_2),
        "watering-can-outline" => Some(icons::WATERING_CAN_OUTLINE),
        "numeric-4-box-multiple-outline" => Some(icons::NUMERIC_4_BOX_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "language-haskell" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'language-haskell' is deprecated.")
                    .print(py);
            }
            Some(icons::LANGUAGE_HASKELL)
        }
        "account-plus" => Some(icons::ACCOUNT_PLUS),
        "map-marker-left" => Some(icons::MAP_MARKER_LEFT),
        "standard-definition" => Some(icons::STANDARD_DEFINITION),
        "router-wireless-settings" => Some(icons::ROUTER_WIRELESS_SETTINGS),
        "cloud-refresh-outline" => Some(icons::CLOUD_REFRESH_OUTLINE),
        "weather-partly-snowy-rainy" => Some(icons::WEATHER_PARTLY_SNOWY_RAINY),
        "numeric-3" => Some(icons::NUMERIC_3),
        "arrow-collapse-right" => Some(icons::ARROW_COLLAPSE_RIGHT),
        "patio-heater" => Some(icons::PATIO_HEATER),
        "pail-remove-outline" => Some(icons::PAIL_REMOVE_OUTLINE),
        "alpha-p-box" => Some(icons::ALPHA_P_BOX),
        "eye-remove" => Some(icons::EYE_REMOVE),
        "folder-image" => Some(icons::FOLDER_IMAGE),
        "dice-d8-outline" => Some(icons::DICE_D8_OUTLINE),
        _ => None,
    }
}
