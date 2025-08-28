// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_21(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "lava-lamp" => Some(icons::LAVA_LAMP),
        "layers-edit" => Some(icons::LAYERS_EDIT),
        "layers-minus" => Some(icons::LAYERS_MINUS),
        "layers-off-outline" => Some(icons::LAYERS_OFF_OUTLINE),
        "layers-off" => Some(icons::LAYERS_OFF),
        "layers-outline" => Some(icons::LAYERS_OUTLINE),
        "layers-plus" => Some(icons::LAYERS_PLUS),
        "layers-remove" => Some(icons::LAYERS_REMOVE),
        "layers-search-outline" => Some(icons::LAYERS_SEARCH_OUTLINE),
        "layers-search" => Some(icons::LAYERS_SEARCH),
        "layers-triple-outline" => Some(icons::LAYERS_TRIPLE_OUTLINE),
        "layers-triple" => Some(icons::LAYERS_TRIPLE),
        "layers" => Some(icons::LAYERS),
        "lead-pencil" => Some(icons::LEAD_PENCIL),
        "leaf-circle-outline" => Some(icons::LEAF_CIRCLE_OUTLINE),
        "leaf-circle" => Some(icons::LEAF_CIRCLE),
        "leaf-maple-off" => Some(icons::LEAF_MAPLE_OFF),
        "leaf-maple" => Some(icons::LEAF_MAPLE),
        "leaf-off" => Some(icons::LEAF_OFF),
        "leaf" => Some(icons::LEAF),
        "leak-off" => Some(icons::LEAK_OFF),
        "leak" => Some(icons::LEAK),
        "lectern" => Some(icons::LECTERN),
        "led-off" => Some(icons::LED_OFF),
        "led-on" => Some(icons::LED_ON),
        "led-outline" => Some(icons::LED_OUTLINE),
        "led-strip-variant-off" => Some(icons::LED_STRIP_VARIANT_OFF),
        "led-strip-variant" => Some(icons::LED_STRIP_VARIANT),
        "led-strip" => Some(icons::LED_STRIP),
        "led-variant-off" => Some(icons::LED_VARIANT_OFF),
        "led-variant-on" => Some(icons::LED_VARIANT_ON),
        "led-variant-outline" => Some(icons::LED_VARIANT_OUTLINE),
        "leek" => Some(icons::LEEK),
        "less-than-or-equal" => Some(icons::LESS_THAN_OR_EQUAL),
        "less-than" => Some(icons::LESS_THAN),
        "library-outline" => Some(icons::LIBRARY_OUTLINE),
        "library-shelves" => Some(icons::LIBRARY_SHELVES),
        "library" => Some(icons::LIBRARY),
        "license" => Some(icons::LICENSE),
        "lifebuoy" => Some(icons::LIFEBUOY),
        "light-flood-down" => Some(icons::LIGHT_FLOOD_DOWN),
        "light-flood-up" => Some(icons::LIGHT_FLOOD_UP),
        "light-recessed" => Some(icons::LIGHT_RECESSED),
        "light-switch-off" => Some(icons::LIGHT_SWITCH_OFF),
        "light-switch" => Some(icons::LIGHT_SWITCH),
        "lightbulb-alert-outline" => Some(icons::LIGHTBULB_ALERT_OUTLINE),
        "lightbulb-alert" => Some(icons::LIGHTBULB_ALERT),
        "lightbulb-auto-outline" => Some(icons::LIGHTBULB_AUTO_OUTLINE),
        "lightbulb-auto" => Some(icons::LIGHTBULB_AUTO),
        "lightbulb-cfl-off" => Some(icons::LIGHTBULB_CFL_OFF),
        "lightbulb-cfl-spiral-off" => Some(icons::LIGHTBULB_CFL_SPIRAL_OFF),
        "lightbulb-cfl-spiral" => Some(icons::LIGHTBULB_CFL_SPIRAL),
        "lightbulb-cfl" => Some(icons::LIGHTBULB_CFL),
        "lightbulb-fluorescent-tube-outline" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE_OUTLINE),
        "lightbulb-fluorescent-tube" => Some(icons::LIGHTBULB_FLUORESCENT_TUBE),
        "lightbulb-group-off-outline" => Some(icons::LIGHTBULB_GROUP_OFF_OUTLINE),
        "lightbulb-group-off" => Some(icons::LIGHTBULB_GROUP_OFF),
        "lightbulb-group-outline" => Some(icons::LIGHTBULB_GROUP_OUTLINE),
        "lightbulb-group" => Some(icons::LIGHTBULB_GROUP),
        "lightbulb-multiple-off-outline" => Some(icons::LIGHTBULB_MULTIPLE_OFF_OUTLINE),
        "lightbulb-multiple-off" => Some(icons::LIGHTBULB_MULTIPLE_OFF),
        "lightbulb-multiple-outline" => Some(icons::LIGHTBULB_MULTIPLE_OUTLINE),
        "lightbulb-multiple" => Some(icons::LIGHTBULB_MULTIPLE),
        "lightbulb-night-outline" => Some(icons::LIGHTBULB_NIGHT_OUTLINE),
        "lightbulb-night" => Some(icons::LIGHTBULB_NIGHT),
        "lightbulb-off-outline" => Some(icons::LIGHTBULB_OFF_OUTLINE),
        "lightbulb-off" => Some(icons::LIGHTBULB_OFF),
        "lightbulb-on-10" => Some(icons::LIGHTBULB_ON_10),
        "lightbulb-on-20" => Some(icons::LIGHTBULB_ON_20),
        "lightbulb-on-30" => Some(icons::LIGHTBULB_ON_30),
        "lightbulb-on-40" => Some(icons::LIGHTBULB_ON_40),
        "lightbulb-on-50" => Some(icons::LIGHTBULB_ON_50),
        "lightbulb-on-60" => Some(icons::LIGHTBULB_ON_60),
        "lightbulb-on-70" => Some(icons::LIGHTBULB_ON_70),
        "lightbulb-on-80" => Some(icons::LIGHTBULB_ON_80),
        "lightbulb-on-90" => Some(icons::LIGHTBULB_ON_90),
        "lightbulb-on-outline" => Some(icons::LIGHTBULB_ON_OUTLINE),
        "lightbulb-on" => Some(icons::LIGHTBULB_ON),
        "lightbulb-outline" => Some(icons::LIGHTBULB_OUTLINE),
        "lightbulb-question-outline" => Some(icons::LIGHTBULB_QUESTION_OUTLINE),
        "lightbulb-question" => Some(icons::LIGHTBULB_QUESTION),
        "lightbulb-spot-off" => Some(icons::LIGHTBULB_SPOT_OFF),
        "lightbulb-spot" => Some(icons::LIGHTBULB_SPOT),
        "lightbulb-variant-outline" => Some(icons::LIGHTBULB_VARIANT_OUTLINE),
        "lightbulb-variant" => Some(icons::LIGHTBULB_VARIANT),
        "lightbulb" => Some(icons::LIGHTBULB),
        "lighthouse-on" => Some(icons::LIGHTHOUSE_ON),
        "lighthouse" => Some(icons::LIGHTHOUSE),
        "lightning-bolt-circle" => Some(icons::LIGHTNING_BOLT_CIRCLE),
        "lightning-bolt-outline" => Some(icons::LIGHTNING_BOLT_OUTLINE),
        "lightning-bolt" => Some(icons::LIGHTNING_BOLT),
        "line-scan" => Some(icons::LINE_SCAN),
        "lingerie" => Some(icons::LINGERIE),
        "link-box-outline" => Some(icons::LINK_BOX_OUTLINE),
        "link-box-variant-outline" => Some(icons::LINK_BOX_VARIANT_OUTLINE),
        "link-box-variant" => Some(icons::LINK_BOX_VARIANT),
        "link-box" => Some(icons::LINK_BOX),
        "link-circle-outline" => Some(icons::LINK_CIRCLE_OUTLINE),
        "link-circle" => Some(icons::LINK_CIRCLE),
        "link-edit" => Some(icons::LINK_EDIT),
        "link-lock" => Some(icons::LINK_LOCK),
        "link-off" => Some(icons::LINK_OFF),
        "link-plus" => Some(icons::LINK_PLUS),
        "link-variant-minus" => Some(icons::LINK_VARIANT_MINUS),
        "link-variant-off" => Some(icons::LINK_VARIANT_OFF),
        "link-variant-plus" => Some(icons::LINK_VARIANT_PLUS),
        "link-variant-remove" => Some(icons::LINK_VARIANT_REMOVE),
        "link-variant" => Some(icons::LINK_VARIANT),
        "link" => Some(icons::LINK),
        #[allow(deprecated)]
        "linkedin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linkedin' is deprecated.").print(py);
            }
            Some(icons::LINKEDIN)
        }
        #[allow(deprecated)]
        "linux-mint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux-mint' is deprecated.").print(py);
            }
            Some(icons::LINUX_MINT)
        }
        #[allow(deprecated)]
        "linux" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'linux' is deprecated.").print(py);
            }
            Some(icons::LINUX)
        }
        "lipstick" => Some(icons::LIPSTICK),
        "liquid-spot" => Some(icons::LIQUID_SPOT),
        "liquor" => Some(icons::LIQUOR),
        "list-box-outline" => Some(icons::LIST_BOX_OUTLINE),
        "list-box" => Some(icons::LIST_BOX),
        "list-status" => Some(icons::LIST_STATUS),
        #[allow(deprecated)]
        "litecoin" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'litecoin' is deprecated.").print(py);
            }
            Some(icons::LITECOIN)
        }
        "loading" => Some(icons::LOADING),
        "location-enter" => Some(icons::LOCATION_ENTER),
        "location-exit" => Some(icons::LOCATION_EXIT),
        "lock-alert-outline" => Some(icons::LOCK_ALERT_OUTLINE),
        "lock-alert" => Some(icons::LOCK_ALERT),
        "lock-check-outline" => Some(icons::LOCK_CHECK_OUTLINE),
        "lock-check" => Some(icons::LOCK_CHECK),
        "lock-clock" => Some(icons::LOCK_CLOCK),
        "lock-minus-outline" => Some(icons::LOCK_MINUS_OUTLINE),
        "lock-minus" => Some(icons::LOCK_MINUS),
        "lock-off-outline" => Some(icons::LOCK_OFF_OUTLINE),
        "lock-off" => Some(icons::LOCK_OFF),
        "lock-open-alert-outline" => Some(icons::LOCK_OPEN_ALERT_OUTLINE),
        "lock-open-alert" => Some(icons::LOCK_OPEN_ALERT),
        "lock-open-check-outline" => Some(icons::LOCK_OPEN_CHECK_OUTLINE),
        "lock-open-check" => Some(icons::LOCK_OPEN_CHECK),
        "lock-open-minus-outline" => Some(icons::LOCK_OPEN_MINUS_OUTLINE),
        "lock-open-minus" => Some(icons::LOCK_OPEN_MINUS),
        "lock-open-outline" => Some(icons::LOCK_OPEN_OUTLINE),
        "lock-open-plus-outline" => Some(icons::LOCK_OPEN_PLUS_OUTLINE),
        "lock-open-plus" => Some(icons::LOCK_OPEN_PLUS),
        "lock-open-remove-outline" => Some(icons::LOCK_OPEN_REMOVE_OUTLINE),
        "lock-open-remove" => Some(icons::LOCK_OPEN_REMOVE),
        "lock-open-variant-outline" => Some(icons::LOCK_OPEN_VARIANT_OUTLINE),
        "lock-open-variant" => Some(icons::LOCK_OPEN_VARIANT),
        "lock-open" => Some(icons::LOCK_OPEN),
        "lock-outline" => Some(icons::LOCK_OUTLINE),
        "lock-pattern" => Some(icons::LOCK_PATTERN),
        "lock-percent-open-outline" => Some(icons::LOCK_PERCENT_OPEN_OUTLINE),
        "lock-percent-open-variant-outline" => Some(icons::LOCK_PERCENT_OPEN_VARIANT_OUTLINE),
        "lock-percent-open-variant" => Some(icons::LOCK_PERCENT_OPEN_VARIANT),
        "lock-percent-open" => Some(icons::LOCK_PERCENT_OPEN),
        "lock-percent-outline" => Some(icons::LOCK_PERCENT_OUTLINE),
        "lock-percent" => Some(icons::LOCK_PERCENT),
        "lock-plus-outline" => Some(icons::LOCK_PLUS_OUTLINE),
        "lock-plus" => Some(icons::LOCK_PLUS),
        "lock-question" => Some(icons::LOCK_QUESTION),
        "lock-remove-outline" => Some(icons::LOCK_REMOVE_OUTLINE),
        "lock-remove" => Some(icons::LOCK_REMOVE),
        "lock-reset" => Some(icons::LOCK_RESET),
        "lock-smart" => Some(icons::LOCK_SMART),
        "lock" => Some(icons::LOCK),
        "locker-multiple" => Some(icons::LOCKER_MULTIPLE),
        "locker" => Some(icons::LOCKER),
        "login-variant" => Some(icons::LOGIN_VARIANT),
        "login" => Some(icons::LOGIN),
        "logout-variant" => Some(icons::LOGOUT_VARIANT),
        "logout" => Some(icons::LOGOUT),
        "longitude" => Some(icons::LONGITUDE),
        "looks" => Some(icons::LOOKS),
        "lotion-outline" => Some(icons::LOTION_OUTLINE),
        "lotion-plus-outline" => Some(icons::LOTION_PLUS_OUTLINE),
        "lotion-plus" => Some(icons::LOTION_PLUS),
        "lotion" => Some(icons::LOTION),
        "loupe" => Some(icons::LOUPE),
        #[allow(deprecated)]
        "lumx" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'lumx' is deprecated.").print(py);
            }
            Some(icons::LUMX)
        }
        "lungs" => Some(icons::LUNGS),
        "mace" => Some(icons::MACE),
        "magazine-pistol" => Some(icons::MAGAZINE_PISTOL),
        "magazine-rifle" => Some(icons::MAGAZINE_RIFLE),
        "magic-staff" => Some(icons::MAGIC_STAFF),
        "magnet-on" => Some(icons::MAGNET_ON),
        "magnet" => Some(icons::MAGNET),
        "magnify-close" => Some(icons::MAGNIFY_CLOSE),
        "magnify-expand" => Some(icons::MAGNIFY_EXPAND),
        "magnify-minus-cursor" => Some(icons::MAGNIFY_MINUS_CURSOR),
        "magnify-minus-outline" => Some(icons::MAGNIFY_MINUS_OUTLINE),
        "magnify-minus" => Some(icons::MAGNIFY_MINUS),
        "magnify-plus-cursor" => Some(icons::MAGNIFY_PLUS_CURSOR),
        "magnify-plus-outline" => Some(icons::MAGNIFY_PLUS_OUTLINE),
        "magnify-plus" => Some(icons::MAGNIFY_PLUS),
        "magnify-remove-cursor" => Some(icons::MAGNIFY_REMOVE_CURSOR),
        "magnify-remove-outline" => Some(icons::MAGNIFY_REMOVE_OUTLINE),
        "magnify-scan" => Some(icons::MAGNIFY_SCAN),
        "magnify" => Some(icons::MAGNIFY),
        "mail" => Some(icons::MAIL),
        "mailbox-open-outline" => Some(icons::MAILBOX_OPEN_OUTLINE),
        "mailbox-open-up-outline" => Some(icons::MAILBOX_OPEN_UP_OUTLINE),
        "mailbox-open-up" => Some(icons::MAILBOX_OPEN_UP),
        "mailbox-open" => Some(icons::MAILBOX_OPEN),
        "mailbox-outline" => Some(icons::MAILBOX_OUTLINE),
        _ => None,
    }
}
