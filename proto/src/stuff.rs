use core::fmt;

use crate::util::bitflag_bits;

bitflag_bits! {
    /// derived from https://emanuelecozzi.net/docs/airplay2/features,
    /// https://openairplay.github.io/airplay-spec/features.html and
    /// https://nto.github.io/AirPlay.html
    #[derive(Debug, Clone, Copy)]
    pub struct AirplayFeatures(u64)
    bits: {
        /// video supported
        SUPPORTS_VIDEO_V1: 0,
        SUPPORTS_PHOTO: 1,
        SUPPORTS_VIDEO_FAIR_PLAY: 2,
        SUPPORTS_VIDEO_VOLUME_CONTROL: 3,
        SUPPORTS_VIDEO_HTTP_LIVE_STREAM: 4,
        SUPPORTS_SLIDESHOW: 5,
        SUPPORTS_FROM_CLOUD_0: 6,
        SUPPORTS_SCREEN: 7,
        SUPPORTS_SCREEN_ROTATE: 8,
        SUPPORTS_AUDIO: 9,
        SUPPORTS_AUDIO_REDUNDANT: 11,
        SUPPORTS_FAIRPLAY_SECURE_AUTH: 12, // FPSAPv2pt5_AES_GCM
        SUPPORTS_PHOTO_CACHING: 13,
        AUTHENTICATION_4: 14,
        METADATA_FEATURES_0: 15,
        METADATA_FEATURES_1: 16,
        METADATA_FEATURES_2: 17,
        AUDIO_FORMATS_0: 18,
        AUDIO_FORMATS_1: 19,
        AUDIO_FORMATS_2: 20,
        AUDIO_FORMATS_3: 21,
        AUTHENTICATION_1: 23,
        AUTHENTICATION_8: 26,
        SUPPORTS_LEGACY_PAIRING: 27,
        HAS_UNIFIED_ADVERTISER_INFO: 30, // aka RAOP
        IS_CARPLAY: 32,
        // SUPPORTS_VOLUME = !IS_CARPLAY
        SUPPORTS_VIDEO_QUEUE: 33,
        SUPPORTS_FROM_CLOUD_1: 34, // && SUPPORTS_FROM_CLOUD_0
        SUPPORTS_TLS_PSK: 35,
        SUPPORTS_UNIFIED_MEDIA_CONTROL: 38, // openairplay calls *this* SupportsCoreUtilsPairingAndEncryption. emanuelecozzi calls it unified_media_control.
        SUPPORTS_BUFFERED_AUDIO: 40,
        SUPPORTS_PTP: 41,
        SUPPORTS_SCREEN_MULTI_CODEC: 42,
        SUPPORTS_SYSTEM_PAIRING: 43, // implies bit 48 according to openairplay
        // tf is this
        IS_AP_VALERIA_SCREEN_SENDER: 44,
        SUPPORTS_HK_PAIRING_AND_ACCESS_CONTROL: 46,
        SUPPORTS_CORE_UTILS_PAIRING_AND_ENCRYPTION: 48, // implied by any of 38 || 46 || 43 || 48 according to emanuelecozzi. openairplay calls this SupportsTransientPairing
        SUPPORTS_VIDEO_V2: 49,
        METADATA_FEATURES_3: 50,
        SUPPORTS_UNIFIED_PAIR_SETUP_AND_MFI: 51,
        SUPPORTS_SET_PEERS_EXTENDED_MESSAGE: 52,
        SUPPORTS_AP_SYNC: 54,
        SUPPORTS_WOL_0: 55, // 55 || 56
        SUPPORTS_WOL_1: 56, // 55 || 56
        SUPPORTS_HANGDOG_REMOTE_CONTROL: 58,
        SUPPORTS_AUDIO_STREAM_CONNECTION_SETUP: 59,
        SUPPORTS_AUDIO_MEDIA_DATA_CONTROL: 60,
        SUPPORTS_RFC2198_REDUNDANCY: 61,
    }
}

impl fmt::Display for AirplayFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lo = (self.bits() & u32::MAX as u64) as u32;
        let hi = (self.bits() >> 32) as u32;
        // record format for the features record of an airplay advertisment
        f.write_fmt(format_args!("{lo:#0X},{hi:#0X}"))
    }
}

impl AirplayFeatures {
    pub fn parse(src: &str) -> Option<Self> {
        let mut split = src.split(',');
        let mut read = || u32::from_str_radix(split.next()?.trim_start_matches("0x"), 16).ok();
        let (lo, hi) = (read()?, read()?);
        Some(Self::from_bits_retain(lo as u64 | ((hi as u64) << 32)))
    }
}
