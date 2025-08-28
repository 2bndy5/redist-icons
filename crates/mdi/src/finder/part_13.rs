// This file was generated. DO NOT EDIT.
use crate::{Icon, icons};

#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub(super) fn find_part_13(#[cfg(feature = "pyo3")] py: Python, slug: &str) -> Option<Icon> {
    match slug {
        "email-edit-outline" => Some(icons::EMAIL_EDIT_OUTLINE),
        "email-edit" => Some(icons::EMAIL_EDIT),
        "email-fast-outline" => Some(icons::EMAIL_FAST_OUTLINE),
        "email-fast" => Some(icons::EMAIL_FAST),
        "email-heart-outline" => Some(icons::EMAIL_HEART_OUTLINE),
        "email-lock-outline" => Some(icons::EMAIL_LOCK_OUTLINE),
        "email-lock" => Some(icons::EMAIL_LOCK),
        "email-mark-as-unread" => Some(icons::EMAIL_MARK_AS_UNREAD),
        "email-minus-outline" => Some(icons::EMAIL_MINUS_OUTLINE),
        "email-minus" => Some(icons::EMAIL_MINUS),
        "email-multiple-outline" => Some(icons::EMAIL_MULTIPLE_OUTLINE),
        "email-multiple" => Some(icons::EMAIL_MULTIPLE),
        "email-newsletter" => Some(icons::EMAIL_NEWSLETTER),
        "email-off-outline" => Some(icons::EMAIL_OFF_OUTLINE),
        "email-off" => Some(icons::EMAIL_OFF),
        "email-open-heart-outline" => Some(icons::EMAIL_OPEN_HEART_OUTLINE),
        "email-open-multiple-outline" => Some(icons::EMAIL_OPEN_MULTIPLE_OUTLINE),
        "email-open-multiple" => Some(icons::EMAIL_OPEN_MULTIPLE),
        "email-open-outline" => Some(icons::EMAIL_OPEN_OUTLINE),
        "email-open" => Some(icons::EMAIL_OPEN),
        "email-outline" => Some(icons::EMAIL_OUTLINE),
        "email-plus-outline" => Some(icons::EMAIL_PLUS_OUTLINE),
        "email-plus" => Some(icons::EMAIL_PLUS),
        "email-remove-outline" => Some(icons::EMAIL_REMOVE_OUTLINE),
        "email-remove" => Some(icons::EMAIL_REMOVE),
        "email-seal-outline" => Some(icons::EMAIL_SEAL_OUTLINE),
        "email-seal" => Some(icons::EMAIL_SEAL),
        "email-search-outline" => Some(icons::EMAIL_SEARCH_OUTLINE),
        "email-search" => Some(icons::EMAIL_SEARCH),
        "email-sync-outline" => Some(icons::EMAIL_SYNC_OUTLINE),
        "email-sync" => Some(icons::EMAIL_SYNC),
        "email-variant" => Some(icons::EMAIL_VARIANT),
        "email" => Some(icons::EMAIL),
        #[allow(deprecated)]
        "ember" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ember' is deprecated.").print(py);
            }
            Some(icons::EMBER)
        }
        #[allow(deprecated)]
        "emby" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'emby' is deprecated.").print(py);
            }
            Some(icons::EMBY)
        }
        "emoticon-angry-outline" => Some(icons::EMOTICON_ANGRY_OUTLINE),
        "emoticon-angry" => Some(icons::EMOTICON_ANGRY),
        "emoticon-confused-outline" => Some(icons::EMOTICON_CONFUSED_OUTLINE),
        "emoticon-confused" => Some(icons::EMOTICON_CONFUSED),
        "emoticon-cool-outline" => Some(icons::EMOTICON_COOL_OUTLINE),
        "emoticon-cool" => Some(icons::EMOTICON_COOL),
        "emoticon-cry-outline" => Some(icons::EMOTICON_CRY_OUTLINE),
        "emoticon-cry" => Some(icons::EMOTICON_CRY),
        "emoticon-dead-outline" => Some(icons::EMOTICON_DEAD_OUTLINE),
        "emoticon-dead" => Some(icons::EMOTICON_DEAD),
        "emoticon-devil-outline" => Some(icons::EMOTICON_DEVIL_OUTLINE),
        "emoticon-devil" => Some(icons::EMOTICON_DEVIL),
        "emoticon-excited-outline" => Some(icons::EMOTICON_EXCITED_OUTLINE),
        "emoticon-excited" => Some(icons::EMOTICON_EXCITED),
        "emoticon-frown-outline" => Some(icons::EMOTICON_FROWN_OUTLINE),
        "emoticon-frown" => Some(icons::EMOTICON_FROWN),
        "emoticon-happy-outline" => Some(icons::EMOTICON_HAPPY_OUTLINE),
        "emoticon-happy" => Some(icons::EMOTICON_HAPPY),
        "emoticon-kiss-outline" => Some(icons::EMOTICON_KISS_OUTLINE),
        "emoticon-kiss" => Some(icons::EMOTICON_KISS),
        "emoticon-lol-outline" => Some(icons::EMOTICON_LOL_OUTLINE),
        "emoticon-lol" => Some(icons::EMOTICON_LOL),
        "emoticon-minus-outline" => Some(icons::EMOTICON_MINUS_OUTLINE),
        "emoticon-minus" => Some(icons::EMOTICON_MINUS),
        "emoticon-neutral-outline" => Some(icons::EMOTICON_NEUTRAL_OUTLINE),
        "emoticon-neutral" => Some(icons::EMOTICON_NEUTRAL),
        "emoticon-outline" => Some(icons::EMOTICON_OUTLINE),
        "emoticon-plus-outline" => Some(icons::EMOTICON_PLUS_OUTLINE),
        "emoticon-plus" => Some(icons::EMOTICON_PLUS),
        "emoticon-poop-outline" => Some(icons::EMOTICON_POOP_OUTLINE),
        "emoticon-poop" => Some(icons::EMOTICON_POOP),
        "emoticon-remove-outline" => Some(icons::EMOTICON_REMOVE_OUTLINE),
        "emoticon-remove" => Some(icons::EMOTICON_REMOVE),
        "emoticon-sad-outline" => Some(icons::EMOTICON_SAD_OUTLINE),
        "emoticon-sad" => Some(icons::EMOTICON_SAD),
        "emoticon-sick-outline" => Some(icons::EMOTICON_SICK_OUTLINE),
        "emoticon-sick" => Some(icons::EMOTICON_SICK),
        "emoticon-tongue-outline" => Some(icons::EMOTICON_TONGUE_OUTLINE),
        "emoticon-tongue" => Some(icons::EMOTICON_TONGUE),
        "emoticon-wink-outline" => Some(icons::EMOTICON_WINK_OUTLINE),
        "emoticon-wink" => Some(icons::EMOTICON_WINK),
        "emoticon" => Some(icons::EMOTICON),
        "engine-off-outline" => Some(icons::ENGINE_OFF_OUTLINE),
        "engine-off" => Some(icons::ENGINE_OFF),
        "engine-outline" => Some(icons::ENGINE_OUTLINE),
        "engine" => Some(icons::ENGINE),
        "epsilon" => Some(icons::EPSILON),
        "equal-box" => Some(icons::EQUAL_BOX),
        "equal" => Some(icons::EQUAL),
        "equalizer-outline" => Some(icons::EQUALIZER_OUTLINE),
        "equalizer" => Some(icons::EQUALIZER),
        "eraser-variant" => Some(icons::ERASER_VARIANT),
        "eraser" => Some(icons::ERASER),
        "escalator-box" => Some(icons::ESCALATOR_BOX),
        "escalator-down" => Some(icons::ESCALATOR_DOWN),
        "escalator-up" => Some(icons::ESCALATOR_UP),
        "escalator" => Some(icons::ESCALATOR),
        #[allow(deprecated)]
        "eslint" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'eslint' is deprecated.").print(py);
            }
            Some(icons::ESLINT)
        }
        "et" => Some(icons::ET),
        #[allow(deprecated)]
        "ethereum" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'ethereum' is deprecated.").print(py);
            }
            Some(icons::ETHEREUM)
        }
        "ethernet-cable-off" => Some(icons::ETHERNET_CABLE_OFF),
        "ethernet-cable" => Some(icons::ETHERNET_CABLE),
        "ethernet-off" => Some(icons::ETHERNET_OFF),
        "ethernet" => Some(icons::ETHERNET),
        "ev-plug-ccs1" => Some(icons::EV_PLUG_CCS1),
        "ev-plug-ccs2" => Some(icons::EV_PLUG_CCS2),
        "ev-plug-chademo" => Some(icons::EV_PLUG_CHADEMO),
        "ev-plug-tesla" => Some(icons::EV_PLUG_TESLA),
        "ev-plug-type1" => Some(icons::EV_PLUG_TYPE1),
        "ev-plug-type2" => Some(icons::EV_PLUG_TYPE2),
        "ev-station" => Some(icons::EV_STATION),
        #[allow(deprecated)]
        "evernote" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'evernote' is deprecated.").print(py);
            }
            Some(icons::EVERNOTE)
        }
        "excavator" => Some(icons::EXCAVATOR),
        "exclamation-thick" => Some(icons::EXCLAMATION_THICK),
        "exclamation" => Some(icons::EXCLAMATION),
        "exit-run" => Some(icons::EXIT_RUN),
        "exit-to-app" => Some(icons::EXIT_TO_APP),
        "expand-all-outline" => Some(icons::EXPAND_ALL_OUTLINE),
        "expand-all" => Some(icons::EXPAND_ALL),
        "expansion-card-variant" => Some(icons::EXPANSION_CARD_VARIANT),
        "expansion-card" => Some(icons::EXPANSION_CARD),
        "exponent-box" => Some(icons::EXPONENT_BOX),
        "exponent" => Some(icons::EXPONENT),
        "export-variant" => Some(icons::EXPORT_VARIANT),
        "export" => Some(icons::EXPORT),
        "eye-arrow-left-outline" => Some(icons::EYE_ARROW_LEFT_OUTLINE),
        "eye-arrow-left" => Some(icons::EYE_ARROW_LEFT),
        "eye-arrow-right-outline" => Some(icons::EYE_ARROW_RIGHT_OUTLINE),
        "eye-arrow-right" => Some(icons::EYE_ARROW_RIGHT),
        "eye-check-outline" => Some(icons::EYE_CHECK_OUTLINE),
        "eye-check" => Some(icons::EYE_CHECK),
        "eye-circle-outline" => Some(icons::EYE_CIRCLE_OUTLINE),
        "eye-circle" => Some(icons::EYE_CIRCLE),
        "eye-closed" => Some(icons::EYE_CLOSED),
        "eye-lock-open-outline" => Some(icons::EYE_LOCK_OPEN_OUTLINE),
        "eye-lock-open" => Some(icons::EYE_LOCK_OPEN),
        "eye-lock-outline" => Some(icons::EYE_LOCK_OUTLINE),
        "eye-lock" => Some(icons::EYE_LOCK),
        "eye-minus-outline" => Some(icons::EYE_MINUS_OUTLINE),
        "eye-minus" => Some(icons::EYE_MINUS),
        "eye-off-outline" => Some(icons::EYE_OFF_OUTLINE),
        "eye-off" => Some(icons::EYE_OFF),
        "eye-outline" => Some(icons::EYE_OUTLINE),
        "eye-plus-outline" => Some(icons::EYE_PLUS_OUTLINE),
        "eye-plus" => Some(icons::EYE_PLUS),
        "eye-refresh-outline" => Some(icons::EYE_REFRESH_OUTLINE),
        "eye-refresh" => Some(icons::EYE_REFRESH),
        "eye-remove-outline" => Some(icons::EYE_REMOVE_OUTLINE),
        "eye-remove" => Some(icons::EYE_REMOVE),
        "eye-settings-outline" => Some(icons::EYE_SETTINGS_OUTLINE),
        "eye-settings" => Some(icons::EYE_SETTINGS),
        "eye" => Some(icons::EYE),
        "eyedropper-minus" => Some(icons::EYEDROPPER_MINUS),
        "eyedropper-off" => Some(icons::EYEDROPPER_OFF),
        "eyedropper-plus" => Some(icons::EYEDROPPER_PLUS),
        "eyedropper-remove" => Some(icons::EYEDROPPER_REMOVE),
        "eyedropper-variant" => Some(icons::EYEDROPPER_VARIANT),
        "eyedropper" => Some(icons::EYEDROPPER),
        "face-agent" => Some(icons::FACE_AGENT),
        "face-man-outline" => Some(icons::FACE_MAN_OUTLINE),
        "face-man-profile" => Some(icons::FACE_MAN_PROFILE),
        "face-man-shimmer-outline" => Some(icons::FACE_MAN_SHIMMER_OUTLINE),
        "face-man-shimmer" => Some(icons::FACE_MAN_SHIMMER),
        "face-man" => Some(icons::FACE_MAN),
        "face-mask-outline" => Some(icons::FACE_MASK_OUTLINE),
        "face-mask" => Some(icons::FACE_MASK),
        "face-recognition" => Some(icons::FACE_RECOGNITION),
        "face-woman-outline" => Some(icons::FACE_WOMAN_OUTLINE),
        "face-woman-profile" => Some(icons::FACE_WOMAN_PROFILE),
        "face-woman-shimmer-outline" => Some(icons::FACE_WOMAN_SHIMMER_OUTLINE),
        "face-woman-shimmer" => Some(icons::FACE_WOMAN_SHIMMER),
        "face-woman" => Some(icons::FACE_WOMAN),
        #[allow(deprecated)]
        "facebook-gaming" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-gaming' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_GAMING)
        }
        #[allow(deprecated)]
        "facebook-messenger" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-messenger' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_MESSENGER)
        }
        #[allow(deprecated)]
        "facebook-workplace" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook-workplace' is deprecated.")
                    .print(py);
            }
            Some(icons::FACEBOOK_WORKPLACE)
        }
        #[allow(deprecated)]
        "facebook" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err("The icon 'facebook' is deprecated.").print(py);
            }
            Some(icons::FACEBOOK)
        }
        "factory" => Some(icons::FACTORY),
        "family-tree" => Some(icons::FAMILY_TREE),
        "fan-alert" => Some(icons::FAN_ALERT),
        "fan-auto" => Some(icons::FAN_AUTO),
        "fan-chevron-down" => Some(icons::FAN_CHEVRON_DOWN),
        "fan-chevron-up" => Some(icons::FAN_CHEVRON_UP),
        "fan-clock" => Some(icons::FAN_CLOCK),
        "fan-minus" => Some(icons::FAN_MINUS),
        "fan-off" => Some(icons::FAN_OFF),
        "fan-plus" => Some(icons::FAN_PLUS),
        "fan-remove" => Some(icons::FAN_REMOVE),
        "fan-speed-1" => Some(icons::FAN_SPEED_1),
        "fan-speed-2" => Some(icons::FAN_SPEED_2),
        "fan-speed-3" => Some(icons::FAN_SPEED_3),
        "fan" => Some(icons::FAN),
        "fast-forward-10" => Some(icons::FAST_FORWARD_10),
        "fast-forward-15" => Some(icons::FAST_FORWARD_15),
        "fast-forward-30" => Some(icons::FAST_FORWARD_30),
        "fast-forward-45" => Some(icons::FAST_FORWARD_45),
        "fast-forward-5" => Some(icons::FAST_FORWARD_5),
        "fast-forward-60" => Some(icons::FAST_FORWARD_60),
        "fast-forward-outline" => Some(icons::FAST_FORWARD_OUTLINE),
        "fast-forward" => Some(icons::FAST_FORWARD),
        "faucet-variant" => Some(icons::FAUCET_VARIANT),
        "faucet" => Some(icons::FAUCET),
        "fax" => Some(icons::FAX),
        "feather" => Some(icons::FEATHER),
        "feature-search-outline" => Some(icons::FEATURE_SEARCH_OUTLINE),
        "feature-search" => Some(icons::FEATURE_SEARCH),
        _ => None,
    }
}
