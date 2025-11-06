use serde::{Deserialize, Serialize};

use crate::models::AssetCategory;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompositionStyle {
    Electronic,
    Orchestral,
    Ambient,
    Rock,
}

impl CompositionStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Electronic => "Electronic",
            Self::Orchestral => "Orchestral",
            Self::Ambient => "Ambient",
            Self::Rock => "Rock",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StylePalette {
    pub crypto_primary: CompositionStyle,
    pub crypto_alt: CompositionStyle,
    pub stock_primary: CompositionStyle,
    pub stock_alt: CompositionStyle,
}

impl StylePalette {
    pub fn default() -> Self {
        Self {
            crypto_primary: CompositionStyle::Electronic,
            crypto_alt: CompositionStyle::Ambient,
            stock_primary: CompositionStyle::Orchestral,
            stock_alt: CompositionStyle::Rock,
        }
    }

    pub fn style_for_category(
        &self,
        category: AssetCategory,
        high_volatility: bool,
    ) -> CompositionStyle {
        match category {
            AssetCategory::Crypto => {
                if high_volatility {
                    self.crypto_primary
                } else {
                    self.crypto_alt
                }
            }
            AssetCategory::Stock => {
                if high_volatility {
                    self.stock_alt
                } else {
                    self.stock_primary
                }
            }
        }
    }
}
