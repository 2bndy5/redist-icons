// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_11(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "mace" => Some(icons::MACE),
        "hub" => Some(icons::HUB),
        "bullhorn" => Some(icons::BULLHORN),
        "folder-download-outline" => Some(icons::FOLDER_DOWNLOAD_OUTLINE),
        "binoculars" => Some(icons::BINOCULARS),
        "table" => Some(icons::TABLE),
        "cart-heart" => Some(icons::CART_HEART),
        "comment-processing" => Some(icons::COMMENT_PROCESSING),
        "folder-key-network" => Some(icons::FOLDER_KEY_NETWORK),
        "book-music" => Some(icons::BOOK_MUSIC),
        "food-fork-drink" => Some(icons::FOOD_FORK_DRINK),
        "phone-check-outline" => Some(icons::PHONE_CHECK_OUTLINE),
        #[allow(deprecated)]
        "microsoft-access" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'microsoft-access' is deprecated.")
                    .print(py);
            }
            Some(icons::MICROSOFT_ACCESS)
        }
        "sync-alert" => Some(icons::SYNC_ALERT),
        "compass-outline" => Some(icons::COMPASS_OUTLINE),
        "liquid-spot" => Some(icons::LIQUID_SPOT),
        "music-rest-sixteenth" => Some(icons::MUSIC_REST_SIXTEENTH),
        "account-multiple" => Some(icons::ACCOUNT_MULTIPLE),
        "karate" => Some(icons::KARATE),
        "relation-one-to-one-or-many" => Some(icons::RELATION_ONE_TO_ONE_OR_MANY),
        "temple-buddhist-outline" => Some(icons::TEMPLE_BUDDHIST_OUTLINE),
        "airplane-takeoff" => Some(icons::AIRPLANE_TAKEOFF),
        "shoe-ballet" => Some(icons::SHOE_BALLET),
        "select-inverse" => Some(icons::SELECT_INVERSE),
        "piston" => Some(icons::PISTON),
        "menu-left-outline" => Some(icons::MENU_LEFT_OUTLINE),
        "fountain-pen" => Some(icons::FOUNTAIN_PEN),
        "perspective-less" => Some(icons::PERSPECTIVE_LESS),
        "briefcase" => Some(icons::BRIEFCASE),
        "calendar-lock-open" => Some(icons::CALENDAR_LOCK_OPEN),
        "qrcode-remove" => Some(icons::QRCODE_REMOVE),
        "alarm-light-outline" => Some(icons::ALARM_LIGHT_OUTLINE),
        "fridge-alert" => Some(icons::FRIDGE_ALERT),
        "tea-outline" => Some(icons::TEA_OUTLINE),
        "robot-vacuum-off" => Some(icons::ROBOT_VACUUM_OFF),
        "circle-small" => Some(icons::CIRCLE_SMALL),
        "map-marker-path" => Some(icons::MAP_MARKER_PATH),
        "face-mask" => Some(icons::FACE_MASK),
        "alpha-x-box" => Some(icons::ALPHA_X_BOX),
        "weight-pound" => Some(icons::WEIGHT_POUND),
        "alarm-light-off-outline" => Some(icons::ALARM_LIGHT_OFF_OUTLINE),
        "wave-arrow-down" => Some(icons::WAVE_ARROW_DOWN),
        "bell-minus-outline" => Some(icons::BELL_MINUS_OUTLINE),
        "check" => Some(icons::CHECK),
        "blender-outline" => Some(icons::BLENDER_OUTLINE),
        "invoice-arrow-right" => Some(icons::INVOICE_ARROW_RIGHT),
        "magnify-minus" => Some(icons::MAGNIFY_MINUS),
        "file-send" => Some(icons::FILE_SEND),
        "image-sync-outline" => Some(icons::IMAGE_SYNC_OUTLINE),
        "layers-edit" => Some(icons::LAYERS_EDIT),
        "mailbox-open-up" => Some(icons::MAILBOX_OPEN_UP),
        "file-document-edit-outline" => Some(icons::FILE_DOCUMENT_EDIT_OUTLINE),
        "star-box-multiple" => Some(icons::STAR_BOX_MULTIPLE),
        "store-plus" => Some(icons::STORE_PLUS),
        "cart-arrow-up" => Some(icons::CART_ARROW_UP),
        "find-replace" => Some(icons::FIND_REPLACE),
        "timer-marker-outline" => Some(icons::TIMER_MARKER_OUTLINE),
        "pine-tree-fire" => Some(icons::PINE_TREE_FIRE),
        "desk" => Some(icons::DESK),
        "alpha-i-box-outline" => Some(icons::ALPHA_I_BOX_OUTLINE),
        "store-minus" => Some(icons::STORE_MINUS),
        "bottle-tonic-plus" => Some(icons::BOTTLE_TONIC_PLUS),
        "projector-off" => Some(icons::PROJECTOR_OFF),
        "book-lock" => Some(icons::BOOK_LOCK),
        "content-save-cog-outline" => Some(icons::CONTENT_SAVE_COG_OUTLINE),
        "book-marker" => Some(icons::BOOK_MARKER),
        "shield-lock-open-outline" => Some(icons::SHIELD_LOCK_OPEN_OUTLINE),
        "arrow-bottom-right-thin-circle-outline" => {
            Some(icons::ARROW_BOTTOM_RIGHT_THIN_CIRCLE_OUTLINE)
        }
        "store-cog-outline" => Some(icons::STORE_COG_OUTLINE),
        "sun-wireless" => Some(icons::SUN_WIRELESS),
        "transmission-tower-export" => Some(icons::TRANSMISSION_TOWER_EXPORT),
        "tooltip-text-outline" => Some(icons::TOOLTIP_TEXT_OUTLINE),
        "music-note-half-dotted" => Some(icons::MUSIC_NOTE_HALF_DOTTED),
        "printer-pos-cog-outline" => Some(icons::PRINTER_POS_COG_OUTLINE),
        "message-badge" => Some(icons::MESSAGE_BADGE),
        "eye-arrow-right-outline" => Some(icons::EYE_ARROW_RIGHT_OUTLINE),
        "email-search" => Some(icons::EMAIL_SEARCH),
        "bike" => Some(icons::BIKE),
        "fire-station" => Some(icons::FIRE_STATION),
        #[allow(deprecated)]
        "gnome" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'gnome' is deprecated.").print(py);
            }
            Some(icons::GNOME)
        }
        "robot-vacuum-variant-off" => Some(icons::ROBOT_VACUUM_VARIANT_OFF),
        "necklace" => Some(icons::NECKLACE),
        "cellphone-arrow-down-variant" => Some(icons::CELLPHONE_ARROW_DOWN_VARIANT),
        "oil-level" => Some(icons::OIL_LEVEL),
        "bookmark-music-outline" => Some(icons::BOOKMARK_MUSIC_OUTLINE),
        "hanger" => Some(icons::HANGER),
        "delete-outline" => Some(icons::DELETE_OUTLINE),
        "clock-time-six-outline" => Some(icons::CLOCK_TIME_SIX_OUTLINE),
        "database-cog" => Some(icons::DATABASE_COG),
        "dice-d8" => Some(icons::DICE_D8),
        "wrap-disabled" => Some(icons::WRAP_DISABLED),
        "tag-arrow-down" => Some(icons::TAG_ARROW_DOWN),
        "clock-time-nine-outline" => Some(icons::CLOCK_TIME_NINE_OUTLINE),
        "track-light" => Some(icons::TRACK_LIGHT),
        "arrow-down-left-bold" => Some(icons::ARROW_DOWN_LEFT_BOLD),
        "numeric-9-plus-box" => Some(icons::NUMERIC_9_PLUS_BOX),
        "egg-easter" => Some(icons::EGG_EASTER),
        "trumpet" => Some(icons::TRUMPET),
        "book-clock" => Some(icons::BOOK_CLOCK),
        "car-shift-pattern" => Some(icons::CAR_SHIFT_PATTERN),
        "tape-measure" => Some(icons::TAPE_MEASURE),
        "chevron-up-box-outline" => Some(icons::CHEVRON_UP_BOX_OUTLINE),
        "theater" => Some(icons::THEATER),
        "selection-multiple-marker" => Some(icons::SELECTION_MULTIPLE_MARKER),
        "radioactive-circle" => Some(icons::RADIOACTIVE_CIRCLE),
        "rowing" => Some(icons::ROWING),
        "music-rest-half" => Some(icons::MUSIC_REST_HALF),
        "doorbell-video" => Some(icons::DOORBELL_VIDEO),
        "distribute-horizontal-center" => Some(icons::DISTRIBUTE_HORIZONTAL_CENTER),
        "flag-minus" => Some(icons::FLAG_MINUS),
        "book-multiple" => Some(icons::BOOK_MULTIPLE),
        "airplane-cog" => Some(icons::AIRPLANE_COG),
        "script-text-key-outline" => Some(icons::SCRIPT_TEXT_KEY_OUTLINE),
        "rickshaw" => Some(icons::RICKSHAW),
        "mother-nurse" => Some(icons::MOTHER_NURSE),
        "keg" => Some(icons::KEG),
        "flower-pollen" => Some(icons::FLOWER_POLLEN),
        "dolly" => Some(icons::DOLLY),
        "cards-diamond-outline" => Some(icons::CARDS_DIAMOND_OUTLINE),
        "skate-off" => Some(icons::SKATE_OFF),
        "barn" => Some(icons::BARN),
        "sunglasses" => Some(icons::SUNGLASSES),
        "archive-remove" => Some(icons::ARCHIVE_REMOVE),
        "sort-variant-off" => Some(icons::SORT_VARIANT_OFF),
        "cog" => Some(icons::COG),
        "emoticon-neutral-outline" => Some(icons::EMOTICON_NEUTRAL_OUTLINE),
        "axis-y-arrow" => Some(icons::AXIS_Y_ARROW),
        "flash" => Some(icons::FLASH),
        #[allow(deprecated)]
        "ethereum" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ethereum' is deprecated.").print(py);
            }
            Some(icons::ETHEREUM)
        }
        "vector-point-minus" => Some(icons::VECTOR_POINT_MINUS),
        "table-pivot" => Some(icons::TABLE_PIVOT),
        "hand-heart" => Some(icons::HAND_HEART),
        #[allow(deprecated)]
        "kickstarter" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'kickstarter' is deprecated.").print(py);
            }
            Some(icons::KICKSTARTER)
        }
        "telescope" => Some(icons::TELESCOPE),
        "clock-edit-outline" => Some(icons::CLOCK_EDIT_OUTLINE),
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "file-export" => Some(icons::FILE_EXPORT),
        "shark" => Some(icons::SHARK),
        "timer" => Some(icons::TIMER),
        "weather-snowy" => Some(icons::WEATHER_SNOWY),
        "eye-plus" => Some(icons::EYE_PLUS),
        "card-account-mail" => Some(icons::CARD_ACCOUNT_MAIL),
        "road-variant" => Some(icons::ROAD_VARIANT),
        "numeric-0-box-multiple-outline" => Some(icons::NUMERIC_0_BOX_MULTIPLE_OUTLINE),
        "toy-brick-remove" => Some(icons::TOY_BRICK_REMOVE),
        "anvil" => Some(icons::ANVIL),
        "alpha-h-box-outline" => Some(icons::ALPHA_H_BOX_OUTLINE),
        "keyboard-outline" => Some(icons::KEYBOARD_OUTLINE),
        "flask-empty-outline" => Some(icons::FLASK_EMPTY_OUTLINE),
        "label-multiple-outline" => Some(icons::LABEL_MULTIPLE_OUTLINE),
        #[allow(deprecated)]
        "facebook-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-gaming' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_GAMING)
        }
        "access-point-remove" => Some(icons::ACCESS_POINT_REMOVE),
        "alpha-x-circle-outline" => Some(icons::ALPHA_X_CIRCLE_OUTLINE),
        "handcuffs" => Some(icons::HANDCUFFS),
        "billboard" => Some(icons::BILLBOARD),
        "billiards" => Some(icons::BILLIARDS),
        "gate-xnor" => Some(icons::GATE_XNOR),
        "temple-hindu" => Some(icons::TEMPLE_HINDU),
        "roman-numeral-4" => Some(icons::ROMAN_NUMERAL_4),
        "mirror-rectangle" => Some(icons::MIRROR_RECTANGLE),
        "signal-hspa" => Some(icons::SIGNAL_HSPA),
        "bandage" => Some(icons::BANDAGE),
        "numeric-5-box-multiple" => Some(icons::NUMERIC_5_BOX_MULTIPLE),
        "relative-scale" => Some(icons::RELATIVE_SCALE),
        "fridge-industrial-alert" => Some(icons::FRIDGE_INDUSTRIAL_ALERT),
        "tilde" => Some(icons::TILDE),
        "grain" => Some(icons::GRAIN),
        "mouse-variant-off" => Some(icons::MOUSE_VARIANT_OFF),
        "set-center" => Some(icons::SET_CENTER),
        "arrow-left-circle-outline" => Some(icons::ARROW_LEFT_CIRCLE_OUTLINE),
        "arrow-down-thick" => Some(icons::ARROW_DOWN_THICK),
        "map-marker-minus-outline" => Some(icons::MAP_MARKER_MINUS_OUTLINE),
        "bus-school" => Some(icons::BUS_SCHOOL),
        "equal-box" => Some(icons::EQUAL_BOX),
        "incognito-circle-off" => Some(icons::INCOGNITO_CIRCLE_OFF),
        "weather-hurricane" => Some(icons::WEATHER_HURRICANE),
        "file-key" => Some(icons::FILE_KEY),
        "stretch-to-page" => Some(icons::STRETCH_TO_PAGE),
        "size-xxxl" => Some(icons::SIZE_XXXL),
        "minus-circle-off" => Some(icons::MINUS_CIRCLE_OFF),
        #[allow(deprecated)]
        "opera" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'opera' is deprecated.").print(py);
            }
            Some(icons::OPERA)
        }
        "pencil-circle" => Some(icons::PENCIL_CIRCLE),
        "alphabetical-variant" => Some(icons::ALPHABETICAL_VARIANT),
        "subtitles-outline" => Some(icons::SUBTITLES_OUTLINE),
        "timer-cancel-outline" => Some(icons::TIMER_CANCEL_OUTLINE),
        "compass-off-outline" => Some(icons::COMPASS_OFF_OUTLINE),
        "currency-sign" => Some(icons::CURRENCY_SIGN),
        "dice-3" => Some(icons::DICE_3),
        "trophy" => Some(icons::TROPHY),
        "hexagram-outline" => Some(icons::HEXAGRAM_OUTLINE),
        "sun-angle" => Some(icons::SUN_ANGLE),
        "set-square" => Some(icons::SET_SQUARE),
        "phone" => Some(icons::PHONE),
        "arrow-u-up-right" => Some(icons::ARROW_U_UP_RIGHT),
        "toy-brick-minus-outline" => Some(icons::TOY_BRICK_MINUS_OUTLINE),
        "tag-edit" => Some(icons::TAG_EDIT),
        "text-search" => Some(icons::TEXT_SEARCH),
        "chart-areaspline" => Some(icons::CHART_AREASPLINE),
        "flask-empty-minus-outline" => Some(icons::FLASK_EMPTY_MINUS_OUTLINE),
        "weather-windy" => Some(icons::WEATHER_WINDY),
        _ => None,
    }
}
